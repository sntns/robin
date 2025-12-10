use macaddr::MacAddr6;

#[derive(Debug, Clone)]
pub struct Interface {
    pub ifindex: u32,           // BATADV_ATTR_MESH_IFINDEX
    pub ifname: String,         // BATADV_ATTR_MESH_IFNAME
    pub addr: Option<MacAddr6>, // BATADV_ATTR_MESH_ADDRESS
    pub flags: Option<u32>,     // BATADV_ATTR_FLAGS
}
