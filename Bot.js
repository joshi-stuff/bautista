const BotFather = require('botfather')

const TOKEN = "***************************************";

class Bot {
	constructor() {
		this._bf = new BotFather(TOKEN);
		this._chats = {};
	}

	async connect() {
		const updates = await this._call('getUpdates');

		for (const update of updates) {
			if (update.message) {
				this._chats[update.message.chat.id] = update.message.chat;
			}
		}
	}

	async send(text) {
		for (const chat of Object.values(this._chats)) {
			return this._call('sendMessage', {
				chat_id: chat.id,
				text
			});
		}

	}

	async _call(method, params) {
		const json = await this._bf.api(method, params);

		if (json.ok) {
			return json.result
		}
		else {
			throw new Error(json.description);
		}
	}
}

module.exports = Bot;
