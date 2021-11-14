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

pub struct VncHandler {
    inner: Box<dyn VncState>,
}

impl ProtocalImpl for VncHandler {
    fn new() -> Self {
        Self {
            inner: Box::new(VncHandShake::default()),
        }
    }

    fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput {
        if self.inner.done() {
            self.inner = self.inner.next();
        }
        self.inner.handle(input)
    }

    fn set_credential(&mut self, _username: &str, password: &str) -> ProtocalHandlerOutput {
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
        self.inner.handle(&pass_bytes)
    }

    fn require_frame(&mut self, incremental: u8) -> ProtocalHandlerOutput {
        self.inner.frame_require(incremental)
    }
}

trait VncState {
    fn handle(&mut self, _input: &[u8]) -> ProtocalHandlerOutput;
    fn frame_require(&self, _incremental: u8) -> ProtocalHandlerOutput {
        ProtocalHandlerOutput::Err(VNC_FAILED.to_string())
    }
    fn done(&self) -> bool;
    fn next(&self) -> Box<dyn VncState>;
}

struct VncHandShake {
    done: bool,
}

impl Default for VncHandShake {
    fn default() -> Self {
        Self { done: false }
    }
}

impl VncState for VncHandShake {
    fn handle(&mut self, rfbversion: &[u8]) -> ProtocalHandlerOutput {
        let support_version = match rfbversion {
            b"RFB 003.003\n" => Ok(VNC_RFB33),
            b"RFB 003.007\n" => Ok(VNC_RFB33),
            b"RFB 003.008\n" => Ok(VNC_RFB33),
            _ => Err(VNC_VER_UNSUPPORTED),
        };
        self.done = true;
        if let Ok(support_version) = support_version {
            ProtocalHandlerOutput::WsBuf(support_version.to_vec())
        } else {
            ProtocalHandlerOutput::Err(support_version.err().unwrap().to_string())
        }
    }

    fn done(&self) -> bool {
        self.done
    }

    fn next(&self) -> Box<dyn VncState> {
        Box::new(VncAuthentiacator::default())
    }
}

struct VncAuthentiacator {
    challenge: [u8; 16],
    security_type: SecurityType,
    wait_password: bool,
    done: bool,
}

impl Default for VncAuthentiacator {
    fn default() -> Self {
        Self {
            challenge: [0u8; 16],
            security_type: SecurityType::Invalid,
            wait_password: true,
            done: false,
        }
    }
}

impl VncState for VncAuthentiacator {
    fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput {
        if self.security_type == SecurityType::VncAuth {
            if self.wait_password {
                self.continue_authenticate(input)
            } else {
                self.handle_auth_response(input)
            }
        } else {
            self.start_authenticate(input)
        }
    }

    fn done(&self) -> bool {
        self.done
    }

    fn next(&self) -> Box<dyn VncState> {
        Box::new(VncDrawing::default())
    }
}

impl VncAuthentiacator {
    fn start_authenticate(&mut self, auth: &[u8]) -> ProtocalHandlerOutput {
        let mut sr = StreamReader::new(auth);
        match sr.read_u32() {
            Some(0) => {
                let err_msg = sr.read_string_l32().unwrap();
                ProtocalHandlerOutput::Err(err_msg)
            }
            Some(1) => {
                self.security_type = SecurityType::None;
                self.send_client_initilize()
            }
            Some(2) => {
                self.security_type = SecurityType::VncAuth;
                self.wait_password = true;
                sr.extract_slice(16, &mut self.challenge);
                ProtocalHandlerOutput::RequirePassword
            }
            _ => ProtocalHandlerOutput::Err(VNC_FAILED.to_string()),
        }
    }

    fn handle_auth_response(&mut self, response: &[u8]) -> ProtocalHandlerOutput {
        let mut sr = StreamReader::new(response);
        match sr.read_u32() {
            Some(0) => self.send_client_initilize(),
            Some(1) => {
                let err_msg = sr.read_string_l32().unwrap();
                ProtocalHandlerOutput::Err(err_msg)
            }
            _ => ProtocalHandlerOutput::Err(VNC_FAILED.to_string()),
        }
    }

    fn continue_authenticate(&mut self, key_: &[u8]) -> ProtocalHandlerOutput {
        // let key: &[u8; 8] = key_.try_into().unwrap();
        let key = unsafe { std::mem::transmute(key_.as_ptr()) };
        let output = des::encrypt(&self.challenge, key);
        self.wait_password = false;
        ProtocalHandlerOutput::WsBuf(output.to_vec())
    }

    fn send_client_initilize(&mut self) -> ProtocalHandlerOutput {
        let shared_flag = 1;
        self.done = true;
        ProtocalHandlerOutput::WsBuf(vec![shared_flag].into())
    }
}

struct VncDrawing {
    width: u16,
    height: u16,
    pf: PixelFormat,
    name: String,
    server_init: bool,
    buffer: Vec<u8>,
    rects: Vec<VncRect>,
    num_rects_left: u16,
}

impl Default for VncDrawing {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            pf: PixelFormat::default(),
            name: "".to_string(),
            server_init: false,
            buffer: Vec::with_capacity(50),
            rects: Vec::new(),
            num_rects_left: 0,
        }
    }
}

impl VncState for VncDrawing {
    fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput {
        if self.server_init {
            let mut sr = StreamReader::new(input);
            if self.num_rects_left > 0 {
                // still in the previous update frame request
                self.extract_rects(&mut sr);
                self.render_rects()
            } else {
                let msg_type = sr.read_u8().unwrap();

                match msg_type {
                    0 => self.handle_framebuffer_update(&mut sr),
                    1 => self.handle_set_colour_map(&mut sr),
                    2 => self.handle_bell(&mut sr),
                    3 => self.handle_server_cut_text(&mut sr),
                    _ => ProtocalHandlerOutput::Err(VNC_FAILED.to_string()),
                }
            }
        } else {
            self.handle_server_init(input)
        }
    }

    fn frame_require(&self, incremental: u8) -> ProtocalHandlerOutput {
        if self.num_rects_left > 0 {
            ProtocalHandlerOutput::Ok
        } else {
            self.framebuffer_update_request(incremental)
        }
    }

    fn done(&self) -> bool {
        false
    }

    fn next(&self) -> Box<dyn VncState> {
        Box::new(VncEnds)
    }
}

impl VncDrawing {
    // example
    // [7, 128, 4, 176, 32, 24, 0, 1, 0, 255, 0, 255, 0, 255, 16, 8, 0, 0, 0, 0, 0, 0, 0, 14, 54, 122, 122, 100, 114, 113, 50, 45, 106, 105, 97, 120, 117, 0]
    // No. of bytes Type            [Value] Description
    // 2            CARD16          framebuffer-width
    // 2            CARD16          framebuffer-height
    // 16           PIXEL_FORMAT    server-pixel-format
    // 4            CARD32          name-length
    // name-length  CARD8           array name-string
    fn handle_server_init(&mut self, init: &[u8]) -> ProtocalHandlerOutput {
        self.buffer.extend_from_slice(init);
        if self.buffer.len() > 24 {
            let mut sr = StreamReader::new(init);
            self.width = sr.read_u16().unwrap();
            self.height = sr.read_u16().unwrap();
            let mut pfb: [u8; 16] = [0u8; 16];
            sr.extract_slice(16, &mut pfb);
            // This pixel format will be used unless the client requests a different format using the SetPixelFormat message
            self.pf = (&pfb).into();
            self.name = sr.read_string_l32().unwrap();
            ConsoleService::log(&format!("VNC: {}x{}", self.width, self.height));
            self.server_init = true;
            ProtocalHandlerOutput::SetCanvas(self.width, self.height)
        } else {
            ProtocalHandlerOutput::Ok
        }
    }

    // No. of bytes     Type    [Value]     Description
    // 1                CARD8   3           message-type
    // 1                CARD8               incremental
    // 2                CARD16              x-position
    // 2                CARD16              y-position
    // 2                CARD16              width
    // 2                CARD16              height
    fn framebuffer_update_request(&self, incremental: u8) -> ProtocalHandlerOutput {
        // ConsoleService::log(&format!("VNC: framebuffer_update_request {}", incremental));
        let mut out: Vec<u8> = Vec::new();
        let mut sw = StreamWriter::new(&mut out);
        sw.write_u8(3);
        sw.write_u8(incremental);
        sw.write_u16(0);
        sw.write_u16(0);
        sw.write_u16(self.width);
        sw.write_u16(self.height);
        ProtocalHandlerOutput::WsBuf(out)
    }

    // No. of bytes     Type    [Value]     Description
    // 1                CARD8   0           message-type
    // 1                                    padding
    // 2                CARD16              number-of-rectangles
    // This is followed by number-of-rectanglesrectangles of pixel data.
    fn handle_framebuffer_update(&mut self, sr: &mut StreamReader) -> ProtocalHandlerOutput {
        let _padding = sr.read_u8().unwrap();
        self.num_rects_left = sr.read_u16().unwrap();
        self.rects = Vec::with_capacity(self.num_rects_left as usize);
        self.extract_rects(sr);
        self.render_rects()
    }

    fn handle_set_colour_map(&mut self, _sr: &mut StreamReader) -> ProtocalHandlerOutput {
        unimplemented!()
    }

    fn handle_bell(&mut self, _sr: &mut StreamReader) -> ProtocalHandlerOutput {
        unimplemented!()
    }

    fn handle_server_cut_text(&mut self, _sr: &mut StreamReader) -> ProtocalHandlerOutput {
        unimplemented!()
    }

    fn extract_rects(&mut self, sr: &mut StreamReader) {
        while self.num_rects_left > 0 {
            // we always keep the last rect in the vec
            // and all the rects that has already been re-assembly should already been rendered
            if self.rects.len() > 0 && self.rects.last().unwrap().left_data > 0 {
                // which means that there is one rects that has not been re-assembly
                let last_rect = self.rects.last_mut().unwrap();
                while let Some(v) = sr.read_u8() {
                    last_rect.encoding_data.push(v);
                    last_rect.left_data -= 1;
                    if last_rect.left_data == 0 {
                        // at the end of the rect
                        self.num_rects_left -= 1;
                        break;
                    }
                }
                ConsoleService::log(&format!(
                    "VNC read: {}, pending {}",
                    last_rect.encoding_data.len(),
                    last_rect.left_data
                ));
                if last_rect.left_data == 0 {
                    // there is still some data left
                    // start a new rect
                    // it must be handled in the else branch
                    continue;
                } else {
                    // break the while loop
                    // render as much as we can
                    break;
                }
            } else {
                // a brand new rects
                let x = sr.read_u16().unwrap();
                let y = sr.read_u16().unwrap();
                let width = sr.read_u16().unwrap();
                let height = sr.read_u16().unwrap();
                let encoding_type = sr.read_u32().unwrap();
                match encoding_type {
                    0 => {
                        let mut left_data = width as u32 * height as u32 * 4;
                        let mut encoding_data: Vec<u8> = Vec::with_capacity(left_data as usize);
                        while let Some(v) = sr.read_u8() {
                            // read as much as we can
                            encoding_data.push(v);
                            if encoding_data.len() == left_data as usize {
                                break;
                            }
                        }
                        left_data -= encoding_data.len() as u32;
                        if left_data == 0 {
                            self.num_rects_left -= 1;
                        }
                        // ConsoleService::log(&format!("VNC read new: {}", encoding_data.len()));
                        self.rects.push(VncRect {
                            x,
                            y,
                            width,
                            height,
                            encoding_data,
                            encoding_type,
                            left_data,
                        });
                        // break the while loop
                        // render as much as we can
                        break;
                    }
                    _ => {
                        ConsoleService::log(&format!(
                            "VNC: unknown encoding type {}",
                            encoding_type
                        ));
                        ConsoleService::log(&format!(
                            "VNC: x:{}, y:{}, w:{}, h:{}",
                            x, y, width, height
                        ));
                        ConsoleService::log(&format!("VNC: left_data:{:x?}", sr.read_u32()));
                        unimplemented!()
                    }
                }
            }
        }
    }

    fn render_rects(&mut self) -> ProtocalHandlerOutput {
        let mut out: Vec<CanvasData> = Vec::new();
        if self.rects.len() > 1 || self.num_rects_left == 0 {
            let drain_len = {
                if self.num_rects_left != 0 {
                    self.rects.len() - 1
                } else {
                    self.rects.len()
                }
            };

            ConsoleService::log(&format!("VNC render {} rects", drain_len));
            for x in self.rects.drain(0..drain_len) {
                let mut data: Vec<u8> = Vec::with_capacity(x.encoding_data.len());
                for i in 0..x.width {
                    for j in 0..x.height {
                        let idx = (i as usize + j as usize * x.width as usize) * 4;

                        let b = x.encoding_data[idx + 0];
                        let g = x.encoding_data[idx + 1];
                        let r = x.encoding_data[idx + 2];
                        let a = x.encoding_data[idx + 3];

                        data.extend_from_slice(&[r, g, b, a]);
                    }
                }
                out.push(CanvasData {
                    x: x.x,
                    y: x.y,
                    width: x.width,
                    height: x.height,
                    data,
                });
            }
        }

        ProtocalHandlerOutput::RenderCanvas(out)
    }
}

struct VncEnds;

impl VncState for VncEnds {
    fn handle(&mut self, _input: &[u8]) -> ProtocalHandlerOutput {
        ProtocalHandlerOutput::Err(VNC_FAILED.to_string())
    }

    fn done(&self) -> bool {
        false
    }

    fn next(&self) -> Box<dyn VncState> {
        Box::new(VncEnds)
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
struct VncRect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    encoding_type: u32,
    encoding_data: Vec<u8>,
    left_data: u32,
}
