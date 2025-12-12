use crate::error::RobinError;
use crate::model::{GatewayInfo, GwMode};
use crate::netlink;
use neli::consts::nl::{NlmF, Nlmsg};
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};

/// Get gateway (batctl gw)
pub async fn get_gateway() -> Result<GatewayInfo, RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = netlink::ifname_to_index("bat0")
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get Ifindex: {:?}", e)))?;

    attrs
        .add(
            netlink::Attribute::BatadvAttrMeshIfindex,
            netlink::AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfIndex attribute: {:?}", e))
        })?;

    let msg = netlink::build_genl_msg(netlink::Command::BatadvCmdGetMesh, attrs.build())?;

    let mut socket = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    let mut response = socket
        .send(NlmF::REQUEST, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> = response
        .next()
        .await
        .ok_or_else(|| RobinError::Parse("No response from kernel".into()))?
        .map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

    let attrs = msg
        .get_payload()
        .ok_or_else(|| RobinError::Parse("Message without payload".into()))?
        .attrs()
        .get_attr_handle();

    let mode = match attrs.get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrGwMode.into()) {
        Ok(0) => GwMode::Off,
        Ok(1) => GwMode::Client,
        Ok(2) => GwMode::Server,
        Ok(x) => GwMode::Unknown(x),
        Err(_) => GwMode::Unknown(255),
    };

    let sel_class = attrs
        .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrGwSelClass.into())
        .map_err(|e| RobinError::Parse(format!("Missing GW_SEL_CLASS: {:?}", e)))?;

    let bandwidth_down = attrs
        .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrGwBandwidthDown.into())
        .map_err(|e| RobinError::Parse(format!("Missing GW_BANDWIDTH_DOWN: {:?}", e)))?;

    let bandwidth_up = attrs
        .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrGwBandwidthUp.into())
        .map_err(|e| RobinError::Parse(format!("Missing GW_BANDWIDTH_UP: {:?}", e)))?;

    let algo = attrs
        .get_attr_payload_as::<[u8; 32]>(netlink::Attribute::BatadvAttrAlgoName.into())
        .ok()
        .map(|bytes| {
            let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            String::from_utf8_lossy(&bytes[..nul_pos]).into_owned()
        });

    Ok(GatewayInfo {
        mode,
        sel_class,
        bandwidth_down,
        bandwidth_up,
        algo,
    })
}

/// Set gateway (batctl gw [mode] [sel_class|bandwidth])
pub async fn set_gateway(
    mode: GwMode,
    down: Option<u32>,
    up: Option<u32>,
    sel_class: Option<u32>,
) -> Result<(), RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = netlink::ifname_to_index("bat0")
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get Ifindex: {:?}", e)))?;

    attrs
        .add(
            netlink::Attribute::BatadvAttrMeshIfindex,
            netlink::AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfIndex attribute: {:?}", e))
        })?;

    match mode {
        GwMode::Off => {
            attrs
                .add(
                    netlink::Attribute::BatadvAttrGwMode,
                    netlink::AttrValueForSend::U8(0),
                )
                .map_err(|e| RobinError::Netlink(format!("Failed to add GwMode: {:?}", e)))?;
        }

        GwMode::Client => {
            attrs
                .add(
                    netlink::Attribute::BatadvAttrGwMode,
                    netlink::AttrValueForSend::U8(1),
                )
                .map_err(|e| RobinError::Netlink(format!("Failed to add GwMode: {:?}", e)))?;
        }

        GwMode::Server => {
            attrs
                .add(
                    netlink::Attribute::BatadvAttrGwMode,
                    netlink::AttrValueForSend::U8(2),
                )
                .map_err(|e| RobinError::Netlink(format!("Failed to add GwMode: {:?}", e)))?;

            attrs
                .add(
                    netlink::Attribute::BatadvAttrGwBandwidthDown,
                    netlink::AttrValueForSend::U32(down.unwrap_or(10)),
                )
                .map_err(|e| {
                    RobinError::Netlink(format!("Failed to add GwBandwidthDown: {:?}", e))
                })?;

            attrs
                .add(
                    netlink::Attribute::BatadvAttrGwBandwidthUp,
                    netlink::AttrValueForSend::U32(up.unwrap_or(2)),
                )
                .map_err(|e| {
                    RobinError::Netlink(format!("Failed to add GwBandwidthUp: {:?}", e))
                })?;

            attrs
                .add(
                    netlink::Attribute::BatadvAttrGwSelClass,
                    netlink::AttrValueForSend::U32(sel_class.unwrap_or(0)),
                )
                .map_err(|e| RobinError::Netlink(format!("Failed to add GwSelClass: {:?}", e)))?;
        }

        GwMode::Unknown(x) => {
            return Err(RobinError::Parse(format!(
                "Cannot set unknown gateway mode {}",
                x
            )));
        }
    }

    let msg = netlink::build_genl_msg(netlink::Command::BatadvCmdSetMesh, attrs.build())?;

    let mut socket = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    let mut response = socket
        .send(NlmF::REQUEST | NlmF::ACK, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

        if *msg.nl_type() == Nlmsg::Error.into() {
            match msg.nl_payload() {
                NlPayload::Err(err) => {
                    return if *err.error() == 0 {
                        Ok(())
                    } else {
                        Err(RobinError::Netlink(format!(
                            "kernel rejected set-gw-mode: {}",
                            err.error()
                        )))
                    };
                }
                _ => {}
            }
        }
    }

    Err(RobinError::Netlink("SetGwMode: no ACK from kernel".into()))
}
