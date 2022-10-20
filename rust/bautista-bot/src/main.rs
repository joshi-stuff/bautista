use bautista_bot::config::Config;
use bautista_bot::status::Status;
use bautista_bot::telegram::Bot;
use bautista_bot::Result;
use std::io::{BufRead, BufReader, Lines, Write};
use std::process::{ChildStdout, Command, Stdio};

fn main() {
    let cfg = Config::new();

    let mut status = Status::new();

    let mut bot = Bot::new(&cfg, &mut status);

    let mut bridge = Command::new(&cfg.meross.bridge_path)
        .env("MEROSS_USER", &cfg.meross.user)
        .env("MEROSS_PASSWORD", &cfg.meross.password)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("error launching meross-bridge");

    let mut brin = bridge
        .stdin
        .take()
        .expect("failed to open meross-bridge stdin");

    let mut brout = BufReader::new(
        bridge
            .stdout
            .take()
            .expect("failed to open meross-bridge stdout"),
    )
    .lines();

    loop {
        let msgs = bot
            .get_new_messages(cfg.bautista.poll_seconds)
            .expect("error getting messages");

        for msg in msgs {
            dbg!(&msg);

            brin.write_all(format!("{}\n", &msg.text).as_bytes())
                .expect("error writing to meross-bridge");

            let reply = get_reply(&mut brout).expect("error reading from meross-bridge");

            bot.send_message(msg.user_id, &format!("{}", &reply))
                .expect("error sending message");
        }
    }
}

fn get_reply(brout: &mut Lines<BufReader<ChildStdout>>) -> Result<String> {
    loop {
        let line = brout.next().unwrap()?;

        dbg!(&line);

        if line.starts_with(">") {
            return Ok(line);
        }
    }
}
