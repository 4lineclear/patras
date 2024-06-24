//! The shuttle runtime for the server

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(prod_server::router().into())
}
