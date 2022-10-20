use bautista_bot::command::*;
use bautista_bot::meross::MerossBridge;
use bautista_bot::telegram::Bot;
use bautista_bot::*;

fn main() {
    let cfg = Config::new();
    let mut status = Status::new();

    let mut bot = Bot::new(&cfg, &mut status);
    let mut bridge = MerossBridge::new(&cfg).expect("error spawning meross-bridge");

    loop {
        let msgs = bot
            .get_new_messages(cfg.bautista.poll_seconds)
            .expect("error getting messages");

        for msg in msgs {
            dbg!(&msg);

            // TODO 1: multiplex commands
            // TODO 2: implement TIDES command

            bridge
                .send_text(&msg.text)
                .expect("error writing to meross-bridge");

            let reply = bridge
                .get_reply()
                .expect("error reading from meross-bridge");

            bot.send_message(msg.user_id, &format!("{}", &reply))
                .expect("error sending message");
        }
    }
}
