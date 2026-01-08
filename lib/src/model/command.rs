/// BATADV supported netlink commands (from linux/uapi/batman_adv.h)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    BatadvCmdUnspec = 0,
    BatadvCmdGetMeshInfo = 1,
    BatadvCmdTpMeter = 2,
    BatadvCmdTpMeterCancel = 3,
    BatadvCmdGetRoutingAlgos = 4,
    BatadvCmdGetHardif = 5,
    BatadvCmdGetTranstableLocal = 6,
    BatadvCmdGetTranstableGlobal = 7,
    BatadvCmdGetOriginators = 8,
    BatadvCmdGetNeighbors = 9,
    BatadvCmdGetGateways = 10,
    BatadvCmdGetBlaClaim = 11,
    BatadvCmdGetBlaBackbone = 12,
    BatadvCmdGetDatCache = 13,
    BatadvCmdGetMcastFlags = 14,
    BatadvCmdSetMesh = 15,
    BatadvCmdSetHardif = 16,
    BatadvCmdGetVlan = 17,
    BatadvCmdSetVlan = 18,
}

impl From<Command> for u8 {
    fn from(c: Command) -> Self {
        c as u8
    }
}
