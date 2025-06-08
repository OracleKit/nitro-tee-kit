use std::{ops::{Deref, DerefMut}, os::fd::{AsFd, AsRawFd, BorrowedFd}};
use tun::{Configuration, Device as TunDevice};

use crate::{ENCLAVE_IP, HOST_IP, TUN_NETMASK};

pub struct Device {
    tun: TunDevice
}

impl Device {
    fn new(tun: TunDevice) -> Self {
        Self { tun }
    }

    pub fn inner(&self) -> &TunDevice {
        &self.tun
    }

    pub fn inner_mut(&mut self) -> &mut TunDevice {
        &mut self.tun
    }
}

impl Deref for Device {
    type Target = TunDevice;

    fn deref(&self) -> &Self::Target {
        &self.tun
    }
}

impl DerefMut for Device {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tun
    }
}

impl AsFd for Device {
    fn as_fd(&self) -> BorrowedFd<'_> {
        let fd = self.tun.as_raw_fd();

        unsafe {
            BorrowedFd::borrow_raw(fd)
        }
    }
}

pub enum DeviceType {
    Host,
    Enclave
}

pub fn create_tun_device(t: DeviceType) -> Result<Device, String> {
    let mut config = Configuration::default();
    let (address, destination) = match t {
        DeviceType::Enclave => ( ENCLAVE_IP, HOST_IP ),
        DeviceType::Host => ( HOST_IP, ENCLAVE_IP )
    };

    config
        .address(address)
        .destination(destination)
        .netmask(TUN_NETMASK)
        .up();

    let tun = tun::create(&config)
        .map_err(|e| format!("Unable to create TUN device: {:?}", e))?;

    Ok(Device::new(tun))
}