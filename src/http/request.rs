use log::warn;
use std::net::IpAddr;

use crate::app::context::CTX;

#[derive(Debug)]
pub struct Request {
    pub ip: IpAddr,
    pub host: String,
    pub real_host: String,
    pub environment: String,
}

fn parse_host(host: &String, hostname: &String) -> Option<(String, String)> {
    let mut host_parts = host
        .split('.')
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>();
    let mut hostname_parts = hostname
        .split('.')
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>();
    while let Some(hostname_expected) = hostname_parts.pop() {
        let hostname_given = host_parts.pop()?;
        if hostname_expected != hostname_given {
            warn!(
                "Hostname part mismatch: {} != {}",
                hostname_expected, hostname_given
            );
            return None;
        }
    }
    Some((host_parts.pop()?.to_string(), host_parts.join(".")))
}

impl Request {
    pub fn new(
        req: &axum::extract::Request,
        ctx: &CTX,
        host: &String,
        ip: &IpAddr,
    ) -> Option<Self> {
        let ip = req
            .headers()
            .get(ctx.env.real_ip_header.as_ref())
            .or(req.headers().get("X-Forwarded-For"))
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| ip.clone());
        let real_host = req
            .headers()
            .get(ctx.env.real_host_header.as_ref())
            .and_then(|v| v.to_str().ok())
            .unwrap_or_else(|| host.as_str())
            .to_string();
        let (environment, host) = parse_host(&host, &ctx.env.hostname)
            .ok_or(())
            .map_err(|_| warn!("Could not parse host"))
            .ok()?;
        Some(Self {
            ip,
            host,
            real_host,
            environment,
        })
    }
}
