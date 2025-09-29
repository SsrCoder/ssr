use std::{fmt::Display, io::stdout, ops::Deref};

use anyhow::Result;
use crossterm::{ExecutableCommand, style::Print, terminal::Clear};
use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
};

use crate::{
    command::Command,
    config::{self},
};

#[derive(Debug)]
pub enum Language {
    Chinese,
    English,
}

impl TryFrom<&str> for Language {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().deref() {
            "cn" | "chinese" => Ok(Language::Chinese),
            "en" | "english" => Ok(Language::English),
            _ => Err(anyhow::anyhow!("unsupport language: {value}")),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Language::Chinese => "chinese",
            Language::English => "english",
        };
        write!(f, "{}", text)
    }
}

pub struct TranslateCommand {
    text: String,
    from: Option<Language>,
    to: Language,
}

impl TranslateCommand {
    pub fn new<T: Into<String>>(
        text: T,
        from: Option<&str>,
        to: Option<&str>,
    ) -> anyhow::Result<TranslateCommand> {
        let from = match from {
            Some(lang) => Some(lang.try_into()?),
            None => None,
        };
        Ok(Self {
            text: text.into(),
            from: from.try_into()?,
            to: to.unwrap_or("chinese").try_into()?,
        })
    }
}

impl TranslateCommand {
    fn build_message(&self) -> Vec<ChatCompletionMessage> {
        let system = match &self.from {
            Some(lang) => {
                format!(
                    "Please translate the given text from {} to {}",
                    lang, self.to
                )
            }
            None => {
                format!("Please translate the given text into {}", self.to)
            }
        };

        vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(system),
                ..Default::default()
            },
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(self.text.to_string()),
                ..Default::default()
            },
        ]
    }

    async fn send_request(
        &self,
        key: impl Into<String>,
        base_url: impl Into<String>,
        model: &str,
        messages: Vec<ChatCompletionMessage>,
    ) -> anyhow::Result<ChatCompletionMessage> {
        let credentials = Credentials::new(key, base_url);
        let chat_completion = ChatCompletion::builder(model, messages)
            .credentials(credentials)
            .create()
            .await?;
        let choice = chat_completion.choices.first().unwrap();
        Ok(choice.message.clone())
    }

    async fn run(&self) -> anyhow::Result<()> {
        let ai = config::get_ai_provider(&config::CFG.translate.ai_provider)
            .expect("translate ai provider not set or disabled");
        let key = &ai.key;
        let base_url = &ai.base_url;
        let model = &ai.model;

        stdout().execute(Print("请求处理中！"))?;

        let messages = self.build_message();
        let resp = self.send_request(key, base_url, model, messages).await?;

        stdout()
            .execute(Clear(crossterm::terminal::ClearType::CurrentLine))?
            .execute(Print(format!("\r{}\n", resp.content.unwrap().trim())))?;
        Ok(())
    }
}

impl Command for TranslateCommand {
    fn execute(&self) -> anyhow::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()?
            .block_on(self.run())
    }
}
