//! Service logic
//!
//!  * Join home
//!  * Send CTOS message
//!  * Receive STOC message

mod join_home;
mod waiting;

pub async fn service(addr_port: impl AsRef<str> + 'static) -> anyhow::Result<()> {
    let (stream, host_info) = join_home::handler(addr_port.as_ref()).await?;

    let _stream = waiting::handler(stream, host_info).await?;

    Ok(())
}
