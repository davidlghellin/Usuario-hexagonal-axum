use serde::{Deserialize, Serialize};

use crate::user::user::User;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserAuthenicationResponse {
    pub user: User,
}

impl UserAuthenicationResponse {
    pub fn new(
        nombre: String,
    ) -> Self {
        UserAuthenicationResponse {
            user: User {
                nombre,
            },
        }
    }
}
