//! Service logic
//!
//!  * Join home
//!  * Send CTOS message
//!  * Receive STOC message

mod join_home;

pub async fn service(addr_port: impl AsRef<str> + 'static) -> anyhow::Result<()> {
    let _stream = join_home::handler(addr_port.as_ref()).await?;

    Ok(())
}
