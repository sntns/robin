use crate::error::RobinError;
use crate::{Attribute, Command};
use std::collections::HashMap;

use neli::attr::AttrHandle;
use neli::consts::nl::NlmF;
use neli::genl::{Genlmsghdr, GenlmsghdrBuilder};
use neli::nl::{NlPayload, Nlmsghdr, NlmsghdrBuilder};
use neli::types::{Buffer, GenlBuffer};

#[derive(Debug, Clone)]
pub enum AttrValue {
    U8(u8),
    U16(u16),
    U32(u32),
    Bytes(Vec<u8>),
    String(String),
    Nested(Vec<AttrObject>), // attribute set
}

pub type AttrObject = HashMap<u16, AttrValue>;

#[derive(Debug, Copy, Clone)]
pub enum ExpectedType {
    U8,
    U16,
    U32,
    String,
    Bytes,
    Nested,
}

/// Spec mapping attribute id -> expected type
/// Extend this map from linux/uapi/batman_adv.h as needed.
fn get_attr_spec_map() -> HashMap<u16, ExpectedType> {
    let mut m = HashMap::new();
    m.insert(Attribute::BatadvAttrOrigAddress.into(), ExpectedType::Bytes); // MAC 6 bytes
    m.insert(Attribute::BatadvAttrLastSeenMsecs.into(), ExpectedType::U32);
    m.insert(Attribute::BatadvAttrTq.into(), ExpectedType::U8);
    m.insert(Attribute::BatadvAttrThroughput.into(), ExpectedType::U32);
    m.insert(Attribute::BatadvAttrRouter.into(), ExpectedType::Bytes); // sometimes flag / mac
    m.insert(Attribute::BatadvAttrMeshIfname.into(), ExpectedType::String);
    // add more as needed...
    m
}

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
//  and can contain anything from attrs.rs which can be completed from
//  usr/src/linux-headers-$(uname -r)/include/uapi/linux/batman_adv.h
//  see attrs.rs for corresponding attrs in robin

/// Create a Netlink message for the given command with the given attributes
pub fn build_genl_msg(
    family_id: u16,
    cmd: Command,
    attrs: GenlBuffer<u16, Buffer>,
    seq: u32,
) -> Result<Nlmsghdr<u16, Genlmsghdr<u8, u16>>, RobinError> {
    // Build GENL header with attributes
    let genl_msg = GenlmsghdrBuilder::default()
        .cmd(cmd.into())
        .version(1)
        .attrs(attrs)
        .build()
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

/// Generic parse: turns a Netlink GENL message into a Vec<AttrObject>
/// Each AttrObject represents an "entity" (originator, neighbor, ...).
pub fn parse_genl_msg(
    nl_msg: &Nlmsghdr<u16, Genlmsghdr<u8, u16>>,
) -> Result<Vec<AttrObject>, RobinError> {
    let mut out = Vec::new();

    // Only process payloads
    let genl = match nl_msg.nl_payload() {
        NlPayload::Payload(p) => p,
        _ => return Ok(out),
    };

    // top-level attributes handle (AttrHandle<u16, Buffer>)
    let top = genl
        .get_attr_handle()
        .map_err(|e| RobinError::Parse(format!("get_attr_handle top: {:?}", e)))?;

    for nla in top.iter() {
        // detect nested by type flag or by asking get_attr_handle()
        // We try get_attr_handle(); if succeeds, treat as nested set (common for batadv)
        match nla.get_attr_handle() {
            Ok(nested_handle) => {
                // nested_handle: AttrHandle<u16, Buffer>
                let obj = parse_attr_set(&nested_handle)?;
                out.push(obj);
            }
            Err(_) => {
                // not nested -> treat single attribute as single-object
                let payload = nla.payload().as_ref().to_vec();
                let val = auto_cast_value_with_spec(nla.nla_type() as u16, payload);
                let mut map = AttrObject::new();
                map.insert(nla.nla_type() as u16, val);
                out.push(map);
            }
        }
    }

    Ok(out)
}

/// Parse a nested attribute set into an AttrObject
fn parse_attr_set(nested: &AttrHandle<u16, Buffer>) -> Result<AttrObject, RobinError> {
    let mut obj = AttrObject::new();

    for attr in nested.iter() {
        let t = attr.nla_type() as u16;
        // payload as Vec<u8>
        let payload = attr.payload().as_ref().to_vec();

        // If attribute itself is nested, parse recursively into list
        let value = match attr.get_attr_handle() {
            Ok(sub_handle) => {
                // parse sub-handle: sub-handle may contain several children => collect all
                let mut children = Vec::new();
                // sub_handle is AttrHandle<u16, Buffer>
                for child in sub_handle.iter() {
                    // if grandchild nested, recursion will handle
                    match child.get_attr_handle() {
                        Ok(grand) => {
                            let parsed = parse_attr_set(&grand)?;
                            children.push(parsed);
                        }
                        Err(_) => {
                            let payload_g = child.payload().as_ref().to_vec();
                            let v = auto_cast_value_with_spec(child.nla_type() as u16, payload_g);
                            let mut m = AttrObject::new();
                            m.insert(child.nla_type() as u16, v);
                            children.push(m);
                        }
                    }
                }
                AttrValue::Nested(children)
            }
            Err(_) => {
                // not nested: auto-detect type (using spec map if present)
                auto_cast_value_with_spec(t, payload)
            }
        };

        obj.insert(t, value);
    }

    Ok(obj)
}

/// Improved auto-cast using attribute spec map when available
fn auto_cast_value_with_spec(attr_type: u16, data: Vec<u8>) -> AttrValue {
    let spec = get_attr_spec_map();
    if let Some(expected) = spec.get(&attr_type) {
        match expected {
            ExpectedType::String => {
                // Try NUL-terminated string
                if !data.is_empty() && data[data.len() - 1] == 0 {
                    if let Ok(s) = String::from_utf8(data[..data.len() - 1].to_vec()) {
                        return AttrValue::String(s);
                    }
                }
                // fallback to bytes
                return AttrValue::Bytes(data);
            }
            ExpectedType::U8 => {
                if data.len() >= 1 {
                    return AttrValue::U8(data[0]);
                } else {
                    return AttrValue::Bytes(data);
                }
            }
            ExpectedType::U16 => {
                if data.len() >= 2 {
                    return AttrValue::U16(u16::from_le_bytes([data[0], data[1]]));
                } else {
                    return AttrValue::Bytes(data);
                }
            }
            ExpectedType::U32 => {
                if data.len() >= 4 {
                    return AttrValue::U32(u32::from_le_bytes([
                        data[0], data[1], data[2], data[3],
                    ]));
                } else {
                    return AttrValue::Bytes(data);
                }
            }
            ExpectedType::Bytes => return AttrValue::Bytes(data),
            ExpectedType::Nested => {
                // shouldn't reach here
                return AttrValue::Bytes(data);
            }
        }
    }

    // fallback: try heuristics
    auto_cast_value(data)
}

/// Heuristic fallback auto-detection (if spec missing)
fn auto_cast_value(data: Vec<u8>) -> AttrValue {
    if data.is_empty() {
        return AttrValue::Bytes(data);
    }

    // string null-terminated ?
    if data.len() > 1 && data[data.len() - 1] == 0 {
        if let Ok(s) = String::from_utf8(data[..data.len() - 1].to_vec()) {
            return AttrValue::String(s);
        }
    }

    // u8
    if data.len() == 1 {
        return AttrValue::U8(data[0]);
    }

    // u16 little-endian
    if data.len() == 2 {
        return AttrValue::U16(u16::from_le_bytes([data[0], data[1]]));
    }

    // u32 little-endian
    if data.len() == 4 {
        return AttrValue::U32(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
    }

    // fallback
    AttrValue::Bytes(data)
}
