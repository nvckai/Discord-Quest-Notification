use tokio::sync::oneshot;
use tracing::{error, info};

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::{ctrl_break, ctrl_c};

pub async fn handle_signals(
    shutdown_tx: oneshot::Sender<()>,
    shutdown_components: oneshot::Sender<()>,
) {
    wait_for_signal().await;

    if let Err(e) = shutdown_components.send(()) {
        error!("Failed to send component shutdown signal: {:?}", e);
    }

    if let Err(e) = shutdown_tx.send(()) {
        error!("Failed to send main shutdown signal: {:?}", e);
    }
}

#[cfg(unix)]
async fn wait_for_signal() {
    let mut sigterm = match signal(SignalKind::terminate()) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create SIGTERM signal handler: {}", e);
            return;
        }
    };
    // Handle SIGINT (Ctrl+C)
    let mut sigint = match signal(SignalKind::interrupt()) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create SIGINT signal handler: {}", e);
            return;
        }
    };

    tokio::select! {
        _ = sigterm.recv() => {
            info!("Received SIGTERM signal, initiating graceful shutdown");
        }
        _ = sigint.recv() => {
            info!("Received SIGINT signal, initiating graceful shutdown");
        }
    }
}

#[cfg(windows)]
async fn wait_for_signal() {
    // Handle Ctrl+C
    let mut ctrlc = match ctrl_c() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create Ctrl+C signal handler: {}", e);
            return;
        }
    };
    // Handle Ctrl+Break
    let mut ctrlbreak = match ctrl_break() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to create Ctrl+Break signal handler: {}", e);
            return;
        }
    };

    tokio::select! {
        _ = ctrlc.recv() => {
            info!("Received Ctrl+C signal, initiating graceful shutdown");
        }
        _ = ctrlbreak.recv() => {
            info!("Received Ctrl+Break signal, initiating graceful shutdown");
        }
    }
}
