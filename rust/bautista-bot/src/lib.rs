pub mod config;
pub mod meross;
pub mod status;
pub mod telegram;

pub type Result<T> = ::core::result::Result<T, Box<dyn std::error::Error>>;
