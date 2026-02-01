use crate::commands::utils::if_nametoindex;
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, ClientFlags, Command, TranslocalEntry};
use crate::netlink;

use macaddr::MacAddr6;
use neli::consts::nl::NlmF;
use neli::consts::nl::Nlmsg;
use neli::genl::Genlmsghdr;
use neli::nl::NlPayload;
use neli::nl::Nlmsghdr;

/// Retrieves the local translation table (TT) entries for a given BATMAN-adv mesh interface.
///
/// This corresponds to the `batctl tl` command and returns information about clients directly
/// connected to the local mesh interface.
///
/// # Arguments
///
/// * `mesh_if` - The name of the BATMAN-adv mesh interface to query.
///
/// # Returns
///
/// A vector of `TranslocalEntry` structs, each containing:
/// - `client`: The MAC address of the client.
/// - `vid`: The VLAN ID associated with the client.
/// - `flags`: Client flags (e.g., roaming, isolated, temporary).
/// - `crc32`: CRC32 checksum of the entry.
/// - `last_seen_secs`: Time in seconds since the client was last seen.
/// - `last_seen_msecs`: Remaining milliseconds since the client was last seen.
///
/// Returns a `RobinError` if any netlink operation or parsing fails.
pub async fn get_translocal(mesh_if: &str) -> Result<Vec<TranslocalEntry>, RobinError> {
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
        .map_err(|_| RobinError::Netlink("Failed to add MeshIfIndex attribute".to_string()))?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetTranstableLocal, attrs.build())
        .map_err(|_| RobinError::Netlink("Failed to build Netlink message".to_string()))?;

    let mut sock = netlink::BatadvSocket::connect().await.map_err(|_| {
        RobinError::Netlink("Failed to connect to batman-adv Netlink socket".to_string())
    })?;

    let mut response = sock
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|_| RobinError::Netlink("Failed to send Netlink request".to_string()))?;

    let mut entries = Vec::new();
    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|_| RobinError::Netlink("Failed to parse Netlink message".to_string()))?;

        match *msg.nl_type() {
            x if x == Nlmsg::Done.into() => break,
            x if x == Nlmsg::Error.into() => match &msg.nl_payload() {
                NlPayload::Err(err) if *err.error() == 0 => break,
                NlPayload::Err(err) => {
                    return Err(RobinError::Netlink(format!(
                        "Netlink error {}",
                        err.error()
                    )));
                }
                _ => {
                    return Err(RobinError::Netlink(
                        "Unknown Netlink error payload".to_string(),
                    ));
                }
            },
            _ => {}
        }

        let attrs = msg
            .get_payload()
            .ok_or_else(|| RobinError::Parse("Message without payload".to_string()))?
            .attrs()
            .get_attr_handle();

        let client = attrs
            .get_attr_payload_as::<[u8; 6]>(Attribute::BatadvAttrTtAddress.into())
            .map_err(|_| RobinError::Parse("Missing TT_ADDRESS".to_string()))?;
        let vid = attrs
            .get_attr_payload_as::<u16>(Attribute::BatadvAttrTtVid.into())
            .map_err(|_| RobinError::Parse("Missing TT_VID".to_string()))?;
        let crc32 = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrTtCrc32.into())
            .map_err(|_| RobinError::Parse("Missing TT_CRC32".to_string()))?;
        let raw_flags = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrTtFlags.into())
            .map_err(|_| RobinError::Parse("Missing TT_FLAGS".to_string()))?;
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
