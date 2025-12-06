mod config;
mod types;
mod utils;
mod communication;
mod handlers;
mod shutdown;

use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use std::time::Duration;
use tokio::time;
use tokio::sync::{oneshot, broadcast};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    utils::setup_logging();

    let config = match config::AppConfig::load() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load config: {}", e);
            return;
        }
    };

    // Setup shutdown channels
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
    let (component_shutdown_tx, _component_shutdown_rx) = oneshot::channel();
    
    // Broadcast channel for graceful shutdown to all tasks
    let (broadcast_shutdown_tx, _) = broadcast::channel::<()>(1);
    let broadcast_tx_clone = broadcast_shutdown_tx.clone();

    // Spawn signal handler
    tokio::spawn(async move {
        shutdown::handle_signals(shutdown_tx, component_shutdown_tx).await;
        // Broadcast shutdown to all tasks
        let _ = broadcast_tx_clone.send(());
    });

    let state = Arc::new(RwLock::new(HashSet::new()));
    let mut interval = time::interval(Duration::from_secs(config.polling_interval_sec));

    info!("Starting Discord Quest Notification...");
    info!("Press Ctrl+C to shutdown gracefully");

    let mut is_initial_run = true;
    let mut region_index = 0;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let state_clone = state.clone();
                let shutdown_receiver = broadcast_shutdown_tx.subscribe();
                
                let current_region = if config.discord_regions.is_empty() {
                    "en-US"
                } else {
                    &config.discord_regions[region_index]
                };

                info!("Checking quests for region: {}", current_region);

                if let Err(e) = handlers::lookup::app(&config, state_clone, is_initial_run, current_region, shutdown_receiver).await {
                    error!("Error in app (region: {}): {}", current_region, e);
                }
                
                is_initial_run = false;
                
                if !config.discord_regions.is_empty() {
                    region_index = (region_index + 1) % config.discord_regions.len();
                }
            }
            _ = &mut shutdown_rx => {
                info!("Shutdown signal received, exiting main loop");
                break;
            }
        }
    }
    info!("Application exited gracefully");
}
