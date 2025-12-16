pub fn print_vid(vid: u16) -> i32 {
    if (vid & (1 << 15)) != 0 {
        (vid & 0x0fff) as i32
    } else {
        -1
    }
}
