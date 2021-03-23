use std::fmt;
use chrono::prelude; // for current time
use serde::{Deserialize, Serialize}; // json serialization
use std::env; // CLI args
use std::fs; // reading files
use std::io; // for io::Error
use std::thread; // for sleeping
use std::time; // for sleep duration

#[derive(Serialize, Deserialize)]
struct Header {
    version: u8,
}

#[derive(Serialize, Deserialize)]
struct Status {
    name: String,
    full_text: String,
    separator: bool,
}

impl fmt::Display for Status {
    fn fmt(self: &Status, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t\t{}", self.name, self.full_text)
    }
}

struct StatusList {
    elements: Vec<Status>,
}

impl StatusList {
    // TODO: make dynamic/configurable
    fn new() -> StatusList {
        StatusList {
            elements: vec![
                battery_status(),
                input_volume(),
                output_volume(),
                date_time(),
                current_ip(),
            ],
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut repeat = false;
    for arg in env::args() {
        if arg == "--loop" {
            repeat = true;
            break
        }
    }

    if repeat {
        // systemstatus --loop
        i3bar_loop();
    } else {
        let status_list = StatusList::new();
        for item in &status_list.elements {
            println!("{}", &item);
        }
    }
    Ok(())
}

fn print_statusline(statusline: &str) {
    println!("{},", &statusline);
}

fn format_statusline(line: &StatusList) -> String {
    let mut formatted = String::from("[");
    for element in &line.elements {
        formatted.push_str(&serde_json::to_string(&element).unwrap_or("UNKNOWN".to_string()));
        formatted.push(',');
    }
    formatted.push(']');
    return formatted;
}

fn i3bar_loop() {
    let header = serde_json::to_string(&Header { version: 1 }).unwrap();
    println!("{}", &header);
    // start endless array per i3bar-protocol
    println!("[");
    let duration = time::Duration::from_millis(1000);
    loop {
        print_statusline(&format_statusline(&StatusList::new()));
        thread::sleep(duration);
    }
}

fn battery_status() -> Status {
    let b_status = fs::read_to_string("/sys/class/power_supply/BAT0/status");
    match b_status {
        Ok(status) => {
            let b_capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity");
            match b_capacity {
                Ok(capacity) => Status {
                    name: String::from("Battery"),
                    full_text: format!("{} @ {}%", status.trim_end(), capacity.trim_end()),
                    separator: true,
                },
                Err(_) => Status {
                    name: String::from("Battery"),
                    full_text: "UNKNOWN".to_string(),
                    separator: true,
                },
            }
        }
        Err(_) => Status {
            name: String::from("Battery"),
            full_text: "UNKNOWN".to_string(),
            separator: true,
        },
    }
}

// TODO
fn output_volume() -> Status {
    Status {
        name: String::from("Audio Out"),
        full_text: "VOL OUT".to_string(),
        separator: true,
    }
}

// TODO
fn input_volume() -> Status {
    Status {
        name: String::from("Audio In"),
        full_text: "VOL IN".to_string(),
        separator: true,
    }
}

// TODO
fn current_ip() -> Status {
    Status {
        name: String::from("IP Address"),
        full_text: "IP".to_string(),
        separator: true,
    }
}

fn date_time() -> Status {
    Status {
        name: String::from("Date/Time"),
        full_text: prelude::Local::now().format("%H:%M %a, %d.%m.%Y").to_string(),
        separator: true,
    }
}
