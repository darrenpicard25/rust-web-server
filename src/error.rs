use std::net::SocketAddr;

#[derive(Debug)]
pub enum ServiceStartupError {
    BuildRoute,
    ServiceStartup { addr: SocketAddr },
    DatabaseConnection,
    DatabaseMigration,
}
