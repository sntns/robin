/// Represents the possible types of values that can be sent as netlink attributes
/// to the BATMAN-adv kernel module.
///
/// This enum is used when constructing generic netlink messages for BATMAN-adv.
/// Each variant corresponds to a different type of payload that the kernel expects.
#[derive(Debug, Clone)]
pub enum AttrValueForSend {
    /// 8-bit unsigned integer attribute.
    U8(u8),

    /// 16-bit unsigned integer attribute.
    U16(u16),

    /// 32-bit unsigned integer attribute.
    U32(u32),

    /// Raw bytes attribute.
    Bytes(Vec<u8>),

    /// UTF-8 string attribute.
    String(String),
}
