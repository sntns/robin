use macaddr::MacAddr6;

/// Represents a neighboring node in the batman-adv mesh network.
///
/// A neighbor is a directly reachable node within the mesh. This struct provides
/// information about its MAC address, the interface used to reach it, and metrics such as
/// last seen time and optional throughput.
#[derive(Debug, Clone)]
pub struct Neighbor {
    /// MAC address of the neighbor.
    /// Corresponds to `BATADV_ATTR_NEIGH_ADDRESS`.
    pub neigh: MacAddr6,

    /// Outgoing interface name used to reach the neighbor.
    /// Falls back from `HARD_IFINDEX` if `HARD_IFNAME` is unavailable.
    /// Corresponds to `BATADV_ATTR_HARD_IFNAME`.
    pub outgoing_if: String,

    /// Time since the neighbor was last seen, in milliseconds.
    /// Corresponds to `BATADV_ATTR_LAST_SEEN_MSECS`.
    pub last_seen_ms: u32,

    /// Optional throughput towards this neighbor in kilobits per second.
    /// Corresponds to `BATADV_ATTR_THROUGHPUT`.
    /// Only available in BATMAN_V mode.
    pub throughput_kbps: Option<u32>,
}
