mod canvas;
mod input;
mod rdp_ws;
mod utils;

use rdp_ws::Rdp;
use tracing::warn;
use tracing_wasm::WASMLayerConfigBuilder;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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
        while !rdp.start().await {
            warn!("Wrong credientials");
        }
        rdp.main_loop().await
    });

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::INFO)
            .build(),
    );
    start_websocket()
}
