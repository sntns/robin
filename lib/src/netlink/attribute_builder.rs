use neli::genl::{AttrTypeBuilder, NlattrBuilder};
use neli::types::{Buffer, GenlBuffer};

use crate::error::RobinError;
use crate::model::{AttrValueForSend, Attribute};

pub(crate) struct GenlAttrBuilder {
    buf: GenlBuffer<u16, Buffer>,
}

impl GenlAttrBuilder {
    pub(crate) fn new() -> Self {
        Self {
            buf: GenlBuffer::new(),
        }
    }

    pub(crate) fn add(
        &mut self,
        attr: Attribute,
        value: AttrValueForSend,
    ) -> Result<(), RobinError> {
        let attr_type = AttrTypeBuilder::default()
            .nla_type(attr.into())
            .build()
            .map_err(|e| RobinError::Netlink(format!("Failed to build AttrType: {:?}", e)))?;

        let attr_payload = match value {
            AttrValueForSend::String(s) => {
                let mut b = s.into_bytes();
                b.push(0);
                b
            }
            AttrValueForSend::Bytes(b) => b,
            AttrValueForSend::U32(v) => v.to_le_bytes().to_vec(),
            AttrValueForSend::U16(v) => v.to_le_bytes().to_vec(),
            AttrValueForSend::U8(v) => vec![v],
        };

        let attr = NlattrBuilder::default()
            .nla_type(attr_type)
            .nla_payload(attr_payload)
            .build()
            .map_err(|e| RobinError::Netlink(format!("Failed to build Nlattr: {:?}", e)))?;

        self.buf.push(attr);
        Ok(())
    }

    pub(crate) fn build(self) -> GenlBuffer<u16, Buffer> {
        self.buf
    }
}
