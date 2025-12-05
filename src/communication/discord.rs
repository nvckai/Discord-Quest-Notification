use crate::config::AppConfig;
use crate::types::error::AppError;
use crate::types::quest::{DiscordQuest, CollectibleProduct};
use chrono::DateTime;
use reqwest::Client;
use std::fmt::Write as FmtWrite;
use std::path::Path;
use serde_json::json;

// Discord API endpoints
const COLLECTIBLES_API_URL: &str = "https://discord.com/api/v9/collectibles-products";
const DISCORD_CDN_BASE: &str = "https://cdn.discordapp.com";
const QUEST_URL_BASE: &str = "https://discord.com/quests";
const ORBS_ICON_URL: &str = "https://cdn.discordapp.com/assets/content/fb761d9c206f93cd8c4e7301798abe3f623039a4054f2e7accd019e1bb059fc8.webm?format=webp";

// Reward type constants
const REWARD_TYPE_REDEEMABLE_CODE: i32 = 1;
const REWARD_TYPE_IN_GAME_ITEM: i32 = 2;
const REWARD_TYPE_COLLECTIBLE: i32 = 3;
const REWARD_TYPE_VIRTUAL_CURRENCY: i32 = 4;
const REWARD_TYPE_FRACTIONAL_PREMIUM: i32 = 5;

// Color constants for Discord embeds
const COLOR_VIRTUAL_CURRENCY: i32 = 0x0058_65F2;
const COLOR_COLLECTIBLE: i32 = 0x0057_F287;
const COLOR_DEFAULT: i32 = 0x0099_AAB5;

fn feature_id_to_name(feature_id: &i32) -> &'static str {
    match feature_id {
        1 => "POST_ENROLLMENT_CTA",
        2 => "PLAYTIME_CRITERIA",
        3 => "QUEST_BAR_V2",
        4 => "EXCLUDE_MINORS",
        5 => "EXCLUDE_RUSSIA",
        6 => "IN_HOUSE_CONSOLE_QUEST",
        7 => "MOBILE_CONSOLE_QUEST",
        8 => "START_QUEST_CTA",
        9 => "REWARD_HIGHLIGHTING",
        10 => "FRACTIONS_QUEST",
        11 => "ADDITIONAL_REDEMPTION_INSTRUCTIONS",
        12 => "PACING_V2",
        13 => "DISMISSAL_SURVEY",
        14 => "MOBILE_QUEST_DOCK",
        15 => "QUESTS_CDN",
        16 => "PACING_CONTROLLER",
        17 => "QUEST_HOME_FORCE_STATIC_IMAGE",
        18 => "VIDEO_QUEST_FORCE_HLS_VIDEO",
        _ => "UNKNOWN_FEATURE",
    }
}

fn reward_type_to_description(reward_type: i32) -> &'static str {
    match reward_type {
        REWARD_TYPE_REDEEMABLE_CODE => "Redeemable Code",
        REWARD_TYPE_IN_GAME_ITEM => "In-Game Item",
        REWARD_TYPE_COLLECTIBLE => "Collectible",
        REWARD_TYPE_VIRTUAL_CURRENCY => "Virtual Currency",
        REWARD_TYPE_FRACTIONAL_PREMIUM => "Fractional Premium",
        _ => {
            tracing::warn!("Unknown reward type encountered: {}", reward_type);
            "Unknown"
        }
    }
}

fn format_task_description(event_name: &str, target_seconds: i32) -> String {
    let duration_description = if target_seconds > 60 {
        let minutes = target_seconds.saturating_div(60);
        format!("{minutes} minutes")
    } else {
        format!("{target_seconds} seconds")
    };

    let task_name = match event_name {
        "WATCH_VIDEO" => "Watch video",
        "WATCH_VIDEO_ON_MOBILE" => "Watch video on mobile",
        "PLAY_ON_DESKTOP" => "Play on Desktop",
        "STREAM_ON_DESKTOP" => "Stream on Desktop",
        _ => return format!("- {} ({})", event_name.replace('_', " "), duration_description),
    };
    
    format!("- {task_name} ({duration_description})")
}

async fn fetch_collectible_icon(client: &Client, config: &AppConfig, sku_id: &str) -> Option<String> {
    let url = format!("{COLLECTIBLES_API_URL}/{sku_id}");
    tracing::info!("Fetching collectible product for SKU: {}", sku_id);
    
    let request = client.get(&url)
        .header("Authorization", &config.discord_auth_token)
        .timeout(std::time::Duration::from_secs(10));
    
    match request.send().await {
        Ok(resp) => {
            let status = resp.status();
            tracing::info!("Collectible API response status: {}", status);
            
            if !status.is_success() {
                tracing::warn!("Failed to fetch collectible (status {}): {}", status, resp.text().await.unwrap_or_default());
                return None;
            }

            match resp.json::<CollectibleProduct>().await {
                Ok(product) => {
                    tracing::info!("Successfully parsed collectible product: {:?}", product);
                    if let Some(item) = product.items.first() {
                        let asset_url = format!("https://cdn.discordapp.com/avatar-decoration-presets/{asset}.png", asset = item.asset);
                        tracing::info!("Found collectible asset URL: {}", asset_url);
                        return Some(asset_url);
                    }
                    tracing::warn!("Collectible product has no items");
                },
                Err(e) => {
                    tracing::warn!("Failed to parse collectible product for SKU {}: {}", sku_id, e);
                }
            }
        },
        Err(e) => {
            tracing::warn!("Failed to fetch collectible product for SKU {}: {}", sku_id, e);
        }
    }
    None
}

#[allow(clippy::too_many_lines)]
pub async fn send_webhook(config: &AppConfig, quest: &DiscordQuest) -> Result<(), AppError> {
    let client = Client::new();
    
    let reward = quest.config.rewards_config.rewards.first();
    
    let accent_color = reward.map_or(COLOR_DEFAULT, |reward| match reward.reward_type {
        REWARD_TYPE_VIRTUAL_CURRENCY => COLOR_VIRTUAL_CURRENCY,
        REWARD_TYPE_COLLECTIBLE => COLOR_COLLECTIBLE,
        _ => COLOR_DEFAULT,
    });

    let start_timestamp = DateTime::parse_from_rfc3339(&quest.config.starts_at)
        .map(|dt| dt.timestamp())
        .unwrap_or(0);

    let expires_timestamp = DateTime::parse_from_rfc3339(&quest.config.expires_at)
        .map(|dt| dt.timestamp())
        .unwrap_or(0);

    let build_cdn_url = |asset: &str| {
        if asset.contains('/') {
            format!("{DISCORD_CDN_BASE}/{asset}")
        } else {
            format!("{}/quest_assets/{}/{}", DISCORD_CDN_BASE, quest.id, asset)
        }
    };

    let banner_url = build_cdn_url(&quest.config.assets.hero);
    let cta_link = format!("{}/{}", QUEST_URL_BASE, quest.id);
    let reward_icon_url = if let Some(reward) = reward {
        tracing::info!("Processing reward - Type: {}, SKU: {:?}, Asset: {:?}, Orbs: {:?}", 
            reward.reward_type, reward.sku_id, reward.asset, reward.orb_quantity);
            
        if let Some(orbs) = reward.orb_quantity {
             if orbs > 0 {
                 ORBS_ICON_URL.to_string()
             } else {
                 build_cdn_url(&quest.config.assets.game_tile)
             }
        } else if reward.reward_type == REWARD_TYPE_COLLECTIBLE && reward.sku_id.is_some() {
            if let Some(sku_id) = &reward.sku_id {
                fetch_collectible_icon(&client, config, sku_id)
                    .await
                    .unwrap_or_else(|| build_cdn_url(&quest.config.assets.game_tile))
            } else {
                build_cdn_url(&quest.config.assets.game_tile)
            }
        } else if let Some(asset) = &reward.asset {
            if Path::new(asset)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("mp4")) {
                build_cdn_url(&quest.config.assets.game_tile)
            } else {
                build_cdn_url(asset)
            }
        } else {
            build_cdn_url(&quest.config.assets.game_tile)
        }
    } else {
        build_cdn_url(&quest.config.assets.game_tile)
    };

    let features_list: Vec<String> = quest.config.features
        .iter()
        .map(|feature_id| format!("``{}``", feature_id_to_name(feature_id)))
        .collect();
    
    let features_string = features_list.join(", ");

    let tasks_string = quest.config.task_config.tasks.values()
        .map(|task| format_task_description(&task.event_name, task.target))
        .collect::<Vec<_>>()
        .join("\n");

    // Reward details
    let reward_info = reward.map_or_else(|| "No reward details available".to_string(), |reward| {
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
    });

    // Constructing the payload using Discord's "Container" structure (Components V2)
    let payload = json!({
        "components": [
            {
                "type": 17,
                "accent_color": accent_color,
                "components": [
                    {
                        "type": 10,
                        "content": format!("## **New Quest** - [{}]({})", quest.config.messages.game_title, cta_link)
                    },
                    {
                        "type": 12,
                        "items": [
                            {
                                "media": {
                                    "url": banner_url
                                },
                                "description": null,
                                "spoiler": false
                            }
                        ]
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 10,
                        "content": format!(
                            "\n# Quest Info\n**Duration**: <t:{}:d> - <t:{}:d>\n**Reedemable Platforms**: Cross Platform\n**Game**: {} ({})\n**Application**: [{}]({}) (``{}``)\n**Features**: {}",
                            start_timestamp,
                            expires_timestamp,
                            quest.config.messages.game_title,
                            quest.config.messages.game_publisher,
                            quest.config.application.name,
                            quest.config.application.link,
                            quest.config.application.id,
                            features_string
                        )
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 10,
                        "content": format!("# Tasks\nUser must complete any of the following tasks\n{}", tasks_string)
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 9,
                        "accessory": {
                            "type": 11,
                            "media": {
                                "url": reward_icon_url
                            },
                            "description": null,
                            "spoiler": false
                        },
                        "components": [
                            {
                                "type": 10,
                                "content": format!("# Rewards\n{}", reward_info)
                            }
                        ]
                    },
                    {
                        "type": 14,
                        "divider": true,
                        "spacing": 1
                    },
                    {
                        "type": 10,
                        "content": format!("Quest ID: `{}`", quest.id)
                    }
                ]
            }
        ],
        "flags": 32768
    });

    let webhook_url = if config.discord_webhook_url.contains('?') {
        format!("{}&with_components=true", config.discord_webhook_url)
    } else {
        format!("{}?with_components=true", config.discord_webhook_url)
    };

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
