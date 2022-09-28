mod canvas;
mod utils;
mod vnc;

use canvas::CanvasUtils;
use vnc::Vnc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    // fn setTimeout(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn prompt(s: &str) -> String;
}

fn vnc_out_handler(ws: &WebSocket, vnc: &Vnc, canvas: &CanvasUtils) {
    let out = vnc.get_output();
    if !out.is_empty() {
        for ref o in out {
            match o {
                vnc::VncOutput::Err(err) => {
                    console_log!("Err {}", err);
                }
                vnc::VncOutput::WsBuf(buf) => match ws.send_with_u8_array(buf) {
                    Ok(_) => {}
                    Err(err) => console_log!("error sending message: {:?}", err),
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

                    let handler = Box::new(refresh) as Box<dyn FnMut()>;

                    let cb = Closure::wrap(handler);

                    setInterval(&cb, 20);
                    cb.forget();
                }
                // vnc::VncOutput::SetClipboard(text) => {
                //     self.clipboard
                //         .borrow_mut()
                //         .as_mut()
                //         .unwrap()
                //         .send_message(components::clipboard::ClipboardMsg::UpdateClipboard(text));
                //     // ConsoleService::log(&self.error_msg);
                // }
                _ => unimplemented!(),
            }
        }
    }
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

    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // on message
    let cloned_ws = ws.clone();

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            // let mut canvas_ctx = None;
            vnc.do_input(array.to_vec());
            vnc_out_handler(&cloned_ws, &vnc, &canvas);
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
