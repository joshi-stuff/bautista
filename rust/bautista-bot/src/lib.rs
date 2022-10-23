pub mod command;
pub mod config;
pub mod device;
pub mod rule;
pub mod status;
pub mod telegram;

pub type Result<T> = ::core::result::Result<T, Box<dyn std::error::Error>>;

pub use config::{Config, Rule};
pub use status::Status;
