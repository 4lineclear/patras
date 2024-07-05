//! pre compile utilities

use std::fs;

use anyhow::{bail, Context};
use toml::Table;

const OPTS_PATH: &str = "../../opts.toml";

fn main() {
    if let Err(e) = run() {
        panic!("{e:?}");
    }
}

fn run() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={OPTS_PATH}");
    let opts_text = fs::read_to_string(OPTS_PATH).context("Failed to read opts.toml")?;
    let opts = toml::from_str::<Table>(&opts_text).context("Failed to parse opts.toml")?;

    let server_port = opts["dev"]["ports"]["server"]
        .as_integer()
        .context("Missing server local port")?;
    let client_port = opts["dev"]["ports"]["client"]
        .as_integer()
        .context("Missing client local port")?;

    if client_port == server_port {
        bail!("Ports of server and client are the same: {client_port}");
    }

    println!("cargo:rustc-env=SERVER_PORT={server_port}");
    println!("cargo:rustc-env=CLIENT_PORT={client_port}");
    Ok(())
}
