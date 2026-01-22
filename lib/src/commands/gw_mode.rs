use crate::commands::if_nametoindex;
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command, GatewayInfo, GwMode};
use crate::netlink;

use neli::consts::nl::NlmF;
use neli::genl::Genlmsghdr;
use neli::nl::Nlmsghdr;

/// Retrieves the current gateway settings for a BATMAN-adv mesh interface.
///
/// This includes the gateway mode, selection class, configured upstream/downstream
/// bandwidth, and the routing algorithm used.
///
/// # Arguments
///
/// * `mesh_if` - The name of the BATMAN-adv mesh interface (e.g., "bat0").
///
/// # Returns
///
/// Returns a `GatewayInfo` struct containing the mode, selection class, bandwidths,
/// and routing algorithm, or a `RobinError` if the information could not be retrieved.
pub async fn get_gateway(mesh_if: &str) -> Result<GatewayInfo, RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = if_nametoindex(mesh_if).await.map_err(|_| {
        RobinError::Netlink(format!(
            "Error - interface '{}' is not present or not a batman-adv interface",
            mesh_if
        ))
    })?;

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

    let mut socket = netlink::BatadvSocket::connect().await.map_err(|_| {
        RobinError::Netlink("Error - failed to connect to batman-adv netlink socket".to_string())
    })?;

    let mut response = socket
        .send(NlmF::REQUEST, msg)
        .await
        .map_err(|_| RobinError::Netlink("Error - failed to send netlink request".to_string()))?;

    let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> = response
        .next()
        .await
        .ok_or_else(|| RobinError::Parse("Error - no response from kernel".into()))?
        .map_err(|_| RobinError::Netlink("Error - failed to parse netlink response".to_string()))?;

    let attrs = msg
        .get_payload()
        .ok_or_else(|| RobinError::Parse("Error - message has no payload".into()))?
        .attrs()
        .get_attr_handle();

    let mode = match attrs.get_attr_payload_as::<u8>(Attribute::BatadvAttrGwMode.into()) {
        Ok(0) => GwMode::Off,
        Ok(1) => GwMode::Client,
        Ok(2) => GwMode::Server,
        Ok(_) => GwMode::Unknown,
        Err(_) => GwMode::Unknown,
    };

    let sel_class = attrs
        .get_attr_payload_as::<u32>(Attribute::BatadvAttrGwSelClass.into())
        .map_err(|_| RobinError::Parse("Error - gateway selection class missing".into()))?;

    let bandwidth_down = attrs
        .get_attr_payload_as::<u32>(Attribute::BatadvAttrGwBandwidthDown.into())
        .map_err(|_| RobinError::Parse("Error - gateway downstream bandwidth missing".into()))?;

    let bandwidth_up = attrs
        .get_attr_payload_as::<u32>(Attribute::BatadvAttrGwBandwidthUp.into())
        .map_err(|_| RobinError::Parse("Error - gateway upstream bandwidth missing".into()))?;

    let algo = attrs
        .get_attr_payload_as_with_len::<Vec<u8>>(Attribute::BatadvAttrAlgoName.into())
        .map(|bytes| {
            let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            String::from_utf8_lossy(&bytes[..nul]).into_owned()
        })
        .map_err(|_| RobinError::Parse("Error - routing algorithm name missing".into()))?;

    Ok(GatewayInfo {
        mode,
        sel_class,
        bandwidth_down,
        bandwidth_up,
        algo,
    })
}

/// Configures the gateway settings for a BATMAN-adv mesh interface.
///
/// This function allows setting the gateway mode (Off, Client, or Server) and optionally
/// the upstream/downstream bandwidth and selection class when in Server mode.
///
/// # Arguments
///
/// * `mode` - The gateway mode to set (`GwMode::Off`, `GwMode::Client`, `GwMode::Server`).
/// * `down` - Optional downstream bandwidth in Mbps (used when mode is Server).
/// * `up` - Optional upstream bandwidth in Mbps (used when mode is Server).
/// * `sel_class` - Optional selection class (used when mode is Server).
/// * `mesh_if` - The name of the BATMAN-adv mesh interface (e.g., "bat0").
///
/// # Returns
///
/// Returns `Ok(())` if the settings were applied successfully, or a `RobinError` if
/// the operation failed or was rejected by the kernel.
pub async fn set_gateway(
    mode: GwMode,
    down: Option<u32>,
    up: Option<u32>,
    sel_class: Option<u32>,
    mesh_if: &str,
) -> Result<(), RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = if_nametoindex(mesh_if).await.map_err(|_| {
        RobinError::Netlink(format!(
            "Error - interface '{}' is not present or not a batman-adv interface",
            mesh_if
        ))
    })?;

    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|_| {
            RobinError::Netlink("Error - could not set mesh interface index".to_string())
        })?;

    match mode {
        GwMode::Off => {
            attrs
                .add(Attribute::BatadvAttrGwMode, AttrValueForSend::U8(0))
                .map_err(|_| {
                    RobinError::Netlink("Error - could not set gateway mode to OFF".to_string())
                })?;
        }

        GwMode::Client => {
            attrs
                .add(Attribute::BatadvAttrGwMode, AttrValueForSend::U8(1))
                .map_err(|_| {
                    RobinError::Netlink("Error - could not set gateway mode to CLIENT".to_string())
                })?;
        }

        GwMode::Server => {
            attrs
                .add(Attribute::BatadvAttrGwMode, AttrValueForSend::U8(2))
                .map_err(|_| {
                    RobinError::Netlink("Error - could not set gateway mode to SERVER".to_string())
                })?;

            attrs
                .add(
                    Attribute::BatadvAttrGwBandwidthDown,
                    AttrValueForSend::U32(down.unwrap_or(10000) / 100),
                )
                .map_err(|_| {
                    RobinError::Netlink(
                        "Error - could not set gateway downstream bandwidth".to_string(),
                    )
                })?;

            attrs
                .add(
                    Attribute::BatadvAttrGwBandwidthUp,
                    AttrValueForSend::U32(up.unwrap_or(2000) / 100),
                )
                .map_err(|_| {
                    RobinError::Netlink(
                        "Error - could not set gateway upstream bandwidth".to_string(),
                    )
                })?;

            attrs
                .add(
                    Attribute::BatadvAttrGwSelClass,
                    AttrValueForSend::U32(sel_class.unwrap_or(0)),
                )
                .map_err(|_| {
                    RobinError::Netlink("Error - could not set gateway selection class".to_string())
                })?;
        }

        GwMode::Unknown => {
            return Err(RobinError::Parse(
                "Cannot set unknown gateway mode".to_string(),
            ));
        }
    }

    let msg = netlink::build_genl_msg(Command::BatadvCmdSetMesh, attrs.build())
        .map_err(|_| RobinError::Netlink("Error - failed to build netlink message".to_string()))?;

    let mut socket = netlink::BatadvSocket::connect().await.map_err(|_| {
        RobinError::Netlink("Error - failed to connect to batman-adv netlink socket".to_string())
    })?;

    socket
        .send(NlmF::REQUEST | NlmF::ACK, msg)
        .await
        .map_err(|_| RobinError::Netlink("Error - failed to send netlink request".to_string()))?;

    Ok(())
}
