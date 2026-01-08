use crate::error::RobinError;
use crate::model::Command;

use neli::consts::nl::NlmF;
use neli::genl::{Genlmsghdr, GenlmsghdrBuilder};
use neli::nl::{NlPayload, Nlmsghdr, NlmsghdrBuilder};
use neli::types::{Buffer, GenlBuffer};

// Netlink message structure :
// Header Netlink (Nlmsghdr)
// 	•	Message type = family_id (ex : batadv)
// 	•	Flags = NLM_F_REQUEST
// 	•	Payload = a Genlmsghdr
//
// Header Generic Netlink (Genlmsghdr)
// 	•	cmd = ex BatadvCmdGetOriginators
// 	•	version = 1
// 	•	attributes = NLA attribute list
//
// NLA attributes :
//  must contain BatadvAttrMeshIfname = "bat0\0"
//  and can contain anything from attribute which can be completed from
//  usr/src/linux-headers-$(uname -r)/include/uapi/linux/batman_adv.h
//  see attribute for corresponding attrs in robin

/// Create a Netlink (Genl) message for the given command with the given attributes
pub fn build_genl_msg(
    cmd: Command,
    attrs: GenlBuffer<u16, Buffer>,
) -> Result<Genlmsghdr<u8, u16>, RobinError> {
    // Build GENL header with attributes
    let genl_msg = GenlmsghdrBuilder::default()
        .cmd(cmd.into())
        .version(1)
        .attrs(attrs)
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build GENL header: {:?}", e)))?;

    Ok(genl_msg)
}

/// Create a Netlink (Nl) message for the given command with the given attributes
pub fn build_nl_msg(
    family_id: u16,
    cmd: Command,
    attrs: GenlBuffer<u16, Buffer>,
    seq: u32,
) -> Result<Nlmsghdr<u16, Genlmsghdr<u8, u16>>, RobinError> {
    // Build GENL header with attributes
    let genl_msg = build_genl_msg(cmd, attrs)
        .map_err(|e| RobinError::Netlink(format!("Failed to build GENL header: {:?}", e)))?;

    // Build Netlink header with payload
    let nl_msg = NlmsghdrBuilder::default()
        .nl_type(family_id)
        .nl_flags(NlmF::REQUEST | NlmF::DUMP) // REQUEST for query, DUMP if expecting multiple entries
        .nl_seq(seq) // sequence number, can be incremented externally
        .nl_pid(0) // 0 lets the kernel fill in the sender PID
        .nl_payload(NlPayload::Payload(genl_msg))
        .build()
        .map_err(|e| RobinError::Netlink(format!("Failed to build NL header: {:?}", e)))?;

    Ok(nl_msg)
}
