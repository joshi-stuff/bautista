const MerossCloud = require('meross-cloud');

class Device {
	constructor(name, hours) {
		this.name = name;
		this.hours = hours;

		this.status = {
			on: undefined
		};
	}

	async toggle(on) {
		return new Promise((resolve, reject) => {
			this._proxy.controlToggleX(0, on, (err, _result) => {
				if (err) {
					reject(err);
				}
				else {
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

				this.status.on = res.all.digest.togglex.onoff != 0;

				resolve();
			});
		});
	}

	_on_connect(id, def, proxy) {
		this._id = id;
		this._def = def;
		this._proxy = proxy;
	}
}

Device.connectDevices = connectDevices;

module.exports = Device;

const POLL_RETRIES = 20;

async function connectDevices(creds, devices) {
	const options = {
		email: creds.user,
		password: creds.password,
		logger: () => {},//console.log,
		localHttpFirst: false, // Try to contact the devices locally before trying the cloud
		onlyLocalForGet: false, // When trying locally, do not try the cloud for GET requests at all
		timeout: 3000 // Default is 3000
	};

	const log = console.log;
	console.log = () => {};

	const meross = new MerossCloud(options);

	meross.on('deviceInitialized', (deviceId, deviceDef, device) => {
		const {devName} = deviceDef;

		log('    🔍 Device discovered:', devName);
		//log(stringify(deviceDef));

		device.on('connected', () => {
			const dev = devices.find(dev => dev.name === devName);

			if (!dev) {
				return;
			}

			dev._on_connect(deviceId, deviceDef, device);

			log('    ✅ Device connected:', devName);
		});

		device.on('close', (error) => {
			if (error) {
				log('❌ Device closed:', devName);
				log(error);
			} else {
				log('✅ Device closed:', devName);
			}
		});

		device.on('error', (error) => {
			log('❌ Device errored:', devName);

			if (error) {
				log(error);
			}
		});

		device.on('reconnect', () => {
			log('👀 Device reconnected:', devName);
		});

		device.on('data', (namespace, payload) => {
			const dev = devices.find(dev => dev.name === devName);

			if (!dev) {
				log('📦 Device data received:', devName, `[${namespace}]`);

				return;
			}

			if (namespace === 'Appliance.Control.ToggleX') {
				const {togglex} = payload;

				dev.status.on = (togglex[togglex.length - 1].onoff != 0);
			}
		});

	});

	log('⌛ Waiting for devices to connect...');

	meross.connect((error) => {
		if (error) {
			log('❌ Connect error: ' + error);
		}
	});

	return new Promise((resolve, reject) => {
		pollDeviceConnections(
			POLL_RETRIES,
			devices, 
			(...args) => {
				log('😊 All devices connected');

				console.log = log;

				resolve(...args);
			}, 
			(...args) => {
				log('😞 Devices failed to connect');

				console.log = log;

				reject(...args);
			}
		);
	});
}

function pollDeviceConnections(retriesLeft, devices, resolve, reject) {
	for (const dev of devices) {
		if (!dev._proxy) {
			if (!retriesLeft) {
				reject('Gave up trying to connect');
			} else {
				setTimeout(
					() => 
						pollDeviceConnections(
							retriesLeft - 1, devices, resolve, reject
						), 
					1000
				);
			}

			return;
		}
	}

	resolve();
}


			/*
			if (devName === 'Pruebas') {
				let onoff = true;

				setInterval(() => {
					log('Toggling ...');

					device.controlToggleX(0, onoff, (err, res) => {
						if (err) {
							log('Error:', err);
						}
						else {
							log('Response:', res);
						}
					});

					onoff = !onoff;
				}, 2000);
			}
			*/

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
	/*
	meross.on('connected', (deviceId) => {
		log(deviceId + ' connected');
	});

	meross.on('close', (deviceId, error) => {
		log(deviceId + ' closed: ' + error);
	});

	meross.on('error', (deviceId, error) => {
		log(deviceId + ' error: ' + error);
	});

	meross.on('reconnect', (deviceId) => {
		log(deviceId + ' reconnected');
	});

	meross.on('data', (deviceId, payload) => {
		log(deviceId + ' data: ' + stringify(payload));
	});
	*/

