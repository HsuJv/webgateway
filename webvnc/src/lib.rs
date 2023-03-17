mod canvas;
mod utils;
mod x11cursor;
mod x11keyboard;

use ::vnc::{client::connector::VncConnector, PixelFormat, VncEncoding, VncEvent, X11Event};
use canvas::CanvasUtils;
use futures::StreamExt;
use tracing::{error, info};
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
    spawn_local(async move {
        let vnc = loop {
            // connect
            let url = format!(
                "{scheme}://{host}/websockify",
                scheme = if web_sys::window()
                    .unwrap()
                    .location()
                    .protocol()
                    .unwrap()
                    .starts_with("https")
                {
                    "wss"
                } else {
                    "ws"
                },
                host = web_sys::window().unwrap().location().host().unwrap()
            );

            // start websocket
            let (_ws, wsio) = WsMeta::connect(url, vec!["binary"]).await.unwrap();

            // vnc connect
            let vnc = VncConnector::new(wsio.into_io())
                .set_auth_method(async move { Ok(prompt("Input your password")) })
                .add_encoding(VncEncoding::Tight)
                .add_encoding(VncEncoding::Zrle)
                .add_encoding(VncEncoding::CopyRect)
                .add_encoding(VncEncoding::Raw)
                // .add_encoding(VncEncoding::CursorPseudo)
                .add_encoding(VncEncoding::DesktopSizePseudo)
                .allow_shared(true)
                .set_pixel_format(PixelFormat::rgba())
                .set_version(vnc::VncVersion::RFB33)
                .build()
                .unwrap()
                .try_start()
                .await;

            if vnc.is_err() {
                if let Err(e) = vnc {
                    let msg = format!("connect error {:?}\nRelog in please", e);
                    alert(&msg);
                    continue;
                }
            }
            break vnc;
        };

        let vnc = vnc.unwrap().finish().unwrap();

        let (x11_events_sender, mut x11_events_receiver) = tokio::sync::mpsc::channel(4096);

        let mut canvas = CanvasUtils::new(x11_events_sender.clone());

        fn hande_vnc_event(event: VncEvent, canvas: &mut CanvasUtils) {
            match event {
                VncEvent::SetResolution(screen) => {
                    info!("Resize {:?}", screen);
                    canvas.init(screen.width as u32, screen.height as u32)
                }
                VncEvent::RawImage(rect, data) => {
                    canvas.draw(rect, data);
                }
                VncEvent::Bell => {
                    //ignore
                }
                VncEvent::SetPixelFormat(_) => unreachable!(),
                VncEvent::Copy(dst, src) => {
                    canvas.copy(dst, src);
                }
                VncEvent::JpegImage(rect, data) => {
                    canvas.jpeg(rect, data);
                }
                VncEvent::SetCursor(rect, data) => {
                    if rect.width != 0 {
                        canvas.draw(rect, data)
                    }
                }
                VncEvent::Text(string) => {
                    setClipBoard(string);
                }
                VncEvent::Error(msg) => {
                    error!(msg);
                    alert(&msg);
                    panic!()
                }
                _ => unreachable!(),
            }
        }

        spawn_local(async move {
            let mut interval =
                fluvio_wasm_timer::Interval::new(std::time::Duration::from_millis(1));
            loop {
                match vnc.poll_event().await {
                    Ok(Some(e)) => hande_vnc_event(e, &mut canvas),
                    Ok(None) => {
                        let _ = interval.next().await;
                        let _ = vnc.input(X11Event::Refresh).await;
                    }
                    Err(e) => {
                        alert(&e.to_string());
                        break;
                    }
                }

                while let Ok(x11event) = x11_events_receiver.try_recv() {
                    let _ = vnc.input(x11event).await;
                }
            }
            canvas.close();
            let _ = vnc.close().await;
        })
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
