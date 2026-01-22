use crate::commands::{if_indextoname, if_nametoindex};
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command, Interface};
use crate::netlink;

use neli::consts::{
    nl::{NlmF, Nlmsg},
    rtnl::{Ifla, IflaInfo, RtAddrFamily, Rtm},
    socket::NlFamily,
};
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::router::asynchronous::NlRouter;
use neli::rtnl::{Ifinfomsg, IfinfomsgBuilder, RtattrBuilder};
use neli::types::{Buffer, RtBuffer};
use neli::utils::Groups;

/// Counts the number of physical or virtual interfaces attached to a BATMAN-adv mesh interface.
///
/// # Arguments
///
/// * `mesh_if` - The name of the mesh interface (e.g., `"bat0"`).
///
/// # Returns
///
/// Returns the number of interfaces currently enslaved to the given mesh interface,
/// or a `RobinError` if the query fails.
///
/// # Example
///
/// ```no_run
/// let count = count_interfaces("bat0").await?;
/// println!("Number of interfaces: {}", count);
/// ```
pub async fn count_interfaces(mesh_if: &str) -> Result<u32, RobinError> {
    let mesh_ifindex = if_nametoindex(mesh_if).await.map_err(|e| {
        RobinError::Netlink(format!("Failed to get ifindex for {}: {:?}", mesh_if, e))
    })?;

    let (rtnl, _) = NlRouter::connect(NlFamily::Route, None, Groups::empty())
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect NlRouter: {:?}", e)))?;

    rtnl.enable_ext_ack(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable ext ack: {:?}", e)))?;
    rtnl.enable_strict_checking(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable strict checking: {:?}", e)))?;

    let ifinfomsg = IfinfomsgBuilder::default()
        .ifi_family(RtAddrFamily::Unspecified)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to create Ifinfomsg: {:?}", e)))?;

    let mut response = rtnl
        .send::<_, _, Rtm, Ifinfomsg>(
            Rtm::Getlink,
            NlmF::DUMP | NlmF::ACK,
            NlPayload::Payload(ifinfomsg),
        )
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send Getlink request: {:?}", e)))?;

    let mut count = 0u32;
    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<Rtm, Ifinfomsg> =
            msg.map_err(|e| RobinError::Netlink(format!("Failed to parse message: {:?}", e)))?;

        if let Some(payload) = msg.get_payload() {
            let attrs = payload.rtattrs().get_attr_handle();
            if let Ok(master) = attrs.get_attr_payload_as::<u32>(Ifla::Master) {
                if master == mesh_ifindex {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

/// Retrieves the list of interfaces associated with a BATMAN-adv mesh interface.
///
/// This corresponds to the `batctl if` command. Each entry contains the interface name
/// and whether it is currently active.
///
/// # Arguments
///
/// * `mesh_if` - The name of the mesh interface.
///
/// # Returns
///
/// Returns a vector of `Interface` structs or a `RobinError` if the query fails.
///
/// # Example
///
/// ```no_run
/// let ifaces = get_interfaces("bat0").await?;
/// for iface in ifaces {
///     println!("Interface {} active: {}", iface.ifname, iface.active);
/// }
/// ```
pub async fn get_interfaces(mesh_if: &str) -> Result<Vec<Interface>, RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let mesh_ifindex = if_nametoindex(mesh_if).await.map_err(|e| {
        RobinError::Netlink(format!("Failed to get ifindex for {:?}: {:?}", mesh_if, e))
    })?;

    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(mesh_ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfindex attribute: {:?}", e))
        })?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetHardif, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    let mut sock = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect socket: {:?}", e)))?;

    let mut response = sock
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    let mut interfaces = Vec::new();
    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

        if *msg.nl_type() == Nlmsg::Done.into() {
            break;
        }

        if *msg.nl_type() == Nlmsg::Error.into() {
            match &msg.nl_payload() {
                NlPayload::Err(err) => {
                    if *err.error() == 0 {
                        break;
                    } else {
                        return Err(RobinError::Netlink(format!(
                            "netlink error {}",
                            err.error()
                        )));
                    }
                }
                _ => {
                    return Err(RobinError::Netlink("unknown netlink error payload".into()));
                }
            }
        }

        let attrs = msg
            .get_payload()
            .ok_or_else(|| RobinError::Parse("Message without payload".into()))?
            .attrs()
            .get_attr_handle();

        let hard_ifindex = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrHardIfindex.into())
            .map_err(|e| RobinError::Parse(format!("Missing HARD_IFINDEX: {:?}", e)))?;

        let ifname = if_indextoname(hard_ifindex).await.map_err(|e| {
            RobinError::Netlink(format!(
                "Failed to resolve ifindex {}: {:?}",
                hard_ifindex, e
            ))
        })?;

        let active = attrs
            .get_attribute(Attribute::BatadvAttrActive.into())
            .is_some();

        interfaces.push(Interface { ifname, active });
    }

    Ok(interfaces)
}

/// Adds or removes a physical interface from a BATMAN-adv mesh interface.
///
/// This corresponds to `batctl if add` or `batctl if del`.
///
/// # Arguments
///
/// * `iface` - The name of the interface to add or remove.
/// * `mesh_if` - Optional mesh interface name to attach to. `None` removes it from any mesh.
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `RobinError` if the operation fails.
///
/// # Example
///
/// ```no_run
/// set_interface("eth0", Some("bat0")).await?;
/// set_interface("eth0", None).await?; // remove from mesh
/// ```
pub async fn set_interface(iface: &str, mesh_if: Option<&str>) -> Result<(), RobinError> {
    let iface_ifindex = if_nametoindex(iface).await.map_err(|e| {
        RobinError::Netlink(format!("Failed to get ifindex for {}: {:?}", iface, e))
    })?;

    let mut mesh_ifindex = 0;
    if mesh_if.is_some() {
        mesh_ifindex = if_nametoindex(mesh_if.unwrap()).await.map_err(|e| {
            RobinError::Netlink(format!("Failed to get ifindex for {:?}: {:?}", mesh_if, e))
        })?;
    }

    let (rtnl, _) = NlRouter::connect(NlFamily::Route, None, Groups::empty())
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect NlRouter: {:?}", e)))?;

    rtnl.enable_ext_ack(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable ext ack: {:?}", e)))?;
    rtnl.enable_strict_checking(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable strict checking: {:?}", e)))?;

    let master_attr = RtattrBuilder::default()
        .rta_type(Ifla::Master)
        .rta_payload(mesh_ifindex)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build Rtattr: {:?}", e)))?;

    let mut rtattrs: RtBuffer<Ifla, Buffer> = RtBuffer::new();
    rtattrs.push(master_attr);

    let msg = IfinfomsgBuilder::default()
        .ifi_family(RtAddrFamily::Unspecified)
        .ifi_index(iface_ifindex.cast_signed())
        .rtattrs(rtattrs)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Ifinfomsg build failed: {:?}", e)))?;

    rtnl.send::<_, _, Rtm, Ifinfomsg>(
        Rtm::Setlink,
        NlmF::REQUEST | NlmF::ACK,
        NlPayload::Payload(msg),
    )
    .await
    .map_err(|e| RobinError::Netlink(format!("Failed to send netlink message: {:?}", e)))?;

    Ok(())
}

/// Creates a new BATMAN-adv mesh interface.
///
/// Optionally, a routing algorithm can be specified. This corresponds to `ip link add type batadv`.
///
/// # Arguments
///
/// * `mesh_if` - The name of the mesh interface to create.
/// * `routing_algo` - Optional routing algorithm name (e.g., `"BATMAN_IV"`).
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `RobinError` if creation fails.
///
/// # Example
///
/// ```no_run
/// create_interface("bat0", Some("BATMAN_IV")).await?;
/// ```
pub async fn create_interface(mesh_if: &str, routing_algo: Option<&str>) -> Result<(), RobinError> {
    const IFLA_BATADV_ALGO_NAME: u16 = 1;
    let (rtnl, _) = NlRouter::connect(NlFamily::Route, None, Groups::empty())
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect NlRouter: {:?}", e)))?;

    rtnl.enable_ext_ack(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable ext ack: {:?}", e)))?;
    rtnl.enable_strict_checking(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable strict checking: {:?}", e)))?;

    let ifname_attr = RtattrBuilder::default()
        .rta_type(Ifla::Ifname)
        .rta_payload(mesh_if)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build IFNAME attr: {:?}", e)))?;

    let kind_attr = RtattrBuilder::default()
        .rta_type(IflaInfo::Kind)
        .rta_payload("batadv")
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build INFO_KIND attr: {:?}", e)))?;

    let mut info_data_attrs: RtBuffer<u16, Buffer> = RtBuffer::new();
    if let Some(algo) = routing_algo {
        let algo_attr = RtattrBuilder::default()
            .rta_type(IFLA_BATADV_ALGO_NAME)
            .rta_payload(algo)
            .build()
            .map_err(|e| RobinError::Netlink(format!("Failed to build ALGO_NAME attr: {:?}", e)))?;
        info_data_attrs.push(algo_attr);
    }

    let info_data_attr = RtattrBuilder::default()
        .rta_type(IflaInfo::Data)
        .rta_payload(info_data_attrs)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build INFO_DATA attr: {:?}", e)))?;

    let mut linkinfo_attrs: RtBuffer<IflaInfo, Buffer> = RtBuffer::new();
    linkinfo_attrs.push(kind_attr);
    linkinfo_attrs.push(info_data_attr);

    let linkinfo_attr = RtattrBuilder::default()
        .rta_type(Ifla::Linkinfo)
        .rta_payload(linkinfo_attrs)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build LINKINFO attr: {:?}", e)))?;

    let mut rtattrs: RtBuffer<Ifla, Buffer> = RtBuffer::new();
    rtattrs.push(ifname_attr);
    rtattrs.push(linkinfo_attr);

    let msg = IfinfomsgBuilder::default()
        .ifi_family(RtAddrFamily::Unspecified)
        .rtattrs(rtattrs)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Ifinfomsg build failed: {:?}", e)))?;

    rtnl.send::<_, _, Rtm, Ifinfomsg>(
        Rtm::Newlink,
        NlmF::REQUEST | NlmF::CREATE | NlmF::EXCL | NlmF::ACK,
        NlPayload::Payload(msg),
    )
    .await
    .map_err(|e| RobinError::Netlink(format!("Failed to create mesh interface: {:?}", e)))?;

    Ok(())
}

/// Destroys an existing BATMAN-adv mesh interface.
///
/// This corresponds to `ip link delete <mesh_if>`.
///
/// # Arguments
///
/// * `mesh_if` - The name of the mesh interface to destroy.
///
/// # Returns
///
/// Returns `Ok(())` on success, or a `RobinError` if destruction fails.
///
/// # Example
///
/// ```no_run
/// destroy_interface("bat0").await?;
/// ```
pub async fn destroy_interface(mesh_if: &str) -> Result<(), RobinError> {
    let (rtnl, _) = NlRouter::connect(NlFamily::Route, None, Groups::empty())
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect NlRouter: {:?}", e)))?;

    rtnl.enable_ext_ack(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable ext ack: {:?}", e)))?;
    rtnl.enable_strict_checking(true)
        .map_err(|e| RobinError::Netlink(format!("Failed to enable strict checking: {:?}", e)))?;

    let ifname_attr = RtattrBuilder::default()
        .rta_type(Ifla::Ifname)
        .rta_payload(mesh_if)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build IFNAME attr: {:?}", e)))?;

    let mut rtattrs: RtBuffer<Ifla, Buffer> = RtBuffer::new();
    rtattrs.push(ifname_attr);

    let msg = IfinfomsgBuilder::default()
        .ifi_family(RtAddrFamily::Unspecified)
        .rtattrs(rtattrs)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Ifinfomsg build failed: {:?}", e)))?;

    rtnl.send::<_, _, Rtm, Ifinfomsg>(
        Rtm::Dellink,
        NlmF::REQUEST | NlmF::ACK,
        NlPayload::Payload(msg),
    )
    .await
    .map_err(|e| RobinError::Netlink(format!("Failed to destroy mesh interface: {:?}", e)))?;

    Ok(())
}
