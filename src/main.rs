use std::os::raw::c_ulong;
pub const SIOCGIFINDEX: c_ulong = 0x8933;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("socket");
    let socket = unsafe { libc::socket(libc::AF_CAN,
        libc::SOCK_RAW | libc::SOCK_CLOEXEC,
        socketcan_sys::CAN_RAW as libc::c_int)
    };
    if socket < 0 {
        return Err(std::io::Error::last_os_error())?;
    }

    eprintln!("SIOCGIFINDEX");
    // note: maybeuninit to initialize fields is bad news according to docs, so just zero the whole
    // thing first (as this is a valid state)
    let mut sa: socketcan_sys::sockaddr_can = unsafe { std::mem::zeroed() };
    sa.can_family = libc::AF_CAN as socketcan_sys::__kernel_sa_family_t;
    sa.can_ifindex = unsafe {
        let mut ifr: ifstructs::ifreq = std::mem::zeroed();
        ifr.set_name("vcan0")?;
        let r = libc::ioctl(socket, SIOCGIFINDEX, &ifr);
        if r < 0 {
            return Err(std::io::Error::last_os_error())?;
        }

        ifr.ifr_ifru.ifr_ifindex
    } as i32;

    eprintln!("bind");
    unsafe {
        let r = libc::bind(socket, &sa as *const _ as *mut _, std::mem::size_of_val(&sa) as u32);
        if r < 0 {
            return Err(std::io::Error::last_os_error())?;
        }
    }


    Ok(())
}
