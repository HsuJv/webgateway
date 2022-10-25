use super::ws_bio::*;
use crate::canvas;
use rdp::{
    core::{
        client::{Connector, RdpClient},
        event::RdpEvent,
    },
    model::error::RdpErrorKind,
};
use tokio::sync::mpsc;
use tracing::{info, warn};
use web_sys::Element;

const RDP_HOSTNAME: &str = "webrdp";

pub struct Rdp {
    url: String,
    status_bar: Element,
    username: String,
    password: String,
    domain: String,
    screen: (u16, u16),
    rdp_client: Option<RdpClient<WsStream>>,
}

impl Rdp {
    pub fn new(url: &str, username: &str, password: &str, domain: &str) -> Self {
        let status_bar = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("rdp_status")
            .unwrap();
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        let height = body.client_height() as u16;
        let width = body.client_width() as u16;
        Self {
            url: url.to_owned(),
            status_bar,
            username: username.to_owned(),
            password: password.to_owned(),
            domain: domain.to_owned(),
            rdp_client: None,
            screen: (width, height),
        }
    }

    pub fn set_user(&mut self, username: &str) {
        self.username = username.to_owned();
    }

    pub fn set_password(&mut self, password: &str) {
        self.password = password.to_owned();
    }

    pub fn set_domain(&mut self, domain: &str) {
        self.domain = domain.to_owned();
    }

    pub async fn start(&mut self) -> bool {
        let ws_stream = WsSecureBio::new(&self.url).await;

        let mut rdp_connector = Connector::new()
            .screen(self.screen.0, self.screen.1)
            .credentials(
                self.domain.clone(),
                self.username.clone(),
                self.password.clone(),
            )
            .set_restricted_admin_mode(false)
            .auto_logon(false)
            .blank_creds(false)
            .check_certificate(false)
            .name(RDP_HOSTNAME.to_string())
            .use_nla(true);

        match rdp_connector.connect(Box::new(ws_stream)).await {
            Ok(rdp_client) => {
                info!("Rdp Started");
                self.rdp_client = Some(rdp_client);
                true
            }
            Err(_) => false,
        }
    }

    pub async fn main_loop(mut self) {
        let mut rdp_client = self.rdp_client.take().unwrap();

        let (canvas_sender, mut rdp_reciver) = mpsc::channel(100);
        let canvas = canvas::CanvasUtils::new(canvas_sender);
        canvas.init(self.screen.0 as u32, self.screen.1 as u32);
        'main: loop {
            tokio::select! {
                engine_recv = rdp_client
                .read(|event| match event {
                    RdpEvent::Bitmap(bitmap) => {
                        canvas.draw(bitmap);
                    }
                    _ => unreachable!()
                }) => {
                    if let Err(rdp::model::error::Error::RdpError(e)) = engine_recv {
                        match e.kind() {
                            RdpErrorKind::Disconnect => {
                                info!("Server ask for disconnect");
                                canvas.close();
                                self.disconnect_with_msg("Disconnected");
                                break 'main;
                            }
                            _ => warn!("{:?}", e),
                        }
                    }
                },
                canvas_recv = rdp_reciver.recv() => {
                    if let Some(rdp_event) = canvas_recv {
                        let _ = rdp_client.try_write(rdp_event.into()).await;
                    }
                }
            }
        }
    }

    fn disconnect_with_msg(&self, msg: &str) {
        self.status_bar.set_text_content(Some(msg));
    }
}
