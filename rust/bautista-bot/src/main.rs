use bautista_bot::config::Config;
use bautista_bot::status::Status;
use bautista_bot::telegram::Bot;

fn main() {
    let cfg = Config::new();

    let mut status = Status::new();

    let mut bot = Bot::new(&cfg, &mut status);

    loop {
        let msgs = bot
            .get_new_messages(cfg.bautista.poll_seconds)
            .expect("error getting messages");

        for msg in msgs {
            dbg!(&msg);

            bot.send_message(msg.user_id, &format!("Hola {}", msg.user_name))
                .expect("error sending message");
        }
    }
}
