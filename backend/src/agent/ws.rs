use actix::{Actor, Addr, StreamHandler};
use actix_session::Session;
use actix_web::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::*;

use crate::AppData;

use super::agent::{Agent, AgentManagerMsg, AgentManagerResult};

/// Define Websocket actor
struct WsSession {
    agent: Addr<Agent>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("/ws")]
pub async fn ws_index(
    req: HttpRequest,
    session: Session,
    data: web::Data<AppData>,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let aid = session.get::<u32>("aid").unwrap_or(Some(0)).unwrap();

    let resp = match data.agents.send(AgentManagerMsg::Get(aid)).await.unwrap() {
        AgentManagerResult::Success(agent) => ws::start(WsSession { agent }, &req, stream),
        _ => Err(Error::from(actix_web::error::ErrorInternalServerError(
            "Agent not found",
        ))),
    };

    match &resp {
        Ok(resp) => info!("{:?}", resp),
        Err(e) => error!("{:?}", e),
    }
    resp
}
