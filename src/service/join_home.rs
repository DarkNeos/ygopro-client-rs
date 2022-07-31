//! Join home service logic

use crate::{ygo_log, ygopro};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
};
use ygopro::{traits::IntoExdata, utils::*};

const SERVICE: &'static str = "JoinHome";
const VERSION: u16 = 4947;
const FILLING_TOKEN: u16 = 0xcccc;

pub async fn handler(ip_port: &str) -> anyhow::Result<net::TcpStream> {
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
            if let Ok(stoc) = ygopro::STOCMsg::try_from(packet.proto) {
                if stoc == ygopro::STOCMsg::CHAT {
                    let player = unsafe { (packet.exdata.as_ptr() as *const u16).read() };
                    let msg = u8_utf16_buffer_to_str(&packet.exdata[2..]);

                    ygo_log!(
                        SERVICE,
                        format!("receive STOC Chat packet, player: {}, msg: {}", player, msg)
                    );
                } else if stoc == ygopro::STOCMsg::JOIN_GAME {
                    let join_game = unsafe {
                        STOCJoinGame::from_exdata(std::mem::take(&mut &mut packet.exdata))
                    };

                    ygo_log!(
                        SERVICE,
                        format!(
                            "succeed in joining home! host info: {:?}",
                            join_game.host_info
                        )
                    );

                    return Ok(stream);
                }
            }
        }
    }
}

const PLAYER_NAME_MAX_LEN: usize = 20;

struct CTOSPlayerInfo {
    name: [u16; PLAYER_NAME_MAX_LEN], // alias name of player
}

impl CTOSPlayerInfo {
    pub fn new(name: impl AsRef<str>) -> Self {
        let mut s = Self {
            name: [FILLING_TOKEN; PLAYER_NAME_MAX_LEN],
        };

        str_to_utf16_buffer(name, &mut s.name);

        s
    }
}

impl IntoExdata for CTOSPlayerInfo {
    fn into_exdata(self) -> Vec<u8> {
        let len = u16::BITS as usize * PLAYER_NAME_MAX_LEN / 8;
        let exdata = Vec::with_capacity(len);

        unsafe {
            let (ptr, _, _) = exdata.into_raw_parts();

            (ptr as *mut u16).copy_from(self.name.as_ptr(), self.name.len());

            Vec::from_raw_parts(ptr, len, len)
        }
    }
}

const PASS_MAX_LEN: usize = 20;

#[repr(C)]
struct CTOSJoinGame {
    version: u16,              // version of YGOPro client
    gameid: u32,               // always 0
    pass: [u16; PASS_MAX_LEN], // password
}

impl CTOSJoinGame {
    pub fn new(version: u16, passwd: &str) -> Self {
        let mut s = Self {
            version,
            gameid: 0,
            pass: [FILLING_TOKEN; PASS_MAX_LEN],
        };

        str_to_utf16_buffer(passwd, &mut s.pass);

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
                .copy_from(self.pass.as_ptr(), self.pass.len()); // write passwd

            Vec::from_raw_parts(ptr, len as usize, len as usize)
        }
    }
}

#[repr(C)]
struct STOCJoinGame {
    pub host_info: HostInfo,
}

impl STOCJoinGame {
    pub unsafe fn from_exdata(exdata: Vec<u8>) -> Self {
        std::ptr::read(exdata.as_ptr() as *const _)
    }
}

#[repr(C)]
#[derive(Debug)]
struct HostInfo {
    pub lflist: libc::c_uint,
    pub rule: libc::c_uchar,
    pub mode: libc::c_uchar,
    pub duel_rule: libc::c_uchar,
    pub no_check_deck: bool,
    pub no_shuffle_deck: bool,
    pub start_lp: libc::c_uint,
    pub start_hand: libc::c_uchar,
    pub draw_count: libc::c_uchar,
    pub time_limit: libc::c_ushort,
}
