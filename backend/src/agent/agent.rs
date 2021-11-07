use actix::prelude::*;
use actix_codec::{Decoder, Encoder};
use actix_web::web::Bytes;
use bytes::BytesMut;
use futures::stream::*;
use std::io;
use std::{collections::HashMap, task::Poll};
use tokio::net::{tcp::OwnedWriteHalf, TcpStream};
use tokio::runtime::{self, Runtime};
use tokio_util::codec::FramedRead;

use log::*;

struct TcpCodec;

impl Encoder<Bytes> for TcpCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        info!("encoding: {:?}", item);
        Ok(())
    }
}

impl Decoder for TcpCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        info!("recv from server: {:?}", src);
        Ok(Some(Bytes::from(src.to_vec())))
    }
}

#[derive(MessageResponse)]
pub enum AgentResult {
    Success,
    Failed,
}

#[derive(Message)]
#[rtype(result = "AgentResult")]
pub enum AgentMsg {
    ReadReady,
    SendToServer(String),
    SendToClient(Bytes),
}

pub struct Agent {
    id: u32,
    server_info: String,
    writer: OwnedWriteHalf,
    runtime: Runtime,
}

impl Actor for Agent {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Agent {} started", self.id);
        // ctx.address().do_send(AgentMsg::ReadReady);
    }
}

impl Handler<AgentMsg> for Agent {
    type Result = AgentResult;

    fn handle(&mut self, msg: AgentMsg, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            AgentMsg::SendToServer(data) => {
                self.writer.try_write(data.as_bytes()).unwrap();
                AgentResult::Success
            }
            AgentMsg::SendToClient(_data) => panic!("unexpected message"),
            _ => panic!("unexpected message"),
        }
    }
}

impl StreamHandler<Bytes> for Agent {
    fn handle(&mut self, msg: Bytes, ctx: &mut Context<Self>) {
        info!("recv from server: {:?}", msg);
        ctx.address().do_send(AgentMsg::SendToClient(msg));
    }
}

impl Agent {
    pub async fn new(id: u32, target: (String, u16)) -> Option<Addr<Agent>> {
        let (host, port) = target;
        let server_info = format!("{}:{}", host, port);
        info!("connect to server: {}", server_info);
        let addr = Agent::create(move |ctx| {
            let mut builder = runtime::Builder::new_current_thread();
            builder.enable_all();

            let runtime = builder.build().unwrap();

            let server_stream = runtime.block_on(TcpStream::connect(&server_info));

            if server_stream.is_err() {
                info!("connect to server failed: {}", server_info);
            }
            let server_stream = server_stream.unwrap();
            let (r, w) = server_stream.into_split();
            // let r = FramedRead::new(r, TcpCodec {});
            let xx = poll_fn(move |_a| {
                let mut buf = [0; 16384];
                match r.try_read(&mut buf) {
                    Ok(n) => {
                        if n == 0 {
                            return Poll::Pending;
                        }
                        return Poll::Ready(Some(Bytes::from(buf[..n].to_vec())));
                    }
                    Err(e) => {
                        error!("error: {}", e);
                        if e.kind() == io::ErrorKind::WouldBlock {
                            return Poll::Pending;
                        }
                        return Poll::Ready(None);
                    }
                };
            });
            Agent::add_stream(xx, ctx);
            Self {
                id,
                server_info,
                writer: w,
                runtime,
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
                self.agents.insert(addr.0, addr.1);
                AgentManagerResult::NoReturn
            }
            AgentManagerMsg::Get(aid) => {
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
