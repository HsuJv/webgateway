mod app;
mod utils;
mod pages;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, KeyboardEvent};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let handler_submit = move |e: KeyboardEvent| {
        e.stop_propagation();
        console::log_1(&format!("{:?}", e).into())
    };

    let handler = Box::new(handler_submit) as Box<dyn FnMut(_)>;

    let cb = Closure::wrap(handler);

    window
        .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
    yew::start_app::<app::App>();

    Ok(())
}
