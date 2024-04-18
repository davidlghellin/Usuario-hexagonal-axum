use tracing::debug;

pub async fn hello() -> String {
    debug!("Pasamos por hola mundo");
    "Hola mundo ".to_owned()
}
