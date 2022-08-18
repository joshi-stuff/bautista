const Bot = require('./Bot.js');
const Config = require('./Config.js');
const Device = require('./Device.js');
const Price = require('./Price.js');

async function main() {
	const config = Config.read();

	const devices = config.devices;

	const bot = new Bot(config.telegram.token);

	try {
		await bot.connect();

		say(bot, 'Hola! Ya estoy de vuelta 👋');

		await Device.connectDevices(config.meross, devices, (...args) =>
			deviceEventHandler(bot, ...args)
		);
	} catch (err) {
		say(bot, 'Uf! Algo ha ido mal... 🤕');
		say(bot, err);

		process.exit(1);
	}

	let lastControlHour;
	const schedules = {};

	while (true) {
		if (await updateSchedules(schedules, devices)) {
			say(
				bot,
				`Acabo de actualizar los precios de la luz 🎉

Las horas mas baratas para cada dispositivo son:

${formatSchedules(schedules, devices)}
`
			);

			lastControlHour = await controlDevices(
				devices,
				schedules,
				lastControlHour,
				bot
			);

			await sleep(config.control.period);
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

		const shouldBeOn = schedules[dev.name].onAt[hour];

		await dev.update_status();

		if (shouldBeOn && !dev.status.on) {
			dev.toggle(true);
			say(bot, `Acabo de encender 💡 el dispositivo ${dev.name}`);
		} else if (!shouldBeOn && dev.status.on) {
			dev.toggle(false);
			say(bot, `Acabo de apagar 🔌 el dispositivo ${dev.name}`);
		}
	}
}

function deviceEventHandler(bot, event, dev, error) {
	if (dev) {
		switch (event) {
			case 'connected': {
				say(bot, `Se ha conectado el dispositivo ${dev.name}`);

				break;
			}

			case 'disconnected': {
				if (error) {
					say(
						bot,
						`
Se ha desconectado el dispositivo ${dev.name}.
Ocurrió un error: ${error}`
					);
				} else {
					say(bot, `Se ha desconectado el dispositivo ${dev.name}`);
				}

				break;
			}

			case 'errored': {
				say(
					bot,
					`Ocurrió un error en el dispositivo ${dev.name}: ${error}`
				);

				break;
			}
		}
	} else {
		switch (event) {
			case 'errored': {
				say(bot, `Ocurrió un error de conexión: ${error}`);

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

function say(bot, ...things) {
	console.log(...things);
	console.log('---');

	bot.send(things.join(' '));
}

async function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function updateSchedules(schedules, devices) {
	if (Price.isTodayCached() && schedules.length) {
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
