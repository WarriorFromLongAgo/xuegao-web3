use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Block {
    pub hash: String,
    pub parent_hash: String,
    pub number: i64,
    pub timestamp: i64,
    pub rlp_bytes: String,
}


impl Block {
    // 创建 Block
    pub async fn create(&self, pool: &PgPool) -> Result<Block, sqlx::Error> {
        let query = r#"
            INSERT INTO blocks (hash, parent_hash, number, timestamp, rlp_bytes)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING hash, parent_hash, number, timestamp, rlp_bytes
        "#;

        let result = sqlx::query_as::<_, Block>(query)
            .bind(&self.hash)
            .bind(&self.parent_hash)
            .bind(self.number)
            .bind(self.timestamp)
            .bind(&self.rlp_bytes)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Block
    pub async fn get(pool: &PgPool, hash: &str) -> Result<Option<Block>, sqlx::Error> {
        let query = r#"
            SELECT hash, parent_hash, number, timestamp, rlp_bytes
            FROM blocks WHERE hash = $1
        "#;

        let result = sqlx::query_as::<_, Block>(query)
            .bind(hash)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Block
    pub async fn update(&self, pool: &PgPool, hash: &str) -> Result<Option<Block>, sqlx::Error> {
        let query = r#"
            UPDATE blocks
            SET parent_hash = $1,
                number = $2,
                timestamp = $3,
                rlp_bytes = $4
            WHERE hash = $5
            RETURNING hash, parent_hash, number, timestamp, rlp_bytes
        "#;

        let result = sqlx::query_as::<_, Block>(query)
            .bind(&self.parent_hash)
            .bind(self.number)
            .bind(self.timestamp)
            .bind(&self.rlp_bytes)
            .bind(hash)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 删除 Block
    pub async fn delete(pool: &PgPool, hash: &str) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM blocks WHERE hash = $1
        "#;

        let result = sqlx::query(query)
            .bind(hash)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // 列表 Blocks
    pub async fn list(pool: &PgPool) -> Result<Vec<Block>, sqlx::Error> {
        let query = r#"
            SELECT hash, parent_hash, number, timestamp, rlp_bytes
            FROM blocks
        "#;

        let result = sqlx::query_as::<_, Block>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}



