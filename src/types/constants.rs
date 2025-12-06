// Discord API endpoints
pub const COLLECTIBLES_API_URL: &str = "https://discord.com/api/v9/collectibles-products";
pub const DISCORD_CDN_BASE: &str = "https://cdn.discordapp.com";
pub const QUEST_URL_BASE: &str = "https://discord.com/quests";
pub const ORBS_ICON_URL: &str = "https://cdn.discordapp.com/assets/content/fb761d9c206f93cd8c4e7301798abe3f623039a4054f2e7accd019e1bb059fc8.webm?format=webp";

// Reward type constants
pub const REWARD_TYPE_REDEEMABLE_CODE: i32 = 1;
pub const REWARD_TYPE_IN_GAME_ITEM: i32 = 2;
pub const REWARD_TYPE_COLLECTIBLE: i32 = 3;
pub const REWARD_TYPE_VIRTUAL_CURRENCY: i32 = 4;
pub const REWARD_TYPE_FRACTIONAL_PREMIUM: i32 = 5;

// Color constants for Discord embeds
pub const COLOR_VIRTUAL_CURRENCY: i32 = 0x0058_65F2;
pub const COLOR_COLLECTIBLE: i32 = 0x0057_F287;
pub const COLOR_DEFAULT: i32 = 0x0099_AAB5;

/// Convert Discord feature ID to human-readable name
#[must_use]
pub const fn feature_id_to_name(feature_id: i32) -> &'static str {
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

/// Convert reward type ID to description
#[must_use]
pub fn reward_type_to_description(reward_type: i32) -> &'static str {
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

/// Build CDN URL for quest assets
#[must_use]
pub fn build_cdn_url(quest_id: &str, asset: &str) -> String {
    if asset.contains('/') {
        format!("{DISCORD_CDN_BASE}/{asset}")
    } else {
        format!("{DISCORD_CDN_BASE}/quest_assets/{quest_id}/{asset}")
    }
}
