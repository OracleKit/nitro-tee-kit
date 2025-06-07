use std::{io::ErrorKind, os::fd::RawFd, sync::Arc, thread};
use nix::{sys::socket::{recv, send, MsgFlags}};
use tun::Device;

pub const ENCLAVE_IP: &str = "10.0.0.2";
pub const HOST_IP: &str = "10.0.0.1";
pub const TUN_NETMASK: &str = "255.255.255.0";
pub const ENCLAVE_CID: u32 = 10;
pub const HOST_CID: u32 = 3;
pub const HOST_PORT: u32 = 5005;

fn forward_packets_vsock_to_tun(vsock: &RawFd, tun: &Device) {
    let mut buf = [0u8; 1504];
    let vsock = vsock.clone();

    loop {
        let read_bytes = match recv(vsock, &mut buf, MsgFlags::empty()) {
            Ok(b) => b,
            Err(nix::errno::Errno::EINTR) => continue,
            Err(e) => {
                println!("Error while reading from vsock: {:?}", e);
                return;
            }
        };

        // possible connection dead. TODO: retry connecting?
        if read_bytes == 0 { continue; }

        let mut sent_bytes = 0;
        while sent_bytes < read_bytes {
            sent_bytes += match tun.send(&buf[sent_bytes..read_bytes]) {
                Ok(b) => b,
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => 0,
                    _ => {
                        println!("Error while writing to TUN: {:?}", e);
                        return;
                    }
                }
            };
        }
    }
}

fn forward_packets_tun_to_vsock(vsock: &RawFd, tun: &Device) {
    let mut buf = [0u8; 1504];
    let vsock = vsock.clone();

    loop {
        let read_bytes = match tun.recv(&mut buf) {
            Ok(b) => b,
            Err(e) => match e.kind() {
                ErrorKind::Interrupted => 0,
                _ => {
                    println!("Error while writing to TUN: {:?}", e);
                    return;
                }
            }
        };

        // possible connection dead. TODO: retry connecting?
        if read_bytes == 0 { continue; }

        let mut sent_bytes = 0;
        while sent_bytes < read_bytes {
            sent_bytes += match send(vsock, &buf[sent_bytes..read_bytes], MsgFlags::empty()) {
                Ok(b) => b,
                Err(nix::errno::Errno::EINTR) => continue,
                Err(e) => {
                    println!("Error while writing to vsock: {:?}", e);
                    return;
                }
            };
        }
    }
}

pub fn relay(vsock: RawFd, tun_dev: Arc<Device>) {
    let mut handles = vec!{};

    {
        let tun_dev = tun_dev.clone();
        let handle = thread::spawn(move || {
            forward_packets_vsock_to_tun(&vsock, &tun_dev);
        });
        handles.push(handle);
    }

    {
        let tun_dev = tun_dev.clone();
        let handle = thread::spawn(move || {
            forward_packets_tun_to_vsock(&vsock, &tun_dev);
        });
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}