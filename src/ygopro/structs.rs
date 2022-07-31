use super::{traits::*, utils::*};

const FILLING_TOKEN: u16 = 0xcccc;

#[derive(Debug, Default)]
pub struct Duel {
    pub host_info: HostInfo,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct HostInfo {
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

// ----- CTOS -----

pub struct CTOSEmpty;

impl IntoExdata for CTOSEmpty {
    fn into_exdata(self) -> Vec<u8> {
        vec![]
    }
}

const PLAYER_NAME_MAX_LEN: usize = 20;

pub struct CTOSPlayerInfo {
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
pub struct CTOSJoinGame {
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

#[derive(Debug)]
pub struct CTOSUpdateDeck {
    pub inner: super::deck::Deck,
}

impl IntoExdata for CTOSUpdateDeck {
    fn into_exdata(mut self) -> Vec<u8> {
        let mut v: Vec<i32> = Vec::new();
        v.push(self.inner.main.len() as i32 + self.inner.extra.len() as i32);
        v.push(self.inner.side.len() as i32);
        v.extend(std::mem::take(&mut self.inner.main));
        v.extend(std::mem::take(&mut self.inner.extra));
        v.extend(std::mem::take(&mut self.inner.side));

        unsafe {
            let ratio = std::mem::size_of::<u32>() / std::mem::size_of::<u8>();

            let len = v.len() * ratio;
            let capacity = v.capacity() * ratio;

            let ptr = v.as_mut_ptr() as *mut u8;

            // don't run the destructor for v
            std::mem::forget(v);

            Vec::from_raw_parts(ptr, len, capacity)
        }
    }
}

// ----- STOC -----

#[repr(C)]
pub struct STOCJoinGame {
    pub host_info: HostInfo,
}

impl STOCJoinGame {
    pub unsafe fn from_exdata(exdata: Vec<u8>) -> Self {
        std::ptr::read(exdata.as_ptr() as *const _)
    }
}

#[derive(Debug)]
pub struct STOCChat {
    pub player: u16,
    pub msg: String,
}

impl STOCChat {
    pub fn from_exdata(exdata: Vec<u8>) -> Self {
        let player = unsafe { (exdata.as_ptr() as *const u16).read() };
        let msg = u8_utf16_buffer_to_str(&exdata[2..]);

        Self { player, msg }
    }
}

#[derive(Debug)]
pub struct STOCHsPlayerEnter {
    pub name: String,
    pub pos: libc::c_uchar,
}

impl STOCHsPlayerEnter {
    pub fn from_exdata(exdata: Vec<u8>) -> Self {
        let name = u8_utf16_buffer_to_str(&exdata[..PLAYER_NAME_MAX_LEN * 2]);
        let pos = unsafe {
            exdata
                .as_ptr()
                .offset(PLAYER_NAME_MAX_LEN as isize * 2)
                .read()
        };

        Self { name, pos }
    }
}

#[derive(Debug)]
pub struct STOCTypeChange {
    pub type_: libc::c_uchar,
}

impl STOCTypeChange {
    pub fn from_exdata(exdata: Vec<u8>) -> Self {
        Self { type_: exdata[0] }
    }
}
