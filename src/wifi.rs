use nix::ifaddrs::getifaddrs;
use nix::sys::socket::{AddressFamily,SockaddrLike};
use std::process::Command;
use std::io;
use log::{info, error};

/// wireless interfaces
pub fn list_wireless_interfaces() -> Result<Vec<String>, io::Error> {
    let mut interfaces = Vec::new();
    for ifaddr in getifaddrs().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to get network interfaces: {}", e)))? {
        if let Some(address) = ifaddr.address {
            if let Some(AddressFamily::Packet)= address.family() {
                if ifaddr.interface_name.starts_with("wlan") {
                    interfaces.push(ifaddr.interface_name);
                }
            }
        }
    }
    Ok(interfaces)
}

/// Checking cmd on the system or not
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Setting monitor mode
pub fn set_interface_to_monitor_mode(interface: &str) -> Result<(), io::Error> {
    //  `ip` and `iw` commands checking here
    if !command_exists("ip") {
        error!("The `ip` command is not available on this system.");
        return Err(io::Error::new(io::ErrorKind::NotFound, "`ip` command not found"));
    }
    if !command_exists("iw") {
        error!("The `iw` command is not available on this system.");
        return Err(io::Error::new(io::ErrorKind::NotFound, "`iw` command not found"));
    }

    // user confirmation 
    println!("Are you sure you want to set {} to monitor mode? (y/n)", interface);
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Failed to read input");
    if confirmation.trim() != "y" {
        println!("Operation cancelled.");
        return Ok(());
    }

    // Bring the interface down
    info!("Bringing interface {} down", interface);
    let status = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg(interface)
        .arg("down")
        .status()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to bring the interface down: {}", e)))?;

    if !status.success() {
        error!("Failed to bring {} down. Status: {}", interface, status);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to bring the interface down"));
    }

    // Setting monitor mode
    info!("Setting interface {} to monitor mode", interface);
    let status = Command::new("sudo")
        .arg("iw")
        .arg("dev")
        .arg(interface)
        .arg("set")
        .arg("type")
        .arg("monitor")
        .status()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to set monitor mode: {}", e)))?;

    if !status.success() {
        error!("Failed to set {} to monitor mode. Status: {}", interface, status);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to set monitor mode"));
    }

    // Bring up interface
    info!("Bringing interface {} up", interface);
    let status = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg(interface)
        .arg("up")
        .status()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to bring the interface up: {}", e)))?;

    if !status.success() {
        error!("Failed to bring {} up. Status: {}", interface, status);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to bring the interface up"));
    }
    
    info!("Interface {} successfully set to monitor mode", interface);
    print!("successfully set to monitor mode");
    Ok(())
}


