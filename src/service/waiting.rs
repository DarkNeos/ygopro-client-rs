//! Waiting home service logic
use crate::{ygo_log, ygopro};
use tokio::net::TcpStream;
use ygopro::{traits::IntoExdata, utils::*};

const SERVICE: &'static str = "WaitingHome";

pub async fn handler(mut stream: TcpStream) -> anyhow::Result<TcpStream> {
    let mut buffer = [0; BUFFER_LEN];

    try_recv_from_ygopro_server(&mut stream, &mut buffer, SERVICE).await?;

    Ok(stream)
}
