const fs = require('fs');

const Device = require('./Device.js');
const Rules = require('./Rules.js');

const CONFIG_FILE = '/etc/bautista/config.json';
const CREDS_FILE = '/etc/bautista/creds.json';

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
			adminUser: json.telegram.adminUser,
			allowedUsers: json.telegram.allowedUsers,
			token: json.telegram.token,
		};
	}
}

Config.read = function () {
	// Read config.json and normalize structure

	const json = fs.existsSync(CONFIG_FILE)
		? JSON.parse(fs.readFileSync(CONFIG_FILE, 'utf-8'))
		: {};

	json.control = json.control || {};
	json.control.period = json.control.period || 58000;

	json.devices = json.devices || {};

	json.meross = json.meross || {};

	json.telegram = json.telegram || {};
	json.telegram.adminUser = json.telegram.adminUser || [];
	json.telegram.allowedUsers = json.telegram.allowedUsers || [];

	// Read creds.json and normalize structure

	const creds = fs.existsSync(CREDS_FILE)
		? JSON.parse(fs.readFileSync(CREDS_FILE, 'utf-8'))
		: {};

	json.meross.user = creds.meross.user || '';
	json.meross.password = creds.meross.password || '';

	json.telegram.token = creds.telegram.token || '';

	// Canonicalize values

	if (
		json.telegram.adminUser &&
		!json.telegram.allowedUsers.includes(json.telegram.adminUser)
	) {
		json.telegram.allowedUsers.push(json.telegram.adminUser);
	}

	// Return the config


	return new Config(json);
};

module.exports = Config;
