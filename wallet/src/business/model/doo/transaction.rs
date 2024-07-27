use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub guid: i32,
    pub block_hash: String,
    pub block_number: i64,
    pub hash: String,
    pub from_address: String,
    pub to_address: String,
    pub token_address: String,
    pub fee: f64,
    pub amount: f64,
    pub status: i16,
    pub transaction_index: i64,
    pub tx_type: i16,
    pub timestamp: i64,
}

impl Transaction {
    // 创建 Transaction
    pub async fn insert(&self, pool: &PgPool) -> Result<Transaction, sqlx::Error> {
        let query = r#"
            INSERT INTO transactions (block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, tx_type, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, tx_type, timestamp
        "#;

        let result = sqlx::query_as::<_, Transaction>(query)
            .bind(&self.block_hash)
            .bind(self.block_number)
            .bind(&self.hash)
            .bind(&self.from_address)
            .bind(&self.to_address)
            .bind(&self.token_address)
            .bind(self.fee)
            .bind(self.amount)
            .bind(self.status)
            .bind(self.transaction_index)
            .bind(self.tx_type)
            .bind(self.timestamp)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Transaction
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<Transaction>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, tx_type, timestamp
            FROM transactions WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, Transaction>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Transaction
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<Transaction>, sqlx::Error> {
        let query = r#"
            UPDATE transactions
            SET block_hash = $1,
                block_number = $2,
                hash = $3,
                from_address = $4,
                to_address = $5,
                token_address = $6,
                fee = $7,
                amount = $8,
                status = $9,
                transaction_index = $10,
                tx_type = $11,
                timestamp = $12
            WHERE guid = $13
            RETURNING guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, tx_type, timestamp
        "#;

        let result = sqlx::query_as::<_, Transaction>(query)
            .bind(&self.block_hash)
            .bind(self.block_number)
            .bind(&self.hash)
            .bind(&self.from_address)
            .bind(&self.to_address)
            .bind(&self.token_address)
            .bind(self.fee)
            .bind(self.amount)
            .bind(self.status)
            .bind(self.transaction_index)
            .bind(self.tx_type)
            .bind(self.timestamp)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 删除 Transaction
    pub async fn delete(pool: &PgPool, guid: i32) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM transactions WHERE guid = $1
        "#;

        let result = sqlx::query(query)
            .bind(guid)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // 列表 Transactions
    pub async fn list(pool: &PgPool) -> Result<Vec<Transaction>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, tx_type, timestamp
            FROM transactions
        "#;

        let result = sqlx::query_as::<_, Transaction>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}
