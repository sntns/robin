/// BATMAN-adv supported generic netlink commands.
///
/// These commands correspond to the BATMAN-adv netlink operations
/// defined in `linux/uapi/batman_adv.h`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Unspecified / no operation.
    BatadvCmdUnspec = 0,

    /// Get information about the mesh interface.
    BatadvCmdGetMeshInfo = 1,

    /// Start a throughput measurement (TP meter).
    BatadvCmdTpMeter = 2,

    /// Cancel an ongoing throughput measurement.
    BatadvCmdTpMeterCancel = 3,

    /// Get available routing algorithms.
    BatadvCmdGetRoutingAlgos = 4,

    /// Get hard interface (physical interface) information.
    BatadvCmdGetHardif = 5,

    /// Get the local translation table (local clients).
    BatadvCmdGetTranstableLocal = 6,

    /// Get the global translation table (originators from other nodes).
    BatadvCmdGetTranstableGlobal = 7,

    /// Get originator nodes.
    BatadvCmdGetOriginators = 8,

    /// Get neighboring nodes.
    BatadvCmdGetNeighbors = 9,

    /// Get gateway information.
    BatadvCmdGetGateways = 10,

    /// Get B.A.T.M.A.N. Loop Avoidance claims.
    BatadvCmdGetBlaClaim = 11,

    /// Get B.A.T.M.A.N. Loop Avoidance backbone status.
    BatadvCmdGetBlaBackbone = 12,

    /// Get datagram cache information.
    BatadvCmdGetDatCache = 13,

    /// Get multicast flags.
    BatadvCmdGetMcastFlags = 14,

    /// Set mesh interface parameters.
    BatadvCmdSetMesh = 15,

    /// Set hard interface parameters.
    BatadvCmdSetHardif = 16,

    /// Get VLAN settings.
    BatadvCmdGetVlan = 17,

    /// Set VLAN settings.
    BatadvCmdSetVlan = 18,
}

impl From<Command> for u8 {
    fn from(c: Command) -> Self {
        c as u8
    }
}
