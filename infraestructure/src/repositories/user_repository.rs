use core::users::repository::UserEntity;

use anyhow::Context;
use async_trait::async_trait;
use mockall::automock;
use sqlx::query_as;

use crate::connection_pool::ConduitConnectionPool;

#[derive(Clone)]
pub struct PostgresUsersRepository {
    pool: ConduitConnectionPool,
}

impl PostgresUsersRepository {
    pub fn new(pool: ConduitConnectionPool) -> Self {
        Self { pool }
    }
}
