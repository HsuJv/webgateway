use std::{rc::Rc, sync::Mutex};

pub struct CanvasData {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

pub enum MouseEventType {
    MouseDown,
    MouseUp,
    MouseMove,
}

pub enum ProtocalHandlerOutput {
    WsBuf(Vec<u8>),
    Err(String),
    RequireUsername,
    RequirePassword,
    SetCanvas(u16, u16),
    RenderCanvas(CanvasData),
    SetClipboard(String),
}

pub struct ProtocalHandler<T>
where
    T: ProtocalImpl,
{
    inner: Rc<Mutex<T>>,
}

impl<T> Clone for ProtocalHandler<T>
where
    T: ProtocalImpl,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> ProtocalHandler<T>
where
    T: ProtocalImpl,
{
    pub fn new() -> Self {
        Self {
            inner: Rc::new(Mutex::new(T::new())),
        }
    }

    pub fn do_input(&self, input: Vec<u8>) {
        self.inner.as_ref().lock().unwrap().do_input(input);
    }

    pub fn get_output(&self) -> Vec<ProtocalHandlerOutput> {
        self.inner.as_ref().lock().unwrap().get_output()
    }

    pub fn set_credential(&self, username: &str, password: &str) {
        self.inner
            .as_ref()
            .lock()
            .unwrap()
            .set_credential(username, password);
    }

    pub fn set_resolution(&self, width: u16, height: u16) {
        self.inner
            .as_ref()
            .lock()
            .unwrap()
            .set_resolution(width, height);
    }

    pub fn require_frame(&self, incremental: u8) {
        self.inner
            .as_ref()
            .lock()
            .unwrap()
            .require_frame(incremental);
    }

    pub fn key_press(&self, key: web_sys::KeyboardEvent, down: bool) {
        self.inner.as_ref().lock().unwrap().key_press(key, down);
    }

    pub fn mouse_event(&self, mouse: web_sys::MouseEvent, et: MouseEventType) {
        self.inner.as_ref().lock().unwrap().mouse_event(mouse, et);
    }
}

pub trait ProtocalImpl {
    fn new() -> Self
    where
        Self: Sized;
    fn do_input(&mut self, input: Vec<u8>);
    fn get_output(&mut self) -> Vec<ProtocalHandlerOutput>;
    fn set_credential(&mut self, username: &str, password: &str);
    fn set_resolution(&mut self, width: u16, height: u16);
    fn key_press(&mut self, key: web_sys::KeyboardEvent, down: bool);
    fn mouse_event(&mut self, mouse: web_sys::MouseEvent, et: MouseEventType);
    fn require_frame(&mut self, incremental: u8);
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

    pub fn write_string_with_len(&mut self, s: &str) {
        self.inner.extend_from_slice(s.as_bytes());
    }

    pub fn write_string_l16(&mut self, s: &str) {
        self.write_u16(s.len() as u16);
        self.write_string_with_len(s);
    }

    pub fn write_string_l32(&mut self, s: &str) {
        self.write_u32(s.len() as u32);
        self.write_string_with_len(s);
    }

    pub fn write_slice(&mut self, s: &[u8]) {
        self.inner.extend_from_slice(s);
    }
}
