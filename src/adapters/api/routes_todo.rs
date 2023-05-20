use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        entities::todo::Todo,
        services::todo_service::{CreateInput, UpdateInput},
    },
};

use super::error::ApiResult;

#[derive(Serialize)]
struct ApiTodo {
    id: String,
    title: String,
    description: String,
}

impl From<Todo> for ApiTodo {
    fn from(value: Todo) -> Self {
        tracing::info!("Transforming Todo into ApiTodo");

        Self {
            id: value.id,
            title: value.title,
            description: value.description,
        }
    }
}

impl IntoResponse for ApiTodo {
    fn into_response(self) -> axum::response::Response {
        tracing::info!("Transforming ApiTodo into response");
        Json(self).into_response()
    }
}

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/todo", post(handler_create).get(handler_list))
        .route("/todo/:id", post(handler_update).get(handler_get))
        .with_state(app_state)
}

async fn handler_list(
    State(AppState { todo_service }): State<AppState>,
) -> ApiResult<Json<Vec<ApiTodo>>> {
    tracing::info!("->> {:12} - todo list handler", "HANDLER");

    Ok(Json(
        todo_service
            .list()
            .await?
            .into_iter()
            .map(|entity| entity.into())
            .collect(),
    ))
}

async fn handler_get(
    State(AppState { todo_service }): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<ApiTodo> {
    tracing::info!("->> {:12} - todo get handler: {:?}", "HANDLER", id);

    Ok(todo_service.get(id).await?.into())
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    title: String,
    description: String,
}

async fn handler_create(
    State(AppState { todo_service }): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> ApiResult<ApiTodo> {
    tracing::info!("->> {:12} - todo create handler: {:?}", "HANDLER", payload);

    let input = CreateInput {
        title: payload.title,
        description: payload.description,
    };

    Ok(todo_service.create(input).await?.into())
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
) -> ApiResult<ApiTodo> {
    tracing::info!("->> {:12} - todo update handler: {:?}", "HANDLER", payload);

    let input = UpdateInput {
        title: payload.title,
        description: payload.description,
    };

    Ok(todo_service.update(id, input).await?.into())
}
