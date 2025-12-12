use crate::error::RobinError;
use crate::model::Neighbor;
use crate::netlink;
use macaddr::MacAddr6;
use neli::consts::nl::{NlmF, Nlmsg};
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};

/// Neighbors (batctl n)
pub async fn get_neighbors() -> Result<Vec<Neighbor>, RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = netlink::ifname_to_index("bat0")
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get ifindex for bat0: {:?}", e)))?;

    attrs
        .add(
            netlink::Attribute::BatadvAttrMeshIfindex,
            netlink::AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfindex attribute: {:?}", e))
        })?;

    let msg = netlink::build_genl_msg(netlink::Command::BatadvCmdGetOriginators, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    let mut sock = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect socket: {:?}", e)))?;

    let mut response = sock
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    let mut neighbors: Vec<Neighbor> = Vec::new();
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

        let attrs = msg
            .get_payload()
            .ok_or_else(|| RobinError::Parse("Message without payload".into()))?
            .attrs()
            .get_attr_handle();

        let neigh_addr = attrs
            .get_attr_payload_as::<[u8; 6]>(netlink::Attribute::BatadvAttrNeighAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing NEIGH_ADDRESS: {:?}", e)))?;

        let last_seen_ms = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrLastSeenMsecs.into())
            .map_err(|e| RobinError::Parse(format!("Missing LAST_SEEN_MSECS: {:?}", e)))?;

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
                    RobinError::Netlink(format!("Failed to resolve ifindex -> name: {:?}", e))
                })?
            }
        };

        let throughput_kbps = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrThroughput.into())
            .ok();

        // push entry
        neighbors.push(Neighbor {
            neigh: MacAddr6::from(neigh_addr),
            outgoing_if,
            last_seen_ms,
            throughput_kbps,
        });
    }

    Ok(neighbors)
}
