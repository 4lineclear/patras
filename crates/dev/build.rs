//! pre compile utilities

use std::env;

use anyhow::Context;

const ENV_PATH: &str = "../../.env";

fn main() {
    if let Err(e) = run() {
        panic!("{e:?}");
    }
}

fn run() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={ENV_PATH}");

    dotenvy::from_path(ENV_PATH)?;

    let server_port = env::var("VITE_DEV_SERVER_PORT").context("Missing server local port")?;
    let client_port = env::var("VITE_DEV_CLIENT_PORT").context("Missing client local port")?;
    let db_url = env::var("DATABASE_URL").context("Missing local database port")?;

    println!("cargo:rustc-env=SERVER_PORT={server_port}");
    println!("cargo:rustc-env=CLIENT_PORT={client_port}");
    println!("cargo:rustc-env=DATABASE_URL={db_url}");

    Ok(())
}
