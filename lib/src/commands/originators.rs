use crate::error::RobinError;
use crate::model::Originator;
use crate::netlink;
use neli::consts::nl::NlmF;

pub fn get_originators() -> Result<Vec<Originator>, RobinError> {
    let mut socket = netlink::BatadvSocket::connect()
        .map_err(|e| RobinError::Netlink(format!("Failed to connect to socket: {:?}", e)))?;

    // Create the value and the attribute
    let mut attrs = netlink::GenlAttrBuilder::new();
    let value = netlink::AttrValueForSend::Bytes("bat0\0".as_bytes().to_vec());
    attrs
        .add(netlink::Attribute::BatadvAttrMeshIfname, value)
        .map_err(|e| RobinError::Netlink(format!("Failed to add GENL attribute: {:?}", e)))?;

    // Build the message
    let msg = netlink::build_genl_msg(
        //family_id,
        netlink::Command::BatadvCmdGetOriginators,
        attrs.build(),
        //socket.next_seq(),
    )?;

    // Send message and get response
    let mut response = socket
        .send(NlmF::REQUEST | NlmF::DUMP, msg)
        .map_err(|e| RobinError::Netlink(format!("Failed to send message: {:?}", e)))?;

    // Parse response
    let mut originators: Vec<Originator> = Vec::new();
    for msg in &mut response {
        let msg = msg.map_err(|e| RobinError::Netlink(format!("{:?}", e)))?;
        let parsed = netlink::parse_nl_msg(&msg)
            .map_err(|e| RobinError::Parse(format!("Failed to parse message: {:?}", e)))?;
        for orig in parsed {
            let originator = Originator::try_from_attr_object(&orig).map_err(|e| {
                RobinError::Parse(format!(
                    "Failed to convert AttrObject to Originator: {:?}",
                    e
                ))
            })?;
            originators.push(originator);
        }
    }

    // socket drops automatically with end of scope

    Ok(originators)
}
