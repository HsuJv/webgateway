use super::*;
use super::{des, x11cursor::MouseUtils, x11keyboard, MouseEventType};
use crate::{console_log, log};

const VNC_RFB33: &[u8; 12] = b"RFB 003.003\n";
// const VNC_RFB37: &[u8; 12] = b"RFB 003.007\n";
// const VNC_RFB38: &[u8; 12] = b"RFB 003.008\n";
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VncEncoding {
    Raw = 0,
    CopyRect = 1,
    RRE = 2,
    Hextile = 5,
    TRLE = 15,
    ZRLE = 16,
    CursorPseudo = -239,
    DesktopSizePseudo = -223,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VncState {
    Init,
    Handshake,
    Authing,
    ServerInit,
    Connected,
    Disconnected,
}

pub enum ServerMessage {
    FramebufferUpdate,
    SetColourMap,
    ServerCutText,
    None,
}

pub struct Vnc {
    state: VncState,
    // supported_versions: Vec<u8>,
    supported_encodings: Vec<VncEncoding>,
    security_type: SecurityType,
    challenge: [u8; 16],
    reader: StreamReader,
    mouse: MouseUtils,
    require: usize,
    width: u16,
    height: u16,
    pf: PixelFormat,
    name: String,
    msg_handling: ServerMessage,
    num_rect_left: u16,
    padding_rect: Option<VncRect>,
    outbuf: Vec<u8>,
    outs: Vec<VncOutput>,
}

impl Vnc {
    pub fn new() -> Self {
        Self {
            state: VncState::Init,
            supported_encodings: vec![
                VncEncoding::Raw,
                VncEncoding::CopyRect,
                // VncEncoding::RRE,
                // VncEncoding::Hextile,
                // VncEncoding::TRLE,
                // VncEncoding::ZRLE,
                // VncEncoding::CursorPseudo,
                // VncEncoding::DesktopSizePseudo,
            ],
            security_type: SecurityType::Invalid,
            challenge: [0; 16],
            reader: StreamReader::new(Vec::with_capacity(10)),
            mouse: MouseUtils::new(),
            require: 12, // the handleshake message length
            width: 0,
            height: 0,
            pf: PixelFormat::default(),
            name: String::new(),
            msg_handling: ServerMessage::None,
            num_rect_left: 0,
            padding_rect: None,
            outbuf: Vec::with_capacity(128),
            outs: Vec::with_capacity(10),
        }
    }

    pub fn do_input(&mut self, input: Vec<u8>) {
        // ConsoleService::info(&format!(
        //     "VNC input {}, left {}, require {}",
        //     input.len(),
        //     self.reader.remain(),
        //     self.require
        // ));
        self.reader.append(input);
        while self.reader.remain() >= self.require {
            self.handle_input();
            // ConsoleService::info(&format!(
            //     "left {}, require {}",
            //     self.reader.remain(),
            //     self.require
            // ));
        }
    }

    pub fn get_output(&mut self) -> Vec<VncOutput> {
        if let ServerMessage::None = self.msg_handling {
            let mut out = Vec::with_capacity(self.outs.len());
            // console_log!("Get {} output", self.outs.len());
            for o in self.outs.drain(..) {
                out.push(o);
            }
            if !self.outbuf.is_empty() {
                out.push(VncOutput::WsBuf(self.outbuf.clone()));
                self.outbuf.clear();
            }
            return out;
        };
        Vec::new()
    }

    pub fn set_credential(&mut self, _username: &str, password: &str) {
        // referring
        // https://github.com/whitequark/rust-vnc/blob/0697238f2706dd34a9a95c1640e385f6d8c02961/src/client.rs
        // strange behavior

        let pass_len = password.len();
        let mut key = [0u8; 8];
        for (i, key_i) in key.iter_mut().enumerate() {
            let c = if i < pass_len {
                password.as_bytes()[i]
            } else {
                0
            };
            let mut cs = 0u8;
            for j in 0..8 {
                cs |= ((c >> j) & 1) << (7 - j)
            }
            *key_i = cs;
        }
        // console_log!("challenge {:x?}", self.challenge);
        let output = des::encrypt(&self.challenge, &key);

        self.outbuf.extend_from_slice(&output);
        self.state = VncState::Authing;
        self.require = 4; // the auth result message length
    }

    pub fn set_clipboard(&mut self, text: &str) {
        self.send_client_cut_text(text);
    }

    pub fn key_press(&mut self, key: web_sys::KeyboardEvent, down: bool) {
        if self.state != VncState::Connected {
            return;
        }
        let key = x11keyboard::KeyboardUtils::get_keysym(key);
        self.send_key_event(key, down);
    }

    pub fn mouse_event(&mut self, mouse: web_sys::MouseEvent, et: MouseEventType) {
        if self.state != VncState::Connected {
            return;
        }
        let (x, y, mask) = self.mouse.get_mouse_sym(mouse, et);
        self.send_pointer_event(x, y, mask);
    }

    pub fn require_frame(&mut self, incremental: u8) {
        if 0 == incremental {
            // first frame
            // set the client encoding
            self.send_client_encodings();
        }
        if let ServerMessage::None = self.msg_handling {
            self.framebuffer_update_request(incremental)
        }
    }
}

#[allow(dead_code)]
impl Vnc {
    fn read_u8(&mut self) -> u8 {
        self.reader.read_u8()
    }

    fn read_u16(&mut self) -> u16 {
        self.reader.read_u16()
    }

    fn read_u32(&mut self) -> u32 {
        self.reader.read_u32()
    }

    fn read_u64(&mut self) -> u64 {
        self.reader.read_u64()
    }

    fn read_exact_vec(&mut self, out_vec: &mut Vec<u8>, len: usize) {
        self.reader.read_exact_vec(out_vec, len);
    }

    fn read_exact(&mut self, buf: &mut [u8], len: usize) {
        self.reader.read_exact(buf, len)
    }

    fn read_string(&mut self, len: usize) -> String {
        self.reader.read_string(len)
    }

    fn read_string_l8(&mut self) -> String {
        let len = self.read_u8() as usize;
        self.reader.read_string(len)
    }

    fn read_string_l16(&mut self) -> String {
        let len = self.read_u16() as usize;
        self.reader.read_string(len)
    }

    fn read_string_l32(&mut self) -> String {
        let len = self.read_u32() as usize;
        self.reader.read_string(len)
    }
}

impl Vnc {
    fn disconnect_with_err(&mut self, err: &str) {
        console_log!("{:#?}", err);
        self.state = VncState::Disconnected;
        self.outs.push(VncOutput::Err(err.to_string()));
    }

    fn send_client_initilize(&mut self) {
        self.state = VncState::ServerInit;
        self.require = 25; // the minimal length of server_init message

        // send client_init message
        let shared_flag = 1;
        self.outbuf.push(shared_flag);
    }

    //  +--------------+--------------+---------------------+
    // | No. of bytes | Type [Value] | Description         |
    // +--------------+--------------+---------------------+
    // | 1            | U8 [2]       | message-type        |
    // | 1            |              | padding             |
    // | 2            | U16          | number-of-encodings |
    // +--------------+--------------+---------------------+

    //    This is followed by number-of-encodings repetitions of the following:
    //    +--------------+--------------+---------------+
    //    | No. of bytes | Type [Value] | Description   |
    //    +--------------+--------------+---------------+
    //    | 4            | S32          | encoding-type |
    //    +--------------+--------------+---------------+
    fn send_client_encodings(&mut self) {
        let mut out = Vec::with_capacity(10);
        let mut sw = StreamWriter::new(&mut out);
        sw.write_u8(2); // message-type
        sw.write_u8(0); // padding
        sw.write_u16(self.supported_encodings.len().try_into().unwrap()); // number-of-encodings

        for i in &self.supported_encodings {
            sw.write_u32(*i as u32);
        }

        self.outbuf.extend_from_slice(&out);
    }

    // +--------------+--------------+--------------+
    // | No. of bytes | Type [Value] | Description  |
    // +--------------+--------------+--------------+
    // | 1            | U8 [4]       | message-type |
    // | 1            | U8           | down-flag    |
    // | 2            |              | padding      |
    // | 4            | U32          | key          |
    // +--------------+--------------+--------------+
    fn send_key_event(&mut self, key: u32, down: bool) {
        let mut out = Vec::with_capacity(10);
        let mut sw = StreamWriter::new(&mut out);
        sw.write_u8(4); // message-type
        sw.write_u8(if down { 1 } else { 0 }); // down
        sw.write_u16(0); // padding
        sw.write_u32(key); // key

        // console_log!("send key event {:x?} {:?}", key, down);
        self.outbuf.extend_from_slice(&out);
    }

    // +--------------+--------------+--------------+
    // | No. of bytes | Type [Value] | Description  |
    // +--------------+--------------+--------------+
    // | 1            | U8 [5]       | message-type |
    // | 1            | U8           | button-mask  |
    // | 2            | U16          | x-position   |
    // | 2            | U16          | y-position   |
    // +--------------+--------------+--------------+
    fn send_pointer_event(&mut self, x: u16, y: u16, mask: u8) {
        let mut out = Vec::with_capacity(10);
        let mut sw = StreamWriter::new(&mut out);
        sw.write_u8(5); // message-type
        sw.write_u8(mask); // mask
        sw.write_u16(x); // x
        sw.write_u16(y); // y

        // console_log!("send mouse event {:x?} {:x?} {:#08b}", x, y, mask);
        self.outbuf.extend_from_slice(&out);
    }

    // +--------------+--------------+--------------+
    // | No. of bytes | Type [Value] | Description  |
    // +--------------+--------------+--------------+
    // | 1            | U8 [6]       | message-type |
    // | 3            |              | padding      |
    // | 4            | U32          | length       |
    // | length       | U8 array     | text         |
    // +--------------+--------------+--------------+
    fn send_client_cut_text(&mut self, text: &str) {
        let mut out = Vec::with_capacity(10);
        let mut sw = StreamWriter::new(&mut out);
        let len: u32 = text.len().try_into().unwrap_or(0);
        sw.write_u8(6); // message-type
        sw.write_u8(0); // padding
        sw.write_u16(0); // padding
        sw.write_u32(len); // length
        sw.write_string(text); // text

        // console_log!("send client cut text {:?}", len);
        self.outbuf.extend_from_slice(&out);
    }

    // No. of bytes     Type    [Value]     Description
    // 1                CARD8   3           message-type
    // 1                CARD8               incremental
    // 2                CARD16              x-position
    // 2                CARD16              y-position
    // 2                CARD16              width
    // 2                CARD16              height
    fn framebuffer_update_request(&mut self, incremental: u8) {
        // console_log!("VNC: framebuffer_update_request {}", incremental);
        let mut out: Vec<u8> = Vec::new();
        let mut sw = StreamWriter::new(&mut out);
        sw.write_u8(3);
        sw.write_u8(incremental);
        sw.write_u16(0);
        sw.write_u16(0);
        sw.write_u16(self.width);
        sw.write_u16(self.height);
        self.outbuf.extend_from_slice(&out);
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
                self.outbuf.extend_from_slice(v);
            }
            Err(e) => self.disconnect_with_err(e),
        }
    }

    fn do_authenticate(&mut self) {
        // console_log!("VNC: do_authenticate {}", self.reader.remain());
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
            self.outs.push(VncOutput::RequirePassword);
        }
    }

    fn handle_auth_result(&mut self) {
        let response = self.read_u32();
        console_log!("Auth resp {}", response);
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
        console_log!("VNC: {}x{}", self.width, self.height);
        self.name = self.read_string_l32();
        self.state = VncState::Connected;
        self.require = 1; // any message from sever will be handled
        self.outs
            .push(VncOutput::SetCanvas(self.width, self.height));
    }

    fn handle_server_message(&mut self) {
        match self.msg_handling {
            ServerMessage::SetColourMap => self.read_colour_map(),
            ServerMessage::ServerCutText => self.read_cut_text(),
            ServerMessage::FramebufferUpdate => self.read_rect(),
            ServerMessage::None => {
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
    }

    // No. of bytes     Type    [Value]     Description
    // 1                CARD8   0           message-type
    // 1                                    padding
    // 2                CARD16              number-of-rectangles
    // This is followed by number-of-rectanglesrectangles of pixel data.
    fn handle_framebuffer_update(&mut self) {
        let _padding = self.read_u8();
        self.num_rect_left = self.read_u16();
        // console_log!("VNC: {} rects", self.num_rects_left);
        self.require = 12; // the length of the first rectangle hdr
        self.msg_handling = ServerMessage::FramebufferUpdate;
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
            let encoding_enum = unsafe { std::mem::transmute(encoding_type) };
            match encoding_enum {
                VncEncoding::Raw => self.handle_raw_encoding(x, y, width, height),
                VncEncoding::CopyRect => self.handle_copy_rect_encoding(x, y, width, height),
                VncEncoding::RRE => self.handle_rre_encoding(x, y, width, height),
                VncEncoding::Hextile => self.handle_hextile_encoding(x, y, width, height),
                VncEncoding::TRLE => self.handle_trle_encoding(x, y, width, height),
                VncEncoding::ZRLE => self.handle_zrle_encoding(x, y, width, height),
                VncEncoding::CursorPseudo => {
                    self.handle_cursor_pseudo_encoding(x, y, width, height)
                }
                VncEncoding::DesktopSizePseudo => {
                    self.handle_desktop_size_pseudo_encoding(x, y, width, height)
                }
            }
        } else {
            // we now have an entire rectangle
            let rect = self.padding_rect.take().unwrap();
            let mut image_data: Vec<u8> = Vec::with_capacity(self.require);
            match rect.encoding_type {
                0 => {
                    // for _ in 0..rect.height {
                    //     for _ in 0..rect.width {
                    //         let mut pixel = [0u8; 4];
                    //         self.read_exact(&mut pixel, 4);
                    //         let b = pixel[0];
                    //         let g = pixel[1];
                    //         let r = pixel[2];
                    //         let a = pixel[3];
                    //         image_data.extend_from_slice(&[r, g, b, a]);
                    //     }
                    // }
                    self.read_exact_vec(&mut image_data, self.require);
                    let mut y = 0;
                    let mut x = 0;

                    // for y in 0..rect.height {
                    //     for x in 0..rect.width {
                    //         let idx = (y as usize * rect.width as usize + x as usize) * 4;
                    //         image_data.swap(idx, idx + 2)
                    //     }
                    // }
                    while y < rect.height {
                        while x < rect.width {
                            let idx = (y as usize * rect.width as usize + x as usize) * 4;
                            image_data.swap(idx, idx + 2);
                            x += 1;
                        }
                        x = 0;
                        y += 1;
                    }
                }
                1 => {
                    // copy rectangle
                    self.read_exact_vec(&mut image_data, 4);
                }
                _ => unimplemented!(),
            }
            self.outs.push(VncOutput::RenderCanvas(CanvasData {
                type_: rect.encoding_type,
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                data: image_data,
            }));
            self.num_rect_left -= 1;
            if 0 == self.num_rect_left {
                self.msg_handling = ServerMessage::None;
                self.require = 1; // any message from sever will be handled
            } else {
                self.require = 12; // the length of the next rectangle hdr
            }
        }
    }

    // Currently there is little or no support for colour maps. Some preliminary work was done
    // on this, but is incomplete. It was intended to be something like this:
    //      When the pixel format uses a “colour map”, this message tells the client
    //      that the specified pixel values should be mapped to the given RGB intensities.
    //      The server may only specify pixel values for which the client has
    //      not already set the RGB intensities using FixColourMapEntries (section
    //      5.2.2).
    // No. of bytes     Type    [Value]     Description
    // 1                CARD8   1           message-type
    // 1                                    padding
    // 2                CARD16              first-colour
    // 2                CARD16              number-of-colours
    fn handle_set_colour_map(&mut self) {
        let _padding = self.read_u8();
        let _first_colour = self.read_u16();
        self.require = self.read_u16() as usize * 6;
        self.msg_handling = ServerMessage::SetColourMap;
    }

    // No. of bytes Type        [Value]     Description
    // 2            CARD16                  red
    // 2            CARD16                  green
    // 2            CARD16                  blue
    fn read_colour_map(&mut self) {
        // while self.num_block_left > 0 {
        //     let _r = self.read_u16();
        //     let _g = self.read_u16();
        //     let _b = self.read_u16();
        //     self.num_block_left -= 1;
        // }

        // just consume the data
        let mut v = Vec::with_capacity(self.require);
        self.read_exact_vec(&mut v, self.require);
        self.require = 1;
        self.msg_handling = ServerMessage::None;
    }

    // Ring a bell on the client if it has one
    fn handle_bell(&mut self) {
        // just do nothing
    }

    // The server has new ASCII text in its cut buffer. End of lines are represented by the
    // linefeed / newline character (ASCII value 10) alone. No carriage-return (ASCII value
    // 13) is needed.
    //     No. of bytes     Type    [Value]     Description
    //      1               CARD8   3           message-type
    //      3                                   padding
    //      4               CARD32              length
    //      length          CARD8               array text
    fn handle_server_cut_text(&mut self) {
        for _ in 0..3 {
            self.read_u8();
        }
        self.require = self.read_u32() as usize;
        self.msg_handling = ServerMessage::ServerCutText;
        console_log!("VNC: ServerCutText {} bytes", self.require);
    }

    fn read_cut_text(&mut self) {
        let text = self.read_string(self.require);
        self.require = 1;
        self.msg_handling = ServerMessage::None;
        self.outs.push(VncOutput::SetClipboard(text));
    }

    fn handle_raw_encoding(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.require = width as usize * height as usize * self.pf.bits_per_pixel as usize / 8;
        self.padding_rect = Some(VncRect {
            x,
            y,
            width,
            height,
            encoding_type: 0,
        });
    }

    fn handle_copy_rect_encoding(&mut self, x: u16, y: u16, width: u16, height: u16) {
        console_log!("VNC: CopyRect {} {} {} {}", x, y, width, height);
        self.require = 4;
        self.padding_rect = Some(VncRect {
            x,
            y,
            width,
            height,
            encoding_type: 1,
        });
    }

    fn handle_rre_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        //   Note: RRE encoding is obsolescent.  In general, ZRLE and TRLE
        //    encodings are more compact.

        unimplemented!()
    }

    fn handle_hextile_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        // Note: Hextile encoding is obsolescent.  In general, ZRLE and TRLE
        // encodings are more compact.
        unimplemented!()
    }

    fn handle_trle_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        unimplemented!()
    }

    fn handle_zrle_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        unimplemented!()
    }

    fn handle_cursor_pseudo_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
        unimplemented!()
    }

    fn handle_desktop_size_pseudo_encoding(&mut self, _x: u16, _y: u16, _width: u16, _height: u16) {
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
#[derive(Debug, Clone, Copy, Default)]
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
        vec![
            pf.bits_per_pixel,
            pf.depth,
            pf.big_endian_flag,
            pf.true_color_flag,
            (pf.red_max >> 8) as u8,
            pf.red_max as u8,
            (pf.green_max >> 8) as u8,
            pf.green_max as u8,
            (pf.blue_max >> 8) as u8,
            pf.blue_max as u8,
            pf.red_shift,
            pf.green_shift,
            pf.blue_shift,
            pf.padding_1,
            pf.padding_2,
            pf.padding_3,
        ]
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

struct VncRect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    encoding_type: u32,
}
