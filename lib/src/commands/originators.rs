use crate::commands::{if_indextoname, if_nametoindex};
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command, Originator};
use crate::netlink;

use macaddr::MacAddr6;
use neli::consts::nl::NlmF;
use neli::consts::nl::Nlmsg;
use neli::genl::Genlmsghdr;
use neli::nl::NlPayload;
use neli::nl::Nlmsghdr;

/// Retrieves the list of originators for a BATMAN-adv mesh interface.
///
/// This corresponds to the `batctl o` command. Each originator entry includes
/// the originator's MAC address, the next-hop neighbor MAC, the outgoing interface,
/// the last seen timestamp in milliseconds, optional TQ (link quality) and throughput,
/// and a flag indicating if this originator is currently the best route.
///
/// # Arguments
///
/// * `mesh_if` - The name of the mesh interface (e.g., `"bat0"`).
///
/// # Returns
///
/// Returns a vector of `Originator` structs or a `RobinError` if the query fails.
///
/// # Example
///
/// ```no_run
/// let originators = get_originators("bat0").await?;
/// for o in originators {
///     println!(
///         "Originator {} via {} (last seen {} ms, best: {})",
///         o.originator, o.outgoing_if, o.last_seen_ms, o.is_best
///     );
/// }
/// ```
pub async fn get_originators(mesh_if: &str) -> Result<Vec<Originator>, RobinError> {
    // Create the value and the attribute
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = if_nametoindex(mesh_if)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get Ifindex: {:?}", e)))?;

    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfIndex attribute: {:?}", e))
        })?;

    // Build the message
    let msg = netlink::build_genl_msg(Command::BatadvCmdGetOriginators, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    // Connect to socket
    let mut socket = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    // Send message and get response
    let mut response = socket
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    // Parse response
    let mut originators: Vec<Originator> = Vec::new();
    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

        if *msg.nl_type() == Nlmsg::Done.into() {
            // End of message
            break;
        }

        if *msg.nl_type() == Nlmsg::Error.into() {
            match &msg.nl_payload() {
                NlPayload::Err(err) => {
                    if *err.error() == 0 {
                        // Not a real error, indicates end of dump
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

        // handle of all top-level attributes
        let attrs = msg
            .get_payload()
            .ok_or_else(|| RobinError::Parse("Message without payload".into()))?
            .attrs()
            .get_attr_handle();

        // Required attributes (batctl fails hard if missing)
        let orig = attrs
            .get_attr_payload_as::<[u8; 6]>(Attribute::BatadvAttrOrigAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing ORIG_ADDRESS: {:?}", e)))?;

        let neigh = attrs
            .get_attr_payload_as::<[u8; 6]>(Attribute::BatadvAttrNeighAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing NEIGH_ADDRESS: {:?}", e)))?;

        // HARD_IFNAME or fallback to HARD_IFINDEX
        let ifname =
            match attrs.get_attr_payload_as::<[u8; 16]>(Attribute::BatadvAttrHardIfname.into()) {
                Ok(bytes) => {
                    let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                    String::from_utf8_lossy(&bytes[..nul_pos]).into_owned()
                }
                Err(_) => {
                    let ifindex = attrs
                        .get_attr_payload_as::<u32>(Attribute::BatadvAttrHardIfindex.into())
                        .map_err(|e| RobinError::Parse(format!("Missing HARD_IFINDEX: {:?}", e)))?;
                    if_indextoname(ifindex).await.map_err(|e| {
                        RobinError::Netlink(format!("Failed to get ifname from ifindex: {:?}", e))
                    })?
                }
            };

        // last_seen is mandatory
        let last_seen_ms = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrLastSeenMsecs.into())
            .map_err(|e| RobinError::Parse(format!("Missing LAST_SEEN: {:?}", e)))?;

        // Optional attributes
        let tq = attrs
            .get_attr_payload_as::<u8>(Attribute::BatadvAttrTq.into())
            .ok();

        let tp = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrThroughput.into())
            .ok();

        // BEST flag ("*")
        let is_best = attrs
            .get_attribute(Attribute::BatadvAttrFlagBest.into())
            .is_some();

        originators.push(Originator {
            originator: MacAddr6::from(orig),
            next_hop: MacAddr6::from(neigh),
            outgoing_if: ifname,
            last_seen_ms,
            tq,
            throughput: tp,
            is_best,
        });
    }

    // socket drops automatically with end of scope

    Ok(originators)
}
