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
/// Finishes once quit command recieved.
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
            enter h to show help(this text)\n\
            enter o to open client in browser\n\
            enter u to show server url\n\
            enter c clear screen\n\
            enter q to quit\
            "
        ),
        "o" => {
            if webbrowser::open(concat!("http://localhost:", env!("CLIENT_PORT"))).is_ok() {
                tracing::info!("Opening browser...");
            } else {
                tracing::info!("Unable to open browser");
            }
        }
        "c" => scroll(),
        "u" => println!("Local port: http://localhost:{}/", env!("SERVER_PORT")),
        "q" => return true,
        _ => (),
    }
    false
}

pub(super) fn scroll() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = std::io::Write::flush(&mut std::io::stdout());
}
