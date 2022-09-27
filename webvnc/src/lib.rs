mod utils;
mod vnc;

use vnc::{MouseEventType, Vnc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    ErrorEvent, HtmlCanvasElement, ImageData, KeyboardEvent, MessageEvent, MouseEvent, WebSocket,
};

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

fn bind_mouse_and_key(vnc: &Vnc, canvas: &HtmlCanvasElement) {
    let _window = web_sys::window().unwrap();
    let handler = vnc.clone();
    let key_down = move |e: KeyboardEvent| {
        e.prevent_default();
        e.stop_propagation();
        handler.key_press(e, true);
    };

    let handler = Box::new(key_down) as Box<dyn FnMut(_)>;

    let cb = Closure::wrap(handler);

    canvas
        .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();

    let handler = vnc.clone();
    let key_up = move |e: KeyboardEvent| {
        e.prevent_default();
        e.stop_propagation();
        handler.key_press(e, false);
    };

    let handler = Box::new(key_up) as Box<dyn FnMut(_)>;

    let cb = Closure::wrap(handler);

    canvas
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
    let handler = vnc.clone();
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

    let handler = vnc.clone();
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

    let handler = vnc.clone();
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

fn find_canvas() -> HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("vnc-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas
}

fn set_canvas(vnc: &Vnc, x: u16, y: u16) {
    let canvas = find_canvas();

    // set hight & width
    canvas.set_height(y as u32);
    canvas.set_width(x as u32);

    // bind keyboard & mouse
    bind_mouse_and_key(vnc, &canvas);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.rect(0 as f64, 0 as f64, x as f64, y as f64);
    ctx.fill();
}

fn vnc_out_handler(ws: &WebSocket, vnc: &Vnc) {
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
                // vnc::VncOutput::RequirePassword => {
                //     self.request_password = true;
                // }
                vnc::VncOutput::RenderCanvas(cr) => {
                    let canvas = find_canvas();
                    let ctx = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();

                    match cr.type_ {
                        1 => {
                            //copy
                            let sx = (cr.data[0] as u16) << 8 | cr.data[1] as u16;
                            let sy = (cr.data[2] as u16) << 8 | cr.data[3] as u16;

                            let _ = ctx.
                                draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                                &canvas,
                                sx as f64,
                                sy as f64,
                                cr.width as f64,
                                cr.height as f64,
                                cr.x as f64,
                                cr.y as f64,
                                cr.width as f64,
                                cr.height as f64
                            );
                        }
                        _ => {
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
                        }
                    }
                }
                vnc::VncOutput::SetCanvas(x, y) => {
                    set_canvas(vnc, *x, *y);

                    let vnc_cloned = vnc.clone();
                    let ws_cloned = ws.clone();
                    let mut incremental = 0;

                    // set a interval for fps enhance
                    let refresh = move || {
                        vnc_cloned.require_frame(incremental);
                        incremental = if incremental > 0 { incremental } else { 1 };
                        vnc_out_handler(&ws_cloned, &vnc_cloned);
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
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let vnc = Vnc::new();

    // on message
    let cloned_ws = ws.clone();

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            let array = js_sys::Uint8Array::new(&abuf);
            // let mut canvas_ctx = None;
            vnc.do_input(array.to_vec());
            vnc_out_handler(&cloned_ws, &vnc);
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

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    start_websocket()
}
