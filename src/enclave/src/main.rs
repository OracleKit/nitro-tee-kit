use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use nix::sys::socket::{connect, socket, AddressFamily, MsgFlags, SockFlag, SockType, VsockAddr};
use ntk_common::{relay, ENCLAVE_CID, ENCLAVE_IP, HOST_IP, HOST_PORT, TUN_NETMASK};
use tun::{AbstractDevice, Configuration, Device};

fn create_tun_device() -> Device {
    let mut config = Configuration::default();
    config
        .address(ENCLAVE_IP)
        .destination(HOST_IP)
        .netmask(TUN_NETMASK)
        .up();

    let tun_dev = tun::create(&config).unwrap();
    tun_dev
}

fn vsock_connect() -> Result<OwnedFd, ()> {
    let sockaddr = VsockAddr::new(ENCLAVE_CID, HOST_PORT);
    let socket = socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::empty(),
        None
    ).unwrap();

    match connect(socket.as_raw_fd(), &sockaddr) {
        Ok(_) => Ok(socket),
        Err(_) => Err(())
    }
}

fn main() {
    let tun_dev = create_tun_device();
    println!("TUN device {} connected.", tun_dev.tun_name().unwrap());

    let vsock = vsock_connect().unwrap();
    println!("Vsock connected.");

    relay(vsock.as_raw_fd(), &tun_dev);
}