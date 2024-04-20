use axum::Router;
use dotenvy_macro::dotenv;

use crate::create_routers::create_routes;

pub async fn run() {
    let app: Router = create_routes();
    let port: u16 = dotenv!("PORT").parse().unwrap();
    let database: &str = dotenv!("DATABASE");

    let url: String = format!("{}:{}", database, port);

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
