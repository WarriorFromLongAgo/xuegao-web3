use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DepositDo {
    /// 唯一标识符
    pub guid: Option<i32>,
    /// 区块哈希值
    pub block_hash: String,
    /// 区块编号
    pub block_number: Decimal,
    /// 交易哈希值
    pub hash: String,
    /// 发起地址
    pub from_address: String,
    /// 目标地址
    pub to_address: String,
    /// 代币地址
    pub token_address: String,
    /// 手续费
    pub fee: Decimal,
    /// 交易金额
    pub amount: Decimal,
    /// 交易状态
    pub status: i16,
    /// 交易索引
    pub transaction_index: Decimal,
    /// 时间戳
    pub timestamp: NaiveDateTime,
}

impl DepositDo {
    // 插入新的 Deposit 对象到数据库
    pub async fn insert(&self, pool: &PgPool) -> Result<DepositDo, sqlx::Error> {
        let query = r#"
            INSERT INTO deposits (block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp
        "#;

        let result = sqlx::query_as::<_, DepositDo>(query)
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
            .fetch_one(pool)
            .await?;

        Ok(result)
    }


    // 通过 guid 查询 Deposit 对象
    pub async fn get_by_guid(pool: &PgPool, guid: i32) -> Result<Option<DepositDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp
            FROM deposits WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, DepositDo>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 列表 Deposits
    pub async fn list(pool: &PgPool) -> Result<Vec<DepositDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp
            FROM deposits
        "#;

        let result = sqlx::query_as::<_, DepositDo>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    // 更新当前 Deposit 对象
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<DepositDo>, sqlx::Error> {
        let query = r#"
            UPDATE deposits
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
                timestamp = $11
            WHERE guid = $12
            RETURNING guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp
        "#;

        let result = sqlx::query_as::<_, DepositDo>(query)
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
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }
    // 通过 guid 删除 Deposit 对象
    pub async fn delete_by_guid(pool: &PgPool, guid: i32) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM deposits WHERE guid = $1
        "#;

        let result = sqlx::query(query)
            .bind(guid)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}