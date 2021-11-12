use serde_json::{json, Value};
use yew::{
    format::Json,
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
};

use crate::{
    components::{self, input::Input, ws::WebsocketMsg},
    protocal::{common::*, vnc::VncHandler},
    utils::WeakComponentLink,
};

pub struct PageRemote {
    link: ComponentLink<Self>,
    target: (String, u16),
    error_msg: String,
    fetch_task: Option<FetchTask>,
    connected: bool,
    handler: ProtocalHandler<VncHandler>,
    ws_link: WeakComponentLink<components::ws::WebsocketCtx>,
    request_username: bool,
    request_password: bool,
    username: String,
    password: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RemoteProps {}

pub enum RemoteMsg {
    Connect((String, u16)),
    ConnectResp(Result<Value, anyhow::Error>),
    Connected,
    Recv(Vec<u8>),
    Send(Vec<u8>),
    UpdateUsername(String),
    UpdatePassword(String),
    SendCredential,
}

impl Component for PageRemote {
    type Message = RemoteMsg;
    type Properties = RemoteProps;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        PageRemote {
            link,
            target: (String::from(""), 0),
            error_msg: String::from(""),
            fetch_task: None,
            connected: false,
            handler: ProtocalHandler::new(),
            ws_link: WeakComponentLink::default(),
            request_username: false,
            request_password: false,
            username: String::from(""),
            password: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RemoteMsg::Connect(target) => {
                self.target = target;
                // ConsoleService::log(&self.target);
                let to_post = json!({
                    "ip": self.target.0,
                    "port": self.target.1,
                });

                // 1. build the request
                let request = Request::post("/target/remote")
                    .header("Content-Type", "application/json")
                    .body(Json(&to_post))
                    .expect("Could not build auth request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                            // ConsoleService::error(&format!("{:?}", response));
                            let Json(data) = response.into_body();
                            RemoteMsg::ConnectResp(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                true
            }
            RemoteMsg::ConnectResp(response) => {
                if let Ok(response) = response {
                    self.error_msg = response["status"].to_string();

                    if "\"success\"" == self.error_msg {
                        self.link.send_message(RemoteMsg::Connected);
                    } else {
                        self.error_msg = response["message"].to_string();
                    }
                } else {
                    self.error_msg = String::from("Connect host failed with unknown reason");
                    ConsoleService::error(&format!("{:?}", response.unwrap_err().to_string()));
                }
                // release resources
                self.fetch_task = None;
                true
            }
            RemoteMsg::Connected => {
                self.connected = true;
                true
            }
            RemoteMsg::Recv(v) => {
                let out = self.handler.handle(&v);
                self.protocal_out_handler(out)
            }
            RemoteMsg::Send(v) => {
                self.ws_link
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send_message(WebsocketMsg::Send(Ok(v)));
                false
            }
            RemoteMsg::UpdateUsername(username) => {
                self.username = username;
                true
            }
            RemoteMsg::UpdatePassword(password) => {
                self.password = password;
                true
            }
            RemoteMsg::SendCredential => {
                self.request_username = false;
                self.request_password = false;
                let out = self.handler.set_credential(&self.username, &self.password);
                let _ = self.protocal_out_handler(out);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if !self.connected {
            let connect_remote = self.link.callback(RemoteMsg::Connect);
            html! {
                <>
                    <components::host::Host onsubmit=connect_remote/>
                    {self.error_msg.clone()}
                </>
            }
        } else {
            let recv_msg = self.link.callback(|v| RemoteMsg::Recv(v));
            let ws_link = &self.ws_link;
            html! {
                <>
                    <div class="horizontal-centre vertical-centre">
                        {self.username_view()}
                        {self.password_view()}
                        {self.button_connect_view()}
                        <components::ws::WebsocketCtx
                        weak_link=ws_link onrecv=recv_msg/>
                        {self.error_msg.clone()}
                    </div>
                </>
            }
        }
    }
}

// impl PageRemote
impl PageRemote {
    fn protocal_out_handler(&mut self, out: ProtocalHandlerOutput) -> ShouldRender {
        match out {
            ProtocalHandlerOutput::Err(err) => {
                self.error_msg = err.clone();
                self.ws_link
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .send_message(WebsocketMsg::Disconnected);
                true
            }
            ProtocalHandlerOutput::Ok => false,
            ProtocalHandlerOutput::WsBuf(out) => {
                self.link.send_message(RemoteMsg::Send(out));
                false
            }
            ProtocalHandlerOutput::RequirePassword => {
                self.request_password = true;
                true
            }
            _ => unimplemented!(),
        }
    }

    fn username_view(&self) -> Html {
        if self.request_username {
            let update_username = self.link.callback(|v| RemoteMsg::UpdateUsername(v));
            html! {
                <>
                    <Input id="username" type_="text" placeholder="username" on_change={update_username}/>
                    <br/>
                </>
            }
        } else {
            html! {}
        }
    }

    fn password_view(&self) -> Html {
        if self.request_password {
            let update_password = self.link.callback(|v| RemoteMsg::UpdatePassword(v));
            html! {
                <>
                    <Input id="password" type_="password" placeholder="password" on_change={update_password}/>
                    <br/>
                </>
            }
        } else {
            html! {}
        }
    }

    fn button_connect_view(&self) -> Html {
        if self.request_username || self.request_password {
            let send_credential = self.link.callback(|_| RemoteMsg::SendCredential);
            html! {
                <>
                    <button type="submit" onclick={send_credential}>{"Connect"}</button>
                    <br/>
                </>
            }
        } else {
            html! {}
        }
    }
}
