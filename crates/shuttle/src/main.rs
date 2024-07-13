//! The shuttle runtime for the server

use prod_server::sqlx::PgPool;

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(local_uri = "{secrets.LOCAL_DB_URL}")] url: String,
) -> shuttle_axum::ShuttleAxum {
    let pool = PgPool::connect(&url).await.map_err(map_err)?;
    Ok(prod_server::router(pool)
        .await
        .map_err(map_err)?
        .router
        .into())
}

fn map_err(e: impl std::fmt::Display) -> shuttle_runtime::Error {
    shuttle_runtime::Error::Database(e.to_string())
}
