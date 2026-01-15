/// BATMAN-adv Netlink attributes (from `linux/uapi/batman_adv.h`).
///
/// These attributes are used when communicating with the kernel via
/// the BATMAN-adv generic netlink interface. They represent various
/// settings, metrics, and state information for mesh interfaces,
/// translation tables, neighbors, gateways, and protocol features.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    /// Unspecified / placeholder attribute.
    BatadvAttrUnspec = 0,

    /// BATMAN-adv protocol version.
    BatadvAttrVersion = 1,

    /// Name of the routing algorithm in use.
    BatadvAttrAlgoName = 2,

    /// Mesh interface index.
    BatadvAttrMeshIfindex = 3,

    /// Mesh interface name.
    BatadvAttrMeshIfname = 4,

    /// MAC address of the mesh interface.
    BatadvAttrMeshAddress = 5,

    /// Hard interface index.
    BatadvAttrHardIfindex = 6,

    /// Hard interface name.
    BatadvAttrHardIfname = 7,

    /// MAC address of the hard interface.
    BatadvAttrHardAddress = 8,

    /// Originator (source) MAC address.
    BatadvAttrOrigAddress = 9,

    /// Throughput meter result.
    BatadvAttrTpMeterResult = 10,

    /// Throughput test duration in seconds.
    BatadvAttrTpMeterTestTime = 11,

    /// Number of bytes transferred in throughput test.
    BatadvAttrTpMeterBytes = 12,

    /// Throughput meter cookie.
    BatadvAttrTpMeterCookie = 13,

    /// Padding attribute (unused).
    BatadvAttrPad = 14,

    /// Interface active state.
    BatadvAttrActive = 15,

    /// Translation table client MAC address.
    BatadvAttrTtAddress = 16,

    /// Translation table TTVN.
    BatadvAttrTtTtvn = 17,

    /// Last TTVN in translation table.
    BatadvAttrTtLastTtvn = 18,

    /// CRC32 for translation table entry.
    BatadvAttrTtCrc32 = 19,

    /// VLAN ID for translation table entry.
    BatadvAttrTtVid = 20,

    /// Flags for translation table entry (see `ClientFlags`).
    BatadvAttrTtFlags = 21,

    /// Marks the "best" client in the translation table.
    BatadvAttrFlagBest = 22,

    /// Last seen timestamp in milliseconds.
    BatadvAttrLastSeenMsecs = 23,

    /// Neighbor MAC address.
    BatadvAttrNeighAddress = 24,

    /// Transmission quality (TQ) metric.
    BatadvAttrTq = 25,

    /// Throughput in bytes per second (optional, BATMAN_V).
    BatadvAttrThroughput = 26,

    /// Gateway bandwidth upstream (kbit/s).
    BatadvAttrBandwidthUp = 27,

    /// Gateway bandwidth downstream (kbit/s).
    BatadvAttrBandwidthDown = 28,

    /// Gateway MAC address for router.
    BatadvAttrRouter = 29,

    /// Backbone link advertisement (BLA) own attribute.
    BatadvAttrBlaOwn = 30,

    /// BLA MAC address.
    BatadvAttrBlaAddress = 31,

    /// BLA VLAN ID.
    BatadvAttrBlaVid = 32,

    /// BLA backbone flag.
    BatadvAttrBlaBackbone = 33,

    /// BLA CRC.
    BatadvAttrBlaCrc = 34,

    /// DAT IPv4 address.
    BatadvAttrDatCacheIp4Address = 35,

    /// DAT hardware (MAC) address.
    BatadvAttrDatCacheHwAddress = 36,

    /// DAT VLAN ID.
    BatadvAttrDatCacheVid = 37,

    /// Multicast flags.
    BatadvAttrMcastFlags = 38,

    /// Private multicast flags.
    BatadvAttrMcastFlagsPriv = 39,

    /// VLAN ID.
    BatadvAttrVlanId = 40,

    /// OGMs aggregation enabled.
    BatadvAttrAggregatedOgmsEnabled = 41,

    /// AP isolation enabled.
    BatadvAttrApIsolationEnabled = 42,

    /// Isolation mark.
    BatadvAttrIsolationMark = 43,

    /// Isolation mask.
    BatadvAttrIsolationMask = 44,

    /// Bonding enabled flag.
    BatadvAttrBondingEnabled = 45,

    /// Bridge loop avoidance enabled flag.
    BatadvAttrBridgeLoopAvoidanceEnabled = 46,

    /// Distributed ARP table enabled flag.
    BatadvAttrDistributedArpTableEnabled = 47,

    /// Fragmentation enabled flag.
    BatadvAttrFragmentationEnabled = 48,

    /// Gateway bandwidth downstream.
    BatadvAttrGwBandwidthDown = 49,

    /// Gateway bandwidth upstream.
    BatadvAttrGwBandwidthUp = 50,

    /// Gateway mode (off/client/server).
    BatadvAttrGwMode = 51,

    /// Gateway selection class.
    BatadvAttrGwSelClass = 52,

    /// Hop penalty.
    BatadvAttrHopPenalty = 53,

    /// Log level.
    BatadvAttrLogLevel = 54,

    /// Force multicast flooding enabled.
    BatadvAttrMulticastForceFloodEnabled = 55,

    /// Network coding enabled.
    BatadvAttrNetworkCodingEnabled = 56,

    /// Originator interval.
    BatadvAttrOrigInterval = 57,

    /// ELP interval.
    BatadvAttrElpInterval = 58,

    /// Throughput override.
    BatadvAttrThroughputOverride = 59,

    /// Multicast fanout.
    BatadvAttrMulticastFanout = 60,
}

impl From<Attribute> for u16 {
    fn from(a: Attribute) -> Self {
        a as u16
    }
}
