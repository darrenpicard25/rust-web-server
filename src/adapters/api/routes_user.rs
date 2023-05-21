use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        entities::user::User,
        services::user_service::{CreateInput, UpdateInput},
    },
};

use super::error::ApiResult;

#[derive(Serialize)]
struct ApiUser {
    id: String,
    email: String,
    first_name: String,
}

impl From<User> for ApiUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            first_name: value.first_name,
        }
    }
}

impl IntoResponse for ApiUser {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/user", post(handler_create))
        .route("/user/:id", patch(handler_update).get(handler_get))
        .with_state(app_state)
}

async fn handler_get(
    State(AppState { user_service, .. }): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<ApiUser> {
    tracing::info!("Get /user/{id}");

    Ok(user_service.get(id).await?.into())
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    email: String,
    first_name: String,
}

async fn handler_create(
    State(AppState { user_service, .. }): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> ApiResult<ApiUser> {
    tracing::info!("Post /user | {payload:?}");

    let input = CreateInput {
        email: payload.email,
        first_name: payload.first_name,
    };

    Ok(user_service.create(input).await?.into())
}

#[derive(Debug, Deserialize)]
struct UpdatePayload {
    email: Option<String>,
    first_name: Option<String>,
}

async fn handler_update(
    State(AppState { user_service, .. }): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePayload>,
) -> ApiResult<ApiUser> {
    tracing::info!("Patch /user/{id} | {payload:?}");

    let input = UpdateInput {
        email: payload.email,
        first_name: payload.first_name,
    };

    Ok(user_service.update(id, input).await?.into())
}
