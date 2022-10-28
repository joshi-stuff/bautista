pub mod commands;
pub mod config;
pub mod devices;
pub mod prices;
pub mod report;
pub mod rules;
pub mod status;
pub mod telegram;

pub type Result<T> = ::core::result::Result<T, Box<dyn std::error::Error>>;

pub use config::{Config, Rule};
pub use status::Status;
