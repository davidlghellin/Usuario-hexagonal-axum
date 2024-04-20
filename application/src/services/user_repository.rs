use crate::ports::user::{create_user::CreateUser, find_all_users::FindAllUser};

pub trait Repository: CreateUser + FindAllUser {}
impl<T> Repository for T where T:CreateUser + FindAllUser {}
