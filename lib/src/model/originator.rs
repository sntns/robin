use crate::error::RobinError;
use crate::netlink::Attribute;
use crate::netlink::{AttrObject, AttrValue};

use macaddr::MacAddr6;

#[derive(Debug, Clone)]
pub struct Originator {
    /// originator MAC
    pub originator: MacAddr6,
    /// last seen in milliseconds
    pub last_seen_ms: Option<u32>,
    /// TQ (aggregated)
    pub tq: Option<u8>,
    /// estimated throughput if present (bytes per second or per header semantics)
    pub throughput: Option<u32>,
    /// is gateway/router (if attribute present)
    pub is_router: bool,
}

impl Originator {
    pub(crate) fn try_from_attr_object(obj: &AttrObject) -> Result<Self, RobinError> {
        // ORIG_ADDRESS mandatory
        let mac_val = obj
            .get(&(Attribute::BatadvAttrOrigAddress as u16))
            .ok_or_else(|| RobinError::Parse("Missing ORIG_ADDRESS".into()))?;

        let mac_bytes: [u8; 6] = match mac_val {
            AttrValue::Bytes(v) if v.len() >= 6 => v[..6]
                .try_into()
                .map_err(|_| RobinError::Parse("Invalid ORIG_ADDRESS".into()))?,
            AttrValue::String(s) if s.len() == 6 => s
                .as_bytes()
                .try_into()
                .map_err(|_| RobinError::Parse("Invalid ORIG_ADDRESS".into()))?,
            _ => return Err(RobinError::Parse("Invalid ORIG_ADDRESS".into())),
        };

        let mac = MacAddr6::from(mac_bytes);

        let last_seen = obj
            .get(&(Attribute::BatadvAttrLastSeenMsecs as u16))
            .and_then(|v| match v {
                AttrValue::U32(n) => Some(*n),
                AttrValue::Bytes(b) if b.len() >= 4 => {
                    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                }
                _ => None,
            });

        let tq = obj
            .get(&(Attribute::BatadvAttrTq as u16))
            .and_then(|v| match v {
                AttrValue::U8(x) => Some(*x),
                AttrValue::Bytes(b) if !b.is_empty() => Some(b[0]),
                _ => None,
            });

        let throughput = obj
            .get(&(Attribute::BatadvAttrThroughput as u16))
            .and_then(|v| match v {
                AttrValue::U32(x) => Some(*x),
                AttrValue::Bytes(b) if b.len() >= 4 => {
                    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                }
                _ => None,
            });

        let is_router = obj.contains_key(&(Attribute::BatadvAttrRouter as u16));

        Ok(Originator {
            originator: mac,
            last_seen_ms: last_seen,
            tq,
            throughput,
            is_router,
        })
    }
}
