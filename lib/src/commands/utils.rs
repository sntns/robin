use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute, Command};
use crate::netlink;
use neli::consts::nl::NlmF;
use neli::genl::Genlmsghdr;
use neli::nl::Nlmsghdr;

/// Utils function to get algo name
pub async fn get_algo_name() -> Result<String, RobinError> {
    let ifindex = netlink::ifname_to_index("bat0")
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to get Ifindex: {:?}", e)))?;

    let mut attrs = netlink::GenlAttrBuilder::new();
    attrs
        .add(
            Attribute::BatadvAttrMeshIfindex,
            AttrValueForSend::U32(ifindex),
        )
        .map_err(|e| {
            RobinError::Netlink(format!("Failed to add MeshIfIndex attribute: {:?}", e))
        })?;

    let msg = netlink::build_genl_msg(Command::BatadvCmdGetMeshInfo, attrs.build())
        .map_err(|e| RobinError::Netlink(format!("Failed to build message: {:?}", e)))?;

    let mut sock = netlink::BatadvSocket::connect()
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    let mut response = sock
        .send(NlmF::REQUEST, msg)
        .await
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    while let Some(msg) = response.next().await {
        let msg: Nlmsghdr<u16, Genlmsghdr<u8, u16>> =
            msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;

        let payload = match msg.get_payload() {
            Some(p) => p,
            None => continue,
        };

        for attr in payload.attrs().iter() {
            if *attr.nla_type().nla_type() == Attribute::BatadvAttrAlgoName.into() {
                let bytes = attr.nla_payload().as_ref();
                let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
                let algo = String::from_utf8_lossy(&bytes[..nul]).to_string();
                return Ok(algo);
            }
        }
    }

    Err(RobinError::NotFound(
        "Algorithm name not found for interface bat0".to_string(),
    ))
}
