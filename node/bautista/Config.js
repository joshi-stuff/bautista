const fs = require('fs');

const Device = require('./Device.js');
const Rules = require('./Rules.js');

const CONFIG_FILE = '/etc/bautista/config.json';

class Config {
	constructor(json) {
		this.control = {
			period: json.control.period,
		};

		this.devices = Object.entries(json.devices).map(([name, desc]) => {
			return new Device(
				name,
				new Rules(desc.rules.hoursOn, desc.rules.consecutive),
				desc.controlled
			);
		});

		this.meross = {
			user: json.meross.user,
			password: json.meross.password,
		};

		this.telegram = {
			allowedUsers: json.telegram.allowedUsers,
			token: json.telegram.token,
		};
	}
}

Config.read = function () {
	const json = fs.existsSync(CONFIG_FILE)
		? JSON.parse(fs.readFileSync(CONFIG_FILE, 'utf-8'))
		: {};

	json.control = json.control || {};
	json.control.period = json.control.period || 58000;

	json.devices = json.devices || {};

	json.meross = json.meross || {};
	json.meross.user = json.meross.user || '***';
	json.meross.password = json.meross.password || '***';

	json.telegram = json.telegram || {};
	json.telegram.token = json.telegram.token || '***';
	json.telegram.allowedUsers = json.telegram.allowedUsers || [];

	return new Config(json);
};

module.exports = Config;
