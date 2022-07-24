//! Utils

#[macro_export]
macro_rules! ygo_log {
    ($service:expr, $msg:expr) => {
        log::info!("ygopro service: {:?}, msg: {:?}", $service, $msg);
    };
}
