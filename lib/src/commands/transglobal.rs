use crate::error::RobinError;
use crate::model::TransglobalEntry;
use crate::netlink;
use macaddr::MacAddr6;
use neli::consts::nl::NlmF;
use neli::consts::nl::Nlmsg;
use neli::genl::Genlmsghdr;
use neli::nl::NlPayload;
use neli::nl::Nlmsghdr;

pub async fn get_transglobal() -> Result<Vec<TransglobalEntry>, RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = netlink::ifname_to_index("bat0")
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get Ifindex: {:?}", e)))?;

    attrs
        .add(
            netlink::Attribute::BatadvAttrMeshIfindex,
            netlink::AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| RobinError::Netlink(format!("Failed to add MeshIfIndex: {:?}", e)))?;

    let msg = netlink::build_genl_msg(
        netlink::Command::BatadvCmdGetTranstableGlobal,
        attrs.build(),
    )
    .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    // Send
    let mut sock = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    let mut response = sock
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    let mut entries = Vec::new();
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

        let client = attrs
            .get_attr_payload_as::<[u8; 6]>(netlink::Attribute::BatadvAttrTtAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_ADDRESS: {:?}", e)))?;
        let orig = attrs
            .get_attr_payload_as::<[u8; 6]>(netlink::Attribute::BatadvAttrOrigAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing ORIG_ADDRESS: {:?}", e)))?;
        let vid = attrs
            .get_attr_payload_as::<u16>(netlink::Attribute::BatadvAttrTtVid.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_VID: {:?}", e)))?;
        let ttvn = attrs
            .get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrTtTtvn.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_TTVN: {:?}", e)))?;
        let last_ttvn = attrs
            .get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrTtLastTtvn.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_LAST_TTVN: {:?}", e)))?;
        let crc32 = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrTtCrc32.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_CRC32: {:?}", e)))?;
        let flags = attrs
            .get_attr_payload_as::<u32>(netlink::Attribute::BatadvAttrTtFlags.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_FLAGS: {:?}", e)))?;
        let is_best =
            match attrs.get_attr_payload_as::<u8>(netlink::Attribute::BatadvAttrRouter.into()) {
                Ok(_) => true,
                Err(_) => false,
            };

        entries.push(TransglobalEntry {
            client: MacAddr6::from(client),
            orig: MacAddr6::from(orig),
            vid,
            ttvn,
            last_ttvn,
            flags,
            crc32,
            is_best,
        });
    }

    Ok(entries)
}
