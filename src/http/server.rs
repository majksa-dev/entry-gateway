use axum::{
    body::Body,
    extract::{Host, Request, State},
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use axum_client_ip::{SecureClientIp, SecureClientIpSource};
use axum_server::tls_rustls::RustlsConfig;
use hyper::{header::HOST, StatusCode};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use log::{info, warn};
use std::{io, net::SocketAddr};

use crate::app::context::{self, Context, CTX};

use super::request::Request as RequestData;

#[derive(Debug)]
pub enum ServerError {
    Io(io::Error),
    Context(context::Error),
}

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub async fn create_server() -> Result<(), ServerError> {
    let ctx = Context::new().map_err(ServerError::Context)?;
    let tls = RustlsConfig::from_pem_file(&ctx.env.ssl_cert, &ctx.env.ssl_key)
        .await
        .unwrap();
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());
    let addr = SocketAddr::from(([127, 0, 0, 1], ctx.env.port));
    let app = Router::new()
        .route("/", any(handler))
        .with_state((ctx, client))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());
    info!("Listening on {}", addr);
    axum_server::bind_rustls(addr, tls)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .map_err(ServerError::Io)
}

async fn handler(
    SecureClientIp(ip): SecureClientIp,
    Host(host): Host,
    State((ctx, client)): State<(CTX, Client)>,
    req: Request,
) -> Result<Response, StatusCode> {
    let request = RequestData::new(&req, &ctx, &host, &ip).ok_or(StatusCode::BAD_REQUEST)?;
    let upstream = ctx
        .config
        .environments
        .iter()
        .find(|u| u.name == request.environment)
        .ok_or(StatusCode::NOT_FOUND)?;
    let mut headers = req.headers().clone();
    let mut upstream_req = hyper::Request::builder()
        .method(req.method())
        .uri(
            upstream.create_uri(
                req.uri()
                    .path_and_query()
                    .map(|v| v.as_str())
                    .unwrap_or_else(|| req.uri().path()),
            ),
        )
        .body(req.into_body())
        .unwrap();
    headers.insert(HOST, request.host.parse().unwrap());
    headers.insert(
        ctx.env.real_ip_header.as_ref(),
        request.ip.to_string().parse().unwrap(),
    );
    headers.insert(
        ctx.env.real_host_header.as_ref(),
        request.real_host.parse().unwrap(),
    );
    headers.insert(
        ctx.env.environment_header.as_ref(),
        request.environment.parse().unwrap(),
    );
    *upstream_req.headers_mut() = headers;

    Ok(client
        .request(upstream_req)
        .await
        .map_err(|e| {
            warn!("Failed to connect to upstream: {}", e);
            StatusCode::BAD_GATEWAY
        })?
        .into_response())
}
