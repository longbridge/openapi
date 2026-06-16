#include "types.hpp"
#include "longbridge.h"

namespace longbridge {
namespace quote {

SubFlags
SubFlags::QUOTE()
{
  return SubFlags(LB_SUBFLAGS_QUOTE);
}

SubFlags
SubFlags::DEPTH()
{
  return SubFlags(LB_SUBFLAGS_DEPTH);
}

SubFlags
SubFlags::BROKER()
{
  return SubFlags(LB_SUBFLAGS_BROKER);
}

SubFlags
SubFlags::TRADE()
{
  return SubFlags(LB_SUBFLAGS_TRADE);
}

bool
DerivativeType::has_option()
{
  return (value & LB_DERIVATIVE_TYPE_OPTION) > 0;
}

bool
DerivativeType::has_warrant()
{
  return (value & LB_DERIVATIVE_TYPE_WARRANT) > 0;
}

} // namespace quote

namespace market {

TradeStatus
from_trade_status_code(int32_t value)
{
  switch (value) {
    case -1:
      return TradeStatus::UNKNOWN;
    case 0:
      return TradeStatus::NO_REGISTER_QUOTE;
    case 101:
      return TradeStatus::CLEAN;
    case 102:
      return TradeStatus::OPEN_BID;
    case 103:
      return TradeStatus::MORNING_CLOSING;
    case 105:
      return TradeStatus::TRADING;
    case 106:
      return TradeStatus::NOON_CLOSING;
    case 107:
      return TradeStatus::CLOSE_BID;
    case 108:
      return TradeStatus::CLOSING;
    case 110:
      return TradeStatus::DARK_WAIT;
    case 111:
      return TradeStatus::DARK_TRADING;
    case 112:
      return TradeStatus::DARK_CLOSING;
    case 120:
      return TradeStatus::AFTER_FIX;
    case 121:
      return TradeStatus::HALF_CLOSING;
    case 122:
      return TradeStatus::NOT_OPENED;
    case 123:
      return TradeStatus::REALTIME_QUOTE;
    case 201:
      return TradeStatus::US_PREV;
    case 202:
      return TradeStatus::US_TRADING;
    case 203:
      return TradeStatus::US_AFTER;
    case 204:
      return TradeStatus::US_CLOSING;
    case 205:
      return TradeStatus::US_STOP;
    case 206:
      return TradeStatus::US_CLEAN;
    case 207:
      return TradeStatus::US_NIGHT;
    case 209:
      return TradeStatus::US_PREV_MARKET_CLEAN;
    case 210:
      return TradeStatus::US_AFTER_MARKET_CLEAN;
    case 1000:
      return TradeStatus::REFRESH;
    case 1001:
      return TradeStatus::DELIST;
    case 1002:
      return TradeStatus::PREPARE;
    case 1003:
      return TradeStatus::CODE_CHANGE;
    case 1004:
      return TradeStatus::STOP;
    case 1005:
      return TradeStatus::WILL_OPEN;
    case 1006:
      return TradeStatus::COMMON_SUSPEND;
    case 1007:
      return TradeStatus::EXPIRE;
    case 1008:
      return TradeStatus::NO_QUOTE;
    case 1009:
      return TradeStatus::UNITED;
    case 1010:
      return TradeStatus::TRADING_HALT;
    case 1011:
      return TradeStatus::WAIT_LISTING;
    case 2001:
      return TradeStatus::FUSE;
    default:
      return TradeStatus::UNKNOWN;
  }
}

int32_t
code(TradeStatus status)
{
  return static_cast<int32_t>(status);
}

const char*
trade_status_as_string(TradeStatus status)
{
  switch (status) {
    case TradeStatus::UNKNOWN:
      return "UNKNOWN";
    case TradeStatus::NO_REGISTER_QUOTE:
      return "NO_REGISTER_QUOTE";
    case TradeStatus::CLEAN:
      return "CLEAN";
    case TradeStatus::OPEN_BID:
      return "OPEN_BID";
    case TradeStatus::MORNING_CLOSING:
      return "MORNING_CLOSING";
    case TradeStatus::TRADING:
      return "TRADING";
    case TradeStatus::NOON_CLOSING:
      return "NOON_CLOSING";
    case TradeStatus::CLOSE_BID:
      return "CLOSE_BID";
    case TradeStatus::CLOSING:
      return "CLOSING";
    case TradeStatus::DARK_WAIT:
      return "DARK_WAIT";
    case TradeStatus::DARK_TRADING:
      return "DARK_TRADING";
    case TradeStatus::DARK_CLOSING:
      return "DARK_CLOSING";
    case TradeStatus::AFTER_FIX:
      return "AFTER_FIX";
    case TradeStatus::HALF_CLOSING:
      return "HALF_CLOSING";
    case TradeStatus::NOT_OPENED:
      return "NOT_OPENED";
    case TradeStatus::REALTIME_QUOTE:
      return "REALTIME_QUOTE";
    case TradeStatus::US_PREV:
      return "US_PREV";
    case TradeStatus::US_TRADING:
      return "US_TRADING";
    case TradeStatus::US_AFTER:
      return "US_AFTER";
    case TradeStatus::US_CLOSING:
      return "US_CLOSING";
    case TradeStatus::US_STOP:
      return "US_STOP";
    case TradeStatus::US_CLEAN:
      return "US_CLEAN";
    case TradeStatus::US_NIGHT:
      return "US_NIGHT";
    case TradeStatus::US_PREV_MARKET_CLEAN:
      return "US_PREV_MARKET_CLEAN";
    case TradeStatus::US_AFTER_MARKET_CLEAN:
      return "US_AFTER_MARKET_CLEAN";
    case TradeStatus::REFRESH:
      return "REFRESH";
    case TradeStatus::DELIST:
      return "DELIST";
    case TradeStatus::PREPARE:
      return "PREPARE";
    case TradeStatus::CODE_CHANGE:
      return "CODE_CHANGE";
    case TradeStatus::STOP:
      return "STOP";
    case TradeStatus::WILL_OPEN:
      return "WILL_OPEN";
    case TradeStatus::COMMON_SUSPEND:
      return "COMMON_SUSPEND";
    case TradeStatus::EXPIRE:
      return "EXPIRE";
    case TradeStatus::NO_QUOTE:
      return "NO_QUOTE";
    case TradeStatus::UNITED:
      return "UNITED";
    case TradeStatus::TRADING_HALT:
      return "TRADING_HALT";
    case TradeStatus::WAIT_LISTING:
      return "WAIT_LISTING";
    case TradeStatus::FUSE:
      return "FUSE";
    default:
      return "UNKNOWN";
  }
}

TradeStatus
normalize(TradeStatus status)
{
  switch (status) {
    case TradeStatus::CLEAN:
      return TradeStatus::CLOSING;
    case TradeStatus::US_PREV_MARKET_CLEAN:
      return TradeStatus::US_CLOSING;
    case TradeStatus::US_CLEAN:
      return TradeStatus::US_PREV;
    case TradeStatus::US_AFTER_MARKET_CLEAN:
      return TradeStatus::US_TRADING;
    default:
      return status;
  }
}

const char*
name(TradeStatus status)
{
  switch (normalize(status)) {
    case TradeStatus::UNKNOWN:
    case TradeStatus::NO_REGISTER_QUOTE:
      return "Unknown";
    case TradeStatus::OPEN_BID:
      return "Open Bid";
    case TradeStatus::MORNING_CLOSING:
      return "Morning Break";
    case TradeStatus::TRADING:
    case TradeStatus::US_TRADING:
      return "Trading";
    case TradeStatus::NOON_CLOSING:
      return "Mid-Day Break";
    case TradeStatus::CLOSE_BID:
      return "Close Bid";
    case TradeStatus::CLOSING:
    case TradeStatus::HALF_CLOSING:
    case TradeStatus::US_CLOSING:
      return "Closed";
    case TradeStatus::DARK_WAIT:
      return "Dark Wait";
    case TradeStatus::DARK_TRADING:
      return "Dark Trading";
    case TradeStatus::DARK_CLOSING:
      return "Closing";
    case TradeStatus::AFTER_FIX:
      return "After Fix";
    case TradeStatus::NOT_OPENED:
      return "Not Open";
    case TradeStatus::REALTIME_QUOTE:
      return "Temporary Break";
    case TradeStatus::US_PREV:
      return "Pre-Market";
    case TradeStatus::US_AFTER:
      return "Post-Market";
    case TradeStatus::US_STOP:
    case TradeStatus::STOP:
      return "Stop";
    case TradeStatus::US_NIGHT:
      return "Overnight";
    case TradeStatus::REFRESH:
      return "Refresh";
    case TradeStatus::DELIST:
      return "Delist";
    case TradeStatus::PREPARE:
      return "Prepare";
    case TradeStatus::CODE_CHANGE:
      return "Code Change";
    case TradeStatus::WILL_OPEN:
      return "Will Open";
    case TradeStatus::COMMON_SUSPEND:
      return "Common Suspend";
    case TradeStatus::EXPIRE:
      return "Expire";
    case TradeStatus::NO_QUOTE:
      return "No Quote";
    case TradeStatus::UNITED:
      return "Not Listed";
    case TradeStatus::TRADING_HALT:
      return "Terminated";
    case TradeStatus::WAIT_LISTING:
      return "Wait Listing";
    case TradeStatus::FUSE:
      return "Fuse";
    default:
      return "Unknown";
  }
}

const char*
label(TradeStatus status)
{
  TradeStatus normalized = normalize(status);
  switch (normalized) {
    case TradeStatus::US_PREV:
    case TradeStatus::US_TRADING:
    case TradeStatus::US_AFTER:
    case TradeStatus::US_NIGHT:
    case TradeStatus::US_CLOSING:
    case TradeStatus::TRADING:
    case TradeStatus::CLOSING:
      return name(normalized);
    default:
      return "";
  }
}

bool
is_us_market(TradeStatus status)
{
  return code(status) >= 200 && code(status) < 300;
}

bool
is_us_pre_post(TradeStatus status)
{
  return is_us_prev(status) || is_us_after(status);
}

bool
is_us_night(TradeStatus status)
{
  return status == TradeStatus::US_NIGHT;
}

bool
is_us_closing(TradeStatus status)
{
  return status == TradeStatus::US_CLOSING ||
         status == TradeStatus::US_PREV_MARKET_CLEAN;
}

bool
is_closing(TradeStatus status)
{
  return status == TradeStatus::US_CLOSING ||
         status == TradeStatus::US_PREV_MARKET_CLEAN ||
         status == TradeStatus::CLOSING || status == TradeStatus::HALF_CLOSING;
}

bool
is_us_prev(TradeStatus status)
{
  return status == TradeStatus::US_PREV || status == TradeStatus::US_CLEAN;
}

bool
is_us_after(TradeStatus status)
{
  return status == TradeStatus::US_AFTER;
}

bool
is_trading(TradeStatus status)
{
  return status == TradeStatus::TRADING || status == TradeStatus::US_TRADING ||
         status == TradeStatus::US_AFTER_MARKET_CLEAN;
}

bool
is_dark(TradeStatus status)
{
  return status == TradeStatus::DARK_WAIT ||
         status == TradeStatus::DARK_TRADING ||
         status == TradeStatus::DARK_CLOSING;
}

bool
allow_trading(TradeStatus status)
{
  return status == TradeStatus::OPEN_BID || status == TradeStatus::TRADING ||
         status == TradeStatus::CLOSE_BID ||
         status == TradeStatus::NOT_OPENED ||
         status == TradeStatus::NOON_CLOSING ||
         status == TradeStatus::US_TRADING ||
         status == TradeStatus::US_AFTER_MARKET_CLEAN;
}

bool
is_special(TradeStatus status)
{
  return code(status) < 100 || status == TradeStatus::US_STOP ||
         code(status) >= 1000;
}

} // namespace market

} // namespace longbridge
