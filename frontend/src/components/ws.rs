use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::ConsoleService;
use yew::{format::Binary, utils::host};

pub struct WebsocketCtx {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    error_msg: String,
    onrecv: Callback<Vec<u8>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct WebsocketProps {
    #[prop_or_default]
    pub onrecv: Callback<Vec<u8>>,
}

pub enum WebsocketMsg {
    Connect,
    Disconnected,
    Ignore,
    Send(Binary),
    Recv(Binary),
}

impl Component for WebsocketCtx {
    type Message = WebsocketMsg;
    type Properties = WebsocketProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws: None,
            link: link,
            error_msg: String::new(),
            onrecv: props.onrecv,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            WebsocketMsg::Connect => {
                ConsoleService::log("Connecting");
                let cbout = self.link.callback(|data| WebsocketMsg::Recv(data));
                let cbnot = self.link.callback(|input| {
                    ConsoleService::log(&format!("Notification: {:?}", input));
                    match input {
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            WebsocketMsg::Disconnected
                        }
                        _ => WebsocketMsg::Ignore,
                    }
                });
                if self.ws.is_none() {
                    let task = WebSocketService::connect_binary(
                        &format!("ws://{}/ws", host().unwrap()),
                        cbout,
                        cbnot,
                    );
                    self.ws = Some(task.unwrap());
                }
                true
            }
            WebsocketMsg::Disconnected => {
                self.ws = None;
                self.error_msg = "Disconnected".to_string();
                true
            }
            WebsocketMsg::Ignore => false,
            WebsocketMsg::Send(data) => {
                if let Some(ref mut ws) = self.ws {
                    ws.send_binary(data);
                }
                false
            }
            WebsocketMsg::Recv(Ok(s)) => {
                // ConsoleService::log(&format!("recv {:?}", s));
                self.onrecv.emit(s);
                false
            }
            WebsocketMsg::Recv(Err(s)) => {
                self.error_msg = format!("Error when reading from server: {}\n", &s.to_string());
                self.link.send_message(WebsocketMsg::Disconnected);
                true
            }
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            {self.error_msg.clone()}
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.ws.is_none() {
            self.link.send_message(WebsocketMsg::Connect);
        }
    }
}
