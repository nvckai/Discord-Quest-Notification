use crate::config::AppConfig;
use crate::types::error::AppError;
use crate::types::quest::DiscordQuest;
use crate::types::constants::{
    COLOR_DEFAULT, COLOR_VIRTUAL_CURRENCY, COLOR_COLLECTIBLE,
    REWARD_TYPE_VIRTUAL_CURRENCY, REWARD_TYPE_COLLECTIBLE,
    QUEST_URL_BASE, build_cdn_url, feature_id_to_name,
};
use crate::communication::{
    collectibles::resolve_reward_icon_url,
    reward::format_reward_info,
    task::format_task_description,
    payload::{build_webhook_payload, WebhookPayloadData},
};
use chrono::DateTime;
use reqwest::Client;

/// Send a Discord webhook notification for a new quest
///
/// # Errors
///
/// Returns `AppError` if:
/// - Quest date parsing fails
/// - HTTP request to webhook fails
/// - Webhook returns non-success status
pub async fn send_webhook(
    client: &Client,
    config: &AppConfig,
    quest: &DiscordQuest,
) -> Result<(), AppError> {
    let reward = quest.config.rewards_config.rewards.first();
    
    // Determine accent color based on reward type
    let accent_color = reward.map_or(COLOR_DEFAULT, |reward| match reward.reward_type {
        REWARD_TYPE_VIRTUAL_CURRENCY => COLOR_VIRTUAL_CURRENCY,
        REWARD_TYPE_COLLECTIBLE => COLOR_COLLECTIBLE,
        _ => COLOR_DEFAULT,
    });

    // Parse timestamps
    let start_timestamp = DateTime::parse_from_rfc3339(&quest.config.starts_at)
        .map(|dt| dt.timestamp())
        .map_err(|e| AppError::Parse(format!("Invalid start date '{}': {}", quest.config.starts_at, e)))?;

    let expires_timestamp = DateTime::parse_from_rfc3339(&quest.config.expires_at)
        .map(|dt| dt.timestamp())
        .map_err(|e| AppError::Parse(format!("Invalid expiry date '{}': {}", quest.config.expires_at, e)))?;

    // Build URLs and assets
    let banner_url = build_cdn_url(&quest.id, &quest.config.assets.hero);
    let cta_link = format!("{QUEST_URL_BASE}/{}", quest.id);
    let reward_icon_url = resolve_reward_icon_url(client, config, quest, reward).await;

    // Format features
    let features_list: Vec<String> = quest.config.features
        .iter()
        .map(|feature_id| format!("``{}``", feature_id_to_name(*feature_id)))
        .collect();
    let features_string = features_list.join(", ");

    // Format tasks
    let tasks_string = quest.config.task_config.tasks.values()
        .map(|task| format_task_description(&task.event_name, task.target))
        .collect::<Vec<_>>()
        .join("\n");

    // Format reward info
    let reward_info = format_reward_info(reward);

    // Build payload
    let payload = build_webhook_payload(&WebhookPayloadData {
        accent_color,
        game_title: &quest.config.messages.game_title,
        game_publisher: &quest.config.messages.game_publisher,
        cta_link: &cta_link,
        banner_url,
        start_timestamp,
        expires_timestamp,
        app_name: &quest.config.application.name,
        app_link: &quest.config.application.link,
        app_id: &quest.config.application.id,
        features_string,
        tasks_string,
        reward_icon_url,
        reward_info,
        quest_id: &quest.id,
    });

    // Send webhook
    let separator = if config.discord_webhook_url.contains('?') { '&' } else { '?' };
    let webhook_url = format!("{}{separator}with_components=true", config.discord_webhook_url);

    let res = client.post(&webhook_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| AppError::Request(e.to_string()))?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        tracing::error!("Webhook failed. Status: {}, Body: {}", status, body);
        return Err(AppError::Config(format!("Webhook failed: {status} - {body}")));
    }

    Ok(())
}
