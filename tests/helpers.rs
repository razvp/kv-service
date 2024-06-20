#![allow(dead_code)]
use std::net::TcpListener;

use actix_web::dev::Server;
use tokio::runtime::Runtime;

pub fn spawn_test_server() -> u16 {
    let (server, test_port) = build_test_server();

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(server);

    test_port
}

pub fn build_test_server() -> (Server, u16) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let local_addr = listener.local_addr().unwrap();
    let server = kv_service::http::build_server(listener).unwrap();

    (server, local_addr.port())
}

pub fn build_tokio_rt() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}
