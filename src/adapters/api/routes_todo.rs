use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::services::todo_service::{CreateInput, UpdateInput},
};

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/todo", post(handler_create).get(handler_list))
        .route("/todo/:id", post(handler_update).get(handler_get))
        .with_state(app_state)
}

async fn handler_list(State(AppState { todo_service }): State<AppState>) -> impl IntoResponse {
    tracing::info!("->> {:12} - todo list handler", "HANDLER");

    let todos = todo_service.list().await;

    format!(
        "Todo List, {}!",
        todos.into_iter().fold(String::new(), |mut acc, item| {
            acc.push_str(&item.id);
            acc.push(',');
            acc
        })
    )
}

async fn handler_get(
    State(AppState { todo_service }): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    tracing::info!("->> {:12} - todo get handler: {:?}", "HANDLER", id);

    let todo = todo_service.get(id).await;

    format!("Todo Get, {}", todo.id)
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    title: String,
    description: String,
}

async fn handler_create(
    State(AppState { todo_service }): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> impl IntoResponse {
    tracing::info!("->> {:12} - todo create handler: {:?}", "HANDLER", payload);

    let todo = todo_service
        .create(CreateInput {
            title: payload.title,
            description: payload.description,
        })
        .await;

    format!("Todo Create, {}", todo.id)
}

#[derive(Debug, Deserialize)]
struct UpdatePayload {
    title: Option<String>,
    description: Option<String>,
}

async fn handler_update(
    State(AppState { todo_service }): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePayload>,
) -> impl IntoResponse {
    tracing::info!("->> {:12} - todo update handler: {:?}", "HANDLER", payload);

    let todo = todo_service
        .update(
            id,
            UpdateInput {
                title: payload.title,
                description: payload.description,
            },
        )
        .await;

    format!("Todo Update, {}", todo.id)
}
