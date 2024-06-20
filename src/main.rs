use std::net::TcpListener;

use anyhow::anyhow;

use kv_service::http::build_server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let addr_and_port = args
        .nth(1)
        .ok_or(anyhow!("Provide IP:PORT, ex: `kv-service 127.0.0.1:8080`"))?;

    let listener = TcpListener::bind(addr_and_port)?;

    build_server(listener)?.await?;
    Ok(())
}
