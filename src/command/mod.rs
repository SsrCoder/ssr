mod crontab;
mod json;
mod timestamp;
mod translate;

pub use crontab::CrontabCommand;
pub use json::JsonCommand;
pub use timestamp::TimestampCommand;
pub use translate::TranslateCommand;

pub trait Command {
    fn execute(&self) -> anyhow::Result<()>;
}
