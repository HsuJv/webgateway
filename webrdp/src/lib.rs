mod rdp_ws;
mod utils;

use rdp_ws::Rdp;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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

fn read_credentials() {
    unimplemented!();
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

    spawn_local(async move {
        let mut rdp = Rdp::new(&url, "", "", "");
        while !rdp.start().await {}
    });

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    start_websocket()
}
