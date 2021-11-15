

use super::common::*;
use super::des;
use yew::services::ConsoleService;

const VNC_RFB33: &[u8; 12] = b"RFB 003.003\n";
const VNC_RFB37: &[u8; 12] = b"RFB 003.007\n";
const VNC_RFB38: &[u8; 12] = b"RFB 003.008\n";
const VNC_VER_UNSUPPORTED: &str = "unsupported version";
const VNC_FAILED: &str = "Connection failed with unknow reason";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SecurityType {
    Invalid = 0,
    None = 1,
    VncAuth = 2,
    // RA2 = 5,
    // RA2ne = 6,
    // Tight = 16,
    // Ultra = 17,
    // TLS = 18,
    // VeNCrypt = 19,
}

pub enum VncState {
    Init,
    Handshake,
    Authing,
    ServerInit,
    Connected,
    Disconnected,
}

pub struct VncHandler {
    state: VncState,
    security_type: SecurityType,
    challenge: [u8; 16],
    buf_num: usize,
    reader: StreamReader,
    require: usize,
    width: u16,
    height: u16,
    pf: PixelFormat,
    name: String,
    server_init: bool,
    during_update: bool,
    num_rects_left: u16,
    padding_rect: Option<VncRect>,
    outs: Vec<ProtocalHandlerOutput>,
}

impl ProtocalImpl for VncHandler {
    fn new() -> Self {
        Self {
            state: VncState::Init,
            security_type: SecurityType::Invalid,
            challenge: [0; 16],
            buf_num: 0,
            reader: StreamReader::new(Vec::with_capacity(10)),
            require: 12, // the handleshake message length
            width: 0,
            height: 0,
            pf: PixelFormat::default(),
            name: String::new(),
            server_init: false,
            during_update: false,
            num_rects_left: 0,
            padding_rect: None,
            outs: Vec::with_capacity(10),
        }
    }

    fn do_input(&mut self, input: Vec<u8>) {
        self.buf_num += input.len();
        ConsoleService::info(&format!(
            "VNC input {}, left {}, require {}",
            input.len(),
            self.buf_num,
            self.require
        ));
        self.reader.append(input);
        while self.buf_num >= self.require {
            self.handle_input();
        }
    }

    fn get_output(&mut self) -> Vec<ProtocalHandlerOutput> {
        let mut out = Vec::with_capacity(self.outs.len());
        // ConsoleService::log(&format!("Get {} output", self.outs.len()));
        for o in self.outs.drain(..) {
            out.push(o);
        }
        out
    }

    fn set_credential(&mut self, _username: &str, password: &str) {
        // referring
        // https://github.com/whitequark/rust-vnc/blob/0697238f2706dd34a9a95c1640e385f6d8c02961/src/client.rs
        // strange behavior

        let pass_len = password.len();
        let mut key = [0u8; 8];
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
            key[i] = cs;
        }
        // ConsoleService::log(&format!("challenge {:x?}", self.challenge));
        let output = des::encrypt(&self.challenge, &key);

        self.outs
            .push(ProtocalHandlerOutput::WsBuf(output.to_vec()));
        self.state = VncState::Authing;
        self.require = 4; // the auth result message length
    }

    fn require_frame(&mut self, incremental: u8) {
        if !self.during_update {
            self.framebuffer_update_request(incremental)
        }
    }
}

#[allow(dead_code)]
impl VncHandler {
    fn read_u8(&mut self) -> u8 {
        self.buf_num -= 1;
        self.reader.read_u8()
    }

    fn read_u16(&mut self) -> u16 {
        self.buf_num -= 2;
        self.reader.read_u16()
    }

    fn read_u32(&mut self) -> u32 {
        self.buf_num -= 4;
        self.reader.read_u32()
    }

    fn read_u64(&mut self) -> u64 {
        self.buf_num -= 8;
        self.reader.read_u64()
    }

    fn read_exact(&mut self, buf: &mut [u8], len: usize) {
        self.buf_num -= len;
        self.reader.read_exact(buf, len)
    }

    fn read_string_l8(&mut self) -> String {
        let len = self.read_u8() as usize;
        self.buf_num -= len;
        self.reader.read_string(len)
    }

    fn read_string_l16(&mut self) -> String {
        let len = self.read_u16() as usize;
        self.buf_num -= len;
        self.reader.read_string(len)
    }

    fn read_string_l32(&mut self) -> String {
        let len = self.read_u32() as usize;
        self.buf_num -= len;
        self.reader.read_string(len)
    }
}

impl VncHandler {
    fn disconnect_with_err(&mut self, err: &str) {
        ConsoleService::error(err);
        self.state = VncState::Disconnected;
        self.outs.push(ProtocalHandlerOutput::Err(err.to_string()));
    }

    fn send_client_initilize(&mut self) {
        self.state = VncState::ServerInit;
        self.require = 25; // the minimal length of server_init message

        // send client_init message
        let shared_flag = 1;
        self.outs
            .push(ProtocalHandlerOutput::WsBuf(vec![shared_flag].into()));
    }

    // No. of bytes     Type    [Value]     Description
    // 1                CARD8   3           message-type
    // 1                CARD8               incremental
    // 2                CARD16              x-position
    // 2                CARD16              y-position
    // 2                CARD16              width
    // 2                CARD16              height
    fn framebuffer_update_request(&mut self, incremental: u8) {
        // ConsoleService::log(&format!("VNC: framebuffer_update_request {}", incremental));
        self.during_update = true;
        let mut out: Vec<u8> = Vec::new();
        let mut sw = StreamWriter::new(&mut out);
        sw.write_u8(3);
        sw.write_u8(incremental);
        sw.write_u16(0);
        sw.write_u16(0);
        sw.write_u16(self.width);
        sw.write_u16(self.height);
        self.outs.push(ProtocalHandlerOutput::WsBuf(out));
    }

    fn handle_input(&mut self) {
        match self.state {
            VncState::Init => self.do_handshake(),
            VncState::Handshake => self.do_authenticate(),
            VncState::Authing => self.handle_auth_result(),
            VncState::ServerInit => self.handle_server_init(),
            VncState::Connected => self.handle_server_message(),
            _ => unimplemented!(),
        }
    }

    fn do_handshake(&mut self) {
        let mut rfbversion: [u8; 12] = [0; 12];
        self.read_exact(&mut rfbversion, 12);
        let support_version = match &rfbversion {
            b"RFB 003.003\n" => Ok(VNC_RFB33),
            b"RFB 003.007\n" => Ok(VNC_RFB33),
            b"RFB 003.008\n" => Ok(VNC_RFB33),
            _ => Err(VNC_VER_UNSUPPORTED),
        };

        match support_version {
            Ok(v) => {
                self.state = VncState::Handshake;
                self.require = 4; // the length of the security type message
                self.outs.push(ProtocalHandlerOutput::WsBuf(v.to_vec()));
            }
            Err(e) => self.disconnect_with_err(e),
        }
    }

    fn do_authenticate(&mut self) {
        if self.security_type == SecurityType::Invalid {
            let auth_type = self.read_u32();
            match auth_type {
                1 => {
                    self.security_type = SecurityType::None;
                    self.send_client_initilize();
                }

                2 => {
                    self.security_type = SecurityType::VncAuth;
                    self.require = 16; // the length of server challenge
                }
                _ => self.disconnect_with_err(VNC_FAILED),
            }
        } else {
            let mut challenge = [0u8; 16];
            self.read_exact(&mut challenge, 16);
            self.challenge = challenge;
            self.outs.push(ProtocalHandlerOutput::RequirePassword);
        }
    }

    fn handle_auth_result(&mut self) {
        let response = self.read_u32();
        ConsoleService::log(&format!("Auth resp {}", response));
        match response {
            0 => self.send_client_initilize(),
            1 => {
                let err_msg = self.read_string_l32();
                self.disconnect_with_err(&err_msg);
            }
            _ => self.disconnect_with_err(VNC_FAILED),
        }
    }

    // No. of bytes Type            [Value] Description
    // 2            CARD16          framebuffer-width
    // 2            CARD16          framebuffer-height
    // 16           PIXEL_FORMAT    server-pixel-format
    // 4            CARD32          name-length
    // name-length  CARD8           array name-string
    fn handle_server_init(&mut self) {
        self.width = self.read_u16();
        self.height = self.read_u16();
        let mut pfb: [u8; 16] = [0u8; 16];
        self.read_exact(&mut pfb, 16);
        // This pixel format will be used unless the client requests a different format using the SetPixelFormat message
        self.pf = (&pfb).into();
        ConsoleService::log(&format!("VNC: {}x{}", self.width, self.height));
        self.name = self.read_string_l32();
        self.state = VncState::Connected;
        self.require = 1; // any message from sever will be handled
        self.outs
            .push(ProtocalHandlerOutput::SetCanvas(self.width, self.height));
    }

    fn handle_server_message(&mut self) {
        if self.num_rects_left > 0 {
            self.read_rect();
        } else {
            let msg_type = self.read_u8();

            match msg_type {
                0 => self.handle_framebuffer_update(),
                1 => self.handle_set_colour_map(),
                2 => self.handle_bell(),
                3 => self.handle_server_cut_text(),
                _ => self.disconnect_with_err(VNC_FAILED),
            }
        }
    }

    // No. of bytes     Type    [Value]     Description
    // 1                CARD8   0           message-type
    // 1                                    padding
    // 2                CARD16              number-of-rectangles
    // This is followed by number-of-rectanglesrectangles of pixel data.
    fn handle_framebuffer_update(&mut self) {
        let _padding = self.read_u8();
        self.num_rects_left = self.read_u16();
        ConsoleService::log(&format!("VNC: {} rects", self.num_rects_left));
        self.require = 12; // the length of the first rectangle
    }

    //Each rectangle consists of:
    // 2                CARD16              x-position
    // 2                CARD16              y-position
    // 2                CARD16              width
    // 2                CARD16              height
    // 4                CARD32              encoding-type:
    //                          0           raw encoding
    //                          1           copy rectangle encoding
    //                          2           RRE encoding
    //                          4           CoRRE encoding
    //                          5           Hextile encoding
    fn read_rect(&mut self) {
        if self.padding_rect.is_none() {
            // a brand new rectangle
            let x = self.read_u16();
            let y = self.read_u16();
            let width = self.read_u16();
            let height = self.read_u16();
            let encoding_type = self.read_u32();
            match encoding_type {
                0 => self.handle_raw_encoding(x, y, width, height),
                1 => self.handle_copy_rect_encoding(x, y, width, height),
                2 => self.handle_rre_encoding(x, y, width, height),
                4 => self.handle_corre_encoding(x, y, width, height),
                5 => self.handle_hextile_encoding(x, y, width, height),
                _ => self.disconnect_with_err(VNC_FAILED),
            }
        } else {
            // we now have an entire rectangle
            let rect = self.padding_rect.take().unwrap();
            let mut image_data: Vec<u8> = Vec::with_capacity(self.require);
            match rect.encoding_type {
                0 => {
                    for _ in 0..rect.height {
                        for _ in 0..rect.width {
                            let mut pixel = [0u8; 4];
                            self.read_exact(&mut pixel, 4);
                            let b = pixel[0];
                            let g = pixel[1];
                            let r = pixel[2];
                            let a = pixel[3];
                            image_data.extend_from_slice(&[r, g, b, a]);
                        }
                    }
                }
                _ => unimplemented!(),
            }
            self.outs
                .push(ProtocalHandlerOutput::RenderCanvas(CanvasData {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: rect.height,
                    data: image_data,
                }));
            self.num_rects_left -= 1;
        }
        if 0 == self.num_rects_left {
            self.during_update = false;
            self.require = 1;
        }
        ConsoleService::log(&format!("{} rects left", self.num_rects_left));
    }

    fn handle_set_colour_map(&mut self) {
        unimplemented!()
    }

    fn handle_bell(&mut self) {
        unimplemented!()
    }

    fn handle_server_cut_text(&mut self) {
        unimplemented!()
    }

    fn handle_raw_encoding(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.require = width as usize * height as usize * self.pf.bits_per_pixel as usize / 8;
        self.padding_rect = Some(VncRect {
            x,
            y,
            width,
            height,
            encoding_type: 0,
            encoding_data: Vec::new(), // we donnot need to store the data
        });
    }

    fn handle_copy_rect_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        unimplemented!()
    }

    fn handle_rre_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        unimplemented!()
    }

    fn handle_corre_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        unimplemented!()
    }

    fn handle_hextile_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        unimplemented!()
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
        let bits_per_pixel = pf[0];
        let depth = pf[1];
        let big_endian_flag = pf[2];
        let true_color_flag = pf[3];
        let red_max = pf[4] as u16 | ((pf[5] as u16) << 8);
        let green_max = pf[6] as u16 | ((pf[7] as u16) << 8);
        let blue_max = pf[8] as u16 | ((pf[9] as u16) << 8);
        let red_shift = pf[10];
        let green_shift = pf[11];
        let blue_shift = pf[12];
        let padding_1 = pf[13];
        let padding_2 = pf[14];
        let padding_3 = pf[15];
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

struct VncRect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    encoding_type: u32,
    encoding_data: Vec<u8>,
}
