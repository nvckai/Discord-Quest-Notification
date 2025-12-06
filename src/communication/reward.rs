use crate::types::constants::reward_type_to_description;
use crate::types::quest::QuestReward;
use std::fmt::Write as FmtWrite;

/// Format reward information for Discord webhook
#[must_use]
pub fn format_reward_info(reward: Option<&QuestReward>) -> String {
    let Some(reward) = reward else {
        return "No reward details available".to_string();
    };

    let reward_type_description = reward_type_to_description(reward.reward_type);
    let sku_id = reward.sku_id.as_deref().unwrap_or("N/A");

    let mut info = format!(
        "**Reward Type**: {}\n**SKU ID**: `{}`\n**Name**: {}",
        reward_type_description, sku_id, reward.messages.name
    );

    if let Some(orbs) = reward.orb_quantity {
        let _ = write!(info, "\n**Orbs Amount**: {orbs}");
    }

    if let Some(qty) = reward.quantity {
        let _ = write!(info, "\n**Quantity**: {qty}");
    }

    info
}
