const BotFather = require('botfather');

const PersistedStatus = require('./PersistedStatus.js');

const status = PersistedStatus.get();

class Bot {
	constructor(token, allowedUsers) {
		this._allowedUsers = allowedUsers;
		this._bf = new BotFather(token);
		this._chats = status.get('chats', {});
		this._lastUpdateId = status.get('lastUpdateId');
	}

	async connect() {
		const params = {};

		if (this._lastUpdateId) {
			params.offset = this._lastUpdateId + 1;
		}

		const updates = await this._call('getUpdates', params);

		for (const update of updates) {
			if (update.message) {
				const username = update.message.chat.username;

				if (this._allowedUsers.includes(username)) {
					this._chats[username] = update.message.chat;
				}
			}

			this._lastUpdateId = Math.max(this._lastUpdateId, update.update_id);
		}

		status.set('chats', this._chats);
		status.set('lastUpdateId', this._lastUpdateId);

		/*
		for (const chat of Object.values(this._chats)) {
			console.log('setChatMenuButton', chat);
			console.log(
				await this._call('setChatMenuButton', {
					chat_id: chat.id,
					type: 'commands',
				})
			);
		}
		*/
	}

	async send(text) {
		for (const chat of Object.values(this._chats)) {
			await this._call('sendMessage', {
				chat_id: chat.id,
				text,
			});
		}
	}

	async _call(method, params) {
		const json = await this._bf.api(method, params);

		if (json.ok) {
			return json.result;
		} else {
			throw new Error(json.description);
		}
	}
}

module.exports = Bot;
