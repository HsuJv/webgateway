use super::super::vnc_impl::{PixelFormat, VncRect};
use super::super::{ImageData, ImageType, StreamReader};
use super::zlib::ZlibReader;
use std::io::{Error, ErrorKind, Read, Result};

const MAX_PALETTE: usize = 256;

#[derive(Default)]
pub struct Decoder {
    zlibs: [Option<flate2::Decompress>; 4],
    ctrl: Option<u8>,
    filter: Option<u8>,
    len: usize,
    num_colors: usize,
    palette: Vec<u8>,
}

impl Decoder {
    pub fn new() -> Self {
        let mut new = Self {
            palette: Vec::with_capacity(MAX_PALETTE * 4),
            ..Default::default()
        };
        for i in 0..4 {
            let decompressor = flate2::Decompress::new(true);
            new.zlibs[i] = Some(decompressor);
        }
        new
    }

    pub fn decode(
        &mut self,
        format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        out_wait: &mut usize,
    ) -> Result<Option<Vec<ImageData>>> {
        if self.ctrl.is_none() {
            if input.remain < 1 {
                *out_wait = 1;
                return Ok(None);
            }
            let ctrl = input.read_u8();
            for i in 0..4 {
                if (ctrl >> i) & 1 == 1 {
                    self.zlibs[i].as_mut().unwrap().reset(true);
                }
            }

            // Figure out filter
            self.ctrl = Some(ctrl >> 4);
        }

        let mut ret = Vec::new();
        match self.ctrl.as_ref().unwrap() {
            8 => {
                // fill Rect
                self.fill_rect(format, rect, input, out_wait, &mut ret)?
            }
            9 => {
                // jpeg Rect
                self.jpeg_rect(format, rect, input, out_wait, &mut ret)?
            }
            10 => {
                // png Rect
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "PNG received in standard Tight rect",
                ));
            }
            x if x & 0x8 == 0 => {
                // basic Rect
                self.basic_rect(format, rect, input, out_wait, &mut ret)?
            }
            _ => {
                let err_msg = format!(
                    "Illegal tight compression received ({})",
                    self.ctrl.as_ref().unwrap()
                );
                return Err(Error::new(ErrorKind::InvalidData, err_msg));
            }
        }

        if !ret.is_empty() {
            self.ctrl = None;
            Ok(Some(ret))
        } else {
            Ok(None)
        }
    }

    fn read_data(&mut self, input: &mut StreamReader, wait: &mut usize) -> Option<Vec<u8>> {
        if self.len == 0 {
            if input.remain < 3 {
                *wait = 3;
                return None;
            }

            let mut byte = input.read_u8() as usize;
            self.len = byte & 0x7f;
            if byte & 0x80 == 0x80 {
                byte = input.read_u8() as usize;
                self.len |= (byte & 0x7f) << 7;

                if byte & 0x80 == 0x80 {
                    byte = input.read_u8() as usize;
                    self.len |= (byte as usize) << 14;
                }
            }
        }

        if input.remain < self.len {
            *wait = self.len;
            None
        } else {
            let mut data = Vec::with_capacity(self.len);
            input.read_exact_vec(&mut data, self.len);
            self.len = 0;
            Some(data)
        }
    }

    fn fill_rect(
        &mut self,
        _format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        wait: &mut usize,
        out: &mut Vec<ImageData>,
    ) -> Result<()> {
        if input.remain < 3 {
            *wait = 3;
        } else {
            let mut rgb = [0; 3];
            input.read_exact(&mut rgb, 3);
            rgb.swap(0, 2);
            out.push(ImageData {
                type_: ImageType::Fill,
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                data: rgb.to_vec(),
            })
        }
        Ok(())
    }

    fn jpeg_rect(
        &mut self,
        _format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        wait: &mut usize,
        out: &mut Vec<ImageData>,
    ) -> Result<()> {
        if let Some(d) = self.read_data(input, wait) {
            out.push(ImageData {
                type_: ImageType::Jpeg,
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
                data: d,
            });
        }
        Ok(())
    }

    fn basic_rect(
        &mut self,
        format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        wait: &mut usize,
        out: &mut Vec<ImageData>,
    ) -> Result<()> {
        if self.filter.is_none() {
            let filter;
            if self.ctrl.as_ref().unwrap() & 0x4 == 4 {
                if input.remain < 1 {
                    *wait = 1;
                    return Ok(());
                }
                filter = input.read_u8();
            } else {
                // Implicit CopyFilter
                filter = 0;
            }

            self.filter = Some(filter);
        }

        let stream_id = self.ctrl.as_ref().unwrap() & 0x3;
        match self.filter.as_ref().unwrap() {
            0 => {
                // copy filter
                self.copy_filter(stream_id, format, rect, input, wait, out)?;
            }
            1 => {
                // palette
                self.palette_filter(stream_id, format, rect, input, wait, out)?
            }
            2 => {
                // gradient
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Gradient filter not implemented",
                ));
            }
            _ => {
                let err_msg = format!(
                    "Illegal tight filter received (filter: {})",
                    self.filter.as_ref().unwrap()
                );
                return Err(Error::new(ErrorKind::InvalidData, err_msg));
            }
        }

        if !out.is_empty() {
            self.filter = None;
        }
        Ok(())
    }

    fn copy_filter(
        &mut self,
        stream: u8,
        _format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        wait: &mut usize,
        out: &mut Vec<ImageData>,
    ) -> Result<()> {
        let uncompressed_size = rect.width as usize * rect.height as usize * 3;
        if uncompressed_size == 0 {
            return Ok(());
        };

        let mut data;
        if uncompressed_size < 12 {
            if input.remain < uncompressed_size {
                *wait = uncompressed_size;
                return Ok(());
            }
            data = Vec::with_capacity(uncompressed_size);
            input.read_exact_vec(&mut data, uncompressed_size);
        } else if let Some(d) = self.read_data(input, wait) {
            let mut reader = ZlibReader::new(self.zlibs[stream as usize].take().unwrap(), &d);
            data = Vec::with_capacity(uncompressed_size);
            unsafe { data.set_len(uncompressed_size) }
            reader.read_exact(&mut data)?;
            self.zlibs[stream as usize] = Some(reader.into_inner()?);
        } else {
            return Ok(());
        }
        let mut image = Vec::with_capacity(uncompressed_size / 3 * 4);
        let mut j = 0;
        while j < uncompressed_size {
            image.extend_from_slice(&data[j..j + 3]);
            image.push(255);
            j += 3;
        }

        out.push(ImageData {
            type_: ImageType::Raw,
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            data: image,
        });

        Ok(())
    }

    fn palette_filter(
        &mut self,
        stream: u8,
        _format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        wait: &mut usize,
        out: &mut Vec<ImageData>,
    ) -> Result<()> {
        if self.num_colors == 0 {
            if input.remain < 1 {
                *wait = 1;
                return Ok(());
            }
            self.num_colors = input.read_u8() as usize + 1;
        }

        let palette_size = self.num_colors * 3;

        if self.palette.is_empty() {
            if input.remain < palette_size {
                *wait = palette_size;
                return Ok(());
            }
            input.read_exact_vec(&mut self.palette, palette_size);
        }

        let bpp = if self.num_colors <= 2 { 1 } else { 8 };
        let row_size = (rect.width as usize * bpp + 7) / 8;
        let uncompressed_size = rect.height as usize * row_size;

        if uncompressed_size == 0 {
            return Ok(());
        }

        let mut data;
        if uncompressed_size < 12 {
            if input.remain < uncompressed_size {
                *wait = uncompressed_size;
                return Ok(());
            }
            data = Vec::with_capacity(uncompressed_size);
            input.read_exact_vec(&mut data, uncompressed_size);
        } else if let Some(d) = self.read_data(input, wait) {
            let mut reader = ZlibReader::new(self.zlibs[stream as usize].take().unwrap(), &d);
            data = Vec::with_capacity(uncompressed_size);
            unsafe { data.set_len(uncompressed_size) }
            reader.read_exact(&mut data)?;
            self.zlibs[stream as usize] = Some(reader.into_inner()?);
        } else {
            return Ok(());
        }

        if self.num_colors == 2 {
            self.mono_rect(data, rect, out)?
        } else {
            self.palette_rect(data, rect, out)?
        }

        self.num_colors = 0;
        self.palette.clear();

        Ok(())
    }

    fn mono_rect(&mut self, data: Vec<u8>, rect: &VncRect, out: &mut Vec<ImageData>) -> Result<()> {
        // Convert indexed (palette based) image data to RGB
        // TODO: reduce number of calculations inside loop
        let total = rect.width as usize * rect.height as usize * 4;
        let mut image = Vec::with_capacity(total);
        unsafe { image.set_len(total) }

        let w = (rect.width as usize + 7) / 8;
        let w1 = rect.width as usize / 8;

        for y in 0..rect.height as usize {
            let mut dp;
            let mut sp;
            for x in 0..w1 {
                for b in (0..=7).rev() {
                    dp = (y * rect.width as usize + x * 8 + 7 - b) * 4;
                    sp = (data[y * w + x] as usize >> b & 1) * 3;
                    image[dp] = self.palette[sp];
                    image[dp + 1] = self.palette[sp + 1];
                    image[dp + 2] = self.palette[sp + 2];
                    image[dp + 3] = 255;
                }
            }
            let x = w1;
            let mut b = 7;
            while b >= 8 - rect.width as usize % 8 {
                dp = (y * rect.width as usize + x * 8 + 7 - b) * 4;
                sp = (data[y * w + x] as usize >> b & 1) * 3;
                image[dp] = self.palette[sp];
                image[dp + 1] = self.palette[sp + 1];
                image[dp + 2] = self.palette[sp + 2];
                image[dp + 3] = 255;
                b -= 1;
            }
        }
        out.push(ImageData {
            type_: ImageType::Raw,
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            data: image,
        });
        Ok(())
    }

    fn palette_rect(
        &mut self,
        data: Vec<u8>,
        rect: &VncRect,
        out: &mut Vec<ImageData>,
    ) -> Result<()> {
        // Convert indexed (palette based) image data to RGB
        let total = rect.width as usize * rect.height as usize * 4;
        let mut image = Vec::with_capacity(total);
        let mut i = 0;
        let mut j = 0;
        while i < total {
            let sp = data[j] as usize * 3;
            image.extend_from_slice(&self.palette[sp..sp + 3]);
            image.push(255);
            i += 4;
            j += 1;
        }
        out.push(ImageData {
            type_: ImageType::Raw,
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            data: image,
        });
        Ok(())
    }
}
