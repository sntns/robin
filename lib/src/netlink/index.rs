use crate::error::RobinError;
use neli::consts::{
    nl::NlmF,
    rtnl::{Ifla, RtAddrFamily, Rtm},
    socket::NlFamily,
};
use neli::nl::{NlPayload, Nlmsghdr};
use neli::router::asynchronous::NlRouter;
use neli::rtnl::{Ifinfomsg, IfinfomsgBuilder};
use neli::utils::Groups;

pub async fn ifname_to_index(ifname: &str) -> Result<u32, RobinError> {
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

pub async fn ifindex_to_name(ifindex: u32) -> Result<String, RobinError> {
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
