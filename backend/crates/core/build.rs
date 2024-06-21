//! pre compile utilities

use std::path::Path;

use anyhow::{bail, Context};

const CONFIG_PATH: &str = "../../../.env";

fn main() {
    if let Err(e) = run() {
        panic!("{e:?}");
    }
}

fn run() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={CONFIG_PATH}");
    dotenvy::from_path(
        Path::new(CONFIG_PATH)
            .canonicalize()
            .context("Unable to get absolute path to .env file")?,
    )?;
    dotenvy::dotenv().context("Unable load env variables")?;

    let backend_port = std::env::var("BACKEND_PORT")
        .context("Missing backend local port")?
        .parse::<u16>()
        .context("BACKEND_PORT was not 16 bit unsigned integer")?;
    let frontend_port = std::env::var("FRONTEND_PORT")
        .context("Missing frontend local port")?
        .parse::<u16>()
        .context("FRONTEND_PORT was not 16 bit unsigned integer")?;

    if frontend_port == backend_port {
        bail!("Ports of fronted and backend are the same: {frontend_port}");
    }

    println!("cargo:rustc-env=FRONTEND_PORT={frontend_port}");
    println!("cargo:rustc-env=BACKEND_PORT={backend_port}");
    Ok(())
}
