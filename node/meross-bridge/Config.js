const fs = require('fs');

class Config {
	constructor(json) {
		this.user = json.user;
		this.password = json.password;
	}
}

Config.read = function () {
	const json = {
		user: process.env['MEROSS_USER'],
		password: process.env['MEROSS_PASSWORD'],
	};

	json.user = json.user || '';
	json.password = json.password || '';

	return new Config(json);
};

module.exports = Config;
