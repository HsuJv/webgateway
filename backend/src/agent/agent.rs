use crate::agent::ws;
use actix::prelude::*;
use actix_codec::{Decoder, Encoder};
use actix_web::web::Bytes;
use bytes::BytesMut;
use std::collections::HashMap;
use std::io;
use tokio::net::{tcp::OwnedWriteHalf, TcpStream};
use tokio_util::codec::FramedRead;

use log::*;

struct TcpCodec;

impl Encoder<Bytes> for TcpCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Bytes, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        info!("encoding: {:?}", item);
        Ok(())
    }
}

impl Decoder for TcpCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        info!("recv from server: {:?}", src);
        if 0 == src.len() {
            return Ok(None);
        }
        let web_bytes = Bytes::from(src.to_vec());
        src.clear();
        Ok(Some(web_bytes))
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum AgentMsg {
    Ready(Addr<ws::WsSession>),
    SendToServer(Bytes),
    SendToClient(Bytes),
    Shutdown,
}

pub struct Agent {
    id: u32,
    server_info: String,
    writer: OwnedWriteHalf,
    ws_addr: Option<Addr<ws::WsSession>>,
    pending: Vec<Bytes>,
}

impl Actor for Agent {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Agent {} started", self.id);
        // ctx.address().do_send(AgentMsg::ReadReady);
    }
}

impl Handler<AgentMsg> for Agent {
    type Result = ();

    fn handle(&mut self, msg: AgentMsg, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            AgentMsg::Ready(ws_addr) => {
                self.ws_addr = Some(ws_addr);
                info!("Agent {} - Websocket connect ready", self.server_info);
                for msg in self.pending.drain(..) {
                    self.ws_addr
                        .as_ref()
                        .unwrap()
                        .do_send(ws::WsMsg::SendToClient(msg));
                }
            }
            AgentMsg::SendToServer(data) => {
                let to_send = data.to_vec();
                self.writer.try_write(&to_send).unwrap();
            }
            AgentMsg::SendToClient(data) => {
                if self.ws_addr.is_some() {
                    self.ws_addr
                        .as_ref()
                        .unwrap()
                        .do_send(ws::WsMsg::SendToClient(data));
                }
            }
            _ => panic!("unexpected message"),
        }
    }
}

impl StreamHandler<Result<Bytes, io::Error>> for Agent {
    fn handle(&mut self, msg: Result<Bytes, io::Error>, ctx: &mut Context<Self>) {
        match msg {
            Ok(data) => {
                info!("recv from server: {:?}", data);
                if self.ws_addr.is_some() {
                    ctx.address().do_send(AgentMsg::SendToClient(data));
                } else {
                    info!("Websocket session not ready");
                    self.pending.push(data);
                }
            }
            Err(err) => {
                error!("error: {:?}", err);
                ctx.address().do_send(AgentMsg::Shutdown);
            }
        }
    }
}

impl Agent {
    pub async fn new(id: u32, target: (String, u16)) -> Option<Addr<Agent>> {
        let (host, port) = target;
        let server_info = format!("{}:{}", host, port);
        info!("connect to server: {}", server_info);
        let server_stream = TcpStream::connect(&server_info).await;
        if server_stream.is_err() {
            info!("connect to server failed: {}", server_info);
        }
        let server_stream = server_stream.unwrap();
        let addr = Agent::create(move |ctx| {
            let (r, w) = server_stream.into_split();
            let r = FramedRead::new(r, TcpCodec {});
            Agent::add_stream(r, ctx);
            Self {
                id,
                server_info,
                writer: w,
                ws_addr: None,
                pending: vec![],
            }
        });
        Some(addr)
    }
}

#[derive(MessageResponse)]
pub enum AgentManagerResult {
    Success(Addr<Agent>),
    Failed,
    NoReturn,
}

#[derive(Message)]
#[rtype(result = "AgentManagerResult")]
pub enum AgentManagerMsg {
    Add((u32, Addr<Agent>)),
    Get(u32),
    Del(u32),
}

pub struct AgentManager {
    agents: HashMap<u32, Addr<Agent>>,
}

impl AgentManager {
    pub fn new() -> Addr<Self> {
        Self {
            agents: HashMap::new(),
        }
        .start()
    }
}

impl Actor for AgentManager {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("AgentManager started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("AgentManager stopped");
    }
}

impl Handler<AgentManagerMsg> for AgentManager {
    type Result = AgentManagerResult;

    fn handle(&mut self, msg: AgentManagerMsg, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            AgentManagerMsg::Add(addr) => {
                info!("add agent: {:?}", addr.0);
                self.agents.insert(addr.0, addr.1);
                AgentManagerResult::NoReturn
            }
            AgentManagerMsg::Get(aid) => {
                info!("get agent: {}", aid);
                if let Some(addr) = self.agents.get(&aid) {
                    AgentManagerResult::Success(addr.clone())
                } else {
                    AgentManagerResult::Failed
                }
            }
            AgentManagerMsg::Del(id) => {
                self.agents.remove(&id);
                AgentManagerResult::NoReturn
            }
        }
    }
}
