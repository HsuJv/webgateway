use actix::{Actor, Context, Handler, Message, MessageResponse};
use actix_web::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use log::info;

#[derive(Message)]
#[rtype(result = "Self")]
#[derive(MessageResponse)]
#[allow(dead_code)]
enum AuthMessage {
    DoAuth,
    AuthSuccess,
    AuthFailure,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthInfo {
    username: String,
    password: String,
}

impl Actor for AuthInfo {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("AuthInfo started");
        info!("{:?}", self);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("AuthInfo stopped");
    }
}

impl Handler<AuthMessage> for AuthInfo {
    type Result = AuthMessage;

    fn handle(&mut self, _msg: AuthMessage, _ctx: &mut Context<Self>) -> Self::Result {
        info!("AuthInfo handle");
        AuthMessage::AuthSuccess
    }
}

#[post("/auth")]
pub async fn auth(params: web::Json<AuthInfo>) -> Result<HttpResponse, Error> {
    let auth = params.into_inner();
    let auth_addr = auth.start();
    let res = auth_addr.send(AuthMessage::DoAuth).await;

    match res {
        Ok(AuthMessage::AuthSuccess) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
        }))),
        Ok(AuthMessage::AuthFailure) => Ok(HttpResponse::Ok().json(json!({
            "status": "failure",
        }))),
        _ => Ok(HttpResponse::Ok().json(json!({
            "status": "failure",
        }))),
    }
}
