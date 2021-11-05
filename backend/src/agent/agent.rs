use actix::{Actor, Addr, Context, Handler, Message, MessageResponse};
use actix_web::web::Bytes;
use std::net::*;

use log::info;
#[derive(MessageResponse)]
pub enum AgentResp {
    Success,
    Failed,
}

#[derive(Message)]
#[rtype(result = "AgentResp")]
pub enum AgentMsg {
    ConnectServer(SocketAddr),
    SendToServer(Bytes),
    SendToClient(Bytes),
}

pub struct Agent {
    id: u32,
    server_info: Option<SocketAddr>,
    server_stream: Option<TcpStream>,
    // client_info: SocketAddr,
}

impl Actor for Agent {
    type Context = Context<Self>;
}

impl Handler<AgentMsg> for Agent {
    type Result = AgentResp;

    fn handle(&mut self, msg: AgentMsg, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            AgentMsg::ConnectServer(addr) => {
                info!("connect to server: {}", addr);
                self.server_info = Some(addr);
                if let Ok(stream) = TcpStream::connect(addr) {
                    stream
                        .set_nonblocking(true)
                        .expect("set_nonblocking call failed");
                    self.server_stream = Some(stream);
                    AgentResp::Success
                } else {
                    AgentResp::Failed
                }
            }
            AgentMsg::SendToServer(_data) => AgentResp::Success,
            AgentMsg::SendToClient(_data) => AgentResp::Success,
        }
    }
}

impl Agent {
    pub fn new(id: u32) -> Addr<Agent> {
        Self {
            id,
            server_info: None,
            server_stream: None,
        }
        .start()
    }
}
