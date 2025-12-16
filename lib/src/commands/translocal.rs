use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, ClientFlags, Command, TranslocalEntry};
use crate::netlink;
use macaddr::MacAddr6;
use neli::consts::nl::NlmF;
use neli::consts::nl::Nlmsg;
use neli::genl::Genlmsghdr;
use neli::nl::NlPayload;
use neli::nl::Nlmsghdr;

pub async fn get_translocal() -> Result<Vec<TranslocalEntry>, RobinError> {
    let mut attrs = netlink::GenlAttrBuilder::new();
    let ifindex = netlink::ifname_to_index("bat0")
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get Ifindex: {:?}", e)))?;

    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| RobinError::Netlink(format!("Failed to add MeshIfIndex: {:?}", e)))?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetTranstableLocal, attrs.build())
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
                _ => return Err(RobinError::Netlink("unknown netlink error payload".into())),
            }
        }

        let attrs = msg
            .get_payload()
            .ok_or_else(|| RobinError::Parse("Message without payload".into()))?
            .attrs()
            .get_attr_handle();

        let client = attrs
            .get_attr_payload_as::<[u8; 6]>(Attribute::BatadvAttrTtAddress.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_ADDRESS: {:?}", e)))?;
        let vid = attrs
            .get_attr_payload_as::<u16>(Attribute::BatadvAttrTtVid.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_VID: {:?}", e)))?;
        let crc32 = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrTtCrc32.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_CRC32: {:?}", e)))?;
        let raw_flags = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrTtFlags.into())
            .map_err(|e| RobinError::Parse(format!("Missing TT_FLAGS: {:?}", e)))?;
        let flags = ClientFlags::from_bits_truncate(raw_flags);

        let (last_seen_secs, last_seen_msecs) =
            match attrs.get_attr_payload_as::<u32>(Attribute::BatadvAttrLastSeenMsecs.into()) {
                Ok(ms) => (ms / 1000, ms % 1000),
                Err(_) => (0, 0),
            };

        entries.push(TranslocalEntry {
            client: MacAddr6::from(client),
            vid,
            flags,
            crc32,
            last_seen_secs,
            last_seen_msecs,
        });
    }

    Ok(entries)
}
