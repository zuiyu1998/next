use next_service::sea_orm::Database;
use poem::{listener::TcpListener, middleware::Tracing, EndpointExt, Server};
pub use tokio;

pub mod admin;
pub mod users;

pub mod app;
pub mod data;
pub mod models;

pub mod error;
pub mod extra;
pub mod middlewares;

pub mod config;

use error::Result;

pub async fn init() -> Result<()> {
    tracing_subscriber::fmt().init();

    dotenvy::dotenv().ok();

    let config = config::config()?;

    let conn = Database::connect(&config.database_url).await?;

    let service = next_service::Service::new(conn);

    config::init(&service).await;

    let app = app::create()
        .with(Tracing)
        .data(config.clone())
        .data(service);

    Server::new(TcpListener::bind(&format!(
        "{}:{}",
        config.host, config.port
    )))
    .run(app)
    .await?;
    Ok(())
}
