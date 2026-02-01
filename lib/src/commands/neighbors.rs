use crate::commands::{if_indextoname, if_nametoindex};
use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command, Neighbor};
use crate::netlink;

use macaddr::MacAddr6;
use neli::consts::nl::{NlmF, Nlmsg};
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};

/// Retrieves the list of neighbors for a BATMAN-adv mesh interface.
///
/// This corresponds to the `batctl n` command. Each neighbor entry contains
/// the neighbor's MAC address, the outgoing interface used to reach it,
/// the last time it was seen in milliseconds, and optionally the throughput in kb/s.
///
/// # Arguments
///
/// * `mesh_if` - The name of the mesh interface (e.g., `"bat0"`).
///
/// # Returns
///
/// Returns a vector of `Neighbor` structs or a `RobinError` if the query fails.
///
/// # Example
///
/// ```no_run
/// # use robin::model::Neighbor;
/// # async fn example() {
/// # let neighbors: Vec<Neighbor> = vec![];
/// // let neighbors = get_neighbors("bat0").await?;
/// for n in neighbors {
///     println!("Neighbor {} via {} (last seen {} ms)", n.neigh, n.outgoing_if, n.last_seen_ms);
/// }
/// # }
/// ```
pub async fn get_neighbors(mesh_if: &str) -> Result<Vec<Neighbor>, RobinError> {
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
            RobinError::Netlink("Error - failed to add MeshIfindex attribute".to_string())
        })?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetOriginators, attrs.build())
        .map_err(|_| RobinError::Netlink("Error - failed to build netlink message".to_string()))?;

    let mut sock = netlink::BatadvSocket::connect().await.map_err(|_| {
        RobinError::Netlink("Error - failed to connect to batman-adv socket".to_string())
    })?;

    let mut response = sock
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .await
        .map_err(|_| RobinError::Netlink("Error - failed to send netlink request".to_string()))?;

    let mut neighbors: Vec<Neighbor> = Vec::new();
    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> = msg.map_err(|_| {
            RobinError::Netlink("Error - failed to parse netlink message".to_string())
        })?;

        match *msg.nl_type() {
            x if x == Nlmsg::Done.into() => break,
            x if x == Nlmsg::Error.into() => {
                match &msg.nl_payload() {
                    NlPayload::Err(err) if *err.error() == 0 => break, // end of dump
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
                }
            }
            _ => {}
        }

        let attrs = msg
            .get_payload()
            .ok_or_else(|| RobinError::Parse("Error - message has no payload".into()))?
            .attrs()
            .get_attr_handle();

        let neigh_addr = attrs
            .get_attr_payload_as::<[u8; 6]>(Attribute::BatadvAttrNeighAddress.into())
            .map_err(|_| RobinError::Parse("Error - missing NEIGH_ADDRESS".into()))?;

        let last_seen_ms = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrLastSeenMsecs.into())
            .map_err(|_| RobinError::Parse("Error - missing LAST_SEEN_MSECS".into()))?;

        let outgoing_if =
            match attrs.get_attr_payload_as::<[u8; 16]>(Attribute::BatadvAttrHardIfname.into()) {
                Ok(bytes) => {
                    let nul_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                    String::from_utf8_lossy(&bytes[..nul_pos]).into_owned()
                }
                Err(_) => {
                    let ifindex = attrs
                        .get_attr_payload_as::<u32>(Attribute::BatadvAttrHardIfindex.into())
                        .map_err(|_| RobinError::Parse("Error - missing HARD_IFINDEX".into()))?;
                    if_indextoname(ifindex).await.map_err(|_| {
                        RobinError::Netlink(format!(
                            "Error - failed to resolve interface index {}",
                            ifindex
                        ))
                    })?
                }
            };

        let throughput_kbps = attrs
            .get_attr_payload_as::<u32>(Attribute::BatadvAttrThroughput.into())
            .ok();

        neighbors.push(Neighbor {
            neigh: MacAddr6::from(neigh_addr),
            outgoing_if,
            last_seen_ms,
            throughput_kbps,
        });
    }

    Ok(neighbors)
}
