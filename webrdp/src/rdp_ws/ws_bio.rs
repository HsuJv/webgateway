use async_io_stream::IoStream;
use async_trait::async_trait;
use rdp::model::{error::RdpResult, link::AsyncSecureBio};
use tokio::io::AsyncReadExt;
use tracing::{info, trace};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use ws_stream_wasm::*;

pub type WsStream = IoStream<WsStreamIo, Vec<u8>>;

pub struct WsSecureBio {
    peer_cert: Vec<u8>,
    ws_stream: WsStream,
    ws_meta: WsMeta,
}

#[async_trait]
impl AsyncSecureBio<WsStream> for WsSecureBio {
    async fn start_ssl(&mut self, _check_certificate: bool) -> RdpResult<()> {
        let _ = self.ws_meta.wrapped().send_with_str("SSL");
        let mut ber_cert = [0; 1500];
        let size = self.ws_stream.read(&mut ber_cert).await.unwrap();
        trace!("Read {} byte public cert", size);
        self.peer_cert = ber_cert.to_vec();
        Ok(())
    }
    fn get_peer_certificate_der(&self) -> RdpResult<Option<Vec<u8>>> {
        Ok(Some(self.peer_cert.clone()))
    }
    async fn shutdown(&mut self) -> std::io::Result<()> {
        let _ = self.ws_meta.close().await;
        Ok(())
    }

    fn get_io(&mut self) -> &mut WsStream {
        &mut self.ws_stream
    }
}

impl WsSecureBio {
    pub async fn new(url: &str) -> Self {
        let (ws, wsio) = WsMeta::connect(url, vec!["binary"]).await.unwrap();

        let onclose_callback = Closure::<dyn FnMut()>::new(move || {
            info!("socket close");
            let status_bar = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("rdp_status")
                .unwrap();
            status_bar.set_text_content(Some("Server Disconnected"));
            panic!("Closed");
        });

        ws.wrapped()
            .set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
        Self {
            peer_cert: vec![],
            ws_stream: wsio.into_io(),
            ws_meta: ws,
        }
    }
}
