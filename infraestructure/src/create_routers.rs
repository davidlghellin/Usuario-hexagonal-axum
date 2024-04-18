use axum::{body::Body, routing::get, Router};

use crate::hello::hello;

pub fn create_routes() -> Router<Body> {
    Router::new().route("/", get(hello))
}
