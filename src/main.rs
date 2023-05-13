use std::net::{Ipv4Addr, SocketAddr};

use clap::Parser;
use rust_web_server::{error::ServiceStartupError, App};

use crate::config::Config;

mod config;

#[tokio::main]
async fn main() -> Result<(), ServiceStartupError> {
    tracing_subscriber::fmt().init();

    let config = Config::parse();
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, config.port));

    App::new(addr, config.connection_string).run().await?;

    Ok(())
}
