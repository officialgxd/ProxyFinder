mod telegram_bot;
mod server;


#[tokio::main]
async fn main() {
    telegram_bot::bot().await.unwrap();
    server::server().await.unwrap();
}