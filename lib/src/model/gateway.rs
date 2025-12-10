use macaddr::MacAddr6;

#[derive(Debug, Clone)]
pub struct Gateway {
    pub mac_addr: MacAddr6,          // BATADV_ATTR_ORIG_ADDRESS
    pub router: MacAddr6,            // BATADV_ATTR_ROUTER
    pub outgoing_if: String,         // BATADV_ATTR_HARD_IFNAME / fallback IFINDEX
    pub bandwidth_down: Option<u32>, // BATADV_ATTR_BANDWIDTH_DOWN
    pub bandwidth_up: Option<u32>,   // BATADV_ATTR_BANDWIDTH_UP
    pub throughput: Option<u32>,     // BATADV_ATTR_THROUGHPUT
    pub tq: Option<u8>,              // BATADV_ATTR_TQ
    pub is_best: bool,               // BATADV_ATTR_FLAG_BEST
}

#[derive(Debug)]
pub struct GatewayInfo {
    pub mode: GwMode,                // BATADV_ATTR_GW_MODE
    pub sel_class: Option<u32>,      // BATADV_ATTR_GW_SEL_CLASS
    pub bandwidth_down: Option<u32>, // BATADV_ATTR_GW_BANDWIDTH_DOWN
    pub bandwidth_up: Option<u32>,   // BATADV_ATTR_GW_BANDWIDTH_UP
    pub algo: Option<String>,        // BATADV_ATTR_ALGO_NAME
}

#[derive(Debug)]
pub enum GwMode {
    Off,
    Client,
    Server,
    Unknown(u8),
}
