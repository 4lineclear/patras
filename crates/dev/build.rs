//! pre compile utilities

use std::fs;

use anyhow::{bail, Context};
use toml::Table;

const ENV_PATH: &str = "../../env.toml";

fn main() {
    if let Err(e) = run() {
        panic!("{e:?}");
    }
}

fn run() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={ENV_PATH}");
    let env_text = fs::read_to_string(ENV_PATH).context("Failed to read env.toml")?;
    let env = toml::from_str::<Table>(&env_text).context("Failed to parse env.toml")?;

    let server_port = env["dev"]["ports"]["server"]
        .as_integer()
        .context("Missing server local port")?;
    let client_port = env["dev"]["ports"]["client"]
        .as_integer()
        .context("Missing client local port")?;
    let db_url = env["db"]["url"]
        .as_str()
        .context("Missing local database port")?;

    if client_port == server_port {
        bail!("Ports of server and client are the same: {client_port}");
    }

    println!("cargo:rustc-env=SERVER_PORT={server_port}");
    println!("cargo:rustc-env=CLIENT_PORT={client_port}");
    println!("cargo:rustc-env=DATABASE_URL={db_url}");
    Ok(())
}
