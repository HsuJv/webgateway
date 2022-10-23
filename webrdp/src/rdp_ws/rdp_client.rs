use rdp::{
    core::client::{Connector, RdpClient},
    model::{
        error::{RdpError, RdpErrorKind, RdpResult},
        link::{self, AsyncSecureBio},
    },
};
use web_sys::Element;

use crate::{console_log, log};

use super::ws_bio::WsSecureBio;

const RDP_HOSTNAME: &str = "webrdp";

pub struct Rdp {
    url: String,
    status_bar: Element,
    username: String,
    password: String,
    domain: String,
}

impl Rdp {
    pub fn new(url: &str, username: &str, password: &str, domain: &str) -> Self {
        let status_bar = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("rdp_status")
            .unwrap();
        Self {
            url: url.to_owned(),
            status_bar,
            username: username.to_owned(),
            password: password.to_owned(),
            domain: domain.to_owned(),
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

        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        let mut rdp_connector = Connector::new()
            .screen(body.client_width() as u16, body.client_height() as u16)
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

        let mut rdp_client = rdp_connector.connect(Box::new(ws_stream)).await.unwrap();
        console_log!("Rdp Started");
        loop {
            if let Err(rdp::model::error::Error::RdpError(e)) = rdp_client
                .read(|event| match event {
                    _ => console_log!("ignore event"),
                })
                .await
            {
                match e.kind() {
                    RdpErrorKind::Disconnect => {
                        console_log!("Server ask for disconnect");
                    }
                    _ => console_log!("{:?}", e),
                }
            }
        }
        true
    }

    fn disconnect_with_msg(&self, msg: &str) {
        self.status_bar.set_text_content(Some(msg));
    }
}
