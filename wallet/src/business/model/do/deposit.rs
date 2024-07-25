use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Deposit {
    pub guid: i32,                 // 唯一标识符
    pub block_hash: String,       // 区块哈希值
    pub block_number: Decimal, // 区块编号
    pub hash: String,             // 交易哈希值
    pub from_address: String,     // 发起地址
    pub to_address: String,       // 目标地址
    pub token_address: String,    // 代币地址
    pub fee: Decimal,          // 手续费
    pub amount: Decimal,       // 交易金额
    pub status: i16,              // 交易状态
    pub transaction_index: Decimal, // 交易索引
    pub timestamp: i64,           // 时间戳
}

impl Deposit {
    // 插入新的 Deposit 对象到数据库
    pub async fn insert(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        INSERT INTO deposits (
            guid,
            block_hash,
            block_number,
            hash,
            from_address,
            to_address,
            token_address,
            fee,
            amount,
            status,
            transaction_index,
            timestamp
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
        )
        "#,
        )
            .bind(self.guid) // 绑定 i32 类型的字段
            .bind(&self.block_hash) // 绑定 String 类型的字段
            .bind(self.block_number) // 绑定 Decimal 类型的字段
            .bind(&self.hash) // 绑定 String 类型的字段
            .bind(&self.from_address) // 绑定 String 类型的字段
            .bind(&self.to_address) // 绑定 String 类型的字段
            .bind(&self.token_address) // 绑定 String 类型的字段
            .bind(self.fee) // 绑定 Decimal 类型的字段
            .bind(self.amount) // 绑定 Decimal 类型的字段
            .bind(self.status) // 绑定 i16 类型的字段
            .bind(self.transaction_index) // 绑定 Decimal 类型的字段
            .bind(self.timestamp) // 绑定 i64 类型的字段
            .execute(pool)
            .await
            .map(|_| ())
    }


    // 通过 guid 查询 Deposit 对象
    pub async fn get_by_guid(pool: &PgPool, guid: i32) -> Result<Deposit, sqlx::Error> {
        let deposit = sqlx::query_as::<_, Deposit>(
            r#"
        SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp
        FROM deposits
        WHERE guid = $1
        "#,
        )
            .bind(guid)
            .fetch_one(pool)
            .await?;

        Ok(deposit)
    }

    pub fn list(pool: &PgPool) -> Result<Vec<Deposit>, sqlx::Error> {
        // 执行查询并自动映射到 Deposit 结构体
        let deposits = sqlx::query_as(
            r#"
        SELECT guid, block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp
        FROM deposits
        "#
        )
            .fetch_all(pool)?;
        Ok(deposits)
    }

    // 更新当前 Deposit 对象
    pub async fn update(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE deposits SET
                block_hash = $2,
                block_number = $3,
                hash = $4,
                from_address = $5,
                to_address = $6,
                token_address = $7,
                fee = $8,
                amount = $9,
                status = $10,
                transaction_index = $11,
                timestamp = $12
            WHERE guid = $1
            "#,
        )
            .bind(self.guid)
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
            .execute(pool)
            .await
            .map(|_| ())
    }
    // 通过 guid 删除 Deposit 对象
    pub async fn delete_by_guid(pool: &PgPool, guid: i32) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM deposits
            WHERE guid = $1
            "#,
        )
            .bind(guid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}