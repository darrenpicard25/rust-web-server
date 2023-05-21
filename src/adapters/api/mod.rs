use axum::Router;

use crate::{app_state::AppState, error::ServiceStartupError};

pub mod error;
mod routes_hello;
mod routes_todo;
mod routes_user;

pub fn build_route(app_state: AppState) -> Result<Router, ServiceStartupError> {
    Ok(Router::new()
        .merge(routes_hello::routes())
        .merge(routes_todo::routes(app_state.clone()))
        .merge(routes_user::routes(app_state.clone())))
}
