//! Join home service logic

use crate::ygo_log;
use tokio::net;

pub async fn handler(ip_port: &str) -> anyhow::Result<net::TcpStream> {
    let stream = net::TcpStream::connect(ip_port).await?;
    ygo_log!(
        "JoinHome",
        format!("Connection to {} [tcp] succeeded!", ip_port)
    );

    Ok(stream)
}
