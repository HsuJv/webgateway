mod canvas;
mod utils;
mod x11cursor;
mod x11keyboard;

use ::vnc::{client::connector::VncConnector, PixelFormat, VncEncoding, VncEvent, X11Event};
use anyhow::Result;
use canvas::CanvasUtils;
use tokio;
use tracing_wasm::WASMLayerConfigBuilder;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::WsMeta;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    pub fn setClipBoard(s: String);
    pub fn getClipBoard() -> String;
    fn prompt(msg: &str) -> String;
}

fn run() -> Result<(), JsValue> {
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
        // start websocket
        let (_ws, wsio) = WsMeta::connect(url, vec!["binary"]).await.unwrap();

        // vnc connect
        let vnc = VncConnector::new(wsio.into_io())
            .set_auth_method(|| {
                let passwd = prompt("Input your password");
                passwd
            })
            .add_encoding(VncEncoding::Raw)
            .allow_shared(true)
            .set_pixel_format(PixelFormat::rgba())
            .set_version(vnc::VncVersion::RFB33)
            .build()
            .unwrap()
            .try_start()
            .await;

        if vnc.is_err() {
            if let Err(e) = vnc {
                let msg = format!("connect error {:?}\nRe log in", e);
                alert(&msg);
                panic!("{}", msg);
            }
        }

        let vnc = vnc.unwrap().finish().unwrap();

        let (vnc_evnets_sender, mut vnc_events_receiver) = tokio::sync::mpsc::channel(100);
        let (x11_events_sender, x11_events_receiver) = tokio::sync::mpsc::channel(100);

        spawn_local(async move {
            vnc.run(vnc_evnets_sender, x11_events_receiver)
                .await
                .unwrap()
        });
        let canvas = CanvasUtils::new(x11_events_sender.clone(), 60);

        while let Some(event) = vnc_events_receiver.recv().await {
            match event {
                VncEvent::SetResulotin(width, height) => canvas.init(width as u32, height as u32),
                VncEvent::BitImage(rect, data) => canvas.draw(rect, data),
                VncEvent::SetPixelFormat(_) => unreachable!(),
                _ => unreachable!(),
            }
            x11_events_sender.send(X11Event::Refresh).await.unwrap();
        }
        canvas.close();
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
    run()
}
