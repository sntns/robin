use crate::commands::if_nametoindex;
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command};
use crate::netlink;

use neli::consts::nl::NlmF;
use neli::genl::Genlmsghdr;
use neli::nl::Nlmsghdr;

/// Retrieves the current state of bridge loop avoidance for a BATMAN-adv mesh interface.
///
/// # Arguments
///
/// * `mesh_if` - The name of the BATMAN-adv mesh interface (e.g., "bat0").
///
/// # Returns
///
/// Returns `Ok(true)` if bridge loop avoidance is enabled, `Ok(false)` if disabled,
/// or a `RobinError` if the value could not be retrieved.
pub async fn get_bridge_loop_avoidance(mesh_if: &str) -> Result<bool, RobinError> {
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
            if *attr.nla_type().nla_type() == Attribute::BatadvAttrBridgeLoopAvoidanceEnabled.into()
            {
                let bytes = attr.nla_payload().as_ref();
                if let Some(&val) = bytes.get(0) {
                    return Ok(val != 0);
                }
            }
        }
    }

    Err(RobinError::NotFound(
        "Bridge loop avoidance attribute not found".to_string(),
    ))
}

/// Enables or disables bridge loop avoidance for a BATMAN-adv mesh interface.
///
/// # Arguments
///
/// * `mesh_if` - The name of the BATMAN-adv mesh interface (e.g., "bat0").
/// * `enabled` - `true` to enable bridge loop avoidance, `false` to disable.
///
/// # Returns
///
/// Returns `Ok(())` if the operation succeeds, or a `RobinError` if it fails.
pub async fn set_bridge_loop_avoidance(mesh_if: &str, enabled: bool) -> Result<(), RobinError> {
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
            Attribute::BatadvAttrBridgeLoopAvoidanceEnabled,
            AttrValueForSend::U8(enabled as u8),
        )
        .map_err(|e| {
            RobinError::Netlink(format!(
                "Failed to add BRIDGE_LOOP_AVOIDANCE attribute: {:?}",
                e
            ))
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
