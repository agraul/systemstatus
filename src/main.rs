use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use std::{env, fs, thread, time};

#[derive(Serialize, Deserialize)]
struct Header {
    version: u8,
}

#[derive(Serialize, Deserialize)]
struct Status {
    full_text: String,
    separator: bool,
}

struct StatusList {
    elements: Vec<Status>,
}

// TODO: make dynamic/configurable
fn build_status_list() -> StatusList {
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

fn main() {
    let mut repeat = false;
    for arg in env::args() {
        if arg == "--loop" {
            repeat = true;
        }
    }

    if repeat {
        // systemstatus --loop
        i3bar_loop();
    } else {
        // let status_list = build_status_list();

        println!("foo");
    }
}

fn print_statusline(statusline: &str) {
    println!("{},", &statusline);
}

fn format_statusline(line: StatusList) -> String {
    let mut formatted: String = "[".to_string();
    for element in line.elements {
        formatted.push_str(&serde_json::to_string(&element).unwrap_or("UNKNOWN".to_string()));
        formatted.push(',');
    }
    formatted.push(']');
    return formatted;
}

fn i3bar_loop() {
    let mut statusline: String;
    let header = serde_json::to_string(&Header { version: 1 }).unwrap();
    println!("{}", &header);
    // start endless array per i3bar-protocol
    println!("[");
    let duration = time::Duration::from_millis(1000);
    loop {
        statusline = format_statusline(build_status_list());
        print_statusline(&statusline);
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
                    full_text: format!("{} @ {}%", status.trim_end(), capacity.trim_end()),
                    separator: true,
                },
                Err(_) => Status {
                    full_text: "UNKNOWN".to_string(),
                    separator: true,
                },
            }
        }
        Err(_) => Status {
            full_text: "UNKNOWN".to_string(),
            separator: true,
        },
    }
}

// TODO
fn output_volume() -> Status {
    Status {
        full_text: "VOL OUT".to_string(),
        separator: true,
    }
}

// TODO
fn input_volume() -> Status {
    Status {
        full_text: "VOL IN".to_string(),
        separator: true,
    }
}

// TODO
fn current_ip() -> Status {
    Status {
        full_text: "IP".to_string(),
        separator: true,
    }
}

fn date_time() -> Status {
    Status {
        full_text: Local::now().format("%H:%M %a, %d.%m.%Y").to_string(),
        separator: true,
    }
}
