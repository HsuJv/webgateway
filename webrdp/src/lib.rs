mod canvas;
mod rdp;
mod utils;

use canvas::CanvasUtils;
use rdp::Rdp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, HtmlButtonElement, MessageEvent, WebSocket};

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn setClipBoard(s: String);
    pub fn getClipBoard() -> String;
}

fn rdp_close_handle(rdp: &Rdp, canvas: &CanvasUtils, msg: &str) {
    rdp.close();
    // unsafe {
    //     REFRESHER.take();
    // }
    canvas.close();
    let status = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("rdp_status")
        .unwrap();
    status.set_text_content(Some(msg));
}

fn rdp_out_handler(ws: &WebSocket, rdp: &Rdp, canvas: &CanvasUtils) {
    let out = rdp.get_output();
    if let Some(out) = out {
        for ref o in out {
            match o {
                rdp::RdpOutput::Err(err) => {
                    console_log!("Err {}", err);
                    rdp_close_handle(rdp, canvas, err);
                }
                rdp::RdpOutput::WsBuf(buf) => match ws.send_with_u8_array(buf) {
                    Ok(_) => {}
                    Err(err) => {
                        let err = format!("error sending message: {:?}", err);
                        rdp_close_handle(rdp, canvas, &err);
                    }
                },
                rdp::RdpOutput::RequireSSL => match ws.send_with_str("SSL") {
                    Ok(_) => {}
                    Err(err) => {
                        let err = format!("error launching ssl: {:?}", err);
                        rdp_close_handle(rdp, canvas, &err);
                    }
                },
                rdp::RdpOutput::RequirePassword => {
                    // let pwd = prompt("Please input the password");
                    // rdp.set_credential(&pwd);
                    // rdp_out_handler(ws, rdp, canvas);
                }
                rdp::RdpOutput::RenderImage(ri) => {
                    canvas.draw(ri);
                }
                rdp::RdpOutput::SetResolution(x, y) => {
                    canvas.init(*x as u32, *y as u32);
                    canvas.bind(rdp);
                    // rdp.require_frame(0);
                    rdp_out_handler(ws, rdp, canvas);

                    // let vnc_cloned = rdp.clone();
                    // let ws_cloned = ws.clone();
                    // let canvas_cloned = canvas.clone();

                    // set a interval for fps enhance
                    // let refresh = move || {
                    //     vnc_cloned.require_frame(1);
                    //     rdp_out_handler(&ws_cloned, &vnc_cloned, &canvas_cloned);
                    // };

                    // let refersher = Interval::new(20, refresh);

                    // unsafe {
                    //     REFRESHER = Some(refersher);
                    // }
                }
                rdp::RdpOutput::SetClipboard(text) => {
                    setClipBoard(text.to_owned());
                    // ConsoleService::log(&self.error_msg);
                }
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
    let rdp = Rdp::new();

    let clipboard = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("clipboardsend")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap();
    let rdp_cloned = rdp.clone();
    let onclickcb = Closure::<dyn FnMut()>::new(move || {
        console_log!("Send {:?}", getClipBoard());
        rdp_cloned.set_clipboard(&getClipBoard());
    });
    clipboard.set_onclick(Some(onclickcb.as_ref().unchecked_ref()));
    onclickcb.forget();

    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // on message
    let cloned_ws = ws.clone();
    let rdp_cloned = rdp.clone();
    let canvas_cloned = canvas.clone();

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            // let mut canvas_ctx = None;
            rdp_cloned.do_input(array.to_vec());
            rdp_out_handler(&cloned_ws, &rdp_cloned, &canvas_cloned);
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
    let rdp_cloned = rdp.clone();
    let ws_cloned = ws.clone();
    let canvas_cloned = canvas.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");
        rdp_cloned.init();
        rdp_out_handler(&ws_cloned, &rdp_cloned, &canvas_cloned);
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let onclose_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket close");
        rdp_close_handle(&rdp, &canvas, "Disconnected");
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
