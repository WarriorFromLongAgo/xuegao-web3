use std::collections::HashMap;

use chrono::NaiveDateTime;
use log::info;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TokenDo {
    /// 可选的唯一标识符
    pub guid: Option<i32>,
    /// 代币地址
    pub token_address: String,
    /// 代币的小数位数
    pub decimal: i16,
    /// 代币名称
    pub token_name: String,
    /// 收集的金额
    pub collect_amount: Decimal,
    /// 时间戳
    pub timestamp: NaiveDateTime,
}

impl TokenDo {
    // 创建 Token
    pub async fn create(&self, pool: &PgPool) -> Result<TokenDo, sqlx::Error> {
        let query = r#"
            INSERT INTO tokens (token_address, decimal, token_name, collect_amount, timestamp)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING guid, token_address, decimal, token_name, collect_amount, timestamp
        "#;

        let result = sqlx::query_as::<_, TokenDo>(query)
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
    pub async fn get(pool: &PgPool, guid: i32) -> Result<Option<TokenDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, token_address, decimal, token_name, collect_amount, timestamp
            FROM tokens WHERE guid = $1
        "#;

        let result = sqlx::query_as::<_, TokenDo>(query)
            .bind(guid)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    // 更新 Token
    pub async fn update(&self, pool: &PgPool, guid: i32) -> Result<Option<TokenDo>, sqlx::Error> {
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

        let result = sqlx::query_as::<_, TokenDo>(query)
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
    pub async fn list(pool: &PgPool) -> Result<Vec<TokenDo>, sqlx::Error> {
        let query = r#"
            SELECT guid, token_address, decimal, token_name, collect_amount, timestamp
            FROM tokens
        "#;

        let result = sqlx::query_as::<_, TokenDo>(query)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    // 列表 Tokens
    pub async fn list_return_map(pool: &PgPool) -> Result<HashMap<String, TokenDo>, sqlx::Error> {
        // 转换为 HashMap
        let mut result_map = HashMap::new();

        let db_list_result = Self::list(pool).await;

        if db_list_result.is_err() {
            let db_list_result_error = db_list_result.err().unwrap();
            info!("[xuegao-web3][token_do][token_do][db_list_result_error={}]", db_list_result_error);
            // 处理具体的 sqlx::Error 类型
            return match db_list_result_error {
                sqlx::Error::RowNotFound => {
                    Ok(result_map)
                }
                _ => {
                    // 处理其他 sqlx::Error 错误类型
                    info!("[xuegao-web3][token_do][token_do][other err]");
                    Ok(result_map)
                }
            };
        } else {
            let db_list = db_list_result.unwrap();

            for token in db_list {
                result_map.insert(token.token_address.clone(), token);
            }
        }
        Ok(result_map)
    }
}