use ntk_common::{ip::add_forwarding_rules, relay, tun::{create_tun_device, DeviceType}, vsock::vsock_listen, HOST_PORT, SUBNET};
use tun::AbstractDevice;

fn main() -> Result<(), String> {
    let mut tun_dev = create_tun_device( DeviceType::Host )?;
    let tun_dev_name = tun_dev.tun_name().unwrap();
    println!("TUN device {} connected.", &tun_dev_name);

    add_forwarding_rules(&tun_dev_name, SUBNET);

    let listener = vsock_listen( HOST_PORT )?;
    for vsock in listener.incoming() {
        match vsock {
            Ok(vsock) => {
                println!("Vsock connected!");

                let mut vsock = vsock;
                relay(&mut vsock, &mut tun_dev);
            },
            Err(e) => return Err(format!("Error encountered in vsock listener, {:?}", e))
        };
    };

    Ok(())
}