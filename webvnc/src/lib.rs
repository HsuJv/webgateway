mod canvas;
mod utils;
mod vnc;

use canvas::CanvasUtils;
use vnc::Vnc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, HtmlButtonElement, MessageEvent, WebSocket};

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    // fn setTimeout(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(token: f64);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn prompt(s: &str) -> String;
    pub fn setClipBoard(s: String);
    pub fn getClipBoard() -> String;
}

static mut REFRESHER: Option<Interval> = None;

#[wasm_bindgen]
pub struct Interval {
    _closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut(),
    {
        // Construct a new closure.
        let closure = Closure::new(f);
        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval {
            _closure: closure,
            token,
        }
    }
}

// When the Interval is destroyed, cancel its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        console_log!("interval dropped");
        clearInterval(self.token);
    }
}

fn vnc_out_handler(ws: &WebSocket, vnc: &Vnc, canvas: &CanvasUtils) {
    let out = vnc.get_output();
    if !out.is_empty() {
        for ref o in out {
            match o {
                vnc::VncOutput::Err(err) => {
                    console_log!("Err {}", err);
                    vnc_close_handle(vnc, canvas, err);
                }
                vnc::VncOutput::WsBuf(buf) => match ws.send_with_u8_array(buf) {
                    Ok(_) => {}
                    Err(err) => {
                        let err = format!("error sending message: {:?}", err);
                        vnc_close_handle(vnc, canvas, &err);
                    }
                },
                vnc::VncOutput::RequirePassword => {
                    let pwd = prompt("Please input the password");
                    vnc.set_credential(&pwd);
                    vnc_out_handler(ws, vnc, canvas);
                }
                vnc::VncOutput::RenderImage(ri) => {
                    canvas.draw(ri);
                }
                vnc::VncOutput::SetResolution(x, y) => {
                    canvas.init(*x as u32, *y as u32);
                    canvas.bind(vnc);
                    vnc.require_frame(0);
                    vnc_out_handler(ws, vnc, canvas);

                    let vnc_cloned = vnc.clone();
                    let ws_cloned = ws.clone();
                    let canvas_cloned = canvas.clone();

                    // set a interval for fps enhance
                    let refresh = move || {
                        vnc_cloned.require_frame(1);
                        vnc_out_handler(&ws_cloned, &vnc_cloned, &canvas_cloned);
                    };

                    let refersher = Interval::new(20, refresh);

                    unsafe {
                        REFRESHER = Some(refersher);
                    }
                }
                vnc::VncOutput::SetClipboard(text) => {
                    setClipBoard(text.to_owned());
                    // ConsoleService::log(&self.error_msg);
                }
            }
        }
    }
}

fn vnc_close_handle(vnc: &Vnc, canvas: &CanvasUtils, msg: &str) {
    console_log!("Websocket disconnect with message {}", msg);
    vnc.close();
    unsafe {
        REFRESHER.take();
    }
    canvas.close();
    let status = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("vnc_status")
        .unwrap();
    status.set_text_content(Some(msg));
}

fn start_websocket() -> Result<(), JsValue> {
    // connect
    let url = format!(
        "{scheme}://{host}/websockify",
        scheme = if web_sys::window()
            .unwrap()
            .location()
            .protocol()?
            .starts_with("https")
        {
            "wss"
        } else {
            "ws"
        },
        host = web_sys::window().unwrap().location().host()?
    );
    let ws = WebSocket::new_with_str(&url, "binary")?;
    let canvas = CanvasUtils::new();
    let vnc = Vnc::new();

    let clipboard = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("clipboardsend")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap();
    let vnc_cloned = vnc.clone();
    let onclickcb = Closure::<dyn FnMut()>::new(move || {
        console_log!("Send {:?}", getClipBoard());
        vnc_cloned.set_clipboard(&getClipBoard());
    });
    clipboard.set_onclick(Some(onclickcb.as_ref().unchecked_ref()));
    onclickcb.forget();

    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // on message
    let cloned_ws = ws.clone();
    let vnc_cloned = vnc.clone();
    let canvas_cloned = canvas.clone();

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            // let mut canvas_ctx = None;
            vnc_cloned.do_input(array.to_vec());
            vnc_out_handler(&cloned_ws, &vnc_cloned, &canvas_cloned);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    });
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    // onerror
    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    // onopen
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onclose_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket close");
        vnc_close_handle(&vnc, &canvas, "Disconnected");
    });
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    start_websocket()
}
