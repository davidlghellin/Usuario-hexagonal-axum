fn main() {
    println!("Hello, world!");
    let usuario = domain::user::User::new("David".to_owned());
    println!("{}", usuario);
}
