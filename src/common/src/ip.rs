use std::process::{Command, ExitStatus};

fn run_command(command: &str) -> Result<(), ExitStatus> {
    let status = Command::new("bash")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Command exeuction failed");

    if !status.success() {
        return Err(status);
    }

    Ok(())
}

pub fn add_default_gateway(dev_name: &str, gateway_ip: &str) {
    let _ = run_command("ip route del default");

    run_command(&format!("ip route add default dev {} via {}", dev_name, gateway_ip))
        .map_err(|e| { format!("ip route add failed with exit status: {}", e) })
        .unwrap();
}

pub fn add_forwarding_rules(dev_name: &str, subnet: &str) {
    run_command("echo 1 > /proc/sys/net/ipv4/ip_forward")
        .map_err(|e| { format!("echo 1 > ip_forward failed with exit status: {}", e) })
        .unwrap();

    let _ = run_command("nft add table ip nat");
    let _ = run_command("nft add chain ip nat POSTROUTING { type nat hook postrouting priority 100 \\; }");

    run_command(&format!("nft add rule ip nat POSTROUTING ip saddr {} masquerade", subnet))
        .map_err(|e| { format!("adding nat masquerade rule failed with exit status: {}", e) })
        .unwrap();

    let _ = run_command("nft add table ip filter");
    let _ = run_command("nft add chain ip filter FORWARD { type filter hook forward priority 0 \\; policy drop \\; }");

    run_command(&format!("nft add rule ip filter FORWARD iifname \"{}\" accept", dev_name))
        .map_err(|e| { format!("adding forward rule from tun failed with exit status: {}", e) })
        .unwrap();

    run_command(&format!("nft add rule ip filter FORWARD oifname \"{}\" ct state established,related accept", dev_name))
        .map_err(|e| { format!("adding forward rule from tun failed with exit status: {}", e) })
        .unwrap();
}