use crate::ClientFlags;
use macaddr::MacAddr6;

/// A single entry in the batman-adv transglobal table (TT).
///
/// The transglobal table contains information about clients known across the entire
/// mesh network, including the client's MAC address, the originator node, and the
/// route state.
#[derive(Debug, Clone)]
pub struct TransglobalEntry {
    /// MAC address of the client.
    /// Corresponds to `BATADV_ATTR_TT_ADDRESS`.
    pub client: MacAddr6,

    /// MAC address of the originator announcing this client.
    /// Corresponds to `BATADV_ATTR_ORIG_ADDRESS`.
    pub orig: MacAddr6,

    /// VLAN ID associated with this client.
    /// Corresponds to `BATADV_ATTR_TT_VID`.
    pub vid: u16,

    /// Transglobal table version used for this client.
    /// Corresponds to `BATADV_ATTR_TT_TTVN`.
    pub ttvn: u8,

    /// Last known transglobal table version.
    /// Corresponds to `BATADV_ATTR_TT_LAST_TTVN`.
    pub last_ttvn: u8,

    /// Flags associated with the client, wrapped in `ClientFlags`.
    /// Corresponds to `BATADV_ATTR_TT_FLAGS`.
    pub flags: ClientFlags,

    /// CRC32 checksum for this entry.
    /// Corresponds to `BATADV_ATTR_TT_CRC32`.
    pub crc32: u32,

    /// Indicates if this route is considered the best route to this client.
    /// Corresponds to `BATADV_ATTR_FLAG_BEST`.
    pub is_best: bool,
}

/// A single entry in the batman-adv translocal table (TL).
///
/// The translocal table contains clients directly known by the local node,
/// including last-seen timestamps and flags.
pub struct TranslocalEntry {
    /// MAC address of the client.
    pub client: MacAddr6,

    /// VLAN ID associated with this client.
    pub vid: u16,

    /// Flags associated with the client, wrapped in `ClientFlags`.
    pub flags: ClientFlags,

    /// CRC32 checksum for this entry.
    pub crc32: u32,

    /// Seconds since the last update for this entry.
    pub last_seen_secs: u32,

    /// Milliseconds since the last update for this entry.
    pub last_seen_msecs: u32,
}
