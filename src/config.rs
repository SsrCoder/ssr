use lazy_static::lazy_static;
use std::{env, fs, path::Path};

use mlua::{Lua, LuaSerdeExt};
use serde::Deserialize;

fn ai_config_default_base_url() -> String {
    openai::DEFAULT_BASE_URL.to_string()
}

fn ai_config_default_model() -> String {
    String::from("GPT-5")
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AiConfig {
    #[serde(default)]
    pub enable: bool,
    #[serde(default = "ai_config_default_base_url")]
    pub base_url: String,
    #[serde(default)]
    pub key: String,
    #[serde(default = "ai_config_default_model")]
    pub model: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub ai: AiConfig,
}

lazy_static! {
    pub static ref CFG: Config = load_config();
}

fn load_config() -> Config {
    let lua = Lua::new();
    let home_path = env::var("HOME").expect("fail to find home dir");
    let path = Path::new(&home_path).join(".config/ssr/init.lua");
    if !path.exists() {
        return Config::default();
    }
    let lua_code = fs::read_to_string(path).expect("fail to read file");
    let config: Config = lua.from_value(lua.load(lua_code).eval().unwrap()).unwrap();
    config
}
