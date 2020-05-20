use std::os::raw::c_ulong;
pub const SIOCGIFINDEX: c_ulong = 35123;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = nix::sys::socket::socket(
        nix::sys::socket::AddressFamily::Can,
        nix::sys::socket::SockType::Raw,
        nix::sys::socket::SockFlag::SOCK_CLOEXEC,
        None)?;

    // note: maybeuninit to initialize fields is bad news according to docs, so just zero the whole
    // thing first (as this is a valid state)
    let mut sa: socketcan_sys::sockaddr_can = unsafe { std::mem::zeroed() };
    sa.can_family = libc::AF_CAN as socketcan_sys::__kernel_sa_family_t;
    sa.can_ifindex = unsafe {
        let mut ifr: ifstructs::ifreq = std::mem::zeroed();
        ifr.set_name("can0")?;
        let r = libc::ioctl(socket, SIOCGIFINDEX, &ifr);
        if r < 0 {
            return Err(std::io::Error::last_os_error())?;
        }

        ifr.ifr_ifru.ifr_ifindex
    } as i32;

    unsafe {
        let r = libc::bind(socket, &sa as *const _ as *mut _, std::mem::size_of_val(&sa) as u32);
        if r < 0 {
            return Err(std::io::Error::last_os_error())?;
        }
    }


    Ok(())
}
