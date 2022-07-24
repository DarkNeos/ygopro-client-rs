//! Join home service logic

use crate::{ygo_log, ygopro};
use tokio::{io::AsyncWriteExt, net};
use ygopro::traits::IntoExdata;

const SERVICE: &'static str = "JoinHome";

struct CTOSPlayerInfo {
    pub name: Vec<u8>,
}

impl CTOSPlayerInfo {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();

        Self {
            name: name.into_bytes(),
        }
    }
}

impl IntoExdata for CTOSPlayerInfo {
    fn into_exdata(self) -> Vec<u8> {
        self.name
    }
}

pub async fn handler(ip_port: &str) -> anyhow::Result<net::TcpStream> {
    let mut stream = net::TcpStream::connect(ip_port).await?;
    ygo_log!(
        SERVICE,
        format!("Connection to {} [tcp] succeeded!", ip_port)
    );

    let player_info = CTOSPlayerInfo::new("sktt1ryze");
    let proto = ygopro::YGOProto::CTOS(ygopro::CTOSMsg::PLAYER_INFO);

    let packet = ygopro::YGOPacket::from_proto(proto, player_info)?;

    let sent_len = stream.write(&packet.into_bytes()?).await?;
    ygo_log!(
        SERVICE,
        format!("send CTOS PlayerInfo packet len: {}", sent_len)
    );

    Ok(stream)
}
