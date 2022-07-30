//! Utils
#[macro_export]
macro_rules! ygo_log {
    ($service:expr, $msg:expr) => {
        log::info!("ygopro service: {:?}, msg: {:?}", $service, $msg);
    };
}

pub fn str_to_utf16_buffer(s: impl AsRef<str>, v: &mut [u16]) {
    let s = s.as_ref();
    let s = &s[..s.len().min(v.len())];
    let s_utf16 = s.encode_utf16();

    let mut p = 0;
    for c in s_utf16 {
        v[p] = c;
        p += 1;
    }

    if p < v.len() {
        v[p] = 0;
    }
}

pub fn u8_utf16_buffer_to_str(v: &[u8]) -> String {
    let v: Vec<u16> = v
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
        .collect();

    let s = String::from_utf16_lossy(v.as_slice());
    let end = s.find('\0').unwrap_or(s.len());

    s[..end].to_string()
}

pub fn packet_len_min() -> usize {
    (u16::BITS / 8 + u8::BITS / 8) as usize
}
