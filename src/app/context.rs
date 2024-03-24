use std::sync::Arc;

use crate::env::Env;

use super::config::Config;

pub struct Context {
    pub env: Env,
    pub config: Config,
}

pub type CTX = Arc<Context>;

#[derive(Debug)]
pub enum Error {
    EnvError(envy::Error),
    ConfigReadError(std::io::Error),
    ConfigParseError(serde_json::Error),
}

impl Context {
    pub fn new() -> Result<Arc<Self>, Error> {
        let env = Env::new().map_err(Error::EnvError)?;
        let config_raw = std::fs::read_to_string(&env.config).map_err(Error::ConfigReadError)?;
        let config = Config::new(&config_raw).map_err(Error::ConfigParseError)?;
        Ok(Arc::new(Self { env, config }))
    }
}
