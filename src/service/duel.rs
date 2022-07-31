//! Duel service logic

use crate::{
    ygo_log,
    ygopro::{self, *},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use ygopro::{structs::*, utils::*};

const SERVICE: &'static str = "Duel";

pub async fn handler(mut stream: TcpStream, mut duel: Duel) -> anyhow::Result<TcpStream> {
    todo!()
}
