use core::config::AppConfig;
use std::sync::Arc;

use dotenvy_macro::dotenv;
use infraestructure::run::run;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber: FmtSubscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG) // TRACE
        // completes the builder.
        .finish();
    let david_name = dotenv!("DAVID_VAR");

    //tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
/* 
    let config = Arc::new(AppConfig::parse());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Iniciamos la app");
    info!("Hello, world!");
    let usuario = domain::user::user::User::new(david_name.to_owned());
    info!("{}", usuario);
*/
    run().await
}
