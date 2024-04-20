pub trait FindAllUser {
    fn find_all_users(&self) -> domain::user::user::User;
}
