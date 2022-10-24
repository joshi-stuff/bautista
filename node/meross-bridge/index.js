const { stdin, stdout } = require('process');
const readline = require('readline/promises');

const Config = require('./Config.js');
const Device = require('./Device.js');

async function main() {
	try {
		const config = Config.read();

		const devices = await Device.connectToMerossCloud(
			config,
			deviceEventHandler
		);

		const rl = readline.createInterface({ input: stdin, output: stdout });

		while (true) {
			const command = (await rl.question('')).trim();

			if (
				await runCommand('STATUS', command, devices, async (dev) => {
					await dev.updateStatus();

					if (dev.status.on == undefined) {
						reply(true, 'UNKNOWN');
					} else {
						reply(true, dev.status.on ? 'ON' : 'OFF');
					}
				})
			) {
				continue;
			}

			if (
				await runCommand('TURNOFF', command, devices, async (dev) => {
					await dev.toggle(false);

					await dev.updateStatus();

					if (dev.status.on == undefined) {
						reply(true, 'UNKNOWN');
					} else {
						reply(true, dev.status.on ? 'ON' : 'OFF');
					}
				})
			) {
				continue;
			}

			if (
				await runCommand('TURNON', command, devices, async (dev) => {
					await dev.toggle(true);

					await dev.updateStatus();

					if (dev.status.on == undefined) {
						reply(true, 'UNKNOWN');
					} else {
						reply(true, dev.status.on ? 'ON' : 'OFF');
					}
				})
			) {
				continue;
			}

			if (command == 'DEVICES') {
				reply(true, ...Object.keys(devices));
				continue;
			}

			if (command == 'QUIT') {
				reply(true);
				process.exit(0);
			}

			reply(false, `Invalid command: '${command}'`);
		}
	} catch (err) {
		announce('FATAL', err);

		process.exit(1);
	}
}

async function deviceEventHandler(event, dev, error) {
	if (dev) {
		switch (event) {
			case 'connected': {
				announce('CONNECTED', dev.name);
				break;
			}

			case 'disconnected': {
				announce('DISCONNECTED', dev.name, error ?? '');
				break;
			}

			case 'errored': {
				announce('ERRORED', dev.name, error ?? '');
				break;
			}
		}
	} else {
		switch (event) {
			case 'errored': {
				announce('ERRORED', error ?? '');
				break;
			}
		}
	}
}

async function announce(eventName, ...things) {
	process.stdout.write('! ', 'utf-8');
	process.stdout.write(eventName, 'utf-8');
	process.stdout.write(' ', 'utf-8');
	process.stdout.write(things.join('|'), 'utf-8');
	process.stdout.write('\n', 'utf-8');
	// TODO: console.error announcements
}

async function reply(ok, ...things) {
	process.stdout.write('> ', 'utf-8');
	process.stdout.write(ok ? 'OK ' : 'ERROR ', 'utf-8');
	process.stdout.write(things.join('|'), 'utf-8');
	process.stdout.write('\n', 'utf-8');
	// TODO: console.error replies
}

async function runCommand(commandName, command, devices, callback) {
	if (!command.startsWith(`${commandName} `)) {
		return false;
	}

	const devName = command.substr(commandName.length + 1).trim();
	const dev = devices[devName];

	if (dev) {
		try {
			await callback(dev);
		} catch (err) {
			reply(false, err);
		}
	} else {
		reply(false, `Unknown device '${devName}'`);
	}

	return true;
}

async function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

main();
