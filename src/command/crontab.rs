use colored::*;
use std::str::FromStr;

use cron::{Schedule, TimeUnitSpec};

use crate::command::Command;

pub struct CrontabCommand {
    schedule: Schedule,
}

impl CrontabCommand {
    pub fn new(expression: &str) -> anyhow::Result<Self> {
        let schedule = if expression.split(" ").count() == 5 {
            cron::Schedule::from_str(&format!("0 {expression}"))?
        } else {
            cron::Schedule::from_str(expression)?
        };
        Ok(Self { schedule })
    }
}

impl Command for CrontabCommand {
    fn execute(&self) -> anyhow::Result<()> {
        println!("{}", "Specs: ".green());
        println!("Seconds: {}", self.schedule.seconds().to_string());
        println!("Minutes: {}", self.schedule.minutes().to_string());
        println!("Hours: {}", self.schedule.hours().to_string());
        println!(
            "Day of Month: {}",
            self.schedule.days_of_month().to_string()
        );
        println!("Month: {}", self.schedule.months().to_string());
        println!("Day of Week: {}", self.schedule.days_of_week().to_string());

        println!("\n{}", "Next 10 times datetime: ".blue());
        for datetime in self.schedule.upcoming(chrono::Local).take(10) {
            println!("{}", datetime);
        }
        Ok(())
    }
}

trait Stringify {
    fn to_string(&self) -> String;
}

impl<T: TimeUnitSpec> Stringify for T {
    fn to_string(&self) -> String {
        let vec: Vec<_> = self.iter().collect();
        format!("{:?}", vec)
    }
}
