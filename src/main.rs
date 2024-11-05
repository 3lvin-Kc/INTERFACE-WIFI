use std::io;
use log::{info, warn, error};

mod wifi;
fn main() {
    env_logger::init(); 

    info!("Starting the application");

    // List all wireless interfaces
    let wireless_interfaces = match wifi::list_wireless_interfaces() {
        Ok(interfaces) => {
            if interfaces.is_empty() {
                println!("No wireless interfaces found.");
                warn!("No wireless interfaces were detected.");
                return;
            }
            interfaces
        },
        Err(e) => {
            error!("Error listing wireless interfaces: {}", e);
            eprintln!("Failed to list wireless interfaces: {}", e);
            return;
        }
    };

    // interfaces and user-selection
    info!("Wireless interfaces found: {:?}", wireless_interfaces);
    println!("\nEnter the name of the interface you want to set to monitor mode:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let selected_interface = input.trim();

    // Validate
    if !wireless_interfaces.contains(&selected_interface.to_string()) {
        println!("Invalid selection. Exiting.");
        warn!("User selected an invalid interface: {}", selected_interface);
        return;
    }

    // Confirm  before setting to monitor mode
    println!("Are you sure you want to set {} to monitor mode? (y/n)", selected_interface);
    let mut confirm_input = String::new();
    io::stdin().read_line(&mut confirm_input).expect("Failed to read confirmation input");
    if confirm_input.trim().to_lowercase() != "y" {
        println!("Operation canceled.");
        info!("User canceled the operation for setting the interface to monitor mode.");
        return;
    }

    // interface to monitor mode
    match wifi::set_interface_to_monitor_mode(selected_interface) {
        Ok(_) => info!("Successfully set {} to monitor mode.", selected_interface),
        Err(e) => {
            error!("Failed to set {} to monitor mode: {}", selected_interface, e);
            eprintln!("Failed to set interface to monitor mode: {}", e);
        }
    }
}
