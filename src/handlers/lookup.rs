use crate::config::AppConfig;
use crate::types::error::AppError;
use crate::communication::{scraper, webhook};
use crate::handlers::processor;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use std::hash::BuildHasher;
use tracing::{info, error, warn};
use reqwest::Client;
use tokio::sync::broadcast;

/// Main application loop for checking and processing quests
///
/// # Errors
///
/// Returns `AppError` if:
/// - Quest fetching from Discord API fails
/// - State lock is poisoned
/// - Webhook sending fails
pub async fn app<S: BuildHasher + Clone + Send + Sync>(
    config: &AppConfig,
    state: Arc<RwLock<HashSet<String, S>>>,
    is_initial_run: bool,
    region: &str,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<(), AppError> {
    let client = Client::new();
    let quests = scraper::fetch_quests(config, region).await?;
    
    let known_ids: HashSet<String, S> = {
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
            for (index, quest) in new_quests.iter().enumerate() {
                // Check for shutdown signal before processing each quest
                if shutdown_rx.try_recv().is_ok() {
                    warn!("Shutdown signal received. Stopping quest processing. Processed {}/{} quests.", index, new_quests.len());
                    return Ok(());
                }

                info!("Found new quest ({}/{}): {}", index + 1, new_quests.len(), processor::format_quest_message(quest));
                if let Err(e) = webhook::send_webhook(&client, config, quest).await {
                    error!("Failed to send webhook for quest {}: {}", quest.id, e);
                }
            }
        }
    }

    Ok(())
}
