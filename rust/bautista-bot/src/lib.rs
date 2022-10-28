pub mod commands;
pub mod config;
pub mod devices;
pub mod prices;
pub mod rules;
pub mod status;
pub mod telegram;
pub mod util;

pub type Result<T> = ::core::result::Result<T, Box<dyn std::error::Error>>;

pub use commands::Commands;
pub use config::Config;
pub use devices::Devices;
pub use rules::Rules;
pub use status::Status;
pub use telegram::Bot;
pub use util::*;
