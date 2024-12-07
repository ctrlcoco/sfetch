use colored::{Color, Colorize};
use sysinfo::{self, System};

use std::{env, fmt::Write, net::UdpSocket};

const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

// const COLORS: [Color; 7] = [
//     Color::BrightRed,
//     Color::BrightGreen,
//     Color::BrightYellow,
//     Color::BrightBlue,
//     Color::BrightMagenta,
//     Color::BrightCyan,
//     Color::BrightWhite,
// ];

fn clear_term() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_desktop_environment() -> Option<String> {
    // Common environment variables for DE
    let variables = ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "GDMSESSION"];

    for var in &variables {
        if let Ok(value) = env::var(var) {
            return Some(value);
        }
    }
    None
}

fn get_bright_colors() -> String {
    let mut colors2 = String::new();

    for i in 8..16 {
        write!(colors2, "\x1b[48;5;{}m   ", i).unwrap();
    }

    write!(colors2, "\x1b[0m").unwrap();

    colors2
}

fn get_custom_uptime() -> String {
    let uptime_seconds: u64 = System::uptime();

    let days: u64 = uptime_seconds / 86400;
    let hours: u64 = (uptime_seconds % 86400) / 3600;
    let minutes: u64 = (uptime_seconds % 3600) / 60;
    let seconds: u64 = uptime_seconds % 60;

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
    let mut size: f64 = bytes as f64;
    let mut unit: usize = 0;

    while size >= 1000.0 && unit < UNITS.len() - 1 {
        size /= 1000.0;
        unit += 1;
    }

    format!("{:.2} {}", size, UNITS[unit])
}

fn print_spec_value(name: colored::ColoredString, value: String) {
    let arrow: colored::ColoredString = "=>".truecolor(112, 112, 112);
    println!("{}\t{} {}", name, arrow, value);
}

fn print_system_specs(sys: &mut System) {
    match env::var("USER") {
        Ok(user) => {
            let to_print: String = format!("{}@{}", user.to_string(), System::host_name().unwrap());
            println!("{}", to_print.color(Color::BrightBlue));
            println!("{}", "=".repeat(to_print.len()));
        }
        Err(_) => println!("SHELL environment variable is not set."),
    }

    print_spec_value(
        "OS".color(Color::BrightRed),
        format!(
            "{} {}",
            System::name().unwrap(),
            System::os_version().unwrap()
        ),
    );

    print_spec_value(
        "Kernel".color(Color::BrightYellow),
        System::kernel_version().unwrap(),
    );

    sys.refresh_memory();

    print_spec_value(
        "Memory".color(Color::BrightMagenta),
        format!(
            "{}/{}",
            bytes_to_human_readable(sys.total_memory() - sys.used_memory()),
            bytes_to_human_readable(sys.total_memory())
        ),
    );

    print_spec_value(
        "Swap".color(Color::BrightCyan),
        format!(
            "{}/{}",
            bytes_to_human_readable(sys.used_swap()),
            bytes_to_human_readable(sys.total_swap())
        ),
    );

    print_spec_value("Ip".color(Color::BrightMagenta), get_local_ip());
    print_spec_value("Uptime".color(Color::BrightRed), get_custom_uptime());

    match env::var("SHELL") {
        Ok(shell) => print_spec_value(
            "Shell".color(Color::BrightGreen),
            shell.split('/').last().unwrap().to_string(),
        ),
        Err(_) => println!("SHELL environment variable is not set."),
    }

    match env::var("TERM") {
        Ok(term) => print_spec_value("Term".color(Color::BrightYellow), term),
        Err(_) => println!("TERM environment variable is not set."),
    }

    if let Some(de) = get_desktop_environment() {
        print_spec_value("De/Wm".color(Color::BrightCyan), de);
    }

    println!("{}", get_bright_colors());
}

fn main() {
    clear_term();

    let mut sys: System = System::new_all();

    print_system_specs(&mut sys);

    println!("\n");
}
