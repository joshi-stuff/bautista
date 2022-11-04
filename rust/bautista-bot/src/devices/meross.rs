use crate::*;
use std::io;
use std::io::{BufRead, BufReader, Lines, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};
use std::thread;
use std::time::Duration;
use thiserror::Error;

const THREE_SECONDS: Duration = Duration::from_secs(3);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Spawn failed for meross-bridge")]
    SpawnFailed(#[source] Option<io::Error>),
    #[error("I/O failed while communicating with meross-bridge")]
    IOFailed(#[source] Option<io::Error>),
}

pub struct MerossBridge {
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

impl MerossBridge {
    pub fn new(config: &Config) -> Result<MerossBridge, Error> {
        let bridge = Command::new(&config.meross.bridge_path)
            .env("MEROSS_USER", &config.meross.user)
            .env("MEROSS_PASSWORD", &config.meross.password)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .or_else(|err| Err(Error::SpawnFailed(Some(err))))?;

        let stdin = bridge.stdin.ok_or(Error::SpawnFailed(None))?;

        let stdout = bridge.stdout.ok_or(Error::SpawnFailed(None))?;

        let stdout = BufReader::new(stdout).lines();

        // Wait for bridge to settle
        thread::sleep(THREE_SECONDS);

        Ok(MerossBridge { stdin, stdout })
    }

    pub fn get_reply(&mut self) -> Result<String, Error> {
        loop {
            let line = self.stdout.next().ok_or(Error::IOFailed(None))?;

            let line = line.or_else(|err| Err(Error::IOFailed(Some(err))))?;

            //dbg!(&line);

            if line.starts_with(">") {
                return Ok(line);
            }
        }
    }

    pub fn send_text(&mut self, text: &str) -> Result<(), Error> {
        self.stdin
            .write_all(format!("{}\n", &text).as_bytes())
            .or_else(|err| Err(Error::IOFailed(Some(err))))?;

        Ok(())
    }
}
