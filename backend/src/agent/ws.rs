use std::sync::Arc;

use actix::{Actor, Addr, Message, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_session::Session;
use actix_web::web::Bytes;
use actix_web::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::*;

use super::agent::*;

#[derive(Message)]
#[rtype(result = "()")]
pub enum WsMsg {
    SendToClient(Bytes),
}

/// Define Websocket actor
pub struct WsSession {
    agent: Addr<Agent>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.agent.do_send(AgentMsg::Ready(ctx.address()));
        info!("Websocket connection is established.");
    }
}

impl Handler<WsMsg> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMsg, ctx: &mut Self::Context) {
        match msg {
            WsMsg::SendToClient(data) => {
                ctx.binary(data);
            }
        };
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => {
                self.agent.do_send(AgentMsg::SendToServer(bin));
            }
            _ => (),
        }
    }
}

#[get("/ws")]
pub async fn ws_index(
    req: HttpRequest,
    session: Session,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let aid = session.get::<u32>("aid").unwrap_or(Some(0)).unwrap();
    let app_data = req.app_data::<Arc<crate::AppData>>().unwrap();

    let resp = match app_data
        .agents
        .send(AgentManagerMsg::Get(aid))
        .await
        .unwrap()
    {
        AgentManagerResult::Success(agent) => ws::start(WsSession { agent }, &req, stream),
        _ => Err(actix_web::error::ErrorInternalServerError(
            "Agent not found",
        )),
    };

    match &resp {
        Ok(resp) => info!("{:?}", resp),
        Err(e) => error!("{:?}", e),
    }
    resp
}
