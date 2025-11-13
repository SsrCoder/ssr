use crate::command::Command;

pub enum Direction {
    Encode,
    Decode,
}

pub struct UrlCommand {
    text: String,
    direction: Direction,
}

impl UrlCommand {
    pub fn new(text: &str, direction: Direction) -> Self {
        Self {
            text: text.to_string(),
            direction,
        }
    }
}

impl Command for UrlCommand {
    fn execute(&self) -> anyhow::Result<()> {
        let result = match self.direction {
            Direction::Encode => urlencoding::encode(&self.text).into_owned(),
            Direction::Decode => urlencoding::decode(&self.text)?.into_owned(),
        };
        println!("{}", result);
        Ok(())
    }
}