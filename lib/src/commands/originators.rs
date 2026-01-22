use crate::commands::{if_indextoname, if_nametoindex};
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command, Originator};
use crate::netlink;

use macaddr::MacAddr6;
use neli::consts::nl::NlmF;
use neli::consts::nl::Nlmsg;
use neli::genl::Genlmsghdr;
use neli::nl::NlPayload;
use neli::nl::Nlmsghdr;

/// Retrieves the list of originators for a BATMAN-adv mesh interface.
///
/// This corresponds to the `batctl o` command. Each originator entry includes
/// the originator's MAC address, the next-hop neighbor MAC, the outgoing interface,
/// the last seen timestamp in milliseconds, optional TQ (link quality) and throughput,
/// and a flag indicating if this originator is currently the best route.
///
/// # Arguments
///
/// * `mesh_if` - The name of the mesh interface (e.g., `"bat0"`).
///
/// # Returns
///
/// Returns a vector of `Originator` structs or a `RobinError` if the query fails.
///
/// # Example
///
/// ```no_run
/// let originators = get_originators("bat0").await?;
/// for o in originators {
///     println!(
///         "Originator {} via {} (last seen {} ms, best: {})",
///         o.originator, o.outgoing_if, o.last_seen_ms, o.is_best
///     );
/// }
/// ```
pub async fn get_originators(mesh_if: &str) -> Result<Vec<Originator>, RobinError> {
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

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetOriginators, attrs.build())
        .map_err(|_| RobinError::Netlink("Failed to build netlink message".to_string()))?;

    let mut socket = netlink::BatadvSocket::connect()
        .await
        .map_err(|_| RobinError::Netlink("Failed to connect to batman-adv socket".to_string()))?;

    let mut response = socket
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|_| RobinError::Netlink("Failed to send netlink request".to_string()))?;

    let mut originators: Vec<Originator> = Vec::new();
    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|_| RobinError::Netlink("Failed to parse netlink message".to_string()))?;

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
                        "Unknown netlink error payload".to_string(),
                    ));
                }
            },
            _ => {}
        }

        let attrs = msg
            .get_payload()
            .ok_or_else(|| RobinError::Parse("Message without payload".into()))?
            .attrs()
            .get_attr_handle();

        let orig = attrs
            .get_attr_payload_as::<[u8; 6]>(Attribute::BatadvAttrOrigAddress.into())
            .map_err(|_| RobinError::Parse("Missing ORIG_ADDRESS".into()))?;

        let neigh = attrs
            .get_attr_payload_as::<[u8; 6]>(Attribute::BatadvAttrNeighAddress.into())
            .map_err(|_| RobinError::Parse("Missing NEIGH_ADDRESS".into()))?;

        let outgoing_if =
            match attrs.get_attr_payload_as::<[u8; 16]>(Attribute::BatadvAttrHardIfname.into()) {
                Ok(bytes) => {
                    let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                    String::from_utf8_lossy(&bytes[..nul_pos]).into_owned()
                }
                Err(_) => {
                    let idx = attrs
                        .get_attr_payload_as::<u32>(Attribute::BatadvAttrHardIfindex.into())
                        .map_err(|_| RobinError::Parse("Missing HARD_IFINDEX".into()))?;
                    if_indextoname(idx).await.map_err(|_| {
                        RobinError::Netlink(format!("Failed to resolve ifindex {} -> name", idx))
                    })?
                }
            };

        let last_seen_ms = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrLastSeenMsecs.into())
            .map_err(|_| RobinError::Parse("Missing LAST_SEEN_MSECS".into()))?;

        let tq = attrs
            .get_attr_payload_as::<u8>(Attribute::BatadvAttrTq.into())
            .ok();
        let tp = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrThroughput.into())
            .ok();
        let is_best = attrs
            .get_attribute(Attribute::BatadvAttrFlagBest.into())
            .is_some();

        originators.push(Originator {
            originator: MacAddr6::from(orig),
            next_hop: MacAddr6::from(neigh),
            outgoing_if,
            last_seen_ms,
            tq,
            throughput: tp,
            is_best,
        });
    }

    Ok(originators)
}
