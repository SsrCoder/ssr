mod command;
use std::ops::Deref;

use crate::command::Command;
use crate::command::CrontabCommand;
use crate::command::JsonCommand;
use crate::command::TimestampCommand;
use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand)]
enum Commands {
    /// Print and copy current timestamp in local, or parse and display target timestamp info
    #[clap(name = "timestamp", alias = "ts")]
    Timestamp { timestamp: Option<i64> },

    /// JSON parse with syntax highlight
    Json {
        /// JSON data parse and display with syntax highlight
        data: String,

        /// JSON path for query from data, RFC: https://www.rfc-editor.org/rfc/rfc9535.html
        #[arg(short, long)]
        path: Option<String>,
    },

    // Parse crontab expression and display next 10 times' datetime
    #[clap(name = "crontab", alias = "cron")]
    Crontab { expression: String },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

impl Cli {
    fn execute(&self) -> anyhow::Result<()> {
        let cmd: Box<dyn Command> = match &self.commands {
            Commands::Timestamp { timestamp } => Box::new(TimestampCommand::new(timestamp.clone())),
            Commands::Json { data, path } => {
                Box::new(JsonCommand::new(data.clone(), path.clone())?)
            }
            Commands::Crontab { expression } => Box::new(CrontabCommand::new(expression.deref())?),
        };
        cmd.execute()?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
