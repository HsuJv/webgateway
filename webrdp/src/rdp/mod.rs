mod protocol;
mod rdp_impl;
mod x11cursor;
mod x11keyboard;

use rdp_impl::RdpInner;
use std::{rc::Rc, sync::Mutex};

pub enum MouseEventType {
    Down,
    Up,
    Move,
}

pub struct ImageData {
    pub type_: ImageType,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

pub enum RdpOutput {
    WsBuf(Vec<u8>),
    Err(String),
    RequireSSL,
    RequirePassword,
    SetResolution(u16, u16),
    RenderImage(ImageData),
    SetClipboard(String),
}

#[allow(dead_code)]
pub enum ImageType {
    Raw,
    Copy,
    Fill,
}

pub struct Rdp {
    inner: Rc<Mutex<RdpInner>>,
}

impl Rdp {
    pub fn new() -> Self {
        Rdp {
            inner: Rc::new(Mutex::new(RdpInner::new())),
        }
    }

    pub fn init(&self) {
        self.inner.lock().unwrap().init();
    }

    pub fn key_press(&self, _key: web_sys::KeyboardEvent, _down: bool) {
        // self.inner.lock().unwrap().key_press(key, down);
        unimplemented!()
    }

    pub fn mouse_event(&self, _mouse: web_sys::MouseEvent, _et: MouseEventType) {
        // self.inner.lock().unwrap().mouse_event(mouse, et);
        unimplemented!()
    }

    pub fn ctrl_alt_del(&self) {
        // self.inner.lock().unwrap().ctrl_alt_del();
        unimplemented!()
    }

    pub fn set_clipboard(&self, _s: &str) {
        unimplemented!()
    }

    pub fn do_input(&self, data: Vec<u8>) {
        self.inner.lock().unwrap().do_input(data);
    }

    pub fn get_output(&self) -> Option<Vec<RdpOutput>> {
        self.inner.lock().unwrap().get_output()
    }

    pub fn close(&self) {
        self.inner.lock().unwrap().close()
    }
}

impl Clone for Rdp {
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
    pub fn new(bufs: Vec<Vec<u8>>, len: usize) -> Self {
        Self {
            inner: bufs,
            remain: len,
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

    pub fn read_to_end(&mut self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.remain);
        self.read_exact_vec(&mut out, self.remain);
        out
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf, 1);
        buf[0]
    }

    pub fn read_u16_be(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf, 2);
        u16::from_be_bytes(buf)
    }

    pub fn read_u32_be(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf, 4);
        u32::from_be_bytes(buf)
    }

    pub fn read_u64_be(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf, 8);
        u64::from_be_bytes(buf)
    }

    pub fn read_u16_le(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf, 2);
        u16::from_le_bytes(buf)
    }

    pub fn read_u32_le(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf, 4);
        u32::from_le_bytes(buf)
    }

    pub fn read_u64_le(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf, 8);
        u64::from_le_bytes(buf)
    }

    pub fn read_string(&mut self, len: usize) -> String {
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf, len);
        String::from_utf8(buf).unwrap()
    }
}

pub struct StreamWriter {
    buf: Vec<u8>,
}

#[allow(dead_code)]
impl StreamWriter {
    pub fn new(buf: Vec<u8>) -> Self {
        Self { buf }
    }

    pub fn write_u8(&mut self, b: u8) {
        self.buf.push(b);
    }

    pub fn write_u16_be(&mut self, b: u16) {
        self.buf.extend_from_slice(&b.to_be_bytes());
    }

    pub fn write_u32_be(&mut self, b: u32) {
        self.buf.extend_from_slice(&b.to_be_bytes());
    }

    pub fn write_u16_le(&mut self, b: u16) {
        self.buf.extend_from_slice(&b.to_le_bytes());
    }

    pub fn write_u32_le(&mut self, b: u32) {
        self.buf.extend_from_slice(&b.to_le_bytes());
    }

    pub fn write_string(&mut self, s: &str) {
        self.buf.extend_from_slice(s.as_bytes());
    }

    pub fn write_slice(&mut self, s: &[u8]) {
        self.buf.extend_from_slice(s);
    }

    pub fn get_inner(&self) -> &[u8] {
        &self.buf
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }
}
