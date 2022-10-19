const MerossCloud = require('meross-cloud');

const POLL_RETRIES = 20;

class Device {
	constructor(name) {
		this.name = name;
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

	async updateStatus() {
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

Device.connectToMerossCloud = async function connectDevices(
	config,
	eventHandler
) {
	const options = {
		email: config.user,
		password: config.password,
		logger: () => {},
		localHttpFirst: false, // Try to contact the devices locally before trying the cloud
		onlyLocalForGet: false, // When trying locally, do not try the cloud for GET requests at all
		timeout: 3000,
	};

	const log = console.log;
	console.log = () => {};

	const meross = new MerossCloud(options);
	const devices = {};

	meross.on('deviceInitialized', (deviceId, deviceDef, device) => {
		const { devName } = deviceDef;

		const dev = (devices[devName] = new Device(devName));

		device.on('connected', async () => {
			dev._id = deviceId;
			dev._def = deviceDef;
			dev._proxy = device;

			dev.status.connected = true;

			await eventHandler('connected', dev);
		});

		device.on('close', async (error) => {
			dev.status.connected = false;

			await eventHandler('disconnected', dev, error);
		});

		device.on('error', async (error) => {
			if (error) {
				await eventHandler('errored', dev, error);
			}
		});

		device.on('reconnect', async () => {
			dev.status.connected = true;

			await eventHandler('connected', dev);
		});

		device.on('data', async (namespace, payload) => {
			if (namespace === 'Appliance.Control.ToggleX') {
				const { togglex } = payload;

				dev.status.on = togglex[togglex.length - 1].onoff != 0;
			}
		});
	});

	//  meross.on("connected", (deviceId) => {
	//    console.log(deviceId + " connected");
	//  });

	//  meross.on("close", (deviceId, error) => {
	//    console.log(deviceId + " closed: " + error);
	//  });

	meross.on('error', async (deviceId, error) => {
		if (!deviceId && error) {
			await eventHandler('errored', null, error);
		}
	});

	//  meross.on("reconnect", (deviceId) => {
	//    console.log(deviceId + " reconnected");
	//  });

	//  meross.on("data", (deviceId, payload) => {
	//    console.log(deviceId + " data: " + JSON.stringify(payload));
	//  });

	meross.connect(async (error) => {
		if (error) {
			await eventHandler('errored', null, error);
		}
	});

	return new Promise((resolve, reject) => {
		pollDeviceConnections(
			POLL_RETRIES,
			devices,
			() => {
				console.log = log;

				resolve(devices);
			},
			(err) => {
				console.log = log;

				reject(err);
			}
		);
	});
};

function pollDeviceConnections(retriesLeft, devices, resolve, reject) {
	for (const dev of Object.values(devices)) {
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
