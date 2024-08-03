use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BalanceDo {
    /// 可选的唯一标识符
    pub guid: Option<i32>,
    /// 地址字符串
    pub address: String,
    /// 地址类型，引用自 [`AddressType`](address_type_enum::AddressType)
    /// 归集地址 热钱包 冷钱包 用户地址
    pub address_type: String,
    /// 代币地址
    pub token_address: String,
    /// 当前余额
    pub balance: Decimal,
    /// 锁定余额
    pub lock_balance: Decimal,
    /// 时间戳
    pub timestamp: NaiveDateTime,
}


impl BalanceDo {
    // 创建 Balance
    pub async fn insert(&self, pool: &PgPool) -> Result<BalanceDo, sqlx::Error> {
        let query = r#"
            INSERT INTO balances (address, address_type, token_address, balance, lock_balance, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING guid, address, address_type, token_address, balance, lock_balance, timestamp
        "#;

        let result = sqlx::query_as::<_, BalanceDo>(query)
            .bind(&self.address)
            .bind(&self.address_type)
            .bind(&self.token_address)
            .bind(self.balance)
            .bind(self.lock_balance)
            .bind(self.timestamp)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Balance
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<BalanceDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, address, address_type, token_address, balance, lock_balance, timestamp
            FROM balances WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, BalanceDo>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Balance
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<BalanceDo>, sqlx::Error> {
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

        let result = sqlx::query_as::<_, BalanceDo>(query)
            .bind(&self.address)
            .bind(&self.address_type)
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
    pub async fn list(pool: &PgPool) -> Result<Vec<BalanceDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, address, address_type, token_address, balance, lock_balance, timestamp
            FROM balances
        "#;

        let result = sqlx::query_as::<_, BalanceDo>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}


