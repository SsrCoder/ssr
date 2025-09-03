mod crontab;
mod json;
mod timestamp;

pub use crontab::CrontabCommand;
pub use json::JsonCommand;
pub use timestamp::TimestampCommand;

pub trait Command {
    fn execute(&self) -> anyhow::Result<()>;
}
