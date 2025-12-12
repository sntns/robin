use macaddr::MacAddr6;

#[derive(Debug, Clone)]
pub struct Neighbor {
    pub neigh: MacAddr6,              // BATADV_ATTR_NEIGH_ADDRESS
    pub outgoing_if: String,          // BATADV_ATTR_HARD_IFNAME or fallback from HARD_IFINDEX
    pub last_seen_ms: u32,            // BATADV_ATTR_LAST_SEEN_MSECS
    pub throughput_kbps: Option<u32>, // BATADV_ATTR_THROUGHPUT (kb/s) — optional, used in BATMAN_V
}
