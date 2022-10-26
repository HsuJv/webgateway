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
    fn prompt(msg: &str) -> String;
}

fn read_credentials(user: &mut String, password: &mut String, domain: &mut String) {
    *user = prompt("User:");
    *password = prompt("Password:");
    *domain = prompt("Domain:");
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
        let mut username = String::new();
        let mut password = String::new();
        let mut domain = String::new();
        read_credentials(&mut username, &mut password, &mut domain);
        let mut rdp = Rdp::new(&url, &username, &password, &domain);
        while !rdp.start().await {
            warn!("Wrong credientials");
            read_credentials(&mut username, &mut password, &mut domain);
            rdp = Rdp::new(&url, &username, &password, &domain);
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
