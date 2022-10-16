const axios = require('axios');
const fs = require('fs');
const os = require('os');
const path = require('path');

const CACHE_DIR = '/var/cache/bautista';

class Price {
	constructor(datum) {
		this.hour = datum.hour;
		this.price = datum.price;
	}
}

Price.isTodayCached = function () {
	const file = path.join(CACHE_DIR, `${getFormattedDate()}.json`);

	return fs.existsSync(file);
};

Price.getTodayPrices = async function getPrices() {
	if (!fs.existsSync(CACHE_DIR)) {
		fs.mkdirSync(CACHE_DIR, { recursive: true });
	}

	const file = path.join(CACHE_DIR, `${getFormattedDate()}.json`);

	if (fs.existsSync(file)) {
		return Promise.resolve(
			JSON.parse(fs.readFileSync(file).toString()).map(
				(datum) => new Price(datum)
			)
		);
	}

	const res = await axios.get(
		'https://api.preciodelaluz.org//v1/prices/all?zone=PCB'
	);

	let prices = [];

	Object.keys(res.data).forEach((key) => {
		const hour = Number(key.split('-')[0]);

		prices[hour] = new Price({
			...res.data[key],
			hour,
		});
	});

	fs.writeFileSync(file, JSON.stringify(prices, null, '\t'));

	return prices;
};

module.exports = Price;

function getFormattedDate() {
	const date = new Date();

	let day = date.getDate();

	if (day < 10) {
		day = `0${day}`;
	} else {
		day = day.toString();
	}

	let month = date.getMonth() + 1;

	if (month < 10) {
		month = `0${month}`;
	} else {
		month = month.toString();
	}

	const year = date.getFullYear().toString();

	return `${year}-${month}-${day}`;
}
