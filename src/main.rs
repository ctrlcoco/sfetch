use colored::{Color, Colorize};
use std::{env, net::UdpSocket};
use sysinfo::{self, System};

const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

// Common environment variables for DE
const DESKTOP_ENV_VARS: [&str; 3] = ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "GDMSESSION"];

fn clear_term() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_spec_value(name: colored::ColoredString, value: String) {
    let arrow: colored::ColoredString = "=>".truecolor(112, 112, 112);
    println!("{}\t{} {}", name, arrow, value);
}

fn get_desktop_environment() -> Option<String> {
    for var in &DESKTOP_ENV_VARS {
        if let Ok(value) = env::var(var) {
            return Some(value);
        }
    }
    None
}

fn get_bright_colors() -> String {
    // Preallocate the string with an estimated capacity
    // 8 colors * 11 chars each + 4 for reset
    let mut colors = String::with_capacity(8 * 11 + 4);
    for i in 8..16 {
        colors.push_str(&format!("\x1b[48;5;{}m   ", i));
    }
    colors.push_str("\x1b[0m");

    colors
}

fn get_custom_uptime() -> String {
    let uptime_seconds: u64 = System::uptime();

    let days: u64 = uptime_seconds / 86400;
    let hours: u64 = (uptime_seconds % 86400) / 3600;
    let minutes: u64 = (uptime_seconds % 3600) / 60;
    let seconds: u64 = uptime_seconds % 60;

    let mut uptime_str = String::new();

    if days > 0 {
        uptime_str.push_str(&format!("{}d ", days));
    }
    if days > 0 || hours > 0 {
        uptime_str.push_str(&format!("{}h ", hours));
    }
    if days > 0 || hours > 0 || minutes > 0 {
        uptime_str.push_str(&format!("{}m ", minutes));
    }
    uptime_str.push_str(&format!("{}s", seconds));

    uptime_str
}

fn get_local_ip() -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                return local_addr.ip().to_string();
            }
            return "Failed to get local address.".into();
        }
        return "Network unreachable.".into();
    }
    "Failed to bind to a socket.".into()
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

fn print_system_specs(sys: &mut System) {
    match env::var("USER") {
        Ok(user) => {
            let to_print: String = format!("{}@{}", user, System::host_name().unwrap());
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
        "Cpu".color(Color::BrightGreen),
        format!("{} ", sys.cpus()[0].brand()),
    );

    sys.refresh_memory();

    print_spec_value(
        "Memory".color(Color::BrightYellow),
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

    print_spec_value(
        "Kernel".color(Color::BrightMagenta),
        System::kernel_version().unwrap(),
    );

    println!("{}", get_bright_colors());
}

fn help() {
    println!(
        "usage:
    sfetch

    [options]
    -v | --version to see version info
    -h | --help to get this message and exit
    "
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let text: &str = &args[1];

            // checks flag
            match text {
                "--help" | "-h" => help(),
                "--version" | "-v" => {
                    println!("sfetch {}", env!("CARGO_PKG_VERSION"));
                }

                _ => {
                    println!("Unknown flag: {}", text);
                }
            }
        }

        _ => {
            clear_term();

            let mut sys: System = System::new_all();

            print_system_specs(&mut sys);

            println!("\n");
        }
    }
}
