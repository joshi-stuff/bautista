use crate::telegram::Message;
use crate::*;

mod tide;

pub trait Command {
    /**
     * Return None if command does not apply or a String to return to the user
     * instead
     */
    fn run(&self, msg: &Message) -> Result<Option<String>>;
}

pub use tide::TideCommand;
