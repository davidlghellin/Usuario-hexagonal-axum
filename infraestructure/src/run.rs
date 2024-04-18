use axum::Router;
use dotenvy_macro::dotenv;
use tracing::debug;

use crate::create_routers::create_routes;

pub async fn run() {
    let app: Router = create_routes();
    let port: u16 = dotenv!("PORT").parse().unwrap();
    let database: &str = dotenv!("DATABASE");

    debug!("The database is: {}", database);
    debug!("The port is:     {}", port);

    let url: String = format!("{}:{}", database, port);
    debug!("The url is:      {}", url);

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
