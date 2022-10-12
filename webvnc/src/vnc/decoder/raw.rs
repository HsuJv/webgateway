use super::super::vnc_impl::{PixelFormat, VncRect};
use super::super::{ImageData, ImageType, StreamReader};
use std::io::Result;

#[derive(Default)]
pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode(
        &mut self,
        format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        out_wait: &mut usize,
    ) -> Result<Option<Vec<ImageData>>> {
        let len = rect.width as usize * rect.height as usize * format.bits_per_pixel as usize / 8;
        if len > input.remain {
            *out_wait = len;
            return Ok(None);
        }

        let mut image_data = Vec::with_capacity(len);
        input.read_exact_vec(&mut image_data, len);
        let mut y = 0;
        let mut x = 0;

        while y < rect.height {
            while x < rect.width {
                let idx = (y as usize * rect.width as usize + x as usize) * 4;
                image_data.swap(idx, idx + 2);
                x += 1;
            }
            x = 0;
            y += 1;
        }
        Ok(Some(vec![
            (ImageData {
                type_: ImageType::Raw,
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                data: image_data,
            }),
        ]))
    }
}
