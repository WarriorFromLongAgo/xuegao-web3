use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub guid: i32,
    pub token_address: String,
    pub decimal: i16,
    pub token_name: String,
    pub collect_amount: f64,
    pub timestamp: i64,
}

impl Token {
    // 创建 Token
    pub async fn create(&self, pool: &PgPool) -> Result<Token, sqlx::Error> {
        let query = r#"
            INSERT INTO tokens (token_address, decimal, token_name, collect_amount, timestamp)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING guid, token_address, decimal, token_name, collect_amount, timestamp
        "#;

        let result = sqlx::query_as::<_, Token>(query)
            .bind(&self.token_address)
            .bind(self.decimal)
            .bind(&self.token_name)
            .bind(self.collect_amount)
            .bind(self.timestamp)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Token
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<Token>, sqlx::Error> {
        let query = r#"
            SELECT guid, token_address, decimal, token_name, collect_amount, timestamp
            FROM tokens WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, Token>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Token
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<Token>, sqlx::Error> {
        let query = r#"
            UPDATE tokens
            SET token_address = $1,
                decimal = $2,
                token_name = $3,
                collect_amount = $4,
                timestamp = $5
            WHERE guid = $6
            RETURNING guid, token_address, decimal, token_name, collect_amount, timestamp
        "#;

        let result = sqlx::query_as::<_, Token>(query)
            .bind(&self.token_address)
            .bind(self.decimal)
            .bind(&self.token_name)
            .bind(self.collect_amount)
            .bind(self.timestamp)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 删除 Token
    pub async fn delete(pool: &PgPool, guid: i32) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM tokens WHERE guid = $1
        "#;

        let result = sqlx::query(query)
            .bind(guid)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // 列表 Tokens
    pub async fn list(pool: &PgPool) -> Result<Vec<Token>, sqlx::Error> {
        let query = r#"
            SELECT guid, token_address, decimal, token_name, collect_amount, timestamp
            FROM tokens
        "#;

        let result = sqlx::query_as::<_, Token>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}