//! Creates a prod binary
// #![allow(clippy::wildcard_imports)]

use rust_embed::Embed;

/// Production assets
#[derive(Embed)]
#[folder = "../../client/dist"]
pub struct Assets;
