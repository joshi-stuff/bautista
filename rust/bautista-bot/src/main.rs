use bautista_bot::command::*;
use bautista_bot::meross::MerossBridge;
use bautista_bot::telegram::Bot;
use bautista_bot::*;

fn main() {
    let cfg = Config::new();
    let mut status = Status::new();

    let mut bot = Bot::new(&cfg, &mut status);
    let bridge = MerossBridge::new(&cfg).expect("error spawning meross-bridge");

    let commands: Vec<Box<dyn Command>> = vec![Box::new(TideCommand::new())];

    loop {
        let msgs = bot
            .get_new_messages(cfg.bautista.poll_seconds)
            .expect("error getting messages");

        for msg in msgs {
            dbg!(&msg);

            for cmd in commands.iter() {
                match cmd.run(&msg) {
                    Err(err) => {
                        let reply = format!("Ocurrió un error:\n{}", &err);

                        dbg!(&reply);

                        bot.send_message(msg.user_id, &reply)
                            .expect("error sending message");
                    }

                    Ok(result) => {
                        if let Some(reply) = result {
                            dbg!(&reply);

                            bot.send_message(msg.user_id, &reply)
                                .expect("error sending message");
                        }
                    }
                }
            }

            /*
            bridge
                .send_text(&msg.text)
                .expect("error writing to meross-bridge");

            let reply = bridge
                .get_reply()
                .expect("error reading from meross-bridge");
            */
        }
    }
}
