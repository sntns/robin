/// Represents a network interface in the batman-adv mesh.
///
/// This struct provides the interface name and whether it is currently active
/// within the mesh.
#[derive(Debug, Clone)]
pub struct Interface {
    /// Name of the interface, e.g., "eth0" or "bat0".
    pub ifname: String,

    /// Indicates whether this interface is currently active in the mesh.
    pub active: bool,
}
