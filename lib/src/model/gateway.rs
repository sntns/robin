use macaddr::MacAddr6;

/// Represents a gateway in the batman-adv mesh.
///
/// This struct contains information about a gateway node, including its MAC address,
/// the router it is associated with, interface used, bandwidth, throughput, and
/// quality metrics.
#[derive(Debug, Clone)]
pub struct Gateway {
    /// MAC address of the gateway (BATADV_ATTR_ORIG_ADDRESS).
    pub mac_addr: MacAddr6,

    /// MAC address of the associated router (BATADV_ATTR_ROUTER).
    pub router: MacAddr6,

    /// Outgoing interface used to reach this gateway.
    /// Usually from BATADV_ATTR_HARD_IFNAME; if not available, falls back to interface index.
    pub outgoing_if: String,

    /// Optional downstream bandwidth in kbps (BATADV_ATTR_BANDWIDTH_DOWN).
    pub bandwidth_down: Option<u32>,

    /// Optional upstream bandwidth in kbps (BATADV_ATTR_BANDWIDTH_UP).
    pub bandwidth_up: Option<u32>,

    /// Optional throughput in kbps (BATADV_ATTR_THROUGHPUT).
    pub throughput: Option<u32>,

    /// Optional transmission quality (TQ) of the gateway (BATADV_ATTR_TQ).
    pub tq: Option<u8>,

    /// Whether this gateway is considered the best among alternatives (BATADV_ATTR_FLAG_BEST).
    pub is_best: bool,
}

/// Contains configuration information about a mesh gateway.
///
/// This struct is used when querying or setting the gateway mode and associated parameters.
#[derive(Debug)]
pub struct GatewayInfo {
    /// Current gateway mode (BATADV_ATTR_GW_MODE).
    pub mode: GwMode,

    /// Selection class for the gateway (BATADV_ATTR_GW_SEL_CLASS).
    pub sel_class: u32,

    /// Downstream bandwidth in kbps (BATADV_ATTR_GW_BANDWIDTH_DOWN).
    pub bandwidth_down: u32,

    /// Upstream bandwidth in kbps (BATADV_ATTR_GW_BANDWIDTH_UP).
    pub bandwidth_up: u32,

    /// Routing algorithm in use (BATADV_ATTR_ALGO_NAME).
    pub algo: String,
}

/// Represents the mode of a batman-adv gateway.
#[derive(Debug, Copy, Clone)]
pub enum GwMode {
    /// Gateway mode is turned off.
    Off,

    /// Node is operating as a gateway client.
    Client,

    /// Node is operating as a gateway server.
    Server,

    /// Unknown or unsupported mode.
    Unknown,
}
