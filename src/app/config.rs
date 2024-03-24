use std::collections::HashMap;

use hyper::Uri;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub struct Config {
    pub environments: Vec<Upstream>,
}

impl Config {
    pub fn new(data: &String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&data)
    }
}

#[derive(Debug, Clone)]
pub struct Upstream {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

impl Upstream {
    pub fn create_uri<S: AsRef<str>>(&self, path: S) -> Uri {
        format!(
            "http{}://{}:{}{}",
            if self.tls { "s" } else { "" },
            self.host,
            self.port,
            path.as_ref()
        )
        .parse()
        .unwrap()
    }
}

impl From<(String, UpstreamRaw)> for Upstream {
    fn from((name, raw): (String, UpstreamRaw)) -> Self {
        Upstream {
            name,
            host: raw.host,
            port: raw.port,
            tls: raw.tls,
        }
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let e = ConfigRaw::deserialize(deserializer)?;
        Ok(Self {
            environments: e.environments.into_iter().map(Upstream::from).collect(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigRaw {
    pub environments: HashMap<String, UpstreamRaw>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpstreamRaw {
    pub host: String,
    #[serde(default = "UpstreamRaw::default_port")]
    pub port: u16,
    #[serde(default = "UpstreamRaw::default_tls")]
    pub tls: bool,
}

impl UpstreamRaw {
    pub fn default_port() -> u16 {
        80
    }

    pub fn default_tls() -> bool {
        false
    }
}
