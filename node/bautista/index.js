const Bot = require('./Bot.js');
const Config = require('./Config.js');
const Device = require('./Device.js');
const Price = require('./Price.js');

async function main() {
	const config = Config.read();

	const devices = config.devices;

	const bot = new Bot(
		config.telegram.token,
		config.telegram.adminUser,
		config.telegram.allowedUsers
	);

	const schedules = {};

	let lastControlHour;

	try {
		await bot.connect();

		await say(bot, 'Hola! Ya estoy de vuelta 👋');

		await Device.connectDevices(config.meross, devices, (...args) =>
			deviceEventHandler(bot, ...args)
		);
	} catch (err) {
		await say(bot, 'Uf! Algo ha ido mal... 🤕');
		await say(bot, err);

		process.exit(1);
	}

	let sleepPromise, getMessagesPromise;

	while (true) {
		if (await updateSchedules(schedules, devices)) {
			await say(
				bot,
				`Acabo de actualizar los precios de la luz 🎉

Las horas más baratas para cada dispositivo son:

${formatSchedules(schedules, devices)}`
			);
		}

		lastControlHour = await controlDevices(
			devices,
			schedules,
			lastControlHour,
			bot
		);

		if (!sleepPromise) {
			sleepPromise = sleep(config.control.period);
		}

		if (!getMessagesPromise) {
			getMessagesPromise = bot.getMessages();
		}

		const messages = await Promise.race([sleepPromise, getMessagesPromise]);

		if (messages) {
			await processMessages(messages, devices, schedules, bot);

			getMessagesPromise = bot.getMessages();
		} else {
			sleepPromise = sleep(config.control.period);
		}
	}
}

async function controlDevices(devices, schedules, lastControlHour, bot) {
	const now = new Date();
	const hour = now.getHours();

	if (hour == lastControlHour) {
		return lastControlHour;
	}

	for (const dev of devices) {
		if (!dev.controlled) {
			continue;
		}

		try {
			const shouldBeOn = schedules[dev.name].onAt[hour];

			await dev.update_status();

			if (shouldBeOn && !dev.status.on) {
				dev.toggle(true);
				await say(
					bot,
					`Acabo de encender el dispositivo ${dev.name} 💡`
				);
			} else if (!shouldBeOn && dev.status.on) {
				dev.toggle(false);
				await say(bot, `Acabo de apagar el dispositivo ${dev.name} 🔌`);
			}
		} catch (error) {
			await say(
				bot,
				`
No he podido controlar el dispositivo ${dev.name}.

Ocurrió un error: ${error}`
			);
		}
	}
}

async function deviceEventHandler(bot, event, dev, error) {
	if (dev) {
		switch (event) {
			case 'connected': {
				await say(bot, `Se ha conectado el dispositivo ${dev.name} 📡`);

				break;
			}

			case 'disconnected': {
				if (error) {
					await say(
						bot,
						`
Se ha desconectado el dispositivo ${dev.name}.

Ocurrió un error: ${error}`
					);
				} else {
					await say(
						bot,
						`Se ha desconectado el dispositivo ${dev.name} 👻`
					);
				}

				break;
			}

			case 'errored': {
				await say(
					bot,
					`Ocurrió un error en el dispositivo ${dev.name} ❌:\n${error}`
				);

				break;
			}
		}
	} else {
		switch (event) {
			case 'errored': {
				await say(bot, `Ocurrió un error de conexión ❌: ${error}`);

				break;
			}
		}
	}
}

function formatSchedules(schedules, devices) {
	let msg = '';

	devices.forEach((dev) => {
		const schedule = schedules[dev.name];

		msg += `    · ${dev.name}:`;

		for (let hour = 0; hour < 24; hour++) {
			if (schedule.onAt[hour]) {
				msg += ` ${hour}:00`;
			}
		}

		msg += '\n';
	});

	return msg;
}

async function processMessages(messages, devices, schedules, bot) {
	for (const userid of Object.keys(messages)) {
		const realName = bot.getRealName(userid);

		for (let message of messages[userid]) {
			message = message.toLowerCase();

			if (message.indexOf('apagar') !== -1) {
				// TODO: apagar
			} else if (message.indexOf('dispositivos') !== -1) {
				await bot.send(
					`
${realName}, los dispositivos que puedo controlar son:

${devices
	.filter((dev) => dev.controlled)
	.map((dev) => `    · ${dev.name}`)
	.join('\n')}

Y los dispositivos para los que únicamente puedo calcular los horarios más baratos son:

${devices
	.filter((dev) => !dev.controlled)
	.map((dev) => `    · ${dev.name}`)
	.join('\n')}
`,
					userid
				);
			} else if (message.indexOf('encender') !== -1) {
				// TODO: encender
			} else if (message.indexOf('hola') !== -1) {
				await bot.send(`Hola ${realName} 👋`, userid);
			} else if (message.indexOf('horario') !== -1) {
				await bot.send(
					`
Las horas más baratas para cada dispositivo son:

${formatSchedules(schedules, devices)}`,
					userid
				);
			}
		}
	}
}

async function say(bot, ...things) {
	console.log(...things);
	console.log('---');

	await bot.send(things.join(' '));
}

async function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function updateSchedules(schedules, devices) {
	if (Price.isTodayCached() && Object.keys(schedules).length) {
		return false;
	}

	for (const key of Object.keys(schedules)) {
		delete schedules[key];
	}

	const prices = await Price.getTodayPrices();

	for (const dev of devices) {
		schedules[dev.name] = dev.rules.getSchedule(prices);
	}

	return true;
}

main();
