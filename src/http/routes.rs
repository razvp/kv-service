use actix_web::{web, HttpResponse};
use serde::Deserialize;

use super::server::AppState;

#[derive(Debug, Deserialize)]
pub struct RememberParams {
    key: String,
}

pub async fn remember(
    key: web::Query<RememberParams>,
    value: String,
    state: web::Data<AppState>,
) -> HttpResponse {
    let key = key.0.key;
    if key.is_empty() {
        return HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("EMPTY KEY");
    }
    if value.is_empty() {
        return HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("EMPTY VALUE");
    }
    state.kv_store.write().insert(key, value);
    HttpResponse::Created().finish()
}

pub async fn lookup(key: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    let kv_store = state.kv_store.read();
    let resp = kv_store.get(key.as_str());

    match resp {
        Some(v) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(v.to_owned()),
        None => HttpResponse::NoContent().finish(),
    }
}
