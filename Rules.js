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
	prices = [...prices];

	prices.sort((l, r) => Number(l.hour) - Number(r.hour));

	let min = 0;
	let minSum = Number.MAX_VALUE;

	for (let i = 0; i < 24 - count + 1; i++) {
		let sum = 0;

		for (let j = 0; j < count; j++) {
			sum += prices[i + j].price;
		}

		if (sum < minSum) {
			min = i;
			minSum = sum;
		}
	}

	return prices.slice(min, min + count);
}

function getCheapestPrices(prices, count) {
	prices = [...prices];

	prices.sort((l, r) => Number(l.price) - Number(r.price));

	prices = prices.slice(0, count);

	return prices.sort((l, r) => Number(l.hour) - Number(r.hour));
}
