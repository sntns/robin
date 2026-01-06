use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command};
use crate::netlink;
use neli::consts::nl::NlmF;
use neli::consts::rtnl::{Ifla, RtAddrFamily, Rtm};
use neli::consts::socket::NlFamily;
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::router::asynchronous::NlRouter;
use neli::rtnl::{Ifinfomsg, IfinfomsgBuilder};
use neli::utils::Groups;

/// Utils function to get algo name
pub async fn get_algo_name(mesh_if: &str) -> Result<String, RobinError> {
    let ifindex = if_nametoindex(mesh_if)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get Ifindex: {:?}", e)))?;

    let mut attrs = netlink::GenlAttrBuilder::new();
    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfIndex attribute: {:?}", e))
        })?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetMeshInfo, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    let mut sock = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    let mut response = sock
        .send(NlmF::REQUEST, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

        let payload = match msg.get_payload() {
            Some(p) => p,
            None => continue,
        };

        for attr in payload.attrs().iter() {
            if *attr.nla_type().nla_type() == Attribute::BatadvAttrAlgoName.into() {
                let bytes = attr.nla_payload().as_ref();
                let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                let algo = String::from_utf8_lossy(&bytes[..nul]).to_string();
                return Ok(algo);
            }
        }
    }

    Err(RobinError::NotFound(
        "Algorithm name not found for interface bat0".to_string(),
    ))
}

pub async fn if_nametoindex(ifname: &str) -> Result<u32, RobinError> {
    let (rtnl, _) = NlRouter::connect(NlFamily::Route, None, Groups::empty())
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect with NlRouter: {:?}", e)))?;

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
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<Rtm, Ifinfomsg> =
            msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

        if let Some(payload) = msg.get_payload() {
            let attrs = payload.rtattrs().get_attr_handle();
            if let Ok(name) = attrs.get_attr_payload_as_with_len::<String>(Ifla::Ifname) {
                if name == ifname {
                    return Ok(payload.ifi_index().cast_unsigned());
                }
            }
        }
    }

    Err(RobinError::NotFound(format!(
        "Interface {} not found",
        ifname
    )))
}

pub async fn if_indextoname(ifindex: u32) -> Result<String, RobinError> {
    let (rtnl, _) = NlRouter::connect(NlFamily::Route, None, Groups::empty())
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect with NlRouter: {:?}", e)))?;

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
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<Rtm, Ifinfomsg> =
            msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

        if let Some(payload) = msg.get_payload() {
            if *payload.ifi_index() == ifindex.cast_signed() {
                let attrs = payload.rtattrs().get_attr_handle();
                if let Ok(name) = attrs.get_attr_payload_as_with_len::<String>(Ifla::Ifname) {
                    return Ok(name);
                }
            }
        }
    }

    Err(RobinError::NotFound(format!(
        "Interface with index {} not found",
        ifindex
    )))
}
