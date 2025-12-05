use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiscordQuest {
    pub id: String,
    pub config: QuestConfig,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestConfig {
    pub starts_at: String,
    pub expires_at: String,
    pub features: Vec<i32>,
    pub messages: QuestMessages,
    pub rewards_config: RewardsConfig,
    pub assets: QuestAssets,
    pub application: QuestApplication,
    pub task_config: QuestTaskConfig,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestApplication {
    pub id: String,
    pub name: String,
    pub link: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestAssets {
    pub hero: String,
    pub game_tile: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestMessages {
    pub game_title: String,
    pub game_publisher: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestTaskConfig {
    #[serde(rename = "type")]
    pub config_type: i32,
    pub tasks: BTreeMap<String, QuestTask>,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestTask {
    pub event_name: String,
    pub target: i32,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RewardsConfig {
    pub rewards: Vec<QuestReward>,
    pub platforms: Vec<i32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestReward {
    #[serde(rename = "type")]
    pub reward_type: i32,
    pub messages: QuestRewardMessages,
    pub sku_id: Option<String>,
    pub orb_quantity: Option<i32>,
    pub asset: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuestRewardMessages {
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CollectibleProduct {
    pub items: Vec<CollectibleItem>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CollectibleItem {
    pub asset: String,
}
