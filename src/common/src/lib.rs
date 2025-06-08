use std::{io::{ErrorKind, Read, Write}, os::fd::AsFd};
use nix::{errno::Errno, poll::{poll, PollFd, PollFlags, PollTimeout}};
use crate::tun::Device;
use ::vsock::VsockStream;

pub mod vsock;
pub mod tun;

pub const ENCLAVE_IP: &str = "10.0.0.2";
pub const HOST_IP: &str = "10.0.0.1";
pub const TUN_NETMASK: &str = "255.255.255.0";
pub const ENCLAVE_CID: u32 = 10;
pub const HOST_CID: u32 = 3;
pub const HOST_PORT: u32 = 5005;

fn forward<S: Read, D: Write>(src: &mut S, dest: &mut D) -> bool {
    let mut buf = [0u8; 1504];

    loop {
        match src.read(&mut buf) {
            Ok(_) => break,
            Err(e) => match e.kind() {
                ErrorKind::Interrupted => continue,
                _ => return true
            }
        }
    };

    match dest.write_all(&mut buf) {
        Ok(()) => (),
        Err(_) => return true
    };

    return false;
}

fn poll_and_ignore_interrupts(fds: &mut [PollFd], timeout: PollTimeout) -> Result<i32, Errno> {
    loop {
        let result = poll(fds, timeout);
        match result {
            Err(Errno::EINTR) => continue,
            _ => return result
        }
    }
}

pub fn relay(vsock: &mut VsockStream, tun_dev: &mut Device) {
    loop {
        let mut pollfds = [
            PollFd::new(vsock.as_fd(), PollFlags::POLLIN),
            PollFd::new(tun_dev.as_fd(), PollFlags::POLLIN),

        ];

        match poll_and_ignore_interrupts(&mut pollfds, PollTimeout::NONE) {
            Ok(_) => (),
            Err(e) => {
                println!("Error encountered while polling: {}", e);
                return;
            }
        };

        let is_vsock_ready = pollfds[0].any().unwrap_or_default();
        let is_tun_ready = pollfds[1].any().unwrap_or_default();

        let mut disconnect = false;
        if is_vsock_ready {
            disconnect = forward(vsock, tun_dev.inner_mut());
        } else if is_tun_ready {
            disconnect = forward(tun_dev.inner_mut(), vsock);
        }

        // TODO: If TUN device disconnects or errors, more handling
        if disconnect {
            return;
        }
    }
}