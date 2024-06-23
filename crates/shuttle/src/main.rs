//! The shuttle runtime for the server

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(prod_server::router().into())
}
