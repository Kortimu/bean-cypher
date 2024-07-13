mod app;
pub use app::BeanCypher;

#[path = "decode.rs"]
mod decode;
#[path = "encode.rs"]
mod encode;
mod hash_convert;

#[derive(Debug, Clone)]
pub enum ErrorState {
    Error(String),
    Warning(String),
    None,
}

impl ErrorState {
    fn into_string(self) -> String {
        match self {
            Self::Warning(s) | Self::Error(s) => s,
            Self::None => String::new(),
        }
    }
}
