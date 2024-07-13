use axum::async_trait;
use deadpool_postgres::{Pool, PoolError};
use time::OffsetDateTime;
use tower_sessions::{
    session::{Id, Record},
    ExpiredDeletion, SessionStore,
};

/// A postgres store
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct PostgresStore {
    pool: Pool,
    schema_name: String,
    table_name: String,
}

/// An error encountered while using the postgres store
#[derive(thiserror::Error, Debug)]
pub enum PostgresStoreError {
    /// Postgres pool error
    #[error(transparent)]
    Pool(#[from] PoolError),
    /// Postgres error
    #[error(transparent)]
    Postgres(#[from] tokio_postgres::Error),
    /// `rmp_serde` encode errors.
    #[error("Rust MsgPack encode error: {0}")]
    RmpSerdeEncode(#[from] rmp_serde::encode::Error),
    /// `rmp_serde` decode errors.
    #[error("Rust MsgPack decode error: {0}")]
    RmpSerdeDecode(#[from] rmp_serde::decode::Error),
}

/// postgres store result
pub type PGResult<T> = Result<T, PostgresStoreError>;

impl From<PostgresStoreError> for SessionError {
    fn from(value: PostgresStoreError) -> Self {
        use PostgresStoreError::*;
        match &value {
            Pool(_) | Postgres(_) => Self::Backend(value.to_string()),
            RmpSerdeEncode(_) => Self::Encode(value.to_string()),
            RmpSerdeDecode(_) => Self::Decode(value.to_string()),
        }
    }
}

impl PostgresStore {
    /// Initializes the store
    #[must_use]
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            schema_name: "tower_sessions".into(),
            table_name: "session".into(),
        }
    }

    /// Migrates the store
    ///
    /// # Errors
    ///
    /// See [`PostgresStoreError`]
    pub async fn migrate(&self) -> Result<(), PostgresStoreError> {
        let create_schema_query = format!(
            r#"create schema if not exists "{schema_name}""#,
            schema_name = self.schema_name,
        );

        let mut conn = self.pool.get().await?;

        let tx = conn.transaction().await?;
        tx.execute(&create_schema_query, &[]).await?;

        let create_table_query = format!(
            r#"
            create table if not exists "{schema_name}"."{table_name}"
            (
                id text primary key not null,
                data bytea not null,
                expiry_date timestamptz not null
            )
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );

        tx.execute(&create_table_query, &[]).await?;
        tx.commit().await?;
        Ok(())
    }
    async fn delete_expired_inner(&self) -> PGResult<()> {
        let query = format!(
            r#"
            delete from "{schema_name}"."{table_name}"
            where expiry_date < (now() at time zone 'utc')
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        let conn = self.pool.get().await?;
        conn.execute(&query, &[]).await?;
        Ok(())
    }
    async fn save_inner(&self, session: &Record) -> PGResult<()> {
        let query = format!(
            r#"
            insert into "{schema_name}"."{table_name}" (id, data, expiry_date)
            values ($1, $2, $3)
            on conflict (id) do update
            set
              data = excluded.data,
              expiry_date = excluded.expiry_date
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        let data = rmp_serde::to_vec(&session)?;
        let conn = self.pool.get().await?;
        conn.execute(
            &query,
            &[&session.id.to_string(), &data, &session.expiry_date],
        )
        .await?;
        Ok(())
    }

    async fn load_inner(&self, session_id: &Id) -> PGResult<Option<Record>> {
        let query = format!(
            r#"
            select data from "{schema_name}"."{table_name}"
            where id = $1 and expiry_date > $2
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        let cur_date = OffsetDateTime::now_utc();
        let conn = self.pool.get().await?;
        let rows = conn
            .query(&query, &[&session_id.to_string(), &cur_date])
            .await?;

        let Some(row) = rows.first() else {
            return Ok(None);
        };

        let data: Vec<u8> = row.get("data");
        Ok(Some(rmp_serde::from_slice(&data)?))
    }

    async fn delete_inner(&self, session_id: &Id) -> PGResult<()> {
        let query = format!(
            r#"delete from "{schema_name}"."{table_name}" where id = $1"#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        let conn = self.pool.get().await?;
        conn.execute(&query, &[&session_id.to_string()]).await?;
        Ok(())
    }
}

type SessionError = tower_sessions::session_store::Error;
type SessionResult<T> = tower_sessions::session_store::Result<T>;

#[async_trait]
impl ExpiredDeletion for PostgresStore {
    async fn delete_expired(&self) -> SessionResult<()> {
        Ok(self
            .delete_expired_inner()
            .await
            .map_err(SessionError::from)?)
    }
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn save(&self, session_record: &Record) -> SessionResult<()> {
        Ok(self
            .save_inner(session_record)
            .await
            .map_err(SessionError::from)?)
    }
    async fn load(&self, session_id: &Id) -> SessionResult<Option<Record>> {
        Ok(self
            .load_inner(session_id)
            .await
            .map_err(SessionError::from)?)
    }
    async fn delete(&self, session_id: &Id) -> SessionResult<()> {
        Ok(self
            .delete_inner(session_id)
            .await
            .map_err(SessionError::from)?)
    }
}
