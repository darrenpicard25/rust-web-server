use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello_2))
}

// e.g. `/hello?name=Darren`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    tracing::info!("->> {:12} - hello handler: {:?}", "HANDLER", params);
    let name = params.name.as_deref().unwrap_or("World");

    format!("Hello, {}!", name)
}

// e.g. `/hello/Darren`
async fn handler_hello_2(Path(name): Path<String>) -> impl IntoResponse {
    tracing::info!("->> {:12} - hello handler 2: {:?}", "HANDLER", name);

    format!("Hello, {}! From handler 2", name)
}
