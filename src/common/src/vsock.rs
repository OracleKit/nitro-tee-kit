use vsock::{VsockAddr, VsockListener, VsockStream, VMADDR_CID_ANY, VMADDR_CID_HOST};

pub fn vsock_listen(port: u32) -> Result<VsockListener, String> {
    let address = VsockAddr::new(VMADDR_CID_ANY, port);
    VsockListener::bind(&address)
        .map_err(|e| format!("Could not bind to {}: {:?}", address, e))
}

pub fn vsock_connect(port: u32) -> Result<VsockStream, String> {
    let address = VsockAddr::new(VMADDR_CID_HOST, port);
    VsockStream::connect(&address)
        .map_err(|e| format!("Could not connect to {}: {:?}", address, e))
}