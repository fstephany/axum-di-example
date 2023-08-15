use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/", get(home))
}

pub async fn home() -> String {
    "Hello Struct".to_owned()
}
