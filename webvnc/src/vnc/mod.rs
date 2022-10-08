mod des;
mod vnc_impl;
mod x11cursor;
mod x11keyboard;

pub enum MouseEventType {
    Down,
    Up,
    Move,
}

use std::{rc::Rc, sync::Mutex};

pub struct ImageData {
    pub type_: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

pub enum VncOutput {
    WsBuf(Vec<u8>),
    Err(String),
    RequirePassword,
    SetResolution(u16, u16),
    RenderImage(ImageData),
    SetClipboard(String),
}

pub struct Vnc {
    inner: Rc<Mutex<vnc_impl::Vnc>>,
}

impl Vnc {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(Mutex::new(vnc_impl::Vnc::new())),
        }
    }

    pub fn do_input(&self, input: Vec<u8>) {
        self.inner.lock().unwrap().do_input(input);
    }

    pub fn get_output(&self) -> Vec<VncOutput> {
        self.inner.lock().unwrap().get_output()
    }

    pub fn set_credential(&self, password: &str) {
        self.inner.lock().unwrap().set_credential(password);
    }

    pub fn set_clipboard(&self, text: &str) {
        self.inner.lock().unwrap().set_clipboard(text);
    }

    pub fn require_frame(&self, incremental: u8) {
        self.inner.lock().unwrap().require_frame(incremental);
    }

    pub fn key_press(&self, key: web_sys::KeyboardEvent, down: bool) {
        self.inner.lock().unwrap().key_press(key, down);
    }

    pub fn mouse_event(&self, mouse: web_sys::MouseEvent, et: MouseEventType) {
        self.inner.lock().unwrap().mouse_event(mouse, et);
    }
}

impl Clone for Vnc {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct StreamReader {
    inner: Vec<Vec<u8>>,
    remain: usize,
}

#[allow(dead_code)]
impl StreamReader {
    pub fn new(bufs: Vec<Vec<u8>>) -> Self {
        Self {
            inner: bufs,
            remain: 0,
        }
    }

    pub fn append(&mut self, buf: Vec<u8>) {
        self.remain += buf.len();
        self.inner.push(buf);
    }

    pub fn remain(&self) -> usize {
        self.remain
    }

    pub fn read_exact_vec(&mut self, out_vec: &mut Vec<u8>, n: usize) {
        let mut i = 0;
        let mut left = n;
        self.remain -= n;
        while i < n {
            if self.inner[0].len() > left {
                for it in self.inner[0].drain(0..left) {
                    out_vec.push(it);
                }
                i += left;
                left = 0;
            } else {
                out_vec.extend(&self.inner[0]);
                left -= self.inner[0].len();
                i += self.inner[0].len();
                self.inner.remove(0);
            }
        }
    }

    pub fn read_exact(&mut self, out_buf: &mut [u8], n: usize) {
        let mut i = 0;
        let mut left = n;
        self.remain -= n;
        while i < n {
            if self.inner[0].len() > left {
                out_buf[i..i + left].copy_from_slice(&self.inner[0][0..left]);
                self.inner[0].drain(0..left);
                i += left;
                left = 0;
            } else {
                out_buf[i..i + self.inner[0].len()].copy_from_slice(&self.inner[0]);
                left -= self.inner[0].len();
                i += self.inner[0].len();
                self.inner.remove(0);
            }
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf, 1);
        buf[0]
    }

    pub fn read_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf, 2);
        u16::from_be_bytes(buf)
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf, 4);
        u32::from_be_bytes(buf)
    }

    pub fn read_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf, 8);
        u64::from_be_bytes(buf)
    }

    pub fn read_string(&mut self, len: usize) -> String {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf, len);
        String::from_utf8(buf).unwrap()
    }
}

pub struct StreamWriter<'a> {
    inner: &'a mut Vec<u8>,
}

#[allow(dead_code)]
impl<'a> StreamWriter<'a> {
    pub fn new(buf: &'a mut Vec<u8>) -> Self {
        Self { inner: buf }
    }

    pub fn write_u8(&mut self, b: u8) {
        self.inner.push(b);
    }

    pub fn write_u16(&mut self, b: u16) {
        self.inner.extend_from_slice(&b.to_be_bytes());
    }

    pub fn write_u32(&mut self, b: u32) {
        self.inner.extend_from_slice(&b.to_be_bytes());
    }

    pub fn write_s8(&mut self, b: i8) {
        self.write_u8(b as u8);
    }

    pub fn write_s16(&mut self, b: i16) {
        self.write_u16(b as u16);
    }

    pub fn write_s32(&mut self, b: i32) {
        self.write_u32(b as u32);
    }

    pub fn write_string(&mut self, s: &str) {
        self.inner.extend_from_slice(s.as_bytes());
    }

    pub fn write_string_l16(&mut self, s: &str) {
        self.write_u16(s.len() as u16);
        self.write_string(s);
    }

    pub fn write_string_l32(&mut self, s: &str) {
        self.write_u32(s.len() as u32);
        self.write_string(s);
    }

    pub fn write_slice(&mut self, s: &[u8]) {
        self.inner.extend_from_slice(s);
    }
}
