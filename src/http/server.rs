use std::collections::HashMap;
use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use parking_lot::RwLock;

use super::routes;

pub struct AppState {
    pub kv_store: RwLock<HashMap<String, String>>,
}

pub fn build_server(listener: TcpListener) -> std::io::Result<Server> {
    let app_state = web::Data::new(AppState {
        kv_store: RwLock::new(HashMap::new()),
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/lookup/{key}", web::get().to(routes::lookup))
            .route("/remember", web::post().to(routes::remember))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
