use chrono::{Datelike, Local};
use colored::*;
use std::str::FromStr;

use crate::command::Command;

pub struct CrontabCommand {
    cron: croner::Cron,
}

impl CrontabCommand {
    pub fn new(expression: &str) -> anyhow::Result<Self> {
        let cron = croner::Cron::from_str(expression)?;
        Ok(Self { cron })
    }
}

impl Command for CrontabCommand {
    fn execute(&self) -> anyhow::Result<()> {
        print!("{}", "Specs: ".green());
        let pattern = &self.cron.pattern;
        println!("{}", pattern.describe());

        println!("\n{}", "Next 10 times datetime: ".blue());
        for datetime in self.cron.iter_after(Local::now()).take(10) {
            println!("{} {}", datetime, datetime.weekday());
        }
        Ok(())
    }
}
