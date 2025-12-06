use crate::config::AppConfig;
use crate::types::constants::{COLLECTIBLES_API_URL, REWARD_TYPE_COLLECTIBLE, build_cdn_url};
use crate::types::quest::{CollectibleProduct, DiscordQuest, QuestReward};
use reqwest::Client;
use std::path::Path;

/// Fetch collectible icon URL from Discord API
async fn fetch_collectible_icon(client: &Client, config: &AppConfig, sku_id: &str) -> Option<String> {
    let url = format!("{COLLECTIBLES_API_URL}/{sku_id}");
    tracing::debug!("Fetching collectible product for SKU: {}", sku_id);
    
    let request = client.get(&url)
        .header("Authorization", &config.discord_auth_token)
        .timeout(std::time::Duration::from_secs(10));
    
    match request.send().await {
        Ok(resp) => {
            let status = resp.status();
            tracing::debug!("Collectible API response status: {}", status);
            
            if !status.is_success() {
                if status.as_u16() == 401 || status.as_u16() == 403 {
                    tracing::error!(
                        "Authentication failed for collectible API (status {}). Token may be invalid or expired.",
                        status
                    );
                } else {
                    let body = resp.text().await.unwrap_or_else(|_| "<unable to read response body>".to_string());
                    tracing::warn!("Failed to fetch collectible (status {}): {}", status, body);
                }
                return None;
            }

            match resp.json::<CollectibleProduct>().await {
                Ok(product) => {
                    tracing::debug!("Successfully parsed collectible product: {:?}", product);
                    if let Some(item) = product.items.first() {
                        let asset_url = format!("https://cdn.discordapp.com/avatar-decoration-presets/{asset}.png", asset = item.asset);
                        tracing::debug!("Found collectible asset URL: {}", asset_url);
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

/// Resolve the appropriate icon URL for a quest reward
pub async fn resolve_reward_icon_url(
    client: &Client,
    config: &AppConfig,
    quest: &DiscordQuest,
    reward: Option<&QuestReward>,
) -> String {
    let game_tile_url = build_cdn_url(&quest.id, &quest.config.assets.game_tile);
    
    let Some(reward) = reward else {
        return game_tile_url;
    };

    tracing::debug!(
        "Processing reward - Type: {}, SKU: {:?}, Asset: {:?}, Orbs: {:?}",
        reward.reward_type, reward.sku_id, reward.asset, reward.orb_quantity
    );

    // Check for orbs reward
    if let Some(orbs) = reward.orb_quantity {
        return if orbs > 0 {
            crate::types::constants::ORBS_ICON_URL.to_string()
        } else {
            game_tile_url
        };
    }

    // Check for collectible reward
    if reward.reward_type == REWARD_TYPE_COLLECTIBLE {
        if let Some(sku_id) = &reward.sku_id {
            if let Some(icon_url) = fetch_collectible_icon(client, config, sku_id).await {
                return icon_url;
            }
        }
        return game_tile_url;
    }

    // Check for asset-based reward
    if let Some(asset) = &reward.asset {
        // Skip video files
        if Path::new(asset)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("mp4"))
        {
            return game_tile_url;
        }
        return build_cdn_url(&quest.id, asset);
    }

    game_tile_url
}
