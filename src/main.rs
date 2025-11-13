mod command;
mod config;

use crate::command::Command;
use crate::command::CrontabCommand;
use crate::command::Direction;
use crate::command::JsonCommand;
use crate::command::TimestampCommand;
use crate::command::TranslateCommand;
use crate::command::UrlCommand;
use clap::Parser;
use clap::Subcommand;
use std::io::Read;
use tracing::level_filters::LevelFilter;
use tracing::*;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::RollingFileAppender;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Subcommand)]
enum Commands {
    /// Alias: [ts]. Print and copy current timestamp in local, or parse and display target timestamp info
    #[clap(name = "timestamp", alias = "ts")]
    Timestamp { timestamp: Option<i64> },

    /// JSON parse with syntax highlight
    Json {
        /// JSON data parse and display with syntax highlight
        data: Option<String>,

        /// JSON path for query from data, RFC: https://www.rfc-editor.org/rfc/rfc9535.html
        #[arg(short, long)]
        path: Option<String>,

        /// Compress output json
        #[arg(short, long)]
        compress: bool,
    },

    /// Alias: [cron]. Parse crontab expression and display next 10 times datetime
    #[clap(name = "crontab", alias = "cron")]
    Crontab { expression: String },

    /// Translate by ai
    #[clap(name = "translate", alias = "trans")]
    Translate {
        /// Text you want to translate
        text: String,

        /// Which language you want to translate from
        #[arg(short, long)]
        from: Option<String>,

        /// Which language you want to translate to, default: CN
        #[arg(short, long)]
        to: Option<String>,
    },

    /// Url encode/decode
    #[clap(name = "url")]
    Url {
        /// Text to encode/decode
        text: String,

        /// Decode instead of encode
        #[arg(short, long)]
        decode: bool,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

impl Cli {
    fn execute(&self) -> anyhow::Result<()> {
        let cmd: &dyn Command = match &self.commands {
            Commands::Timestamp { timestamp } => &TimestampCommand::new(*timestamp),
            Commands::Json {
                data,
                path,
                compress,
            } => {
                let data = match data {
                    Some(data) => data.clone(),
                    None => {
                        let mut buf = String::new();
                        std::io::stdin().read_to_string(&mut buf)?;
                        buf
                    }
                };
                &JsonCommand::new(&data, path.as_deref(), *compress)?
            }
            Commands::Crontab { expression } => &CrontabCommand::new(expression)?,
            Commands::Translate { text, from, to } => {
                &TranslateCommand::new(text, from.as_deref(), to.as_deref())?
            }
            Commands::Url { text, decode } => {
                let direction = if *decode {
                    Direction::Decode
                } else {
                    Direction::Encode
                };
                &UrlCommand::new(text, direction)
            }
        };
        cmd.execute()?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let _guards = init_log()?;
    let cli = Cli::parse();
    cli.execute()
}

fn init_log() -> anyhow::Result<Vec<WorkerGuard>> {
    let base_dirs = cross_xdg::BaseDirs::new()?;
    let path = base_dirs.state_home().join("ssr");

    let error_file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("error")
        .filename_suffix("log")
        .max_log_files(30)
        .build(&path)?;
    let (error_non_blocking, _guard1) = tracing_appender::non_blocking(error_file_appender);

    let info_file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("info")
        .filename_suffix("log")
        .max_log_files(30)
        .build(&path)?;
    let (info_non_blocking, _guard2) = tracing_appender::non_blocking(info_file_appender);

    let all_file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("all")
        .filename_suffix("log")
        .max_log_files(30)
        .build(&path)?;
    let (all_non_blocking, _guard3) = tracing_appender::non_blocking(all_file_appender);

    // let (stdout_non_blocking, _guard3) = tracing_appender::non_blocking(std::io::stdout());

    let error_file_layer = tracing_subscriber::fmt::Layer::new()
        .with_writer(error_non_blocking)
        .with_ansi(false)
        .compact()
        .with_filter(LevelFilter::from_level(Level::WARN));

    let info_file_layer = tracing_subscriber::fmt::Layer::new()
        .with_writer(info_non_blocking)
        .with_ansi(false)
        .compact()
        .with_filter(LevelFilter::from_level(Level::INFO));

    let all_file_layer = tracing_subscriber::fmt::Layer::new()
        .with_writer(all_non_blocking)
        .with_ansi(false)
        .compact()
        .with_filter(LevelFilter::from_level(Level::TRACE));

    tracing_subscriber::registry()
        .with(error_file_layer)
        .with(info_file_layer)
        .with(all_file_layer)
        .init();
    Ok(vec![_guard1, _guard2, _guard3])
}
