#include <iostream>

#include "longport.h"
#include "longport.hpp"

using namespace longport;
using namespace longport::quote;

int main()
{
	longport::Config config;
	longport::Status status = longport::Config::from_env(config);

	if (!status) {
		std::cout << "failed to load configuration from environment: "
			<< status.message() << std::endl;
		return -1;
	}
	QuoteContext::create(config, [&](auto res) {
		if (!res) {
			std::cout << "failed to create quote context: " << res.status().message()
				<< std::endl;
			return;
		}
		DateTime datetime2;
		datetime2.date.year = 2025;
		datetime2.date.month = 8;
		datetime2.date.day = 1;
		datetime2.time.hour = 0;
		datetime2.time.minute = 0;
		datetime2.time.second = 0;

		const std::string symbol = "700.HK";
		res.context().history_candlesticks_by_offset(symbol, Period::Day, AdjustType::NoAdjust, false, datetime2, 10, [symbol](auto res) {
			if (!res)
			{
				std::cout << "failed to subscribe quote: " << res.status().message()
					<< std::endl;
				return;
			}
			for (auto it = res->cbegin(); it != res->cend(); ++it) {
				std::cout << "symbol: " << symbol
					<< " close=" << (double)it->close
					<< " open=" << (double)it->open
					<< " low=" << (double)it->low
					<< " high=" << (double)it->high
					<< " volume=" << (int64_t)it->volume
					<< " turnover=" << (double)it->turnover
					<< " timestamp=" << (int64_t)it->timestamp << std::endl;
			}
			});
		});
	std::cin.get();
	return 0;
}

