use serde_json::Value;
use serde_json_path::JsonPath;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{LinesWithEndings, as_24_bit_terminal_escaped},
};

use crate::command::Command;

pub struct JsonCommand {
    data: Value,
    path: Option<JsonPath>,
}

impl JsonCommand {
    pub fn new(data: &str, path: Option<&str>) -> anyhow::Result<Self> {
        let data: Value = serde_json::from_str(data)?;
        let path = path.map(|p| JsonPath::parse(p).unwrap());
        Ok(Self { data, path })
    }
}

impl Command for JsonCommand {
    fn execute(&self) -> anyhow::Result<()> {
        self.print();
        Ok(())
    }
}

impl JsonCommand {
    fn print(&self) {
        let json_str = match &self.path {
            Some(path) => {
                let res = path.query(&self.data);
                serde_json::to_string_pretty(&res)
            }
            None => serde_json::to_string_pretty(&self.data),
        }
        .unwrap();

        JsonCommand::print_json(&json_str);
    }

    fn print_json(data: &str) {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let syntax = ps
            .find_syntax_by_name("JSON")
            .expect("JSON syntax not found");
        let theme = &ts.themes["base16-ocean.dark"];

        let mut h = HighlightLines::new(syntax, theme);

        for line in LinesWithEndings::from(data) {
            let ranges: Vec<(Style, &str)> =
                h.highlight_line(line, &ps).expect("Fail to highlight line");
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            print!("{}", escaped);
        }
    }
}
