//! Join home service logic

use crate::{ygo_log, ygopro};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
};
use ygopro::{structs::*, utils::*};

const SERVICE: &'static str = "JoinHome";
const VERSION: u16 = 4947;

pub async fn handler(ip_port: &str) -> anyhow::Result<(net::TcpStream, HostInfo)> {
    let mut stream = net::TcpStream::connect(ip_port).await?;
    ygo_log!(
        SERVICE,
        format!("Connection to {} [tcp] succeeded!", ip_port)
    );

    let mut buffer = [0; BUFFER_LEN];

    let player_info = CTOSPlayerInfo::new("sktt1faker");
    let proto = ygopro::YGOProto::CTOS(ygopro::CTOSMsg::PLAYER_INFO);
    let packet = ygopro::YGOPacket::from_proto(proto, player_info)?;
    let sent_len = stream.write(&packet.into_bytes()?).await?;
    ygo_log!(
        SERVICE,
        format!("send CTOS PlayerInfo packet len: {}", sent_len)
    );

    let join_game = CTOSJoinGame::new(VERSION, "TM999#ccc");
    let proto = ygopro::YGOProto::CTOS(ygopro::CTOSMsg::JOIN_GAME);
    let packet = ygopro::YGOPacket::from_proto(proto, join_game)?;
    let sent_len = stream.write(&packet.into_bytes()?).await?;
    ygo_log!(
        SERVICE,
        format!("send CTOS JoinGame packet len: {}", sent_len)
    );

    loop {
        let recv_len = stream.read(&mut buffer).await?;
        if recv_len > 0 {
            let mut packet = ygopro::YGOPacket::from_bytes(&buffer)?;
            let stoc = ygopro::STOCMsg::try_from(packet.proto)?;
            if stoc == ygopro::STOCMsg::CHAT {
                let chat = STOCChat::from_exdata(std::mem::take(&mut packet.exdata));

                ygo_log!(
                    SERVICE,
                    format!("receive STOC Chat packet, chat: {:?}", chat)
                );
            } else if stoc == ygopro::STOCMsg::JOIN_GAME {
                let join_game =
                    unsafe { STOCJoinGame::from_exdata(std::mem::take(&mut &mut packet.exdata)) };

                ygo_log!(
                    SERVICE,
                    format!(
                        "succeed in joining home! host info: {:?}",
                        join_game.host_info
                    )
                );

                return Ok((stream, join_game.host_info));
            }
        }
    }
}
