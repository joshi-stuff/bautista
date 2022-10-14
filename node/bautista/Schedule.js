class Schedule {
	constructor() {
		this.onAt = [];

		for (let i = 0; i < 24; i++) {
			this.onAt[i] = false;
		}
	}
}

module.exports = Schedule;
