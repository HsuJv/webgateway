use async_io_stream::IoStream;
use async_trait::async_trait;
use rdp::model::{error::RdpResult, link::AsyncSecureBio};
use tokio::io::AsyncReadExt;
use ws_stream_wasm::*;

use crate::{console_log, log};
use {futures::stream::StreamExt, pharos::*, wasm_bindgen::UnwrapThrowExt, ws_stream_wasm::*};

pub struct WsSecureBio {
    peer_cert: Vec<u8>,
    ws_stream: IoStream<WsStreamIo, Vec<u8>>,
    ws_meta: WsMeta,
}

#[async_trait]
impl AsyncSecureBio<IoStream<WsStreamIo, Vec<u8>>> for WsSecureBio {
    async fn start_ssl(&mut self, _check_certificate: bool) -> RdpResult<()> {
        let _ = self.ws_meta.wrapped().send_with_str("SSL");
        let mut ber_cert = [0; 1500];
        let size = self.ws_stream.read(&mut ber_cert).await.unwrap();
        console_log!("Read {} byte public cert", size);
        self.peer_cert = ber_cert.to_vec();
        Ok(())
    }
    fn get_peer_certificate_der(&self) -> RdpResult<Option<Vec<u8>>> {
        Ok(Some(self.peer_cert.clone()))
    }
    async fn shutdown(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }

    fn get_io(&mut self) -> &mut IoStream<WsStreamIo, Vec<u8>> {
        &mut self.ws_stream
    }
}

impl WsSecureBio {
    pub async fn new(url: &str) -> Self {
        let (ws, wsio) = WsMeta::connect(url, vec!["binary"]).await.unwrap();
        Self {
            peer_cert: vec![],
            ws_stream: wsio.into_io(),
            ws_meta: ws,
        }
    }
}
