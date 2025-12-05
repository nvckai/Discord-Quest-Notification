use crate::config::AppConfig;
use crate::types::error::AppError;
use crate::communication::{scraper, discord};
use crate::handlers::processor;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use tracing::{info, error};

pub async fn app(config: &AppConfig, state: Arc<RwLock<HashSet<String>>>, is_initial_run: bool, region: &str) -> Result<(), AppError> {
    let quests = scraper::fetch_quests(config, region).await?;
    
    let known_ids: HashSet<String> = {
        let lock = state.read().map_err(|e| {
            error!("Failed to acquire read lock on state: {}", e);
            AppError::Config("State lock poisoned".to_string())
        })?;
        lock.clone()
    };

    let new_quests = processor::filter_new_quests(quests, &known_ids);

    if !new_quests.is_empty() {
        {
            let mut lock = state.write().map_err(|e| {
                error!("Failed to acquire write lock on state: {}", e);
                AppError::Config("State lock poisoned".to_string())
            })?;
            for quest in &new_quests {
                lock.insert(quest.id.clone());
            }
        }

        if is_initial_run && !config.previous_quests {
            info!("Initial fetch: Found {} quests. Skipping notifications to prevent spam.", new_quests.len());
        } else {
            if is_initial_run {
                info!("Initial fetch: Found {} quests. Posting all previous quests (PREVIOUS_QUEST=true).", new_quests.len());
            }
            for quest in new_quests {
                info!("Found new quest: {}", processor::format_quest_message(&quest));
                if let Err(e) = discord::send_webhook(config, &quest).await {
                    error!("Failed to send webhook for quest {}: {}", quest.id, e);
                }
            }
        }
    }

    Ok(())
}
