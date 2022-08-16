function getCheapestPrices(prices, count) {
	prices = [...prices];

	prices.sort((l, r) => Number(l.price) - Number(r.price));

	prices = prices.slice(0, count);

	return prices.sort((l, r) => Number(l.hour) - Number(r.hour));
}

module.exports = getCheapestPrices;
