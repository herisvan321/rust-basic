use axum_session::{DatabasePool, DatabaseError};
use async_trait::async_trait;
use sqlx::AnyPool;

#[derive(Clone, Debug)]
pub struct RustBasicSessionStore {
    pub pool: AnyPool,
}

impl RustBasicSessionStore {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabasePool for RustBasicSessionStore {
    async fn initiate(&self, _table_name: &str) -> Result<(), DatabaseError> {
        Ok(())
    }

    async fn delete_one_by_id(&self, id: &str, table_name: &str) -> Result<(), DatabaseError> {
        let query = format!("DELETE FROM {} WHERE id = $1", table_name);
        sqlx::query(&query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn load(&self, id: &str, table_name: &str) -> Result<Option<String>, DatabaseError> {
        let query = format!("SELECT payload FROM {} WHERE id = $1 AND last_activity > $2", table_name);
        let now = chrono::Utc::now().timestamp();
        
        let row: Option<(String,)> = sqlx::query_as(&query)
            .bind(id)
            .bind(now)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;

        Ok(row.map(|r| r.0))
    }

    async fn store(&self, id: &str, session: &str, expires: i64, table_name: &str) -> Result<(), DatabaseError> {
        let delete_query = format!("DELETE FROM {} WHERE id = $1", table_name);
        sqlx::query(&delete_query).bind(id).execute(&self.pool).await.ok();

        let insert_query = format!(
            "INSERT INTO {} (id, payload, last_activity) VALUES ($1, $2, $3)",
            table_name
        );

        sqlx::query(&insert_query)
            .bind(id)
            .bind(session)
            .bind(expires)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericInsertError(e.to_string()))?;
        Ok(())
    }

    async fn delete_by_expiry(&self, table_name: &str) -> Result<Vec<String>, DatabaseError> {
        let now = chrono::Utc::now().timestamp();
        let select_query = format!("SELECT id FROM {} WHERE last_activity < $1", table_name);
        let ids: Vec<String> = sqlx::query_as::<_, (String,)>(&select_query)
            .bind(now)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?
            .into_iter()
            .map(|r| r.0)
            .collect();

        let delete_query = format!("DELETE FROM {} WHERE last_activity < $1", table_name);
        sqlx::query(&delete_query)
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;

        Ok(ids)
    }

    async fn count(&self, table_name: &str) -> Result<i64, DatabaseError> {
        let query = format!("SELECT COUNT(*) FROM {}", table_name);
        let count: (i64,) = sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        Ok(count.0)
    }

    async fn exists(&self, id: &str, table_name: &str) -> Result<bool, DatabaseError> {
        let query = format!("SELECT id FROM {} WHERE id = $1", table_name);
        let row: Option<(String,)> = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        Ok(row.is_some())
    }

    async fn delete_all(&self, table_name: &str) -> Result<(), DatabaseError> {
        let query = format!("DELETE FROM {}", table_name);
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn get_ids(&self, table_name: &str) -> Result<Vec<String>, DatabaseError> {
        let query = format!("SELECT id FROM {}", table_name);
        let ids: Vec<String> = sqlx::query_as::<_, (String,)>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?
            .into_iter()
            .map(|r| r.0)
            .collect();
        Ok(ids)
    }

    fn auto_handles_expiry(&self) -> bool {
        false
    }
}
