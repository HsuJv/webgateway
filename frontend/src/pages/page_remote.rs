use serde_json::{json, Value};
use wasm_bindgen::{prelude::Closure, Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use yew::{
    format::Json,
    html,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
};

use gloo::timers::callback::Interval;

use crate::{
    components::{self, input::Input, ws::WebsocketMsg},
    protocal::{common::*, vnc::vnc::VncHandler},
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
    canvas: NodeRef,
    canvas_ctx: Option<CanvasRenderingContext2d>,
    interval: Option<Interval>,
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
    RequireFrame(u8),
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
            canvas: NodeRef::default(),
            canvas_ctx: None,
            interval: None,
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
                self.handler.do_input(v);
                self.protocal_out_handler()
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
                self.handler.set_credential(&self.username, &self.password);
                self.protocal_out_handler()
            }
            RemoteMsg::RequireFrame(incremental) => {
                self.handler.require_frame(incremental);
                if self.interval.is_none() {
                    let link = self.link.clone();
                    let tick =
                        Interval::new(20, move || link.send_message(RemoteMsg::RequireFrame(1)));
                    self.interval = Some(tick);
                }
                self.protocal_out_handler()
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
            let recv_msg = self.link.callback(RemoteMsg::Recv);
            let ws_link = &self.ws_link;
            html! {
                <>
                    <div class="horizontal-centre vertical-centre">
                        {self.username_view()}
                        {self.password_view()}
                        {self.button_connect_view()}
                        <components::ws::WebsocketCtx
                        weak_link=ws_link onrecv=recv_msg/>
                        <canvas id="remote-canvas" ref=self.canvas.clone()></canvas>
                        {self.error_msg.clone()}
                    </div>
                </>
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.handler.set_resolution(1366, 768);
        }
    }
}

// impl PageRemote
impl PageRemote {
    fn protocal_out_handler(&mut self) -> ShouldRender {
        let out = self.handler.get_output();
        let mut should_render = false;
        if !out.is_empty() {
            for o in out {
                match o {
                    ProtocalHandlerOutput::Err(err) => {
                        self.error_msg = err.clone();
                        self.ws_link
                            .borrow_mut()
                            .as_mut()
                            .unwrap()
                            .send_message(WebsocketMsg::Disconnected);
                        should_render = true;
                    }
                    ProtocalHandlerOutput::WsBuf(out) => {
                        self.link.send_message(RemoteMsg::Send(out));
                    }
                    ProtocalHandlerOutput::RequirePassword => {
                        self.request_password = true;
                        should_render = true;
                    }
                    ProtocalHandlerOutput::RenderCanvas(cr) => {
                        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
                        let ctx = match &self.canvas_ctx {
                            Some(ctx) => ctx,
                            None => {
                                let ctx = CanvasRenderingContext2d::from(JsValue::from(
                                    canvas.get_context("2d").unwrap().unwrap(),
                                ));
                                self.canvas_ctx = Some(ctx);
                                self.canvas_ctx.as_ref().unwrap()
                            }
                        };

                        let data = ImageData::new_with_u8_clamped_array_and_sh(
                            Clamped(&cr.data),
                            cr.width as u32,
                            cr.height as u32,
                        )
                        .unwrap();
                        // ConsoleService::log(&format!(
                        //     "renderring at ({}, {}), width {}, height {}",
                        //     cr.x, cr.y, cr.width, cr.height
                        // ));
                        let _ = ctx.put_image_data(&data, cr.x as f64, cr.y as f64);
                        should_render = true;
                    }
                    ProtocalHandlerOutput::SetCanvas(width, height) => {
                        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
                        canvas.set_width(width as u32);
                        canvas.set_height(height as u32);
                        self.bind_mouse_and_key(&canvas);
                        self.link.send_message(RemoteMsg::RequireFrame(0));
                        let ctx = match &self.canvas_ctx {
                            Some(ctx) => ctx,
                            None => {
                                let ctx = CanvasRenderingContext2d::from(JsValue::from(
                                    canvas.get_context("2d").unwrap().unwrap(),
                                ));
                                self.canvas_ctx = Some(ctx);
                                self.canvas_ctx.as_ref().unwrap()
                            }
                        };
                        ctx.rect(0 as f64, 0 as f64, width as f64, height as f64);
                        ctx.fill();
                        should_render = true;
                    }
                    ProtocalHandlerOutput::SetClipboard(text) => {
                        self.error_msg = format!("Clipboard get {}", text);
                        ConsoleService::log(&self.error_msg);
                        should_render = true;
                    }
                    _ => unimplemented!(),
                }
            }
        }
        should_render
    }

    fn username_view(&self) -> Html {
        if self.request_username {
            let update_username = self.link.callback(RemoteMsg::UpdateUsername);
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
            let update_password = self.link.callback(RemoteMsg::UpdatePassword);
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

    fn bind_mouse_and_key(&mut self, canvas: &HtmlCanvasElement) {
        let window = web_sys::window().unwrap();
        let handler = self.handler.clone();
        let key_down = move |e: KeyboardEvent| {
            e.stop_propagation();
            handler.key_press(e, true);
        };

        let handler = Box::new(key_down) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        window
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let handler = self.handler.clone();
        let key_up = move |e: KeyboardEvent| {
            e.stop_propagation();
            handler.key_press(e, false);
        };

        let handler = Box::new(key_up) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        window
            .add_event_listener_with_callback("keyup", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        // On a conventional mouse, buttons 1, 2, and 3 correspond to the left,
        // middle, and right buttons on the mouse.  On a wheel mouse, each step
        // of the wheel upwards is represented by a press and release of button
        // 4, and each step downwards is represented by a press and release of
        // button 5.

        // to do:
        // calculate relation position
        let handler = self.handler.clone();
        let mouse_move = move |e: MouseEvent| {
            e.stop_propagation();
            handler.mouse_event(e, MouseEventType::MouseMove);
        };

        let handler = Box::new(mouse_move) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        canvas
            .add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let handler = self.handler.clone();
        let mouse_down = move |e: MouseEvent| {
            e.stop_propagation();
            handler.mouse_event(e, MouseEventType::MouseDown);
        };

        let handler = Box::new(mouse_down) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        canvas
            .add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let handler = self.handler.clone();
        let mouse_up = move |e: MouseEvent| {
            e.stop_propagation();
            handler.mouse_event(e, MouseEventType::MouseUp);
        };

        let handler = Box::new(mouse_up) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        canvas
            .add_event_listener_with_callback("mouseup", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();

        let get_context_menu = move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
        };

        let handler = Box::new(get_context_menu) as Box<dyn FnMut(_)>;

        let cb = Closure::wrap(handler);

        canvas
            .add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }
}
