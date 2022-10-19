use bautista_bot::config::Config;
use bautista_bot::status::Status;
use bautista_bot::telegram::Bot;

fn main() {
    let cfg = Config::new();

    let mut status = Status::new();

    let mut bot = Bot::new(&cfg, &mut status);

    let msgs = bot.get_new_messages(0).expect("error getting messages");

    dbg!(msgs);
}
