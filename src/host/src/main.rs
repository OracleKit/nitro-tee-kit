use ntk_common::{relay, tun::{create_tun_device, DeviceType}, vsock::vsock_listen, HOST_PORT};
use tun::AbstractDevice;

fn main() -> Result<(), String> {
    let mut tun_dev = create_tun_device( DeviceType::Host )?;
    println!("TUN device {} connected.", tun_dev.tun_name().unwrap());

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