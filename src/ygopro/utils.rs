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

pub fn packet_len_min() -> usize {
    (u16::BITS / 8 + u8::BITS / 8) as usize
}
