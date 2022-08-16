const axios = require('axios');
const fs = require('fs');
const os = require('os');
const path = require('path');

const CACHE_DIR = path.join(os.homedir(), '.cache', 'bautista');

if (!fs.existsSync(CACHE_DIR)) {
	fs.mkdirSync(CACHE_DIR, {recursive: true});
}

function getFormattedDate() {
	const date = new Date();

	let day = date.getDate();

	if (day < 10) {
		day = `0${day}`;
	} else {
		day = day.toString();
	}

	let month = date.getMonth()+1;

	if (month < 10) {
		month = `0${month}`;
	} else {
		month = month.toString();
	}

	const year = date.getFullYear().toString();

	return `${year}-${month}-${day}`;
}

async function getPrices() {
	const file = path.join(CACHE_DIR, `${getFormattedDate()}.json`);

	if (fs.existsSync(file)) {
		return Promise.resolve(JSON.parse(fs.readFileSync(file).toString()));
	}

	const res = await axios
		.get('https://api.preciodelaluz.org//v1/prices/all?zone=PCB');

	let data = [];

	Object.keys(res.data).forEach(key => {
		const hour = Number(key.split('-')[0]);

		data[hour] = res.data[key];
		data[hour].hour = hour;

	});

	fs.writeFileSync(file, JSON.stringify(data, null, '\t'));

	return data;
}

getPrices.isCached = function() {
	const file = path.join(CACHE_DIR, `${getFormattedDate()}.json`);

	return fs.existsSync(file);
}

module.exports = getPrices;

