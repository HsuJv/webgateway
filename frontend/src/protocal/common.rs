use std::slice::Iter;

pub struct CanvasData {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

pub enum ProtocalHandlerOutput {
    Ok,
    WsBuf(Vec<u8>),
    Err(String),
    RequireUsername,
    RequirePassword,
    SetCanvas(u16, u16),
    RenderCanvas(Vec<CanvasData>),
}

pub struct ProtocalHandler<T>
where
    T: ProtocalImpl,
{
    inner: T,
}

impl<T> ProtocalHandler<T>
where
    T: ProtocalImpl,
{
    pub fn new() -> Self {
        Self { inner: T::new() }
    }

    pub fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput {
        self.inner.handle(input)
    }

    pub fn set_credential(&mut self, username: &str, password: &str) -> ProtocalHandlerOutput {
        self.inner.set_credential(username, password)
    }

    pub fn require_frame(&mut self, incremental: u8) -> ProtocalHandlerOutput {
        self.inner.require_frame(incremental)
    }
}

pub trait ProtocalImpl {
    fn new() -> Self;
    fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput;
    fn set_credential(&mut self, username: &str, password: &str) -> ProtocalHandlerOutput;
    fn require_frame(&mut self, incremental: u8) -> ProtocalHandlerOutput;
}

pub struct StreamReader<'a> {
    inner: Iter<'a, u8>,
}

impl<'a> StreamReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            inner: data.into_iter(),
        }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.inner.next().map(|&b| b)
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let mut buf = [0u8; 2];
        self.inner.by_ref().take(2).enumerate().for_each(|(i, b)| {
            buf[i] = *b;
        });
        Some(u16::from_be_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let mut buf = [0u8; 4];
        self.inner.by_ref().take(4).enumerate().for_each(|(i, b)| {
            buf[i] = *b;
        });
        Some(u32::from_be_bytes(buf))
    }

    pub fn read_s8(&mut self) -> Option<i8> {
        Some(Self::read_u8(self).map(|b| b as i8)?)
    }

    pub fn read_s16(&mut self) -> Option<i16> {
        Some(Self::read_u16(self).map(|b| b as i16)?)
    }

    pub fn read_s32(&mut self) -> Option<i32> {
        Some(Self::read_u32(self).map(|b| b as i32)?)
    }

    pub fn extract_slice(&mut self, len: usize, buf: &mut [u8]) {
        for x in self.inner.by_ref().take(len).enumerate() {
            buf[x.0] = *x.1;
        }
    }

    pub fn read_string_with_len(&mut self, len: usize) -> Option<String> {
        let mut buf = vec![0u8; len as usize];
        self.inner
            .by_ref()
            .take(len as usize)
            .enumerate()
            .for_each(|(i, b)| {
                buf[i] = *b;
            });
        Some(String::from_utf8(buf).unwrap())
    }

    pub fn read_string_l16(&mut self) -> Option<String> {
        let len = self.read_u16()? as usize;
        Some(self.read_string_with_len(len)?)
    }

    pub fn read_string_l32(&mut self) -> Option<String> {
        let len = self.read_u32()? as usize;
        Some(self.read_string_with_len(len)?)
    }

    pub fn eof(&mut self) -> bool {
        self.inner.next().is_none()
    }
}

pub struct StreamWriter<'a> {
    inner: &'a mut Vec<u8>,
}

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
