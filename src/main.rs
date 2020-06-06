use chrono::prelude::Local;
use serde::{Deserialize, Serialize};
use std::{fs, thread, time};

#[derive(Serialize, Deserialize)]
struct Header {
    version: u8,
}

#[derive(Serialize, Deserialize)]
struct Status {
    full_text: String,
    separator: bool,
}

fn main() {
    let header = serde_json::to_string(&Header { version: 1 }).unwrap();
    let status = Status {
        full_text: "Hello, Sway!".to_string(),
        separator: true,
    };
    let status2 = Status {
        full_text: "Hallo, Ana!".to_string(),
        separator: true,
    };
    let mut statusline = format!(
        "[{},{}]",
        &serde_json::to_string(&status).unwrap(),
        &serde_json::to_string(&status2).unwrap()
    );

    println!("{}", &header);
    // start endless array
    println!("[");

    // first an empty status
    println!("[],");

    let duration = time::Duration::from_millis(1000);
    // then my "fancy" status
    loop {
        print_statusline(&statusline);
        thread::sleep(duration);
        statusline = format!(
            "[{},{},{},{},{},{}]",
            &serde_json::to_string(&Status {
                full_text: "IDLE".to_string(),
                separator: true
            })
            .unwrap(),
            &serde_json::to_string(&input_volume()).unwrap(),
            &serde_json::to_string(&output_volume()).unwrap(),
            &serde_json::to_string(&current_ip()).unwrap(),
            &serde_json::to_string(&battery_status()).unwrap(),
            &serde_json::to_string(&date_time()).unwrap(),
        );
    }
}

fn print_statusline(statusline: &str) {
    println!("{},", &statusline);
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
                    full_text: "".to_string(),
                    separator: true,
                },
            }
        }
        Err(_) => Status {
            full_text: "".to_string(),
            separator: true,
        },
    }
}

fn output_volume() -> Status {
    Status {
        full_text: "VOL OUT".to_string(),
        separator: true,
    }
}

fn input_volume() -> Status {
    Status {
        full_text: "VOL IN".to_string(),
        separator: true,
    }
}

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
