use std::time::Duration;
use chrono::Local;


pub fn run(seconds: u64, format: &str) -> String {

    let now = Local::now() + Duration::from_secs(seconds);

    let time = match format {
        "default" => now.format("%Y-%m-%d %H:%M:%S").to_string(),
        "rfc3339" => now.to_rfc3339(),
        "unix" => now.timestamp().to_string(),
        "iso" => now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        _ => format!("Unknown format: {}", format),
    };
    format!("The time is {:?}", time)
}