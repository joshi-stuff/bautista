use bautista_bot::command::*;
use bautista_bot::device::DeviceStatus;
use bautista_bot::rule::*;
use bautista_bot::telegram::Bot;
use bautista_bot::*;

fn main() {
    // Core app objects
    let cfg = Config::new();
    let mut status = Status::new();

    // Telegram bot and commands
    let mut bot = Bot::new(&cfg, &mut status);
    let commands: Vec<Box<dyn Command>> = vec![Box::new(TideCommand::new())];

    // Devices and rules
    let mut rules = DeviceRules::new(&cfg);
    let mut devices = DeviceStatus::new(&cfg);

    // Report initial state
    eprintln!("{:?}", &rules);

    // Main loop
    loop {
        // Apply rules and remove consumed ones
        let result = rules.eval();

        devices.update(&result);

        let result = devices.commit();

        for (device, result) in result.iter() {
            match result {
                Err(err) => {
                    bot.send_message(
                        cfg.telegram.admin_user,
                        &format!(
                            "No he podido controlar el dispositivo {}:\n{:?}",
                            device, err
                        ),
                    )
                    .expect("error sending message");
                }

                Ok(on) => {
                    let action = if *on { "encendido" } else { "apagado" };

                    bot.send_message(
                        cfg.telegram.admin_user,
                        &format!("He {} el dispositivo {}", action, device),
                    )
                    .expect("error sending message");
                }
            }
        }

        rules.remove_consumed();

        // Get Telegram messages
        let msgs = bot
            .get_new_messages(cfg.bautista.poll_seconds)
            .expect("error getting messages");

        // Process Telegram messages
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
        }
    }
}
