/*
Copyright (c) 2016  whitequark <whitequark@whitequark.org>

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

use super::super::vnc_impl::*;
use super::super::{ImageData, ImageType, StreamReader};
use super::zlib::ZlibReader;
use byteorder::ReadBytesExt;
use std::io::{Read, Result};

struct BitReader<T: Read> {
    reader: T,
    buffer: u8,
    position: usize,
}

impl<T: Read> BitReader<T> {
    fn new(reader: T) -> BitReader<T> {
        BitReader {
            reader,
            buffer: 0,
            position: 8,
        }
    }

    fn into_inner(self) -> Result<T> {
        if self.position == 8 {
            Ok(self.reader)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "leftover ZRLE bit data",
            ))
        }
    }

    fn read_bits(&mut self, count: usize) -> std::io::Result<u8> {
        assert!(count > 0 && count <= 8);

        if self.position == 8 {
            self.buffer = self.reader.read_u8()?;
            self.position = 0;
        }

        if self.position + count <= 8 {
            let shift = 8 - (count + self.position);
            let mask = (1 << count) - 1;
            let result = (self.buffer >> shift) & mask;
            self.position += count;
            Ok(result)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "unaligned ZRLE bit read",
            ))
        }
    }

    fn read_bit(&mut self) -> std::io::Result<bool> {
        Ok(self.read_bits(1)? != 0)
    }

    fn align(&mut self) {
        self.position = 8;
    }
}

impl<T: Read> Read for BitReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.position == 8 {
            self.reader.read(buf)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "unaligned ZRLE byte read",
            ))
        }
    }
}

pub struct Decoder {
    padding_len: u32,
    decompressor: Option<flate2::Decompress>,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            decompressor: Some(flate2::Decompress::new(/*zlib_header*/ true)),
            padding_len: 0,
        }
    }

    pub fn decode(
        &mut self,
        format: &PixelFormat,
        rect: &VncRect,
        input: &mut StreamReader,
        out_wait: &mut usize,
    ) -> Result<Option<Vec<ImageData>>> {
        fn read_run_length(reader: &mut dyn Read) -> Result<usize> {
            let mut run_length_part;
            let mut run_length = 1;
            loop {
                run_length_part = reader.read_u8()?;
                run_length += run_length_part as usize;
                if 255 != run_length_part {
                    break;
                }
            }
            Ok(run_length)
        }

        fn copy_true_color(
            reader: &mut dyn Read,
            pixels: &mut Vec<u8>,
            pad: bool,
            compressed_bpp: usize,
            bpp: usize,
        ) -> Result<()> {
            let mut buf = [255; 4];
            reader.read_exact(&mut buf[pad as usize..pad as usize + compressed_bpp])?;
            buf.swap(0, 2);
            pixels.extend_from_slice(&buf[..bpp]);
            Ok(())
        }

        fn copy_indexed(palette: &[u8], pixels: &mut Vec<u8>, bpp: usize, index: u8) {
            let start = index as usize * bpp;
            pixels.extend_from_slice(&palette[start..start + bpp])
        }

        if self.padding_len == 0 {
            if input.remain < 4 {
                *out_wait = 4;
                return Ok(None);
            } else {
                self.padding_len = input.read_u32();
            }
        }

        if input.remain < self.padding_len as usize {
            *out_wait = self.padding_len as usize;
            return Ok(None);
        }

        let mut ret = Vec::new();
        let mut image_data = Vec::with_capacity(self.padding_len as usize);
        input.read_exact_vec(&mut image_data, self.padding_len as usize);

        let bpp = format.bits_per_pixel as usize / 8;
        let pixel_mask = (format.red_max as u32) << format.red_shift
            | (format.green_max as u32) << format.green_shift
            | (format.blue_max as u32) << format.blue_shift;

        let (compressed_bpp, pad_pixel) =
            if format.bits_per_pixel == 32 && format.true_color_flag > 0 && format.depth <= 24 {
                if pixel_mask & 0x000000ff == 0 {
                    (3, format.big_endian_flag > 0)
                } else if pixel_mask & 0xff000000 == 0 {
                    (3, format.big_endian_flag == 0)
                } else {
                    (4, false)
                }
            } else {
                (bpp, false)
            };

        let mut palette = Vec::with_capacity(128 * bpp);
        let mut reader = BitReader::new(ZlibReader::new(
            self.decompressor.take().unwrap(),
            &image_data,
        ));

        let mut y = 0;
        while y < rect.height {
            let height = if y + 64 > rect.height {
                rect.height - y
            } else {
                64
            };
            let mut x = 0;
            while x < rect.width {
                let width = if x + 64 > rect.width {
                    rect.width - x
                } else {
                    64
                };
                let pixel_count = height as usize * width as usize;

                let is_rle = reader.read_bit()?;
                let palette_size = reader.read_bits(7)?;
                palette.truncate(0);
                for _ in 0..palette_size {
                    copy_true_color(&mut reader, &mut palette, pad_pixel, compressed_bpp, bpp)?
                }

                let mut pixels = Vec::with_capacity(pixel_count * bpp);
                match (is_rle, palette_size) {
                    (false, 0) => {
                        // True Color pixels
                        for _ in 0..pixel_count {
                            copy_true_color(
                                &mut reader,
                                &mut pixels,
                                pad_pixel,
                                compressed_bpp,
                                bpp,
                            )?
                        }
                    }
                    (false, 1) => {
                        // Color fill
                        for _ in 0..pixel_count {
                            copy_indexed(&palette, &mut pixels, bpp, 0)
                        }
                    }
                    (false, 2) | (false, 3..=4) | (false, 5..=16) => {
                        // Indexed pixels
                        let bits_per_index = match palette_size {
                            2 => 1,
                            3..=4 => 2,
                            5..=16 => 4,
                            _ => unreachable!(),
                        };
                        for _ in 0..height {
                            for _ in 0..width {
                                let index = reader.read_bits(bits_per_index)?;
                                copy_indexed(&palette, &mut pixels, bpp, index)
                            }
                            reader.align();
                        }
                    }
                    (true, 0) => {
                        // True Color RLE
                        let mut count = 0;
                        let mut pixel = Vec::new();
                        while count < pixel_count {
                            pixel.truncate(0);
                            copy_true_color(
                                &mut reader,
                                &mut pixel,
                                pad_pixel,
                                compressed_bpp,
                                bpp,
                            )?;
                            let run_length = read_run_length(&mut reader)?;
                            for _ in 0..run_length {
                                pixels.extend(&pixel)
                            }
                            count += run_length;
                        }
                    }
                    (true, 2..=127) => {
                        // Indexed RLE
                        let mut count = 0;
                        while count < pixel_count {
                            let longer_than_one = reader.read_bit()?;
                            let index = reader.read_bits(7)?;
                            let run_length = if longer_than_one {
                                read_run_length(&mut reader)?
                            } else {
                                1
                            };
                            for _ in 0..run_length {
                                copy_indexed(&palette, &mut pixels, bpp, index);
                            }
                            count += run_length;
                        }
                    }
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "ZRLE subencoding",
                        ))
                    }
                }
                ret.push(ImageData {
                    data: pixels,
                    height,
                    width,
                    x: rect.x + x,
                    y: rect.y + y,
                    type_: ImageType::Raw,
                });
                x += width;
            }
            y += height;
        }

        self.decompressor = Some(reader.into_inner()?.into_inner()?);
        self.padding_len = 0;
        Ok(Some(ret))
    }
}
