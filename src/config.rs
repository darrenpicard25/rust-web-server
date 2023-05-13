use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Port to run Server on
    #[arg(short, long, default_value_t = 8000)]
    pub port: u16,

    #[arg(long, default_value = "postgres://admin:admin@localhost:5432")]
    pub connection_string: String,
}
