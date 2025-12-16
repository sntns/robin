use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ClientFlags: u32 {
        const DEL      = 1 << 0;
        const ROAM     = 1 << 1;
        const WIFI     = 1 << 4;
        const ISOLA    = 1 << 5;
        const NOPURGE  = 1 << 8;
        const NEW      = 1 << 9;
        const PENDING  = 1 << 10;
        const TEMP     = 1 << 11;
    }
}
