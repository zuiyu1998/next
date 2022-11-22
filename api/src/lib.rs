use poem::{listener::TcpListener, Server};
pub use tokio;

pub mod users;

pub mod app;

pub async fn init() -> anyhow::Result<()> {
    let app = app::create();
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;
    Ok(())
}
