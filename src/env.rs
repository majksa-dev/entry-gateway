use std::path::PathBuf;

use axum::http::HeaderName;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug, Clone)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_health_check_port")]
    pub health_check_port: u16,
    pub hostname: String,
    #[serde(default = "default_config")]
    pub config: String,
    #[serde(default = "default_real_ip_header")]
    pub real_ip_header: Header,
    #[serde(default = "default_real_host_header")]
    pub real_host_header: Header,
    #[serde(default = "default_environment_header")]
    pub environment_header: Header,
    #[serde(default = "default_ssl_cert")]
    pub ssl_cert: PathBuf,
    #[serde(default = "default_ssl_key")]
    pub ssl_key: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Header(HeaderName);

impl Header {
    pub fn as_ref(&self) -> &HeaderName {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let e = String::deserialize(deserializer)?;
        Ok(Self(e.parse().map_err(serde::de::Error::custom)?))
    }
}

impl Env {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}

fn default_port() -> u16 {
    443
}

fn default_health_check_port() -> u16 {
    9000
}

fn default_config() -> String {
    "/etc/entry-gateway/config.json".to_string()
}

fn default_real_ip_header() -> Header {
    Header("X-Real-IP".parse().unwrap())
}

fn default_real_host_header() -> Header {
    Header("X-Real-Host".parse().unwrap())
}

fn default_environment_header() -> Header {
    Header("X-Environment".parse().unwrap())
}

fn default_ssl_cert() -> PathBuf {
    PathBuf::from("/certs/cert.pem")
}

fn default_ssl_key() -> PathBuf {
    PathBuf::from("/certs/key.pem")
}
