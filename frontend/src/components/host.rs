use serde_json::{json, Value};
use yew::{
    format::Json,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
};

pub enum HostMsg {
    UpdateHost(String),
    UpdatePort(String),
    ValidateResponse(Result<Value, anyhow::Error>),
    ConnectHost,
}

pub struct Host {
    link: ComponentLink<Self>,
    host: String,
    port: u16,
    error_msg: String,
    onsubmit: Callback<(String, u16)>,
    fetch_task: Option<FetchTask>,
}

// Props
#[derive(Clone, PartialEq, Properties)]
pub struct HostProps {
    #[prop_or_default]
    pub onsubmit: Callback<(String, u16)>,
}

impl Component for Host {
    type Message = HostMsg;
    type Properties = HostProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Host {
            link,
            host: "".to_string(),
            port: 0,
            error_msg: "".to_string(),
            onsubmit: props.onsubmit,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            HostMsg::UpdateHost(host) => {
                self.host = host;
                true
            }
            HostMsg::UpdatePort(port) => match port.parse::<u16>() {
                Ok(port) => {
                    self.port = port;
                    true
                }
                Err(_) => {
                    self.error_msg = "Port must be a number".to_string();
                    true
                }
            },
            HostMsg::ValidateResponse(response) => {
                if let Ok(response) = response {
                    self.error_msg = response["status"].to_string();

                    if "\"success\"" == self.error_msg {
                        let mut ip = response["ip"].to_string();
                        let _ = ip.pop();
                        let _ = ip.remove(0);
                        self.onsubmit.emit((ip, self.port));
                    } else {
                        self.error_msg = response["message"].to_string();
                    }
                } else {
                    self.error_msg = String::from("Valid host failed with unknown reason");
                    ConsoleService::error(&format!("{:?}", response.unwrap_err().to_string()));
                }
                // release resources
                self.fetch_task = None;
                true
            }
            HostMsg::ConnectHost => {
                let to_post = json!({
                    "host": self.host,
                });

                // 1. build the request
                let request = Request::post("/target/validate")
                    .header("Content-Type", "application/json")
                    .body(Json(&to_post))
                    .expect("Could not build auth request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                            // ConsoleService::error(&format!("{:?}", response));
                            let Json(data) = response.into_body();
                            HostMsg::ValidateResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let updatehost = self.link.callback(|e: ChangeData| match e {
            ChangeData::Value(val) => HostMsg::UpdateHost(val),
            _ => panic!("unexpected message"),
        });

        let updateport = self.link.callback(|e: ChangeData| match e {
            ChangeData::Value(val) => HostMsg::UpdatePort(val),
            _ => panic!("unexpected message"),
        });

        let connecthost = self.link.callback(|_| HostMsg::ConnectHost);

        html! {
            <div class="horizontal-centre vertical-centre">
            <label for="hostname">{"Hostname: "}</label>
            <input id="hostname" type="text" placeholder="hostname" onchange={updatehost} />
            <br />
            <input id="port" type="text" placeholder="port" onchange={updateport}/>
            <br />
            <button onclick={connecthost}>{"Connect"}</button>
            <br />
            {self.error_msg.clone()}
            </div>
        }
    }
}
