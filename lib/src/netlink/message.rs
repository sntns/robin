use crate::error::RobinError;
use crate::netlink::{/*AttrObject, AttrValue, Attribute,*/ Command};

use neli::consts::nl::NlmF;
use neli::genl::{Genlmsghdr, GenlmsghdrBuilder};
use neli::nl::{NlPayload, Nlmsghdr, NlmsghdrBuilder};
use neli::types::{Buffer, GenlBuffer};

/*#[derive(Debug, Copy, Clone)]
pub enum ExpectedType {
    U8,
    U16,
    U32,
    String,
    Bytes,
    Nested,
}*/

/*/// Spec mapping attribute id -> expected type
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
}*/

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

/*/// Generic parse: turns a Netlink NL message into a Vec<AttrObject>
/// Each AttrObject represents an "entity" (originator, neighbor, ...).
pub fn parse_nl_msg(
    nlmsg: &Nlmsghdr<u16, Genlmsghdr<u8, u16>>,
) -> Result<Vec<AttrObject>, RobinError> {
    let mut out = Vec::new();

    let handle = nlmsg
        .get_payload()
        .ok_or_else(|| RobinError::Parse("No payload found".to_string()))?
        .attrs()
        .get_attr_handle();

    for nla in handle.iter() {
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
                let payload = nla.nla_payload().as_ref().to_vec();
                let val = auto_cast_value_with_spec(nla.nla_type().into(), payload);
                let mut map = AttrObject::new();
                map.insert(nla.nla_type().into(), val);
                out.push(map);
            }
        }
    }

    Ok(out)
}

/// Parse a nested attribute set into an AttrObject
fn parse_attr_set(
    handle: &AttrHandle<GenlBuffer<u16, Buffer>, Nlattr<u16, Buffer>>,
) -> Result<AttrObject, RobinError> {
    let mut obj = AttrObject::new();

    for attr in handle.iter() {
        let t: u16 = attr.nla_type().into();
        let payload = attr.nla_payload().as_ref().to_vec();

        let value = match attr.get_attr_handle::<u16>() {
            Ok(sub_handle) => {
                // parse nested recursively
                let mut children_objs = Vec::new();
                for child in sub_handle.iter() {
                    let child_obj = match child.get_attr_handle() {
                        Ok(grand) => parse_attr_set(&grand)?,
                        Err(_) => {
                            let payload_g = child.nla_payload().as_ref().to_vec();
                            let v = auto_cast_value_with_spec(child.nla_type().into(), payload_g);
                            let mut m = AttrObject::new();
                            m.insert(child.nla_type().into(), v);
                            m
                        }
                    };
                    children_objs.push(child_obj);
                }
                AttrValue::Nested(children_objs)
            }
            Err(_) => auto_cast_value_with_spec(t, payload),
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
    // Check for null-terminated UTF-8 string
    if data.len() > 1 && data.last() == Some(&0) {
        if let Ok(s) = String::from_utf8(data[..data.len() - 1].to_vec()) {
            // Only treat printable ASCII/UTF-8 strings as string
            if s.chars().all(|c| c.is_ascii() && !c.is_control()) {
                return AttrValue::String(s);
            }
        }
    }

    match data.len() {
        1 => AttrValue::U8(data[0]),
        2 => AttrValue::U16(u16::from_le_bytes([data[0], data[1]])),
        4 => AttrValue::U32(u32::from_le_bytes([data[0], data[1], data[2], data[3]])),
        6 => AttrValue::Bytes(data), // e.g., MAC
        _ => AttrValue::Bytes(data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Originator;
    use macaddr::MacAddr6;

    #[test]
    fn test_auto_cast_value() {
        // string (nul-terminated)
        let s = b"bat0\0".to_vec();
        match auto_cast_value(s) {
            AttrValue::String(st) => assert_eq!(st, "bat0"),
            _ => panic!("Expected string"),
        }

        // u8
        let u = vec![0xffu8];
        match auto_cast_value(u) {
            AttrValue::U8(v) => assert_eq!(v, 0xff),
            _ => panic!("Expected u8"),
        }

        // u16 little endian
        let u16b = vec![0x34u8, 0x12u8];
        match auto_cast_value(u16b) {
            AttrValue::U16(v) => assert_eq!(v, 0x1234u16),
            _ => panic!("Expected u16"),
        }

        // u32 little endian
        let u32b = vec![1u8, 0, 0, 0];
        match auto_cast_value(u32b) {
            AttrValue::U32(v) => assert_eq!(v, 1u32),
            _ => panic!("Expected u32"),
        }

        // bytes (fallback)
        let b = vec![1u8, 2, 3, 4, 5];
        match auto_cast_value(b.clone()) {
            AttrValue::Bytes(v) => assert_eq!(v, b),
            _ => panic!("Expected bytes"),
        }
    }

    #[test]
    fn test_auto_cast_with_spec_map() {
        let s = b"bat0\0".to_vec();
        let val = auto_cast_value_with_spec(Attribute::BatadvAttrMeshIfname as u16, s);
        match val {
            AttrValue::String(st) => assert_eq!(st, "bat0"),
            _ => panic!("Expected string from spec map"),
        }

        let n = 0x12345678u32.to_le_bytes().to_vec();
        let val2 = auto_cast_value_with_spec(Attribute::BatadvAttrLastSeenMsecs as u16, n);
        match val2 {
            AttrValue::U32(v) => assert_eq!(v, 0x12345678u32),
            _ => panic!("Expected u32 from spec map"),
        }
    }

    #[test]
    fn test_originator_from_attr_object() {
        let mut obj = AttrObject::new();
        obj.insert(
            Attribute::BatadvAttrOrigAddress as u16,
            AttrValue::Bytes(vec![0x02, 0xaa, 0xbb, 0xcc, 0xdd, 0xee]),
        );
        obj.insert(
            Attribute::BatadvAttrLastSeenMsecs as u16,
            AttrValue::U32(420),
        );
        obj.insert(Attribute::BatadvAttrTq as u16, AttrValue::U8(255));
        obj.insert(
            Attribute::BatadvAttrThroughput as u16,
            AttrValue::U32(123456),
        );

        let o = Originator::try_from_attr_object(&obj).expect("originator parse");
        assert_eq!(
            o.mac_addr,
            MacAddr6::new(0x02, 0xaa, 0xbb, 0xcc, 0xdd, 0xee)
        );
        assert_eq!(o.last_seen_ms, Some(420));
        assert_eq!(o.tq, Some(255));
        assert_eq!(o.throughput, Some(123456));
        assert_eq!(o.is_router, false);
    }

    #[test]
    fn test_nested_attributes_parsing() {
        // Simulate nested attribute set
        let mut inner = AttrObject::new();
        inner.insert(Attribute::BatadvAttrTq as u16, AttrValue::U8(42));
        let nested = AttrValue::Nested(vec![inner.clone()]);

        let mut outer = AttrObject::new();
        outer.insert(Attribute::BatadvAttrOrigAddress as u16, nested.clone());

        match outer.get(&(Attribute::BatadvAttrOrigAddress as u16)) {
            Some(AttrValue::Nested(v)) => {
                assert_eq!(v.len(), 1);
                let child = &v[0];
                match child.get(&(Attribute::BatadvAttrTq as u16)) {
                    Some(AttrValue::U8(val)) => assert_eq!(*val, 42),
                    _ => panic!("Expected nested U8 attribute"),
                }
            }
            _ => panic!("Expected nested attribute"),
        }
    }

    #[test]
    fn test_auto_cast_invalid_payload() {
        // empty payload should fallback to bytes
        let empty = vec![];
        match auto_cast_value_with_spec(Attribute::BatadvAttrTq as u16, empty.clone()) {
            AttrValue::Bytes(v) => assert_eq!(v.len(), 0),
            _ => panic!("Expected bytes fallback for empty payload"),
        }

        // incorrect length for u16 attribute
        let short = vec![0x01];
        match auto_cast_value_with_spec(Attribute::BatadvAttrLastSeenMsecs as u16, short.clone()) {
            AttrValue::Bytes(v) => assert_eq!(v, short),
            _ => panic!("Expected bytes fallback for incorrect length"),
        }
    }

    #[test]
    fn test_heuristic_fallback() {
        // 6 bytes should fallback to bytes (MAC)
        let mac = vec![1, 2, 3, 4, 5, 6];
        match auto_cast_value(mac.clone()) {
            AttrValue::Bytes(v) => assert_eq!(v, mac),
            _ => panic!("Expected fallback to bytes for MAC"),
        }

        // 4 bytes without spec -> u32
        let n = vec![0x78, 0x56, 0x34, 0x12];
        match auto_cast_value(n.clone()) {
            AttrValue::U32(v) => assert_eq!(v, 0x12345678),
            _ => panic!("Expected u32 from heuristic"),
        }
    }
}*/
