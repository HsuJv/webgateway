use std::sync::Arc;

use actix_session::Session;
use actix_web::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use log::info;
use rand::Rng;

use super::agent;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteInfo {
    #[serde(default)]
    host: String,
    #[serde(default)]
    ip: String,
    #[serde(default)]
    port: u16,
}

#[post("/target/validate")]
pub async fn target_validate(
    req: HttpRequest,
    params: web::Json<RemoteInfo>,
) -> Result<HttpResponse, Error> {
    let remote = params.into_inner();
    info!("{:?}", remote);
    let app_data = req.app_data::<Arc<crate::AppData>>().unwrap();

    match app_data.resolver.lockup(remote.host).await {
        Some(ipaddr) => {
            let json = json!({
                "status": "success",
                "ip": ipaddr
            });
            Ok(HttpResponse::Ok().json(json))
        }
        _ => {
            let json = json!({
                "status": "failed",
                "message": "Failed to resolve the target name"
            });
            Ok(HttpResponse::Ok().json(json))
        }
    }
}

#[post("/target/ssh")]
pub async fn target_ssh(
    req: HttpRequest,
    session: Session,
    params: web::Json<RemoteInfo>,
) -> Result<HttpResponse, Error> {
    let aid = rand::thread_rng().gen::<u32>();
    let app_data = req.app_data::<Arc<crate::AppData>>().unwrap();
    let remote = params.into_inner();
    let agent = agent::Agent::new(aid, (remote.ip, remote.port)).await;

    match agent {
        Some(addr) => {
            // add to agent list
            let _ = app_data
                .agents
                .send(agent::AgentManagerMsg::Add((aid, addr)))
                .await;

            // add session, so that the websocket can send message to the agent
            let _ = session.insert("aid", aid);

            // send response
            let json = json!({
                "status": "success",
            });
            Ok(HttpResponse::Ok().json(json))
        }
        _ => {
            let json = json!({
                "status": "failed",
                "message": "Failed to connect to the target"
            });
            Ok(HttpResponse::Ok().json(json))
        }
    }
}
