use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
/// 代表提现信息的结构体
pub struct WithdrawDo {
    /// 可选的唯一标识符
    pub guid: Option<i32>,
    /// 区块哈希值
    pub block_hash: String,
    /// 区块编号
    pub block_number: i64,
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
    pub transaction_index: i64,
    /// 时间戳
    pub timestamp: NaiveDateTime,
    /// 交易签名的十六进制表示
    pub tx_sign_hex: String,
}

impl WithdrawDo {
    // 插入新的 Deposit 对象到数据库
    pub async fn insert(&self, pool: &PgPool) -> Result<WithdrawDo, sqlx::Error> {
        let query = r#"
            INSERT INTO withdraws (block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex
        "#;

        let result = sqlx::query_as::<_, WithdrawDo>(query)
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
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<WithdrawDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex
            FROM withdraws WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, WithdrawDo>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Withdraw
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<WithdrawDo>, sqlx::Error> {
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

        let result = sqlx::query_as::<_, WithdrawDo>(query)
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
    pub async fn list(pool: &PgPool) -> Result<Vec<WithdrawDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex
            FROM withdraws
        "#;

        let result = sqlx::query_as::<_, WithdrawDo>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }


}


