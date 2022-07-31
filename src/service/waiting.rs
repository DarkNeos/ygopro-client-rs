//! Waiting home service logic
use crate::{
    ygo_log,
    ygopro::{self, *},
};
use tokio::{io::AsyncReadExt, net::TcpStream};
use ygopro::{structs::*, traits::IntoExdata, utils::*};

const SERVICE: &'static str = "WaitingHome";

pub async fn handler(mut stream: TcpStream, host_info: HostInfo) -> anyhow::Result<TcpStream> {
    // todo: 这里应该使用多线程并发处理，由于此项目属于demo性质，
    // 因此使用单线程循环简单处理
    let mut buffer = [0; BUFFER_LEN];

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
                _ => {}
            }
        }
    }
}
