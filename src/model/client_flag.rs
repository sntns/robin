use bitflags::bitflags;

bitflags! {
    /// Flags representing the state or behavior of a BATMAN-adv client.
    ///
    /// These flags are associated with entries in the translation table (TT)
    /// and describe attributes such as whether the client is roaming,
    /// isolated, or temporarily connected.
    #[doc = "Flags representing the state or behavior of a BATMAN-adv client."]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ClientFlags: u32 {
        /// Client has been deleted from the translation table.
        const DEL      = 1 << 0;

        /// Client is currently roaming between interfaces.
        const ROAM     = 1 << 1;

        /// Client is connected via Wi-Fi.
        const WIFI     = 1 << 4;

        /// Client is isolated (AP isolation is enabled).
        const ISOLA    = 1 << 5;

        /// Client should not be purged from the translation table automatically.
        const NOPURGE  = 1 << 8;

        /// Client is newly detected in the translation table.
        const NEW      = 1 << 9;

        /// Client entry is pending (not fully validated yet).
        const PENDING  = 1 << 10;

        /// Client entry is temporary.
        const TEMP     = 1 << 11;
    }
}
