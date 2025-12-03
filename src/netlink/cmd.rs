/// Selected BATADV GENL commands (from linux/uapi/batman_adv.h)
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Command {
    BatadvCmdUnspec = 0,
    BatadvCmdGetMeshInfo = 1,
    BatadvCmdGetRoutingAlgos = 2,
    BatadvCmdGetHardifs = 3,
    BatadvCmdGetTranstableLocal = 4,
    BatadvCmdGetTranstableGlobal = 5,
    BatadvCmdGetOriginators = 8,
    BatadvCmdGetGateways = 9,
    BatadvCmdGetBlaClaim = 10,
    BatadvCmdGetBlaBackbone = 11,
    BatadvCmdGetDatCache = 12,
    BatadvCmdSetTranstableLocal = 13,
    // ...
}

impl From<Command> for u8 {
    fn from(c: Command) -> Self {
        c as u8
    }
}

// +--------------------------------+-----------------------------------------------+---------------------------------------------+
// | Command                        | Description                                   | When to use                                 |
// +--------------------------------+-----------------------------------------------+---------------------------------------------+
// | BATADV_CMD_UNSPEC               | Unspecified / unused                          | Never call directly                         |
// | BATADV_CMD_GET_MESH_INFO        | Retrieves global mesh information            | To get mesh name, version, mesh interfaces |
// | BATADV_CMD_GET_ROUTING_ALGOS    | Retrieves available routing algorithms       | To see which routing is active              |
// | BATADV_CMD_GET_HARDIFS           | Retrieves physical interfaces of the mesh    | To list hardware interfaces                 |
// | BATADV_CMD_GET_TRANSTABLE_LOCAL | Retrieves local translation table            | To see local routing entries                |
// | BATADV_CMD_GET_TRANSTABLE_GLOBAL| Retrieves global translation table           | To see global routing entries               |
// | BATADV_CMD_GET_ORIGINATORS       | Retrieves originator table                   | To list all known nodes in the mesh         |
// | BATADV_CMD_GET_GATEWAYS           | Retrieves available gateways                 | To know the mesh gateways                   |
// | BATADV_CMD_GET_BLA_CLAIM          | Retrieves Backbone Loop Avoidance claims     | For backbone debugging                       |
// | BATADV_CMD_GET_BLA_BACKBONE       | Retrieves BLA backbone routes                | For backbone debugging                       |
// | BATADV_CMD_GET_DAT_CACHE           | Retrieves the DAT (Destination Cache)       | For routing debugging                        |
// | BATADV_CMD_SET_TRANSTABLE_LOCAL   | Modifies the local translation table        | To send a modification to the kernel        |
// +--------------------------------+-----------------------------------------------+---------------------------------------------+
