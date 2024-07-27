use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub guid: i32,
    pub user_uid: String,
    pub address: String,
    pub address_type: i16,
    pub private_key: String,
    pub public_key: String,
    pub timestamp: i64,
}

impl Address {
    // 创建 Address
    pub async fn insert(&self, pool: &PgPool) -> Result<Address, sqlx::Error> {
        let query = r#"
            INSERT INTO addresses (user_uid, address, address_type, private_key, public_key, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING guid, user_uid, address, address_type, private_key, public_key, timestamp
        "#;

        let result = sqlx::query_as::<_, Address>(query)
            .bind(&self.user_uid)
            .bind(&self.address)
            .bind(self.address_type)
            .bind(&self.private_key)
            .bind(&self.public_key)
            .bind(self.timestamp)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // 读取 Address
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<Address>, sqlx::Error> {
        let query = r#"
            SELECT guid, user_uid, address, address_type, private_key, public_key, timestamp
            FROM addresses WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, Address>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Address
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<Address>, sqlx::Error> {
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

        let result = sqlx::query_as::<_, Address>(query)
            .bind(&self.user_uid)
            .bind(&self.address)
            .bind(self.address_type)
            .bind(&self.private_key)
            .bind(&self.public_key)
            .bind(self.timestamp)
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
    pub async fn list(pool: &PgPool) -> Result<Vec<Address>, sqlx::Error> {
        let query = r#"
            SELECT guid, user_uid, address, address_type, private_key, public_key, timestamp
            FROM addresses
        "#;

        let result = sqlx::query_as::<_, Address>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }
}