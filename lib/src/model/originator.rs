use macaddr::MacAddr6;

#[derive(Debug, Clone)]
pub struct Originator {
    pub originator: MacAddr6,    // BATADV_ATTR_ORIG_ADDRESS
    pub next_hop: MacAddr6,      // BATADV_ATTR_NEIGH_ADDRESS
    pub outgoing_if: String,     // BATADV_ATTR_HARD_IFNAME or ifindex
    pub last_seen_ms: u32,       // BATADV_ATTR_LAST_SEEN_MSECS
    pub tq: Option<u8>,          // BATADV_ATTR_TQ
    pub throughput: Option<u32>, // BATADV_ATTR_THROUGHPUT
    pub is_best: bool,           // BATADV_ATTR_ROUTER
}
