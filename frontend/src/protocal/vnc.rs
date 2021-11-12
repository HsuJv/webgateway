use yew::services::ConsoleService;

use super::common::*;
use super::des;

const VNC_RFB33: &[u8; 12] = b"RFB 003.003\n";
const VNC_RFB37: &[u8; 12] = b"RFB 003.007\n";
const VNC_RFB38: &[u8; 12] = b"RFB 003.008\n";
const VNC_VER_UNSUPPORTED: &str = "unsupported version";
const VNC_FAILED: &str = "Connection failed with unknow reason";

#[derive(Debug, Clone, Copy)]
enum VncState {
    Handshake,
    Authentication,
    ClientInit, // auth done
}

#[derive(Debug, Clone, Copy)]
enum VncVersion {
    NONE,
    VNC33,
    VNC37,
    VNC38,
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

#[derive(Debug, Clone)]
pub struct VncHandler {
    state: VncState,
    version: VncVersion,
    challenge: [u8; 16],
    security_type: SecurityType,
    width: u16,
    height: u16,
    pf: PixelFormat,
    name: String,
}

impl ProtocalImpl for VncHandler {
    fn new() -> Self {
        VncHandler {
            state: VncState::Handshake,
            version: VncVersion::NONE,
            challenge: [0u8; 16],
            security_type: SecurityType::Invalid,
            width: 0,
            height: 0,
            pf: PixelFormat::default(),
            name: String::new(),
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
                // reuse this state for auth repose handling
                if self.security_type == SecurityType::VncAuth {
                    self.handle_auth_response(input)
                } else {
                    self.start_authenticate(input)
                }
            }
            VncState::ClientInit => {
                ConsoleService::log(&format!("{:?}", input));
                self.handle_server_init(input)
            }
            _ => panic!("unsupported version"),
        }
    }

    fn set_credential(&mut self, _username: &str, password: &str) -> ProtocalHandlerOutput {
        // ConsoleService::log(&format!("{:?}", password));
        // ConsoleService::log(&format!("{:?}", self.challenge));
        // since vnc do not require username, so we just ignore it
        self.continue_authenticate(password)
    }
}

// private methods
impl VncHandler {
    fn handle_handshake(&mut self, rfbversion: &[u8]) -> Result<&'static [u8], &'static str> {
        match rfbversion {
            b"RFB 003.003\n" => {
                self.version = VncVersion::VNC33;
                Ok(VNC_RFB33)
            }
            b"RFB 003.007\n" => {
                self.version = VncVersion::VNC33;
                Ok(VNC_RFB33)
            }
            b"RFB 003.008\n" => {
                self.version = VncVersion::VNC33;
                Ok(VNC_RFB33)
            }
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
                self.security_type = SecurityType::None;
                self.state = VncState::ClientInit;
                self.client_initialisation()
            }
            Some(2) => {
                self.security_type = SecurityType::VncAuth;
                sr.extract_slice(16, &mut self.challenge);
                ProtocalHandlerOutput::RequirePassword
            }
            _ => ProtocalHandlerOutput::Err(VNC_FAILED.to_string()),
        }
    }

    fn client_initialisation(&mut self) -> ProtocalHandlerOutput {
        let shared_flag = 1;

        ProtocalHandlerOutput::WsBuf(vec![shared_flag].into())
    }

    fn continue_authenticate(&mut self, password: &str) -> ProtocalHandlerOutput {
        // referring
        // https://github.com/whitequark/rust-vnc/blob/0697238f2706dd34a9a95c1640e385f6d8c02961/src/client.rs
        // strange behavior

        let pass_len = password.len();
        let mut pass_bytes = [0u8; 8];
        for i in 0..8 {
            let c = if i < pass_len {
                password.as_bytes()[i]
            } else {
                0
            };
            let mut cs = 0u8;
            for j in 0..8 {
                cs |= ((c >> j) & 1) << (7 - j)
            }
            pass_bytes[i] = cs;
        }
        let output = des::encrypt(&self.challenge, &pass_bytes);
        ProtocalHandlerOutput::WsBuf(output.to_vec())
    }

    fn handle_auth_response(&mut self, response: &[u8]) -> ProtocalHandlerOutput {
        let mut sr = StreamReader::new(response);
        match sr.read_u32() {
            Some(0) => {
                self.state = VncState::ClientInit;
                self.client_initialisation()
            }
            Some(1) => {
                let err_msg = sr.read_string_l32().unwrap();
                ProtocalHandlerOutput::Err(err_msg)
            }
            _ => ProtocalHandlerOutput::Err(VNC_FAILED.to_string()),
        }
    }

    // example
    // [7, 128, 4, 176, 32, 24, 0, 1, 0, 255, 0, 255, 0, 255, 16, 8, 0, 0, 0, 0, 0, 0, 0, 14, 54, 122, 122, 100, 114, 113, 50, 45, 106, 105, 97, 120, 117, 0]
    // No. of bytes Type            [Value] Description
    // 2            CARD16          framebuffer-width
    // 2            CARD16          framebuffer-height
    // 16           PIXEL_FORMAT    server-pixel-format
    // 4            CARD32          name-length
    // name-length  CARD8           array name-string
    fn handle_server_init(&mut self, init: &[u8]) -> ProtocalHandlerOutput {
        let mut sr = StreamReader::new(init);
        self.width = sr.read_u16().unwrap();
        self.height = sr.read_u16().unwrap();
        let mut pfb: [u8; 16] = [0u8; 16];
        sr.extract_slice(16, &mut pfb);
        // This pixel format will be used unless the client requests a different format using the SetPixelFormat message
        self.pf = (&pfb).into();
        self.name = sr.read_string_l32().unwrap();

        ConsoleService::log(&format!("{:?}", self));
        ProtocalHandlerOutput::Ok
    }
}

// No. of bytes Type            [Value] Description
// 1            CARD8           bits-per-pixel
// 1            CARD8           depth
// 1            CARD8           big-endian-flag
// 1            CARD8           true-color-flag
// 2            CARD16          red-max
// 2            CARD16          green-max
// 2            CARD16          blue-max
// 1            CARD8           red-shift
// 1            CARD8           green-shift
// 1            CARD8           blue-shift
// 1            CARD8           padding
#[derive(Debug, Clone, Copy)]
struct PixelFormat {
    // the number of bits used for each pixel value on the wire
    // 8, 16, 32(usually) only
    bits_per_pixel: u8,
    depth: u8,
    // true if multi-byte pixels are interpreted as big endian
    big_endian_flag: u8,
    // true then the last six items specify how to extract the red, green and blue intensities from the pixel value
    true_color_flag: u8,
    // the next three always in big-endian order
    // no matter how the `big_endian_flag` is set
    red_max: u16,
    green_max: u16,
    blue_max: u16,
    // the number of shifts needed to get the red value in a pixel to the least significant bit
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,
    padding_1: u8,
    padding_2: u8,
    padding_3: u8,
}

impl From<PixelFormat> for Vec<u8> {
    fn from(pf: PixelFormat) -> Vec<u8> {
        let mut v = Vec::new();
        v.push(pf.bits_per_pixel);
        v.push(pf.depth);
        v.push(pf.big_endian_flag);
        v.push(pf.true_color_flag);
        v.push((pf.red_max >> 8) as u8);
        v.push(pf.red_max as u8);
        v.push((pf.green_max >> 8) as u8);
        v.push(pf.green_max as u8);
        v.push((pf.blue_max >> 8) as u8);
        v.push(pf.blue_max as u8);
        v.push(pf.red_shift);
        v.push(pf.green_shift);
        v.push(pf.blue_shift);
        v.push(pf.padding_1);
        v.push(pf.padding_2);
        v.push(pf.padding_3);
        v
    }
}

impl From<&[u8; 16]> for PixelFormat {
    fn from(pf: &[u8; 16]) -> Self {
        let mut sr = StreamReader::new(pf);
        let bits_per_pixel = sr.read_u8().unwrap();
        let depth = sr.read_u8().unwrap();
        let big_endian_flag = sr.read_u8().unwrap();
        let true_color_flag = sr.read_u8().unwrap();
        let red_max = sr.read_u16().unwrap();
        let green_max = sr.read_u16().unwrap();
        let blue_max = sr.read_u16().unwrap();
        let red_shift = sr.read_u8().unwrap();
        let green_shift = sr.read_u8().unwrap();
        let blue_shift = sr.read_u8().unwrap();
        let padding_1 = sr.read_u8().unwrap();
        let padding_2 = sr.read_u8().unwrap();
        let padding_3 = sr.read_u8().unwrap();
        Self {
            bits_per_pixel,
            depth,
            big_endian_flag,
            true_color_flag,
            red_max,
            green_max,
            blue_max,
            red_shift,
            green_shift,
            blue_shift,
            padding_1,
            padding_2,
            padding_3,
        }
    }
}

impl Default for PixelFormat {
    fn default() -> Self {
        Self {
            bits_per_pixel: 0,
            depth: 0,
            big_endian_flag: 0,
            true_color_flag: 0,
            red_max: 0,
            green_max: 0,
            blue_max: 0,
            red_shift: 0,
            green_shift: 0,
            blue_shift: 0,
            padding_1: 0,
            padding_2: 0,
            padding_3: 0,
        }
    }
}