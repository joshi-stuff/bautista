const MerossCloud = require('meross-cloud');

const POLL_RETRIES = 20;

class Device {
	constructor(name, rules, controlled = true) {
		this.name = name;
		this.rules = rules;
		this.controlled = controlled;

		this.status = new Device.Status();
	}

	async toggle(on) {
		return new Promise((resolve, reject) => {
			this._proxy.controlToggleX(0, on, (err, _result) => {
				if (err) {
					reject(err);
				} else {
					resolve();
				}
			});

			this.status.on = on;
		});
	}

	async update_status() {
		return new Promise((resolve, reject) => {
			this._proxy.getSystemAllData((err, res) => {
				if (err) {
					reject(err);
					return;
				}

				const { togglex } = res.all.digest;

				this.status.on = togglex[togglex.length - 1].onoff != 0;

				resolve();
			});
		});
	}
}

Device.Status = class Status {
	constructor() {
		this.on = undefined;
	}
};

Device.connectDevices = async function connectDevices(
	creds,
	devices,
	eventHandler
) {
	const options = {
		email: creds.user,
		password: creds.password,
		logger: () => {},
		localHttpFirst: false, // Try to contact the devices locally before trying the cloud
		onlyLocalForGet: false, // When trying locally, do not try the cloud for GET requests at all
		timeout: 3000,
	};

	const log = console.log;
	console.log = () => {};

	const meross = new MerossCloud(options);

	meross.on('deviceInitialized', (deviceId, deviceDef, device) => {
		const { devName } = deviceDef;

		device.on('connected', () => {
			withDevice(devices, devName, (dev) => {
				dev._id = deviceId;
				dev._def = deviceDef;
				dev._proxy = device;

				dev.status.connected = true;

				eventHandler('connected', dev);
			});
		});

		device.on('close', (error) => {
			withDevice(devices, devName, (dev) => {
				dev.status.connected = false;

				eventHandler('disconnected', dev, error);
			});
		});

		device.on('error', (error) => {
			withDevice(devices, devName, (dev) => {
				if (error) {
					console.log('>>>errored', dev.name, error);
					eventHandler('errored', dev, error);
				}
			});
		});

		device.on('reconnect', () => {
			withDevice(devices, devName, (dev) => {
				dev.status.connected = true;

				eventHandler('connected', dev);
			});
		});

		device.on('data', (namespace, payload) => {
			withDevice(devices, devName, (dev) => {
				if (namespace === 'Appliance.Control.ToggleX') {
					const { togglex } = payload;

					dev.status.on = togglex[togglex.length - 1].onoff != 0;
				}
			});
		});
	});

	meross.connect((error) => {
		if (error) {
			eventHandler('errored', null, error);
		}
	});

	return new Promise((resolve, reject) => {
		pollDeviceConnections(
			POLL_RETRIES,
			devices,
			(...args) => {
				console.log = log;

				resolve(...args);
			},
			(...args) => {
				console.log = log;

				reject(...args);
			}
		);
	});
};

function pollDeviceConnections(retriesLeft, devices, resolve, reject) {
	for (const dev of devices) {
		if (!dev.controlled) {
			continue;
		}

		if (!dev.status.connected) {
			if (!retriesLeft) {
				reject(new Error('Connection timeout'));
			} else {
				setTimeout(
					() =>
						pollDeviceConnections(
							retriesLeft - 1,
							devices,
							resolve,
							reject
						),
					1000
				);
			}

			return;
		}
	}

	resolve();
}

module.exports = Device;

function withDevice(devices, devName, callback) {
	const dev = devices.find((dev) => dev.name === devName);

	if (!dev) {
		return;
	}

	callback(dev);
}

/*
		device.getSystemAbilities((err, res) => {
			log('Abilities: ' + stringify(res));

			device.getSystemAllData((err, res) => {
				log('All-Data: ' + stringify(res));
			});
		});

		setTimeout(() => {
			log('toggle ...');
			device.controlToggleX(1, true, (err, res) => {
				log('Toggle Response: err: ' + err + ', res: ' + stringify(res));
			});
		}, 2000);
		*/
