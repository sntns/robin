use libc::if_indextoname;
use std::ffi::CStr;

/// Convert ifindex (u32) into a network interface name
pub fn get_ifname_from_index(ifindex: u32) -> Result<String, std::io::Error> {
    // libc::IFNAMSIZ = 16
    let mut buf: [libc::c_char; libc::IFNAMSIZ] = [0; libc::IFNAMSIZ];

    let ptr = unsafe { if_indextoname(ifindex, buf.as_mut_ptr()) };
    if ptr.is_null() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "if_indextoname failed",
        ));
    }

    let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
    Ok(cstr.to_string_lossy().into_owned())
}
