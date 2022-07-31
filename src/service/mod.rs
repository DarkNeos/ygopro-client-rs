//! Service logic
//!
//!  * Join home
//!  * Send CTOS message
//!  * Receive STOC message

mod join_home;
mod waiting;

pub async fn service(addr_port: impl AsRef<str> + 'static) -> anyhow::Result<()> {
    let stream = join_home::handler(addr_port.as_ref()).await?;

    let _stream = waiting::handler(stream).await?;

    Ok(())
}
