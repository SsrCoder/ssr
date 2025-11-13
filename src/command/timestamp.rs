use super::Command;
use chrono::Datelike;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use colored::Colorize;
use crossterm::{clipboard::CopyToClipboard, execute};

pub struct TimestampCommand {
    timestamp: Option<i64>,
}

impl TimestampCommand {
    pub fn new(timestamp: Option<i64>) -> Self {
        Self { timestamp }
    }
}

impl Command for TimestampCommand {
    fn execute(&self) -> anyhow::Result<()> {
        match self.timestamp {
            Some(ts) => {
                let now = Local::now().timestamp();

                let utc_time = if ts >= 1e12 as i64 {
                    Utc.timestamp_opt(ts / 1000, (ts % 1000) as u32).unwrap()
                } else {
                    Utc.timestamp_opt(ts, 0).unwrap()
                };
                let time = utc_time.with_timezone(&Local);

                let mut diff = now - ts;
                let mut after = false;
                if diff < 0 {
                    after = true;
                    diff = -diff;
                }

                let second = diff % 60;
                let minute = diff / 60 % 60;
                let hour = diff / 60 / 60 % 24;
                let day = diff / 60 / 60 / 24;
                let time_dir = if after { "后" } else { "前" };

                let relative = if diff >= 0 && diff < 60 {
                    format!("{second}秒{time_dir}")
                } else if diff >= 60 && diff < 60 * 60 {
                    format!("{minute}分钟 {second}秒{time_dir}")
                } else if diff >= 60 * 60 && diff < 24 * 60 * 60 {
                    format!("{hour}小时 {minute}分钟 {second}秒{time_dir}")
                } else {
                    format!("{day}天 {hour}小时 {minute}分钟 {second}秒{time_dir}")
                };

                println!("{:16} {}", "Unix Timestamp".green(), ts.to_string());
                println!(
                    "{:16} {} {}",
                    "GMT".red(),
                    utc_time.to_string(),
                    utc_time.weekday()
                );
                println!(
                    "{:16} {} {}",
                    "Local".blue(),
                    time.to_string(),
                    utc_time.weekday()
                );
                println!("{:16} {}", "Relative".yellow(), relative);
            }
            None => {
                let now = Local::now();
                let timestamp = now.timestamp().to_string();
                println!("current timestamp: {}", &timestamp);

                execute!(
                    std::io::stdout(),
                    CopyToClipboard::to_clipboard_from(timestamp)
                )
                .expect("fail to set clipboard content");
            }
        }
        Ok(())
    }
}
