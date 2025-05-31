use std::{os::fd::{AsRawFd, RawFd}, thread};
use nix::sys::socket::{recv, send, MsgFlags};
use tun::Device;

pub const ENCLAVE_IP: &str = "";
pub const HOST_IP: &str = "";
pub const TUN_NETMASK: &str = "";
pub const ENCLAVE_CID: u32 = 10;
pub const HOST_PORT: u32 = 9000;

fn forward_packets(from: RawFd, to: RawFd) {
    let mut buf = [0u8; 1504];

    loop {
        let read_bytes = match recv(from, &mut buf, MsgFlags::empty()) {
            Ok(b) => b,
            Err(nix::errno::Errno::EINTR) => continue,
            Err(e) => panic!("Error encountered while reading: {}", e)
        };

        // possible connection dead. TODO: retry connecting?
        if read_bytes == 0 { continue; }

        let mut sent_bytes = 0;
        while sent_bytes < read_bytes {
            sent_bytes += match send(to, &buf[sent_bytes..read_bytes], MsgFlags::empty()) {
                Ok(0) => 0, // possibly dead connection. TODO
                Ok(b) => b,
                Err(nix::errno::Errno::EINTR) => continue,
                Err(e) => panic!("Error encountered while reading: {}", e)
            };
        }
    }
}

pub fn relay(vsock: RawFd, tun_dev: &Device) {
    let tun_dev = tun_dev.as_raw_fd();
    let mut handles = vec!{};

    {
        let vsock = vsock.clone();
        let tun_dev = tun_dev.clone();
        let handle = thread::spawn(move || { forward_packets(vsock, tun_dev); });
        handles.push(handle);
    }

    {
        let vsock = vsock.clone();
        let tun_dev = tun_dev.clone();
        let handle = thread::spawn(move || { forward_packets(tun_dev, vsock); });
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}