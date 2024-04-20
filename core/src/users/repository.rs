use std::time::SystemTime;

use domain::user::user::User;
use sqlx::FromRow;
use sqlx::types::time::OffsetDateTime;

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