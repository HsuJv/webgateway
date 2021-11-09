

use yew::services::ConsoleService;

use super::common::*;

const VNC_RFB33: &[u8; 12] = b"RFB 003.003\n";
const VNC_RFB37: &[u8; 12] = b"RFB 003.007\n";
const VNC_RFB38: &[u8; 12] = b"RFB 003.008\n";

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
    output: Vec<u8>,
}

impl ProtocalImpl for VncHandler {
    fn new() -> Self {
        VncHandler {
            state: VncState::Handshake,
            output: Vec::new(),
        }
    }

    fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput {
        match self.state {
            VncState::Handshake => {
                self.handle_handshake(input);
                self.state = VncState::Authentication;
                return ProtocalHandlerOutput::WsBuf(self.output.clone());
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
    fn handle_handshake(&mut self, rfbversion: &[u8]) {
        match rfbversion {
            b"RFB 003.008\n" => {
                for b in VNC_RFB33 {
                    self.output.push(*b);
                }
            }
            _ => panic!("unsupported version"),
        }
    }
}
