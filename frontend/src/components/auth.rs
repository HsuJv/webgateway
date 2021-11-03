use anyhow;
use serde_json::{json, Value};
use std::fmt::Debug;
use yew::services::{
    fetch::{FetchTask, Request},
    FetchService,
};
use yew::{format::Json, services::fetch::Response};
use yew::{prelude::*, services::ConsoleService};
pub enum AuthMsg {
    UpdateUsername(String),
    UpdatePassword(String),
    AuthRequest,
    AuthResponse(Result<Value, anyhow::Error>),
}

pub struct AuthComponents {
    username: String,
    password: String,
    link: ComponentLink<Self>,
    auth_result: String,
    fetch_task: Option<FetchTask>,
}

impl Debug for AuthComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuthComponents {{ username: {}, password: {} }}",
            self.username, self.password
        )
    }
}

impl Component for AuthComponents {
    type Message = AuthMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        AuthComponents {
            username: String::new(),
            password: String::new(),
            auth_result: String::new(),
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AuthMsg::UpdateUsername(username) => {
                self.username = username;
                self.auth_result.clear();
            }
            AuthMsg::UpdatePassword(password) => {
                self.password = password;
                self.auth_result.clear();
            }
            AuthMsg::AuthRequest => {
                let auth_info = json!({
                    "username": self.username,
                    "password": self.password,
                });

                // 1. build the request
                let request = Request::post("/auth")
                    .header("Content-Type", "application/json")
                    .body(Json(&auth_info))
                    .expect("Could not build auth request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                            // ConsoleService::error(&format!("{:?}", response));
                            let Json(data) = response.into_body();
                            AuthMsg::AuthResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
            }
            AuthMsg::AuthResponse(response) => {
                if let Ok(response) = response {
                    self.auth_result = response["status"].to_string();
                } else {
                    self.auth_result = String::from("Auth failed with unknown reason");
                    ConsoleService::error(&format!("{:?}", response.unwrap_err().to_string()));
                }
                // release resources
                self.fetch_task = None;
            }
            _ => panic!("unexpected message"),
        }
        // ConsoleService::log(&format!(
        //     "username: {}, password {}",
        //     self.username, self.password
        // ));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let link = &self.link;

        let update_uname = link.callback(|e: ChangeData| match e {
            ChangeData::Value(val) => AuthMsg::UpdateUsername(val),
            _ => panic!("unexpected message"),
        });

        let update_pword = link.callback(|e: ChangeData| match e {
            ChangeData::Value(val) => AuthMsg::UpdatePassword(val),
            _ => panic!("unexpected message"),
        });

        let auth_post = link.callback(|_| {
            // ConsoleService::log("Auth post");
            AuthMsg::AuthRequest
        });

        html! {
            <div class="horizontal-centre vertical-centre">
                <label for="username">{"Username: "}</label>
                <input id="username" type="text" placeholder="Username" onchange={update_uname} />
                <br />
                <label for="password">{"Password: "}</label>
                <input id="password" type="password" placeholder="Password" onchange={update_pword} />
                <br />
                <button type="submit" onclick={auth_post}>{"Login"}</button>
                <br />
                {self.auth_result_view()}
            </div>
        }
    }
}

impl AuthComponents {
    fn auth_result_view(&self) -> Html {
        if let Some(_) = &self.fetch_task {
            html! {
                <div>{"Authing..."}</div>
            }
        } else {
            html! {
                <div>{self.auth_result.clone()}</div>
            }
        }
    }
}
