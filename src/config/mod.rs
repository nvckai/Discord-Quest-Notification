use crate::types::error::AppError;
use std::env;
use dotenvy::dotenv;

// Configuration validation constants
const MIN_POLLING_INTERVAL_SEC: u64 = 30;
const MAX_POLLING_INTERVAL_SEC: u64 = 86400; // 24 hours
const DEFAULT_POLLING_INTERVAL_SEC: u64 = 300; // 5 minutes
const DEFAULT_REGION: &str = "en-US";
const DISCORD_WEBHOOK_PREFIX_1: &str = "https://discord.com/api/webhooks/";
const DISCORD_WEBHOOK_PREFIX_2: &str = "https://discordapp.com/api/webhooks/";
const DISCORD_WEBHOOK_PREFIX_3: &str = "https://ptb.discord.com/api/webhooks/";
const DISCORD_WEBHOOK_PREFIX_4: &str = "https://canary.discord.com/api/webhooks/";

#[derive(Clone)]
pub struct AppConfig {
    pub discord_auth_token: String,
    pub discord_webhook_url: String,
    pub polling_interval_sec: u64,
    pub discord_regions: Vec<String>,
    pub previous_quests: bool,
    pub super_properties: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, AppError> {
        dotenv().ok();

        let token = env::var("DISCORD_AUTH_TOKEN")
            .map_err(|_| AppError::Config("DISCORD_AUTH_TOKEN not set".to_string()))?;
        
        if token.trim().is_empty() {
            return Err(AppError::Config("DISCORD_AUTH_TOKEN is empty or contains only whitespace".to_string()));
        }

        let webhook = env::var("DISCORD_WEBHOOK_URL")
            .map_err(|_| AppError::Config("DISCORD_WEBHOOK_URL not set".to_string()))?;
        
        if webhook.trim().is_empty() {
            return Err(AppError::Config("DISCORD_WEBHOOK_URL is empty or contains only whitespace".to_string()));
        }
        
        if !webhook.starts_with(DISCORD_WEBHOOK_PREFIX_1) 
            && !webhook.starts_with(DISCORD_WEBHOOK_PREFIX_2)
            && !webhook.starts_with(DISCORD_WEBHOOK_PREFIX_3)
            && !webhook.starts_with(DISCORD_WEBHOOK_PREFIX_4) {
            return Err(AppError::Config("DISCORD_WEBHOOK_URL must be a valid Discord webhook URL".to_string()));
        }

        let interval_str = env::var("POLLING_INTERVAL_SEC")
            .unwrap_or_else(|_| DEFAULT_POLLING_INTERVAL_SEC.to_string());
        let polling_interval_sec = interval_str.parse::<u64>()
            .map_err(|e| AppError::Config(format!("Invalid POLLING_INTERVAL_SEC: {e}")))?;
        
        if polling_interval_sec < MIN_POLLING_INTERVAL_SEC {
            return Err(AppError::Config(format!(
                "POLLING_INTERVAL_SEC must be at least {MIN_POLLING_INTERVAL_SEC} seconds to avoid rate limiting"
            )));
        }
        
        if polling_interval_sec > MAX_POLLING_INTERVAL_SEC {
            return Err(AppError::Config(format!(
                "POLLING_INTERVAL_SEC must be less than {MAX_POLLING_INTERVAL_SEC} seconds (24 hours)"
            )));
        }

        let regions_str = env::var("DISCORD_REGIONS")
            .unwrap_or_else(|_| DEFAULT_REGION.to_string());
        let discord_regions: Vec<String> = regions_str.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let previous_quests = env::var("PREVIOUS_QUEST")
            .unwrap_or_else(|_| "false".to_string())
            .trim()
            .eq_ignore_ascii_case("true");

        let super_properties = env::var("SUPER_PROPERTIES")
            .unwrap_or_else(|_| "ewogICJvcyI6ICJXaW5kb3dzIiwKICAiYnJvd3NlciI6ICJDaHJvbWUiLAogICJkZXZpY2UiOiAiIiwKICAic3lzdGVtX2xvY2FsZSI6ICJlbi1VUyIsCiAgImJyb3dzZXJfdXNlcl9hZ2VudCI6ICJNb3ppbGxhLzUuMCAoV2luZG93cyBOVCAxMC4wOyBXaW42NDsgeDY0KSBBcHBsZVdlYktpdC81MzcuMzYgKEtIVE1MLCBsaWtlIEdlY2tvKSBDaHJvbWUvMTIwLjAuMC4wIFNhZmFyaS81MzcuMzYiLAogICJicm93c2VyX3ZlcnNpb24iOiAiMTIwLjAuMC4wIiwKICAib3NfdmVyc2lvbiI6ICIxMCIsCiAgInJlZmVycmVyIjogIiIsCiAgInJlZmVycmluZ19kb21haW4iOiAiIiwKICAicmVmZXJyZXJfY3VycmVudCI6ICIiLAogICJyZWZlcnJpbmdfZG9tYWluX2N1cnJlbnQiOiAiIiwKICAicmVsZWFzZV9jaGFubmVsIjogInN0YWJsZSIsCiAgImNsaWVudF9idWlsZF9udW1iZXIiOiA5OTk5OTksCiAgImNsaWVudF9ldmVudF9zb3VyY2UiOiBudWxsCn0d".to_string());

        Ok(Self {
            discord_auth_token: token,
            discord_webhook_url: webhook,
            polling_interval_sec,
            discord_regions,
            previous_quests,
            super_properties,
        })
    }
}

