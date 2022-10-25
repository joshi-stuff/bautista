use bautista_bot::commands::*;
use bautista_bot::devices::Devices;
use bautista_bot::prices::Prices;
use bautista_bot::rules::*;
use bautista_bot::telegram::Bot;
use bautista_bot::*;
use chrono::Local;

fn main() {
    // Core app objects
    let cfg = Config::new();
    let mut status = Status::new();

    // Telegram bot and commands
    let mut bot = Bot::new(&cfg, &mut status);
    let commands: Vec<Box<dyn Command>> = vec![Box::new(TideCommand::new())];

    // Devices and rules
    let mut rules = Rules::new(&cfg);
    let mut devices = Devices::new(&cfg);

    // Power prices
    let mut prices = Prices::new(&cfg);

    // Main loop
    loop {
        // Update prices if necessary
        match prices.update() {
            Err(err) => {
                bot.broadcast(&format!(
                    "No he podido actualizar los precios de la luz:\n{}",
                    err
                ));
            }

            Ok(updated) => {
                if updated {
                    // Update prices
                    rules.update_prices(&prices);

                    // Broadcast message
                    bot.broadcast(&format!(
                        "Acabo de actualizar los precios de la luz:\n{}",
                        prices
                    ));

                    // Broadcast non-controlled devices ON hours
                    let mut report = String::new();

                    let on_hours = rules.get_uncontrolled_on_hours();

                    for (device, on_hours) in on_hours {
                        report.push_str(&format!("  · {}:", device));

                        for hour in on_hours {
                            report.push_str(&format!(" {}:00 ", hour));
                        }

                        report.push_str("\n");
                    }

                    bot.broadcast(&format!(
                        "Los mejores horarios para encender cada dispositivo son:\n{}",
                        report
                    ))
                }
            }
        }

        // Apply rules and remove consumed ones
        let now = Local::now();

        let result = rules.eval(&now);

        devices.update(&result);

        let result = devices.commit();

        for (device, result) in result.iter() {
            match result {
                Err(err) => {
                    bot.broadcast(&format!(
                        "No he podido controlar el dispositivo {}:\n{:?}",
                        device, err
                    ));
                }

                Ok(on) => {
                    let action = if *on { "encendido" } else { "apagado" };

                    bot.broadcast(&format!("He {} el dispositivo {}", action, device));
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

                        //dbg!(&reply);

                        bot.send_message(msg.user_id, &reply);
                    }

                    Ok(result) => {
                        if let Some(reply) = result {
                            dbg!(&reply);

                            bot.send_message(msg.user_id, &reply);
                        }
                    }
                }
            }
        }
    }
}
