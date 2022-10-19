const fs = require('fs');

const CONFIG_FILE = '/etc/meross-bridge/config.json';

class Config {
	constructor(json) {
		this.user = json.user;
		this.password = json.password;
	}
}

Config.read = function () {
	const json = fs.existsSync(CONFIG_FILE)
		? JSON.parse(fs.readFileSync(CONFIG_FILE, 'utf-8'))
		: {};

	json.user = json.user || '';
	json.password = json.password || '';

	return new Config(json);
};

module.exports = Config;
