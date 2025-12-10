/// BATADV supported netlink commands (from linux/uapi/batman_adv.h)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    BatadvCmdUnspec = 0,
    BatadvCmdGetMesh = 1,
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

// +---------------+-------------------------------+-----------------------------+--------------------------------------------------------------+
// | Batctl cmd    | Netlink cmd (GENL)            | Structure des attributs      | Notes                                                        |
// +---------------+-------------------------------+-----------------------------+--------------------------------------------------------------+
// | o (originators)| BATADV_CMD_GET_ORIGINATORS   | Nlmsghdr par originator      | attrs: ORIG_ADDRESS, NEIGH_ADDRESS, HARD_IFNAME/INDEX,      |
// |               |                               |                             | LAST_SEEN_MSECS, TQ, THROUGHPUT, ROUTER                     |
// +---------------+-------------------------------+-----------------------------+--------------------------------------------------------------+
// | gwl (list gw) | BATADV_CMD_GET_GATEWAYS       | Nlmsghdr par gateway         | attrs: GATEWAY_ADDRESS, FLAGS, TQ, THROUGHPUT, IFINDEX      |
// +---------------+-------------------------------+-----------------------------+--------------------------------------------------------------+
// | gw (current gw)| BATADV_CMD_GET_GATEWAY       | Nlmsghdr unique              | attrs: GATEWAY_ADDRESS, FLAGS, TQ, THROUGHPUT, IFINDEX      |
// +---------------+-------------------------------+-----------------------------+--------------------------------------------------------------+
// | tg (translation)| BATADV_CMD_GET_TT           | Nlmsghdr par entry           | attrs: DEST_ADDRESS, VIA_ADDRESS, TQ, FLAGS, TTL, IFINDEX   |
// +---------------+-------------------------------+-----------------------------+--------------------------------------------------------------+
// | if (interfaces)| BATADV_CMD_GET_INTERFACES    | Nlmsghdr par interface       | attrs: IFNAME, IFINDEX, FLAGS, ADDR, HARD_IFNAME/INDEX      |
// +---------------+-------------------------------+-----------------------------+--------------------------------------------------------------+
