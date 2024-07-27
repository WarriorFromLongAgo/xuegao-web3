use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Withdraw {
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
    pub timestamp: i32,
    pub tx_sign_hex: String,
}

impl Withdraw {
    // 插入新的 Deposit 对象到数据库
    pub async fn insert(&self, pool: &PgPool) -> Result<Withdraw, sqlx::Error> {
        let query = r#"
            INSERT INTO withdraws (block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex
        "#;

        let result = sqlx::query_as::<_, Withdraw>(query)
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
            .bind(self.timestamp)
            .bind(&self.tx_sign_hex)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Withdraw
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<Withdraw>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex
            FROM withdraws WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, Withdraw>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Withdraw
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<Withdraw>, sqlx::Error> {
        let query = r#"
            UPDATE withdraws
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
                timestamp = $11,
                tx_sign_hex = $12
            WHERE guid = $13
            RETURNING guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex
        "#;

        let result = sqlx::query_as::<_, Withdraw>(query)
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
            .bind(self.timestamp)
            .bind(&self.tx_sign_hex)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 删除 Withdraw
    pub async fn delete(pool: &PgPool, guid: i32) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM withdraws WHERE guid = $1
        "#;

        let result = sqlx::query(query)
            .bind(guid)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // 列表 Withdraws
    pub async fn list(pool: &PgPool) -> Result<Vec<Withdraw>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex
            FROM withdraws
        "#;

        let result = sqlx::query_as::<_, Withdraw>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }


}


