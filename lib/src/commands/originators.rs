use crate::error::RobinError;
use crate::model::Originator;
use crate::netlink;
use macaddr::MacAddr6;
use neli::consts::nl::NlmF;
use neli::consts::nl::Nlmsg;
use neli::genl::Genlmsghdr;
use neli::nl::NlPayload;
use neli::nl::Nlmsghdr;

/// Originators (batctl o)
pub async fn get_originators() -> Result<Vec<Originator>, RobinError> {
    // Create the value and the attribute
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

    // Build the message
    let msg = netlink::build_genl_msg(netlink::Command::BatadvCmdGetOriginators, attrs.build())
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
            .get_attr_payload_as::<[u8; 6]>(netlink::Attribute::BatadvAttrOrigAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing ORIG_ADDRESS: {:?}", e)))?;

        let neigh = attrs
            .get_attr_payload_as::<[u8; 6]>(netlink::Attribute::BatadvAttrNeighAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing NEIGH_ADDRESS: {:?}", e)))?;

        // HARD_IFNAME or fallback to HARD_IFINDEX
        let ifname = match attrs
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

        // last_seen is mandatory
        let last_seen_ms = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrLastSeenMsecs.into())
            .map_err(|e| RobinError::Parse(format!("Missing LAST_SEEN: {:?}", e)))?;

        // Optional attributes
        let tq = attrs
            .get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrTq.into())
            .ok();

        let tp = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrThroughput.into())
            .ok();

        // BEST flag ("*")
        let is_best =
            match attrs.get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrRouter.into()) {
                Ok(_) => true,
                Err(_) => false,
            };

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

/// Utils function to get algo name
pub async fn get_algo_name() -> Result<String, RobinError> {
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

    let msg = netlink::build_genl_msg(netlink::Command::BatadvCmdGetOriginators, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    let mut socket = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    let mut response = socket
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|e| RobinError::Netlink(format!("Netlink message error: {:?}", e)))?;

        if let Some(payload) = msg.get_payload() {
            let attrs = payload.attrs().get_attr_handle();

            if let Ok(bytes) =
                attrs.get_attr_payload_as::<[u8; 32]>(netlink::Attribute::BatadvAttrAlgoName.into())
            {
                let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                let name = String::from_utf8_lossy(&bytes[..nul_pos]).to_string();
                return Ok(name);
            }
        }
    }

    Err(RobinError::NotFound(
        "Algorithm name not found for interface bat0".into(),
    ))
}
