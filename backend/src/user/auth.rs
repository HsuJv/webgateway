use std::sync::Arc;

use actix::{Actor, Addr, Context, Handler, Message, MessageResponse};
use actix_web::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use log::info;

#[derive(MessageResponse)]
#[allow(dead_code)]
enum AuthResult {
    AuthSuccess,
    AuthFailure,
}

#[derive(Message)]
#[rtype(result = "AuthResult")]
enum AuthMsg {
    DoAuth(AuthInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthInfo {
    username: String,
    password: String,
}

pub struct Authenticator;

impl Authenticator {
    pub fn new() -> Addr<Self> {
        Self {}.start()
    }
}

impl Actor for Authenticator {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("AuthInfo started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("AuthInfo stopped");
    }
}

impl Handler<AuthMsg> for Authenticator {
    type Result = AuthResult;

    fn handle(&mut self, msg: AuthMsg, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            AuthMsg::DoAuth(_auth_info) => {
                // if auth_info.username == "admin" && auth_info.password == "admin" {
                //     AuthResult::AuthSuccess
                // } else {
                //     AuthResult::AuthFailure
                // }
                AuthResult::AuthSuccess
            }
        }
    }
}

#[post("/auth")]
pub async fn auth(params: web::Json<AuthInfo>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let auth_info = params.into_inner();
    let app_data = req.app_data::<Arc<crate::AppData>>().unwrap();
    let res = app_data
        .authenticator
        .send(AuthMsg::DoAuth(auth_info))
        .await;

    match res {
        Ok(AuthResult::AuthSuccess) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
        }))),
        _ => Ok(HttpResponse::Ok().json(json!({
            "status": "failure",
        }))),
    }
}
