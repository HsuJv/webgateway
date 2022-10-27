mod raw;
mod tight;
mod zlib;
mod zrle;
pub use raw::Decoder as RawDecoder;
pub use tight::Decoder as TightDecoder;
pub use zrle::Decoder as ZrleDecoder;
