use crate::commands::if_nametoindex;
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command};
use crate::netlink;

use neli::consts::nl::NlmF;
use neli::genl::Genlmsghdr;
use neli::nl::Nlmsghdr;

pub async fn get_aggregation(mesh_if: &str) -> Result<bool, RobinError> {
    let ifindex = if_nametoindex(mesh_if)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get ifindex: {:?}", e)))?;

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
            if *attr.nla_type().nla_type() == Attribute::BatadvAttrAggregatedOgmsEnabled.into() {
                let bytes = attr.nla_payload().as_ref();
                if let Some(&val) = bytes.get(0) {
                    return Ok(val != 0);
                }
            }
        }
    }

    Err(RobinError::NotFound(
        "Aggregated OGMs attribute not found".to_string(),
    ))
}

pub async fn set_aggregation(mesh_if: &str, enabled: bool) -> Result<(), RobinError> {
    let ifindex = if_nametoindex(mesh_if)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get ifindex: {:?}", e)))?;

    let mut attrs = netlink::GenlAttrBuilder::new();
    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfIndex attribute: {:?}", e))
        })?;

    attrs
        .add(
            Attribute::BatadvAttrAggregatedOgmsEnabled,
            AttrValueForSend::U8(enabled as u8),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add AGGREGATED_OGMS attribute: {:?}", e))
        })?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdSetMesh, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    let mut sock = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    sock.send(NlmF::REQUEST | NlmF::ACK, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    Ok(())
}
