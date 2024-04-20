use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserRequest {
    #[validate]
    pub user: RegisterUserDto,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserDto {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

impl RegisterUserDto {
    pub fn new_stub() -> Self {
        Self {
            username: Some(String::from("stub username")),
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}
