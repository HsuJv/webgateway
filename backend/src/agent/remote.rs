use actix_session::Session;
use actix_web::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use log::info;
use rand::Rng;

use crate::agent::resolver::*;
use crate::AppData;

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
    data: web::Data<AppData>,
    params: web::Json<RemoteInfo>,
) -> Result<HttpResponse, Error> {
    let remote = params.into_inner();
    info!("{:?}", remote);
    let resolved = data.resolver.send(ResolveMsg::Resolve(remote.host)).await;

    match resolved.unwrap() {
        ResolveResp::Success(ipaddr) => {
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
    session: Session,
    data: web::Data<AppData>,
    params: web::Json<RemoteInfo>,
) -> Result<HttpResponse, Error> {
    let aid = rand::thread_rng().gen::<u32>();
    let remote = params.into_inner();
    let agent = agent::Agent::new(aid);

    match agent
        .send(agent::AgentMsg::ConnectServer(
            format!("{}:{}", remote.ip, remote.port).parse().unwrap(),
        ))
        .await
    {
        Ok(agent::AgentResp::Success) => {
            // add to agent list
            data.agents.write().unwrap().insert(aid, agent);

            // add session, so that the websocket can send message to the agent
            let _ = session.set::<u32>("aid", aid);

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
