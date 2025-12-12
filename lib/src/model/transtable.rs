use macaddr::MacAddr6;

#[derive(Debug, Clone)]
pub struct TransglobalEntry {
    pub client: MacAddr6, // BATADV_ATTR_TT_ADDRESS
    pub orig: MacAddr6,   // BATADV_ATTR_ORIG_ADDRESS
    pub vid: u16,         // BATADV_ATTR_TT_VID
    pub ttvn: u8,         // BATADV_ATTR_TT_TTVN
    pub last_ttvn: u8,    // BATADV_ATTR_TT_LAST_TTVN
    pub flags: u32,       // BATADV_ATTR_TT_FLAGS
    pub crc32: u32,       // BATADV_ATTR_TT_CRC32
    pub is_best: bool,    // BATADV_ATTR_FLAG_BEST
}

pub struct TranslocalEntry {
    pub client: MacAddr6,
    pub vid: u16,
    pub flags: u32,
    pub crc32: u32,
    pub last_seen_secs: u32,
    pub last_seen_msecs: u32,
}
