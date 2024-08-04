use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AddressDo {
    /// 可选的唯一标识符
    pub guid: Option<i32>,
    /// 用户的唯一标识符 默认是user_id，然后修改时进行赋值
    pub user_uid: String,
    /// 地址字符串
    pub address: String,
    /// 地址类型，引用自 `address_type_enum.rs`
    /// 归集地址 热钱包 冷钱包 用户地址
    pub address_type: String,
    /// 私钥字符串
    pub private_key: String,
    /// 公钥字符串
    pub public_key: String,
    /// 时间戳
    pub timestamp: NaiveDateTime,
}

impl AddressDo {
    // 创建 Address
    pub async fn insert(&self, pool: &PgPool) -> Result<AddressDo, sqlx::Error> {
        let query = r#"
            INSERT INTO addresses (user_uid, address, address_type, private_key, public_key, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING guid, user_uid, address, address_type, private_key, public_key, timestamp
        "#;

        let result = sqlx::query_as::<_, AddressDo>(query)
            .bind(&self.user_uid)
            .bind(&self.address)
            .bind(&self.address_type)
            .bind(&self.private_key)
            .bind(&self.public_key)
            .bind(&self.timestamp)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Address
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<AddressDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, user_uid, address, address_type, private_key, public_key, timestamp
            FROM addresses WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, AddressDo>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Address
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<AddressDo>, sqlx::Error> {
        let query = r#"
            UPDATE addresses
            SET user_uid = $1,
                address = $2,
                address_type = $3,
                private_key = $4,
                public_key = $5,
                timestamp = $6
            WHERE guid = $7
            RETURNING guid, user_uid, address, address_type, private_key, public_key, timestamp
        "#;

        let result = sqlx::query_as::<_, AddressDo>(query)
            .bind(&self.user_uid)
            .bind(&self.address)
            .bind(&self.address_type)
            .bind(&self.private_key)
            .bind(&self.public_key)
            .bind(&self.timestamp)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 删除 Address
    pub async fn delete(pool: &PgPool, guid: i32) -> Result<bool, sqlx::Error> {
        let query = r#"
            DELETE FROM addresses WHERE guid = $1
        "#;

        let result = sqlx::query(query)
            .bind(guid)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // 列表 Addresses
    pub async fn list(pool: &PgPool) -> Result<Vec<AddressDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, user_uid, address, address_type, private_key, public_key, timestamp
            FROM addresses
        "#;

        let result = sqlx::query_as::<_, AddressDo>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    // 列表 Addresses
    pub async fn list_address(pool: &PgPool, address_type: &str) -> Result<HashSet<String>, sqlx::Error> {
        let query = r#"
            SELECT distinct address FROM addresses where address_type = $1 order by address
        "#;

        // 执行查询并获取所有结果
        let result: Vec<String> = sqlx::query_scalar(query)
            .bind(address_type)
            .fetch_all(pool)
            .await?;

        // 将结果转换为 HashSet
        let address_set: HashSet<String> = result.into_iter().collect();

        Ok(address_set)
    }
}