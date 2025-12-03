pub mod error;
pub mod netlink;
pub mod types;

pub use error::*;
pub use netlink::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use neli::genl::{GenlmsghdrBuilder, NlattrBuilder};
    use neli::types::Buffer;
    use neli::types::GenlBuffer;

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
        let b = vec![1u8, 2u8, 3u8, 4u8, 5u8];
        match auto_cast_value(b.clone()) {
            AttrValue::Bytes(v) => assert_eq!(v, b),
            _ => panic!("Expected bytes"),
        }
    }

    #[test]
    fn test_auto_cast_with_spec_map() {
        let spec = get_attr_spec_map();
        assert!(
            spec.get(&(Attribute::BatadvAttrMeshIfname as u16))
                .is_some()
        );

        // Prepare a payload for MESH_IFNAME
        let s = b"bat0\0".to_vec();
        let v = auto_cast_value_with_spec(Attribute::BatadvAttrMeshIfname as u16, s);
        match v {
            AttrValue::String(st) => assert_eq!(st, "bat0"),
            _ => panic!("expected string from spec"),
        }

        // Prepare a payload for LAST_SEEN_MSECS (u32)
        let n = 0x12345678u32.to_le_bytes().to_vec();
        let v2 = auto_cast_value_with_spec(Attribute::BatadvAttrLastSeenMsecs as u16, n);
        match v2 {
            AttrValue::U32(x) => assert_eq!(x, 0x12345678u32),
            _ => panic!("expected u32 from spec"),
        }
    }

    #[test]
    fn test_originator_from_attr_object() {
        // craft an AttrObject manually
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
            o.originator,
            MacAddr6::new(0x02, 0xaa, 0xbb, 0xcc, 0xdd, 0xee)
        );
        assert_eq!(o.last_seen_ms, Some(420));
        assert_eq!(o.tq, Some(255));
        assert_eq!(o.throughput, Some(123456));
        assert_eq!(o.is_router, false);
    }

    #[test]
    fn test_parse_attr_set_and_genl_integration() {
        // build a nested NLA that mimics one originator object:
        // nested attr set contains:
        //   ORIG_ADDRESS (type 9): 6 bytes mac
        //   LAST_SEEN_MSECS (24): u32
        //   TQ (26): u8
        // We'll assemble a GenlBuffer with a single nested attribute and feed it to parse_genl_msg
        let mut inner_buf = GenlBuffer::new();
        // ORIG_ADDRESS: raw bytes
        let orig_nla = NlattrBuilder::default()
            .nla_type(Attribute::BatadvAttrOrigAddress as u16)
            .nla_payload(vec![0x02, 0x11, 0x22, 0x33, 0x44, 0x55])
            .build()
            .expect("build orig nla");
        inner_buf.push(orig_nla);

        let last_seen_nla = NlattrBuilder::default()
            .nla_type(Attribute::BatadvAttrLastSeenMsecs as u16)
            .nla_payload(123u32.to_le_bytes().to_vec())
            .build()
            .expect("build last_seen nla");
        inner_buf.push(last_seen_nla);

        let tq_nla = NlattrBuilder::default()
            .nla_type(Attribute::BatadvAttrTq as u16)
            .nla_payload(vec![200u8])
            .build()
            .expect("build tq nla");
        inner_buf.push(tq_nla);

        // Now create a top-level NLA that is nested and contains inner_buf
        // We need to create Nlattr whose payload is the serialized inner_buf.
        // Luckily NlattrBuilder accepts nested GenlBuffer via .nla_payload(Buffer)
        let nested_payload = inner_buf.to_bytes().expect("serialize inner buf");
        let top_nla = NlattrBuilder::default()
            .nla_type(0) // arbitrary top-level attribute id (treated as nested)
            .nla_payload(nested_payload)
            .build()
            .expect("build top nla");

        // put this top_nla into a GenlBuffer and build Genlmsghdr
        let mut top_buf = GenlBuffer::new();
        top_buf.push(top_nla);

        let genl = GenlmsghdrBuilder::default()
            .cmd(Command::BatadvCmdGetOriginators.into())
            .version(1)
            .attrs(top_buf)
            .build()
            .expect("build genl");

        // wrap into Nlmsghdr
        use neli::consts::nl::NlmF;
        use neli::nl::NlmsghdrBuilder;
        let nl = NlmsghdrBuilder::default()
            .nl_type(16u16) // dummy family id
            .nl_flags(NlmF::REQUEST)
            .nl_seq(1)
            .nl_pid(0)
            .nl_payload(NlPayload::Payload(genl))
            .build()
            .expect("build nl");

        // now parse
        let objs = parse_genl_msg(&nl).expect("parse genl");
        assert!(!objs.is_empty());
        // map first object to Originator
        let o = Originator::try_from_attr_object(&objs[0]).expect("map originator");
        assert_eq!(
            o.originator,
            MacAddr6::new(0x02, 0x11, 0x22, 0x33, 0x44, 0x55)
        );
        assert_eq!(o.last_seen_ms, Some(123));
        assert_eq!(o.tq, Some(200));
    }
}
