use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_health_check_port")]
    pub health_check_port: u16,
}

impl Env {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}

fn default_port() -> u16 {
    80
}

fn default_health_check_port() -> u16 {
    9000
}
