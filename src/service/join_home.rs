//! Join home service logic

use crate::{ygo_log, ygopro};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
};
use ygopro::traits::IntoExdata;

const SERVICE: &'static str = "JoinHome";
const VERSION: u16 = 4947;
const BUFFER_LEN: usize = 0x100;

pub async fn handler(ip_port: &str) -> anyhow::Result<net::TcpStream> {
    let mut stream = net::TcpStream::connect(ip_port).await?;
    ygo_log!(
        SERVICE,
        format!("Connection to {} [tcp] succeeded!", ip_port)
    );

    let mut buffer = [0; BUFFER_LEN];

    let player_info = CTOSPlayerInfo::new("sktt1ryze");
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

    let recv_len = stream.read(&mut buffer).await?;
    if recv_len > 0 {
        ygo_log!(
            SERVICE,
            format!("receive from ygopro server len: {}", recv_len)
        );
    }

    Ok(stream)
}

struct CTOSPlayerInfo {
    pub name: Vec<u8>, // alias name of player
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

const PASS_MAX_LEN: usize = 20;
const PASS_FILLING_TOKEN: u16 = 0xcccc;

#[repr(C)]
struct CTOSJoinGame {
    version: u16,              // version of YGOPro client
    gameid: u32,               // always 0
    pass: [u16; PASS_MAX_LEN], // password
}

impl CTOSJoinGame {
    pub fn new(version: u16, passwd: &str) -> Self {
        let passwd = &passwd[..passwd.len().min(PASS_MAX_LEN)];
        let passwd_iter = passwd.encode_utf16();

        let mut s = Self {
            version,
            gameid: 0,
            pass: [PASS_FILLING_TOKEN; PASS_MAX_LEN],
        };

        let mut p = 0;
        for c in passwd_iter {
            s.pass[p] = c;
            p += 1;
        }

        if p < PASS_MAX_LEN {
            s.pass[p] = 0;
        }

        s
    }
}

impl IntoExdata for CTOSJoinGame {
    fn into_exdata(self) -> Vec<u8> {
        let len = u16::BITS / 8 + u32::BITS / 8 + u16::BITS * PASS_MAX_LEN as u32 / 8;
        let exdata = Vec::with_capacity(len as usize);

        unsafe {
            let (ptr, _, _) = exdata.into_raw_parts();

            *(ptr as *mut u16) = self.version; // write version

            (ptr as *mut u32).offset(1).write(self.gameid); // write gameid

            (ptr as *mut u16)
                .offset(4)
                .copy_from(self.pass.as_ptr(), PASS_MAX_LEN); // write passwd

            Vec::from_raw_parts(ptr, len as usize, len as usize)
        }
    }
}
