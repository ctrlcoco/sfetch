use colored::Colorize;
use std::env;
use std::net::UdpSocket;

use sysinfo::{self, System};

use std::fmt::Write;

fn get_colors1() -> String {
    let mut colors1 = String::new();

    for i in 0..8 {
        // Append the formatted string to colors1
        write!(colors1, "\x1b[4{}m   ", i).unwrap();
    }

    // Reset the formatting
    write!(colors1, "\x1b[0m").unwrap();

    colors1
}

fn get_colors2() -> String {
    let mut colors2 = String::new();

    for i in 8..16 {
        // Append the formatted string to colors2
        write!(colors2, "\x1b[48;5;{}m   ", i).unwrap();
    }

    // Reset the formatting
    write!(colors2, "\x1b[0m").unwrap();

    colors2
}

// fn main() {
//     // ASCII Art
//     let ascii_art = [
//         "         _______   ",
//         "        /       \\  ",
//         "       |  (o) (o) |",
//         "       |     ^    |",
//         "        \\_______/ ",
//     ];
//
//     // Corresponding text lines to appear on the right
//     let text_lines = [
//         "Rust welcomes you!",
//         "With some cool art.",
//         "Stay safe, and code!",
//         "Let's build together.",
//         "",
//     ];
//
//     // Display ASCII art and text side-by-side
//     for (art_line, text_line) in ascii_art.iter().zip(text_lines.iter()) {
//         println!("{:<20} {}", art_line, text_line);
//     }
// }

fn get_custom_uptime() -> String {
    let uptime_seconds = System::uptime();

    let days = uptime_seconds / 86400;
    let hours = (uptime_seconds % 86400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    let seconds = uptime_seconds % 60;

    format!(
        "{}{}{}{}",
        if days > 0 {
            format!("{}d", days)
        } else {
            String::new()
        },
        if days > 0 || hours > 0 {
            format!("{}h ", hours)
        } else {
            String::new()
        },
        if days > 0 || hours > 0 || minutes > 0 {
            format!("{}m ", minutes)
        } else {
            String::new()
        },
        format!("{}s", seconds)
    )
}

fn get_local_ip() -> String {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(local_addr) = socket.local_addr() {
                    local_addr.ip().to_string()
                } else {
                    "Failed to get local address.".to_string()
                }
            } else {
                "Network unreachable.".to_string()
            }
        }
        Err(_) => "Failed to bind to a socket.".to_string(),
    }
}

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

    match env::var("USER") {
        Ok(user) => print_spec_value("Name".bright_red(), user.to_string()),
        Err(_) => println!("SHELL environment variable is not set."),
    }

    print_spec_value("Host".bright_blue(), System::host_name().unwrap());

    print_spec_value(
        "OS".bright_green(),
        format!(
            "{} {}",
            System::name().unwrap(),
            System::os_version().unwrap()
        ),
    );

    print_spec_value("Kernel".bright_yellow(), System::kernel_version().unwrap());

    sys.refresh_memory();

    print_spec_value(
        "Memory".bright_purple(),
        format!(
            "{}/{}",
            bytes_to_human_readable(sys.total_memory() - sys.used_memory()),
            bytes_to_human_readable(sys.total_memory())
        ),
    );

    print_spec_value(
        "Swap".bright_cyan(),
        format!(
            "{}/{}",
            bytes_to_human_readable(sys.used_swap()),
            bytes_to_human_readable(sys.total_swap())
        ),
    );

    print_spec_value("Ip".bright_magenta(), get_local_ip());
    print_spec_value("Uptime".bright_red(), get_custom_uptime());

    match env::var("SHELL") {
        Ok(shell) => print_spec_value(
            "Shell".bright_green(),
            shell.split('/').last().unwrap().to_string(),
        ),
        Err(_) => println!("SHELL environment variable is not set."),
    }

    match env::var("TERM") {
        Ok(term) => print_spec_value("Term".bright_yellow(), term),
        Err(_) => println!("TERM environment variable is not set."),
    }

    println!("{}", get_colors1());
    println!("{}", get_colors2());
}

fn main() {
    let mut sys: System = System::new_all();

    print_system_specs(&mut sys);
}
