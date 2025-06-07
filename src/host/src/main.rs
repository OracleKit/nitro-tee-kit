use std::{os::fd::{AsRawFd, OwnedFd}, sync::Arc};

use nix::{libc::VMADDR_CID_ANY, sys::socket::{accept, bind, listen, socket, AddressFamily, Backlog, SockFlag, SockType, VsockAddr}};
use ntk_common::{relay, ENCLAVE_IP, HOST_IP, HOST_PORT, TUN_NETMASK};
use tun::{Configuration, Device, AbstractDevice};

fn create_tun_device() -> Device {
    let mut config = Configuration::default();
    config
        .address(HOST_IP)
        .destination(ENCLAVE_IP)
        .netmask(TUN_NETMASK)
        .up();

    let tun_dev = tun::create(&config).unwrap();
    tun_dev
}

fn listen_vsock() -> Result<OwnedFd, ()> {
    let socket = socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::empty(),
        None
    ).unwrap();

    let sockaddr = VsockAddr::new(VMADDR_CID_ANY, HOST_PORT);

    bind(socket.as_raw_fd(), &sockaddr).unwrap();
    listen(&socket, Backlog::new(128).unwrap()).unwrap();

    Ok(socket)
}

fn main() {
    let tun_dev = create_tun_device();
    let tun_dev = Arc::new(tun_dev);
    println!("TUN device {} connected.", tun_dev.tun_name().unwrap());

    let listener = listen_vsock().unwrap();
    let vsock = accept(listener.as_raw_fd()).unwrap();
    println!("Vsock connected.");

    relay(vsock, tun_dev);
}