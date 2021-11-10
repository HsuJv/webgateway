use yew::services::ConsoleService;

use super::common::*;

const VNC_RFB33: &[u8; 12] = b"RFB 003.003\n";
const VNC_RFB37: &[u8; 12] = b"RFB 003.007\n";
const VNC_RFB38: &[u8; 12] = b"RFB 003.008\n";
const VNC_VER_UNSUPPORTED: &str = "unsupported version";
const VNC_FAILED: &str = "Connection failed with unknow reason";

enum VncState {
    Handshake,
    Authentication,
    ClientInit, // auth done
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SecurityType {
    Invalid = 0,
    None = 1,
    VncAuth = 2,
    RA2 = 5,
    RA2ne = 6,
    Tight = 16,
    Ultra = 17,
    TLS = 18,
    VeNCrypt = 19,
}

pub struct VncHandler {
    state: VncState,
    challenge: [u8; 16],
    security_type: SecurityType,
    password: String,
}

impl ProtocalImpl for VncHandler {
    fn new() -> Self {
        VncHandler {
            state: VncState::Handshake,
            challenge: [0u8; 16],
            security_type: SecurityType::Invalid,
            password: String::new(),
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
                return self.start_authenticate(input);
            }
            _ => panic!("unsupported version"),
        }
    }

    fn set_credential(&mut self, _username: &str, password: &str) -> ProtocalHandlerOutput {
        ConsoleService::log(&format!("{:?}", password));
        ConsoleService::log(&format!("{:?}", self.challenge));
        // since vnc do not require username, so we just ignore it
        self.password = password.to_string();
        self.continue_authenticate()
    }
}

// private methods
impl VncHandler {
    fn handle_handshake(&self, rfbversion: &[u8]) -> Result<&'static [u8], &'static str> {
        match rfbversion {
            b"RFB 003.003\n" => Ok(VNC_RFB33),
            b"RFB 003.007\n" => Ok(VNC_RFB33),
            b"RFB 003.008\n" => Ok(VNC_RFB33),
            _ => Err(VNC_VER_UNSUPPORTED),
        }
    }
    fn start_authenticate(&mut self, auth: &[u8]) -> ProtocalHandlerOutput {
        let mut sr = StreamReader::new(auth);
        match sr.read_u32() {
            Some(0) => {
                let err_msg = sr.read_string_l32().unwrap();
                ProtocalHandlerOutput::Err(err_msg)
            }
            Some(1) => {
                self.state = VncState::ClientInit;
                self.client_initialisation()
            }
            Some(2) => {
                sr.extract_slice(16, &mut self.challenge);
                ProtocalHandlerOutput::RequirePassword
            }
            _ => ProtocalHandlerOutput::Err(VNC_FAILED.to_string()),
        }
    }

    fn client_initialisation(&mut self) -> ProtocalHandlerOutput {
        ProtocalHandlerOutput::Ok
    }

    fn continue_authenticate(&mut self) -> ProtocalHandlerOutput {
        ProtocalHandlerOutput::Ok
    }
}
