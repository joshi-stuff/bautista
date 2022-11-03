use bautista_bot::*;
use chrono::{Local, Timelike};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    // Core app objects
    let config = Config::new();
    let mut status = Status::new();

    // Devices and rules
    let mut rules = Rules::new(&config);
    let mut devices = Devices::new(&config);
    let mut last_notification = HashMap::<String, u32>::new();

    // Telegram bot
    let mut bot = Bot::new(&config, &mut status);

    // Command dispatcher
    let commands = Commands::new();

    // Main loop
    loop {
        // Update prices if necessary
        match rules.update_prices() {
            Err(err) => {
                eprintln!("[prices]: Error updating prices: {}", err);

                bot.send_to_admin(&format!(
                    "No he podido actualizar los precios de la luz:\n{}",
                    err
                ));
            }

            Ok(updated) => {
                if updated {
                    bot.broadcast(&format!(
                        "Acabo de actualizar los precios de la luz:\n{}",
                        rules.prices()
                    ));

                    bot.broadcast(&format!(
                        "Los mejores horarios para encender cada dispositivo son:\n{}",
                        format_on_hours(&rules.get_on_hours(0..24))
                    ))
                }
            }
        }

        // Apply rules
        let result = rules.eval(&Local::now());

        devices.update(&result);

        let result = devices.commit();

        for (device, result) in result.iter() {
            match result {
                Err(err) => {
                    if let Some(hour) = last_notification.get(device) {
                        let current_hour = Local::now().hour();

                        if *hour != current_hour {
                            bot.broadcast(&format!(
                                "No he podido controlar el dispositivo {}:\n{:?}",
                                device, err
                            ));

                            last_notification.insert(device.clone(), current_hour);
                        }
                    }
                }

                Ok(on) => {
                    let action = if *on { "encendido" } else { "apagado" };

                    bot.broadcast(&format!("He {} el dispositivo {}", action, device));

                    last_notification.remove(device);
                }
            }
        }

        // Get Telegram messages
        let start = SystemTime::now();

        match bot.get_new_messages(config.bautista.poll_seconds) {
            Err(err) => {
                eprintln!("[telegram] Error getting messages: {}", err);

                let timeout = config.bautista.poll_seconds as u64;
                let elapsed = start
                    .elapsed()
                    .expect("Failed to obtain elapsed time")
                    .as_secs();

                if elapsed < timeout {
                    sleep(Duration::from_secs(timeout - elapsed));
                }
            }

            Ok(msgs) => {
                // Process Telegram messages
                for msg in msgs {
                    if !config.telegram.allowed_users.contains(&msg.user_id)
                        && msg.user_id != config.telegram.admin_user
                    {
                        bot.send_to_admin(&format!(
                            "\
He recibido un mensaje de un usuario desconocido:

{}

  - {} [{}]
",
                            &msg.text, &msg.user_name, &msg.user_id
                        ));
                    }

                    eprintln!(
                        "[telegram] {} ({}): {}",
                        &msg.user_name, &msg.user_id, &msg.text
                    );

                    if let Some(reply) = commands.run(&msg, &rules) {
                        eprintln!("[telegram] Replying to {}: {}", msg.user_id, &reply);

                        bot.send_message(msg.user_id, &reply);
                    }
                }
            }
        }
    }
}
