use next_api::{init, tokio};

#[tokio::main]
async fn main() {
    if let Err(e) = init().await {
        println!("next-api error: {}", e);
    }
}
