use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "bindIP", default)]
    pub bind_ip: String,
    pub credentials: GameCredentials,
    #[serde(default)]
    pub settings: GameSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameCredentials {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    #[serde(rename = "characterName", default)]
    pub character_name: String,
    #[serde(default)]
    pub vocation: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSettings {
    #[serde(rename = "huntTarget", default)]
    pub hunt_target: String,
    #[serde(rename = "autoHunt", default = "default_true")]
    pub auto_hunt: bool,
    #[serde(rename = "autoSell", default = "default_true")]
    pub auto_sell: bool,
    #[serde(rename = "autoBless", default = "default_true")]
    pub auto_bless: bool,
    #[serde(rename = "autoDaily", default = "default_true")]
    pub auto_daily: bool,
    #[serde(rename = "autoTasks", default = "default_true")]
    pub auto_tasks: bool,
    #[serde(rename = "autoQuiver", default = "default_true")]
    pub auto_quiver: bool,
    #[serde(rename = "autoCoinMarket", default = "default_true")]
    pub auto_coin_market: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterInfo {
    pub name: String,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub vocation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonsterTask {
    pub key: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub ativa: bool,
    #[serde(default)]
    pub completa: bool,
    #[serde(default)]
    pub kills: u32,
    #[serde(default)]
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterState {
    pub name: String,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub vocation: String,
    #[serde(default)]
    pub hp: u32,
    #[serde(default)]
    pub mana: u32,
    #[serde(default)]
    pub gold: i64,
    #[serde(default)]
    pub coins: i64,
    #[serde(default)]
    pub blessings: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundPacket {
    pub t: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub characters: Option<Vec<CharacterInfo>>,
    #[serde(default)]
    pub character: Option<CharacterState>,
    #[serde(default)]
    pub tasks: Option<Vec<MonsterTask>>,
    #[serde(default)]
    pub from: Option<String>,
}
