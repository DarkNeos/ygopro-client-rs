//! Utils
use super::{STOCMsg, YGOPacket};

use tokio::io::AsyncReadExt;
#[macro_export]
macro_rules! ygo_log {
    ($service:expr, $msg:expr) => {
        log::info!("ygopro service: {:?}, msg: {:?}", $service, $msg);
    };
}

pub const BUFFER_LEN: usize = 0x1000;

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

pub const fn packet_len_min() -> usize {
    (u16::BITS / 8 + u8::BITS / 8) as usize
}

pub async fn try_recv_from_ygopro_server(
    stream: &mut tokio::net::TcpStream,
    buffer: &mut [u8],
    service: impl AsRef<str>,
) -> anyhow::Result<()> {
    let recv_len = stream.read(buffer).await?;

    if recv_len > 0 {
        let packet = YGOPacket::from_bytes(buffer)?;
        ygo_log!(
            service.as_ref(),
            format!(
                "packet_len: {}, proto: {:?}, exdata: {:?}",
                packet.packet_len,
                STOCMsg::try_from(packet.proto)?,
                packet.exdata
            )
        );
    }

    Ok(())
}
