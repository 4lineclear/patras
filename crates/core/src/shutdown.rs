use tokio::signal;
use tracing::info;

/// The finishing of this future indicates a shutdown signal
///
/// # Panics
///
/// Panics if either the `ctrl_c` signal or `sigterm`
/// signal for unix fails to be installed
#[allow(dead_code)]
pub async fn signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            info!("Ctrl-C received, app shutdown commencing");
        },
        () = terminate => {
            info!("SIGTERM received, app shutdown commencing");
        },
    }
}
