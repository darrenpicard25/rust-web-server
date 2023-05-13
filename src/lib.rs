use std::net::SocketAddr;

use error::ServiceStartupError;

use crate::{adapters::api, app_state::AppState};

mod adapters;
mod app_state;
mod domain;
pub mod error;
mod infrastructure;
mod services;

pub struct App {
    address: SocketAddr,
    connection_string: String,
}

/// Constructor
impl App {
    pub fn new(address: SocketAddr, connection_string: String) -> Self {
        Self {
            address,
            connection_string,
        }
    }
}

/// Methods
impl App {
    pub async fn run(&self) -> Result<(), ServiceStartupError> {
        tracing::info!("Starting Server on: {}", self.address);

        let app_state = AppState::new(&self.connection_string).await?;

        axum::Server::bind(&self.address)
            .serve(api::build_route(app_state)?.into_make_service())
            .await
            .map_err(|_| ServiceStartupError::ServiceStartup { addr: self.address })?;

        Ok(())
    }
}
