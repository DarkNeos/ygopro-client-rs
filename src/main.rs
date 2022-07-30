#![feature(vec_into_raw_parts)]
#![feature(slice_take)]

mod service;
mod ygopro;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;

    rt.block_on(async move {
        if let Err(e) = tokio::spawn(service::service("127.0.0.1:3344")).await {
            log::error!("service error: {:?}", e);
        }
    });
    Ok(())
}
