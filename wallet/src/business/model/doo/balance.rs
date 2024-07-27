use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Balance {
    pub guid: i32,
    pub address: String,
    pub address_type: i16,
    pub token_address: String,
    pub balance: f64,
    pub lock_balance: f64,
    pub timestamp: i64,
}


impl Balance {
    // 创建 Balance
    pub async fn insert(&self, pool: &PgPool) -> Result<Balance, sqlx::Error> {
        let query = r#"
            INSERT INTO balances (address, address_type, token_address, balance, lock_balance, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING guid, address, address_type, token_address, balance, lock_balance, timestamp
        "#;

        let result = sqlx::query_as::<_, Balance>(query)
            .bind(&self.address)
            .bind(self.address_type)
            .bind(&self.token_address)
            .bind(self.balance)
            .bind(self.lock_balance)
            .bind(self.timestamp)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Balance
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<Balance>, sqlx::Error> {
        let query = r#"
            SELECT guid, address, address_type, token_address, balance, lock_balance, timestamp
            FROM balances WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, Balance>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Balance
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<Balance>, sqlx::Error> {
        let query = r#"
            UPDATE balances
            SET address = $1,
                address_type = $2,
                token_address = $3,
                balance = $4,
                lock_balance = $5,
                timestamp = $6
            WHERE guid = $7
            RETURNING guid, address, address_type, token_address, balance, lock_balance, timestamp
        "#;

        let result = sqlx::query_as::<_, Balance>(query)
            .bind(&self.address)
            .bind(self.address_type)
            .bind(&self.token_address)
            .bind(self.balance)
            .bind(self.lock_balance)
            .bind(self.timestamp)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 删除 Balance
    pub async fn delete(pool: &PgPool, guid: i32) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM balances WHERE guid = $1
        "#;

        let result = sqlx::query(query)
            .bind(guid)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // 列表 Balances
    pub async fn list(pool: &PgPool) -> Result<Vec<Balance>, sqlx::Error> {
        let query = r#"
            SELECT guid, address, address_type, token_address, balance, lock_balance, timestamp
            FROM balances
        "#;

        let result = sqlx::query_as::<_, Balance>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}


