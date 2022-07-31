pub trait IntoExdata: Send + 'static {
    fn into_exdata(self) -> Vec<u8>;
}

pub trait FromExdata {
    // todo
}
