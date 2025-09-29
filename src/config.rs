use lazy_static::lazy_static;
use std::fs;

use mlua::{Lua, LuaSerdeExt};
use serde::Deserialize;

fn ai_config_default_base_url() -> String {
    openai::DEFAULT_BASE_URL.to_string()
}

fn ai_config_default_model() -> String {
    String::from("GPT-5")
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AiProvider {
    #[serde(default)]
    pub enable: bool,
    pub name: String,
    #[serde(default = "ai_config_default_base_url")]
    pub base_url: String,
    #[serde(default)]
    pub key: String,
    #[serde(default = "ai_config_default_model")]
    pub model: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AiConfig {
    pub default: String,
    #[serde(default)]
    pub providers: Vec<AiProvider>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct TranslateConfig {
    #[serde(default)]
    pub ai_provider: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub translate: TranslateConfig,
}

pub fn get_ai_provider(name: &str) -> Option<AiProvider> {
    if CFG.ai.providers.len() == 0 {
        return None;
    }
    let name = if name != "" { name } else { &CFG.ai.default };
    if name != "" {
        let provider = CFG.ai.providers.iter().find(|p| p.name == name);
        if provider.is_none_or(|p| !p.enable) {
            None
        } else {
            provider.cloned()
        }
    } else {
        None
    }
}

lazy_static! {
    pub static ref CFG: Config = load_config();
}

fn load_config() -> Config {
    let base_dirs = cross_xdg::BaseDirs::new().unwrap();
    let path = base_dirs.state_home().join("ssr/init.lua");
    if !path.exists() {
        return Config::default();
    }

    let lua = Lua::new();
    let lua_code = fs::read_to_string(path).expect("fail to read file");
    let config: Config = lua.from_value(lua.load(lua_code).eval().unwrap()).unwrap();
    config
}
