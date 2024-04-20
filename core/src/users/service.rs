use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use domain::user::request::RegisterUserDto;
use domain::user::user::User;

use crate::error::MyAppError;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersService {
    async fn register_user(&self, request: RegisterUserDto) -> Result<User, MyAppError>;
}
