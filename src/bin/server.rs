use log::{info, warn};
use template_rust_app::env::Env;

fn main() {
    env_logger::init();
    if color_eyre::install().is_err() {
        warn!("Failed to install color_eyre");
    }
    let env = Env::new().unwrap();
    info!("Server running on port: {}", env.port);
}
