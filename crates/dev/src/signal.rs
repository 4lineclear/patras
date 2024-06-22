use anyhow::Context;
use core_server::*;

use tokio::signal;
use tracing::info;

/// The finishing of this future indicates a shutdown signal
///
/// # Panics
///
/// Panics if either the `ctrl_c` signal or `sigterm`
/// signal for unix fails to be installed
#[allow(clippy::cognitive_complexity)]
pub async fn signal() {
    let console = async { read_console().await.expect("Failed to read console") };
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
        () = console => {
            info!("Console exit recieved, app shutdown commencing");
        }
    }
}

/// Reads console
///
/// Finishes once end command recieved.
async fn read_console() -> anyhow::Result<()> {
    let stdin = async_std::io::stdin();
    let mut buf = String::new();
    loop {
        buf.clear();
        stdin
            .read_line(&mut buf)
            .await
            .context("Failed to read stdin")?;
        if handle_console_input(buf.trim()) {
            break;
        }
    }
    Ok(())
}

fn handle_console_input(s: &str) -> bool {
    match s {
        "h" => println!(
            "\
            press h + enter to show help(this text)\n\
            press o + enter to open client in browser\n\
            press u + enter to show server url\n\
            press q + enter to quit\
            "
        ),
        "o" => match webbrowser::open(concat!("http://localhost:", env!("CLIENT_PORT"))) {
            Ok(()) => tracing::info!("Opening browser..."),
            Err(_) => tracing::info!("Unable to open browser"),
        },
        "u" => println!("Local port: http://localhost:{}/", env!("SERVER_PORT")),
        "q" => return true,
        _ => (),
    }
    false
}
