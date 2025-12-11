use crate::error::RobinError;
use crate::model::{Gateway, GatewayInfo, GwMode};
use crate::netlink;
use macaddr::MacAddr6;
use neli::consts::nl::{NlmF, Nlmsg};
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};

/// Gateways list (batctl gwl)
pub async fn get_gateways_list() -> Result<Vec<Gateway>, RobinError> {
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

    let msg = netlink::build_genl_msg(netlink::Command::BatadvCmdGetGateways, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    let mut socket = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    let mut response = socket
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    let mut gateways = Vec::new();

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

        let is_best = attrs
            .get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrFlagBest.into())
            .map(|_| true)
            .unwrap_or(false);

        let mac_addr = attrs
            .get_attr_payload_as::<[u8; 6]>(netlink::Attribute::BatadvAttrOrigAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing ORIG_ADDRESS: {:?}", e)))?;

        let router = attrs
            .get_attr_payload_as::<[u8; 6]>(netlink::Attribute::BatadvAttrRouter.into())
            .map_err(|e| RobinError::Parse(format!("Missing ROUTER: {:?}", e)))?;

        let outgoing_if = match attrs
            .get_attr_payload_as::<[u8; 16]>(netlink::Attribute::BatadvAttrHardIfname.into())
        {
            Ok(bytes) => {
                let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                String::from_utf8_lossy(&bytes[..nul_pos]).into_owned()
            }
            Err(_) => {
                let ifindex = attrs
                    .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrHardIfindex.into())
                    .map_err(|e| RobinError::Parse(format!("Missing HARD_IFINDEX: {:?}", e)))?;
                netlink::ifindex_to_name(ifindex).await.map_err(|e| {
                    RobinError::Netlink(format!("Failed to get ifname from ifindex: {:?}", e))
                })?
            }
        };

        let bandwidth_down = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrBandwidthDown.into())
            .ok();
        let bandwidth_up = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrBandwidthUp.into())
            .ok();
        let throughput = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrThroughput.into())
            .ok();
        let tq = attrs
            .get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrTq.into())
            .ok();

        gateways.push(Gateway {
            mac_addr: MacAddr6::from(mac_addr),
            router: MacAddr6::from(router),
            outgoing_if,
            bandwidth_down,
            bandwidth_up,
            throughput,
            tq,
            is_best,
        });
    }

    Ok(gateways)
}

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
