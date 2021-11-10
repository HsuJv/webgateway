use yew::services::ConsoleService;

use super::common::*;

const VNC_RFB33: &[u8; 12] = b"RFB 003.003\n";
const VNC_RFB37: &[u8; 12] = b"RFB 003.007\n";
const VNC_RFB38: &[u8; 12] = b"RFB 003.008\n";
const VNC_VER_UNSUPPORTED: &str = "unsupported version";

enum VncState {
    Handshake,
    Authentication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    Unknown(u8),
    Invalid,
    None,
    VncAuth,
    AppleRdp,
}

pub struct VncHandler {
    state: VncState,
    // output: Vec<u8>,
}

impl ProtocalImpl for VncHandler {
    fn new() -> Self {
        VncHandler {
            state: VncState::Handshake,
            // output: Vec::new(),
        }
    }

    fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput {
        match self.state {
            VncState::Handshake => {
                let support_version = self.handle_handshake(input);
                self.state = VncState::Authentication;
                if let Ok(support_version) = support_version {
                    ProtocalHandlerOutput::WsBuf(support_version.into())
                } else {
                    ProtocalHandlerOutput::Err(support_version.err().unwrap().to_string())
                }
            }
            VncState::Authentication => {
                ConsoleService::log(&format!("{:?}", input));
                return ProtocalHandlerOutput::Ok;
            }
            _ => panic!("unsupported version"),
        }
    }
}

// private methods
impl VncHandler {
    fn handle_handshake(&self, rfbversion: &[u8]) -> Result<&'static [u8], &'static str> {
        match rfbversion {
            b"RFB 003.003\n" => Ok(VNC_RFB33),
            b"RFB 003.007\n" => Ok(VNC_RFB37),
            b"RFB 003.008\n" => Ok(VNC_RFB38),
            _ => Err(VNC_VER_UNSUPPORTED),
        }
    }
    fn handle_authenticate(&mut self, auth: &[u8]) -> Result<&'static [u8], &'static str> {
        match auth {
            b"NONE\n" => Ok(b"\x00"),
            b"VNCAUTH\n" => Ok(b"\x01"),
            b"APPLETALK\n" => Ok(b"\x02"),
            _ => Err(VNC_VER_UNSUPPORTED),
        }
    }
}
