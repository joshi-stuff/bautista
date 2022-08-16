const Bot = require('./Bot.js');
const Device = require('./Device.js');
const getCheapestPrices = require('./getCheapestPrices.js');
const getPrices = require('./getPrices.js');

const CONTROL_PERIOD = 58000;
const MEROSS = {
  user: "******************",
  password: "******************",
};

const DEVICES = [
	new Device('Pruebas', 3),
/*
	{
		hours: 3,
		name: 'Calentador'
	},
*/
];

async function main() {
	let lastControlHour;

	const bot = new Bot();

	await bot.connect();

	console.log('🤖 Bautista bot connected');

	bot.send('Hi there! I\'m online again 👋');

	await Device.connectDevices(MEROSS, DEVICES);

	while (true) {
		if (await fillDevicePrices(DEVICES)) {
			console.log('💲 Prices updated');

			bot.send('I\'ve just updated the prices');

			for (const dev of DEVICES) {
				const hours = dev.prices.map(price => price.hour).join(', ');

				console.log(`    Device ${dev.name} on hours: ${hours}`);
			}
		}
		
		lastControlHour = await controlDevices(DEVICES, lastControlHour, bot);

		await sleep(CONTROL_PERIOD);
	}
}

async function controlDevices(devices, lastControlHour, bot) {
	console.log('🧠 Controlling devices...');

	const now = new Date();
	const hour = now.getHours();

	if (hour == lastControlHour) {
		return lastControlHour;
	}

	for (const dev of devices) {
		const price = dev.prices.find(price => price.hour == hour);

		if (price) {
			dev.toggle(true);
			console.log(`    💡 Device turned on: ${dev.name}`);
		}
		else {
			dev.toggle(false);
			console.log(`    🔌 Device turned off: ${dev.name}`);
		}
	}
}

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// TODO: move to Device.updatePrices()
async function fillDevicePrices(devices) {
	if (getPrices.isCached() && devices[0].prices) {
		return false;
	}

	const prices = await getPrices();

	for (const dev of devices) {
		dev.prices = getCheapestPrices(prices, dev.hours);	
	}

	return true;
}

main();
