use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;

use domain::user::user::User;
use sqlx::FromRow;
use sqlx::types::time::OffsetDateTime;

pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn search_user_by_email_or_username(&self, email: &str, username: &str)
        -> anyhow::Result<Option<UserEntity>>;

    async fn create_user(&self, email: &str, username: &str, hashed_password: &str) -> anyhow::Result<UserEntity>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity>;

    async fn update_user(
        &self,
        nombre: String,
    ) -> anyhow::Result<UserEntity>;
}

#[derive(FromRow)]
pub struct UserEntity {
    pub nombre: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}


impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            nombre: "UserByDefautl".to_owned(),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}


impl UserEntity {
    pub fn into_dto(self) -> User {
        User {
            nombre: self.nombre,
        }
    }

}