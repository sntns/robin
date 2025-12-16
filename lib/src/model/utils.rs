pub enum AttrValueForSend {
    U8(u8),
    U16(u16),
    U32(u32),
    Bytes(Vec<u8>),
    String(String),
}
