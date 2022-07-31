//! Waiting home service logic
use crate::{
    ygo_log,
    ygopro::{self, traits::IntoExdata, *},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use ygopro::{structs::*, utils::*};

const SERVICE: &'static str = "WaitingHome";

pub async fn handler(mut stream: TcpStream, host_info: HostInfo) -> anyhow::Result<TcpStream> {
    // todo: should be multi thread
    let mut buffer = [0; BUFFER_LEN];

    // send update deck packet
    let update_deck = CTOSUpdateDeck {
        inner: deck::Deck::from_path("deck/hero.ydk")?,
    };
    let proto = YGOProto::CTOS(CTOSMsg::UPDATE_DECK);
    let packet = YGOPacket::from_proto(proto, Some(update_deck))?;
    let sent_len = stream.write(&packet.into_bytes()?).await?;
    ygo_log!(
        SERVICE,
        format!("send CTOS UpdateDeck packet len: {}", sent_len)
    );

    let proto = YGOProto::CTOS(CTOSMsg::HS_READY);
    let packet = YGOPacket::from_proto(proto, None::<CTOSEmpty>)?;
    let sent_len = stream.write(&packet.into_bytes()?).await?;
    ygo_log!(
        SERVICE,
        format!("send CTOS HsReady packet len: {}", sent_len)
    );

    loop {
        let recv_len = stream.read(&mut buffer).await?;
        if recv_len > 0 {
            let mut packet = YGOPacket::from_bytes(&buffer)?;
            let stoc = STOCMsg::try_from(packet.proto)?;
            match stoc {
                STOCMsg::HS_PLAYER_ENTER => {
                    let hs_player_enter =
                        STOCHsPlayerEnter::from_exdata(std::mem::take(&mut packet.exdata));

                    ygo_log!(
                        SERVICE,
                        format!(
                            "receive STOC HsPlayerEnter packet, info: {:?}",
                            hs_player_enter
                        )
                    );
                }
                STOCMsg::TYPE_CHANGE => {
                    let type_change =
                        STOCTypeChange::from_exdata(std::mem::take(&mut packet.exdata));

                    ygo_log!(
                        SERVICE,
                        format!("receive STOC TypeChange packet, info: {:?}", type_change)
                    );
                }
                STOCMsg::CHAT => {
                    let chat = STOCChat::from_exdata(std::mem::take(&mut packet.exdata));

                    ygo_log!(
                        SERVICE,
                        format!("receive STOC Chat packet, info: {:?}", chat)
                    );
                }
                x => {
                    ygo_log!(SERVICE, format!("unhandled msg: {:?}", x));
                }
            }
        }
    }
}
