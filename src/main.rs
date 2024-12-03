use colored::Colorize;
use std::env;
use std::net::UdpSocket;

use sysinfo::{self, System};

fn get_local_ip() -> String {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(local_addr) = socket.local_addr() {
                    return local_addr.ip().to_string();
                } else {
                    return "Failed to get local address.".to_string();
                }
            } else {
                return "Network unreachable or failed to connect.".to_string();
            }
        }
        Err(_) => "Failed to bind to a socket.".to_string(),
    }
}
// Helper function to format bytes into human-readable units
fn bytes_to_human_readable(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1000.0 && unit < UNITS.len() - 1 {
        size /= 1000.0;
        unit += 1;
    }

    format!("{:.2} {}", size, UNITS[unit])
}

fn print_spec_value(name: colored::ColoredString, value: String) {
    let arrow: colored::ColoredString = "==>".truecolor(112, 112, 112);

    println!("{}\t{} {}", name, arrow, value);
}

fn print_system_specs(sys: &mut System) {
    sys.refresh_memory();
    if let Some(user) = env::var_os("USER") {
        print_spec_value("Name".red(), user.into_string().unwrap());
    } else {
        println!("Could not determine the current user.");
    }

    print_spec_value("Host".blue(), System::host_name().unwrap());

    print_spec_value(
        "OS".green(),
        format!(
            "{} {}",
            System::name().unwrap(),
            System::os_version().unwrap()
        ),
    );

    print_spec_value("Kernel".yellow(), System::kernel_version().unwrap());

    sys.refresh_memory();

    print_spec_value(
        "Memory".purple(),
        format!(
            "{}/{}",
            bytes_to_human_readable(sys.used_memory()),
            bytes_to_human_readable(sys.total_memory())
        ),
    );

    print_spec_value(
        "Swap".cyan(),
        format!(
            "{}/{}",
            bytes_to_human_readable(sys.used_swap()),
            bytes_to_human_readable(sys.total_swap())
        ),
    );

    print_spec_value("Ip".magenta(), get_local_ip());

    let uptime_seconds = System::uptime();

    let days = uptime_seconds / 86400;
    let hours = (uptime_seconds % 86400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    let seconds = uptime_seconds % 60;

    print_spec_value(
        "Uptime".green(),
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds),
    );
}

fn main() {
    let mut sys: System = System::new_all();

    print_system_specs(&mut sys);
}
