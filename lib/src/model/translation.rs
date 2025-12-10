use macaddr::MacAddr6;

#[derive(Debug, Clone)]
pub struct TranslationEntry {
    pub dest: MacAddr6,    // BATADV_ATTR_DEST_ADDRESS
    pub via: MacAddr6,     // BATADV_ATTR_NEIGH_ADDRESS
    pub ifname: String,    // BATADV_ATTR_HARD_IFNAME
    pub tq: Option<u8>,    // BATADV_ATTR_TQ
    pub ttl: Option<u8>,   // BATADV_ATTR_TTL
    pub flags: Option<u8>, // BATADV_ATTR_FLAGS
}
