pub trait CreateUser {
    fn create_user(&self, user: domain::user::user::User);
}
