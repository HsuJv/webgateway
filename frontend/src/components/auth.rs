use super::input::Input;
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
    onauthdone: Callback<()>,
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

#[derive(Clone, PartialEq, Properties)]
pub struct AuthProps {
    #[prop_or_default]
    pub onauthdone: Callback<()>,
}

impl Component for AuthComponents {
    type Message = AuthMsg;
    type Properties = AuthProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        AuthComponents {
            username: String::new(),
            password: String::new(),
            auth_result: String::new(),
            link,
            fetch_task: None,
            onauthdone: props.onauthdone,
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
                    if "\"success\"" == self.auth_result {
                        self.onauthdone.emit(());
                    }
                } else {
                    self.auth_result = String::from("Auth failed with unknown reason");
                    ConsoleService::error(&format!("{:?}", response.unwrap_err().to_string()));
                }
                // release resources
                self.fetch_task = None;
            }
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

        let update_uname = link.callback(AuthMsg::UpdateUsername);

        let update_pword = link.callback(AuthMsg::UpdatePassword);

        let auth_post = link.callback(|_| AuthMsg::AuthRequest);

        html! {
            <div class="horizontal-centre vertical-centre">
                <label for="username">{"Username: "}</label>
                <Input id="username" type_="text" placeholder="Username" on_change={update_uname} />
                <br />
                <label for="password">{"Password: "}</label>
                <Input id="password" type_="password" placeholder="Password" on_change={update_pword} />
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
        if self.fetch_task.is_some() {
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
