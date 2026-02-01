use crate::commands::if_nametoindex;
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command};
use crate::netlink;

use neli::consts::nl::NlmF;
use neli::genl::Genlmsghdr;
use neli::nl::Nlmsghdr;

/// Retrieves the current state of the Aggregated OGMs (Originator Messages) setting for a BATMAN-adv mesh interface.
///
/// # Arguments
///
/// * `mesh_if` - The name of the BATMAN-adv mesh interface (e.g., "bat0").
///
/// # Returns
///
/// Returns `Ok(true)` if Aggregated OGMs are enabled, `Ok(false)` if disabled,
/// or a `RobinError` if the value could not be retrieved.
pub async fn get_aggregation(mesh_if: &str) -> Result<bool, RobinError> {
    let ifindex = if_nametoindex(mesh_if).await.map_err(|_| {
        RobinError::Netlink(format!(
            "Error - interface '{}' is not present or not a batman-adv interface",
            mesh_if
        ))
    })?;

    let mut attrs = netlink::GenlAttrBuilder::new();
    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|_| {
            RobinError::Netlink("Error - could not set mesh interface index".to_string())
        })?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetMeshInfo, attrs.build())
        .map_err(|_| RobinError::Netlink("Error - failed to build netlink message".to_string()))?;

    let mut sock = netlink::BatadvSocket::connect().await.map_err(|_| {
        RobinError::Netlink("Error - failed to connect to batman-adv netlink socket".to_string())
    })?;

    let mut response = sock
        .send(NlmF::REQUEST, msg)
        .await
        .map_err(|_| RobinError::Netlink("Error - failed to send netlink request".to_string()))?;

    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> = msg.map_err(|_| {
            RobinError::Netlink("Error - failed to parse netlink response".to_string())
        })?;

        let payload = match msg.get_payload() {
            Some(p) => p,
            None => continue,
        };

        for attr in payload.attrs().iter() {
            if *attr.nla_type().nla_type() == Attribute::BatadvAttrAggregatedOgmsEnabled.into() {
                let bytes = attr.nla_payload().as_ref();
                if let Some(&val) = bytes.first() {
                    return Ok(val != 0);
                }
            }
        }
    }

    Err(RobinError::NotFound(
        "Error - Aggregated OGMs attribute not found".to_string(),
    ))
}

/// Enables or disables the Aggregated OGMs (Originator Messages) setting for a BATMAN-adv mesh interface.
///
/// # Arguments
///
/// * `mesh_if` - The name of the BATMAN-adv mesh interface (e.g., "bat0").
/// * `enabled` - `true` to enable Aggregated OGMs, `false` to disable.
///
/// # Returns
///
/// Returns `Ok(())` if the operation succeeds, or a `RobinError` if it fails.
pub async fn set_aggregation(mesh_if: &str, enabled: bool) -> Result<(), RobinError> {
    let ifindex = if_nametoindex(mesh_if).await.map_err(|_| {
        RobinError::Netlink(format!(
            "Error - interface '{}' is not present or not a batman-adv interface",
            mesh_if
        ))
    })?;

    let mut attrs = netlink::GenlAttrBuilder::new();
    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|_| {
            RobinError::Netlink("Error - could not set mesh interface index".to_string())
        })?;

    attrs
        .add(
            Attribute::BatadvAttrAggregatedOgmsEnabled,
            AttrValueForSend::U8(enabled as u8),
        )
        .map_err(|_| {
            RobinError::Netlink("Error - could not set Aggregated OGMs attribute".to_string())
        })?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdSetMesh, attrs.build())
        .map_err(|_| RobinError::Netlink("Error - failed to build netlink message".to_string()))?;

    let mut sock = netlink::BatadvSocket::connect().await.map_err(|_| {
        RobinError::Netlink("Error - failed to connect to batman-adv netlink socket".to_string())
    })?;

    sock.send(NlmF::REQUEST | NlmF::ACK, msg)
        .await
        .map_err(|_| RobinError::Netlink("Error - failed to send netlink request".to_string()))?;

    Ok(())
}
