use std::process::{Command, ExitStatus};

fn run_command(program: &str, args: &[&str]) -> Result<(), ExitStatus> {
    let status = Command::new(program)
        .args(args)
        .status()
        .expect("Command exeuction failed");

    if !status.success() {
        return Err(status);
    }

    Ok(())
}

pub fn add_default_gateway(dev_name: &str, gateway_ip: &str) {
    run_command("ip", &["route", "del", "default"])
        .map_err(|e| { format!("ip route del failed with exit status: {}", e) })
        .unwrap();

    run_command("ip", &["route", "add", "default", "dev", dev_name, "via", gateway_ip])
        .map_err(|e| { format!("ip route add failed with exit status: {}", e) })
        .unwrap();
}