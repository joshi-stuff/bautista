use crate::*;
use std::io::{BufRead, BufReader, Lines, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

pub struct MerossBridge {
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

impl MerossBridge {
    pub fn new(config: &Config) -> Result<MerossBridge> {
        let mut bridge = Command::new(&config.meross.bridge_path)
            .env("MEROSS_USER", &config.meross.user)
            .env("MEROSS_PASSWORD", &config.meross.password)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("error launching meross-bridge");

        let stdin = bridge
            .stdin
            .take()
            .expect("failed to open meross-bridge stdin");

        let stdout = BufReader::new(
            bridge
                .stdout
                .take()
                .expect("failed to open meross-bridge stdout"),
        )
        .lines();

        Ok(MerossBridge { stdin, stdout })
    }

    pub fn get_reply(&mut self) -> Result<String> {
        loop {
            let line = self.stdout.next().unwrap()?;

            dbg!(&line);

            if line.starts_with(">") {
                return Ok(line);
            }
        }
    }

    pub fn send_text(&mut self, text: &str) -> Result<()> {
        self.stdin.write_all(format!("{}\n", &text).as_bytes())?;

        Ok(())
    }
}
