use std::{thread, time};
use ntk_common::{ip::add_default_gateway, relay, tun::{create_tun_device, DeviceType}, vsock::vsock_connect, HOST_IP, HOST_PORT};
use tun::AbstractDevice;

fn main() -> Result<(), String> {
    let mut tun_dev = create_tun_device(DeviceType::Enclave)?;
    let tun_dev_name = tun_dev.tun_name().unwrap();
    println!("TUN device {} connected.", tun_dev_name);

    add_default_gateway(tun_dev_name.as_str(), HOST_IP);

    loop {
        let connect_result = vsock_connect(HOST_PORT);
        match connect_result {
            Err(e) => {
                println!("Encountered error while connecting: {:?}", e);
            }
            
            Ok(vsock) => {
                println!("Vsock connected.");
                
                let mut vsock = vsock;
                relay(&mut vsock, &mut tun_dev);
            }
        }

        println!("Sleeping 5 secs before retrying.");
        thread::sleep(time::Duration::from_secs(5));
    }

    Ok(())
}