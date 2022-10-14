const fs = require('fs');
const os = require('os');
const path = require('path');

const FILE_PATH = "/run/bautista/status.json";

let persistedStatus;

class PersistedStatus {
	constructor(file) {
		this._file = file;
		this._json = JSON.parse(fs.readFileSync(file, 'utf-8'));
	}

	get(key, defaultValue) {
		if (this._json[key] === undefined) {
			return defaultValue;
		}

		return this._json[key];
	}

	set(key, value) {
		this._json[key] = value;

		fs.writeFileSync(this._file, JSON.stringify(this._json, null, '\t'));
	}
}

PersistedStatus.get = function get() {
	if (persistedStatus) {
		return persistedStatus;
	}

	fs.mkdirSync(path.dirname(FILE_PATH), { recursive: true });

	if (!fs.existsSync(FILE_PATH)) {
		fs.writeFileSync(FILE_PATH, '{}');
	}

	persistedStatus = new PersistedStatus(FILE_PATH);

	return persistedStatus;
};

module.exports = PersistedStatus;
