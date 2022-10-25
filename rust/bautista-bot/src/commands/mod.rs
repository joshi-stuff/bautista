use crate::telegram::Message;
use crate::*;

mod tides;

pub use tides::TidesCommand;

pub trait Command {
    /**
     * Return None if command does not apply or a String to return to the user
     * instead
     */
    fn run(&self, msg: &Message) -> Result<Option<String>>;
}

pub struct Commands {
    cmds: Vec<Box<dyn Command>>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            cmds: vec![Box::new(TidesCommand::new())],
        }
    }

    pub fn run(&self, msg: &Message) -> Option<String> {
        for cmd in &self.cmds {
            match cmd.run(msg) {
                Err(err) => {
                    return Some(format!("Ocurrió un error:\n{}", &err));
                }

                Ok(result) => {
                    if let Some(reply) = result {
                        return Some(reply);
                    }
                }
            }
        }

        None
    }
}
