const Schedule = require('./Schedule');

class Rules {
	constructor(hoursOn, consecutive = false) {
		this.hoursOn = hoursOn;
		this.consecutive = consecutive;
	}

	getSchedule(prices) {
		const schedule = new Schedule();

		const cheapestPrices = this.consecutive
			? getCheapestConsecutivePrices(prices, this.hoursOn)
			: getCheapestPrices(prices, this.hoursOn);

		cheapestPrices.forEach((price) => {
			schedule.onAt[price.hour] = true;
		});

		return schedule;
	}
}

module.exports = Rules;

function getCheapestConsecutivePrices(prices, count) {
	// TODO: implement
	prices = [...prices];

	prices.sort((l, r) => Number(l.price) - Number(r.price));

	prices = prices.slice(0, count);

	return prices.sort((l, r) => Number(l.hour) - Number(r.hour));
}

function getCheapestPrices(prices, count) {
	prices = [...prices];

	prices.sort((l, r) => Number(l.price) - Number(r.price));

	prices = prices.slice(0, count);

	return prices.sort((l, r) => Number(l.hour) - Number(r.hour));
}
