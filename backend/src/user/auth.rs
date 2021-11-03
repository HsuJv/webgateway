use actix_web::*;
use serde_json::json;

use log::info;

#[post("/auth")]
pub async fn auth(req: HttpRequest) -> Result<HttpResponse, Error> {
    info!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json!({"status": "success"})))
}
