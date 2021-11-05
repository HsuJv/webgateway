use actix::{Actor, Context, Handler, Message, MessageResponse};
use actix_session::Session;
use actix_web::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use log::info;

#[derive(MessageResponse)]
#[allow(dead_code)]
enum AuthResp {
    AuthSuccess,
    AuthFailure,
}

#[derive(Message)]
#[rtype(result = "AuthResp")]
enum AuthMsg {
    DoAuth,
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

impl Handler<AuthMsg> for AuthInfo {
    type Result = AuthResp;

    fn handle(&mut self, _msg: AuthMsg, _ctx: &mut Context<Self>) -> Self::Result {
        info!("AuthInfo handle");
        AuthResp::AuthSuccess
    }
}

#[post("/auth")]
pub async fn auth(params: web::Json<AuthInfo>) -> Result<HttpResponse, Error> {
    let auth = params.into_inner();
    let auth_addr = auth.start();
    let res = auth_addr.send(AuthMsg::DoAuth).await;

    match res {
        Ok(AuthResp::AuthSuccess) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
        }))),
        _ => Ok(HttpResponse::Ok().json(json!({
            "status": "failure",
        }))),
    }
}
