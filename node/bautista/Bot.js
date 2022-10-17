const BotFather = require('botfather');

const PersistedStatus = require('./PersistedStatus.js');

const status = PersistedStatus.get();

class Bot {
	constructor(token, adminUser, allowedUsers) {
		this._adminUser = adminUser;
		this._allowedUsers = allowedUsers;
		this._bf = new BotFather(token);
		this._chats = status.get('chats', {});
		this._lastUpdateId = status.get('lastUpdateId');
	}

	async connect() {
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

	async getMessages() {
		const params = {
			timeout: 600,
		};

		if (this._lastUpdateId) {
			params.offset = this._lastUpdateId + 1;
		}

		const updates = await this._call('getUpdates', params);

		this._updateChats(updates);

		const digest = {};

		for (const update of updates) {
			this._lastUpdateId = Math.max(this._lastUpdateId, update.update_id);

			const { message } = update;

			if (!message) {
				continue;
			}

			const userid = message.from.id;

			if (!this._allowedUsers.includes(userid)) {
				await this.send(
					'He ignorado el siguiente mensaje 🤷:\n\n' +
						JSON.stringify(message, null, 2),
					this._adminUser
				);

				continue;
			}

			if (!digest[userid]) {
				digest[userid] = [];
			}

			digest[userid].push(message.text);
		}

		status.set('lastUpdateId', this._lastUpdateId);

		return digest;
	}

	getRealName(userid) {
		const chat = this._chats[userid];

		if (!chat || !chat.first_name) {
			return 'Desconocido';
		}

		return chat.first_name;
	}

	async send(text, userid = undefined) {
		if (userid) {
			const chat = this._chats[userid];

			if (!chat) {
				throw new Error(`Cannot find chat for user ${userid}`);
			}

			await this._call('sendMessage', {
				chat_id: chat.id,
				text,
			});
		} else {
			for (const chat of Object.values(this._chats)) {
				await this._call('sendMessage', {
					chat_id: chat.id,
					text,
				});
			}
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

	_updateChats(updates) {
		let chatsUpdated = false;

		for (const update of updates) {
			if (update.message) {
				const userid = update.message.chat.userid;

				if (this._allowedUsers.includes(userid)) {
					this._chats[userid] = update.message.chat;

					chatsUpdated = true;
				}
			}
		}

		if (chatsUpdated) {
			status.set('chats', this._chats);
		}
	}
}

module.exports = Bot;
