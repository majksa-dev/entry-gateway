use entry_gateway::create_server;
use log::warn;

#[tokio::main]
async fn main() {
    env_logger::init();
    if color_eyre::install().is_err() {
        warn!("Failed to install color_eyre");
    }
    create_server().await.unwrap();
}
