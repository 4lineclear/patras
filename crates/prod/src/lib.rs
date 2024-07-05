//! Creates a prod binary

use core_server::{
    axum::{
        http::{header, StatusCode, Uri},
        response::{Html, IntoResponse, Response},
        Router,
    },
    CreateRouterError,
};
use rust_embed::Embed;

pub use core_server;

// TODO: add tls
// https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs

/// Production assets
#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/../../client/dist"]
pub struct Assets;

/// Creates a production ready router
///
/// # Errors
///
/// See [`core_server::router`]
pub async fn router() -> Result<Router, CreateRouterError> {
    Ok(core_server::router().await?.fallback(static_handler))
}

static INDEX_HTML: &str = "index.html";

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html();
    }

    if let Some(content) = Assets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
    } else {
        index_html()
    }
}

fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found(),
    }
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "Server Error: 404").into_response()
}
