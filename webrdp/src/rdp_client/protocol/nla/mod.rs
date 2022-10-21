mod cssp;
mod ntlm;
mod rc4;

pub use cssp::CsspClient as Nla;

fn from_unicode(buf: &[u8]) -> String {
    String::from_utf16_lossy(
        buf.chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_le_bytes([a[0], a[1]]))
            .collect::<Vec<u16>>()
            .as_slice(),
    )
}

fn to_unicode(s: &str) -> Vec<u8> {
    s.encode_utf16()
        .collect::<Vec<u16>>()
        .into_iter()
        .map(u16::to_le_bytes)
        .flat_map(IntoIterator::into_iter)
        .collect::<Vec<u8>>()
}
