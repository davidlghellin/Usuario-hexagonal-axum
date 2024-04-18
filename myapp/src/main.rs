use infraestructure::run::run;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let usuario = domain::user::User::new("David".to_owned());
    println!("{}", usuario);

    run().await
}
