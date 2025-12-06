use crate::types::quest::DiscordQuest;
use std::collections::HashSet;
use std::hash::BuildHasher;

#[must_use]
pub fn filter_new_quests<S: BuildHasher>(fetched_quests: Vec<DiscordQuest>, known_ids: &HashSet<String, S>) -> Vec<DiscordQuest> {
    fetched_quests.into_iter()
        .filter(|q| !known_ids.contains(&q.id))
        .collect()
}

#[must_use]
pub fn format_quest_message(quest: &DiscordQuest) -> String {
    let reward_desc = quest.config.rewards_config.rewards.first()
        .map_or("No reward", |r| r.messages.name.as_str());
    format!("Quest: {} - Reward: {}", quest.config.messages.game_title, reward_desc)
}
