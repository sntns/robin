/// Converts a VLAN ID stored in a `u16` to a printable integer.
///
/// The `vid` format uses the highest bit (bit 15) as a validity flag:
/// - If bit 15 is set, the lower 12 bits contain the actual VLAN ID.
/// - If bit 15 is not set, the VLAN ID is considered invalid.
///
/// # Arguments
/// - `vid`: The raw VLAN ID value (`u16`) from the kernel.
///
/// # Returns
/// - The VLAN ID as `i32` if valid (bit 15 set).
/// - `-1` if the VLAN ID is invalid (bit 15 not set).
///
/// # Example
/// ```
/// let vid: u16 = 0x8005; // bit 15 set, VLAN ID = 5
/// assert_eq!(print_vid(vid), 5);
///
/// let invalid_vid: u16 = 0x0005; // bit 15 not set
/// assert_eq!(print_vid(invalid_vid), -1);
/// ```
pub fn print_vid(vid: u16) -> i32 {
    if (vid & (1 << 15)) != 0 {
        (vid & 0x0fff) as i32
    } else {
        -1
    }
}
