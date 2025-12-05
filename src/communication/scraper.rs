use crate::config::AppConfig;
use crate::types::error::AppError;
use crate::types::quest::DiscordQuest;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, debug, warn};

// Discord API configuration
const DISCORD_API_BASE: &str = "https://discord.com/api/v9";
const QUESTS_ENDPOINT: &str = "quests/@me";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const REQUEST_TIMEOUT_SECS: u64 = 30;
const CONNECT_TIMEOUT_SECS: u64 = 10;

pub async fn fetch_quests(config: &AppConfig, region: &str) -> Result<Vec<DiscordQuest>, AppError> {
    let client = Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
        .build()
        .map_err(|e| AppError::Request(e.to_string()))?;
        
    let url = format!("{DISCORD_API_BASE}/{QUESTS_ENDPOINT}");

    let response = client.get(&url)
        .header("Authorization", &config.discord_auth_token)
        .header("x-super-properties", &config.super_properties)
        .header("x-discord-locale", region)
        .send()
        .await
        .map_err(|e| {
            warn!("Failed to fetch quests from Discord API: {}", e);
            AppError::Request(e.to_string())
        })?;

    let status = response.status();
    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_default();
        warn!("Discord API returned error status {}: {}", status, error_body);
        return Err(AppError::Config(format!("Discord API error: {status} - {error_body}")));
    }
    
    let body: serde_json::Value = response.json().await.map_err(|e| {
        warn!("Failed to parse Discord API response as JSON: {}", e);
        AppError::Request(e.to_string())
    })?;
    
    debug!("API Response: {:?}", body);

    if let Some(quests_array) = body.get("quests") {
        let quests: Vec<DiscordQuest> = serde_json::from_value(quests_array.clone())
            .map_err(|e| {
                warn!("Failed to parse quests array from 'quests' field: {}", e);
                AppError::Parse(format!("Failed to parse quests array from 'quests' field: {e}"))
            })?;
        info!("Successfully fetched {} quests from region {}", quests.len(), region);
        Ok(quests)
    } else if body.is_array() {
        let quests: Vec<DiscordQuest> = serde_json::from_value(body)
            .map_err(|e| {
                warn!("Failed to parse quests array: {}", e);
                AppError::Parse(format!("Failed to parse quests array: {e}"))
            })?;
        info!("Successfully fetched {} quests from region {}", quests.len(), region);
        Ok(quests)
    } else {
        warn!("Unexpected API response structure (no 'quests' field and not an array). Response: {:?}", body);
        Ok(vec![])
    }
}
