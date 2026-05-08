#pragma once
#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_calendar_context_t lb_calendar_context_t;

namespace longbridge {
namespace calendar {

/// Key-value metadata entry attached to a calendar event.
struct CalendarDataKv { std::string key; std::string value; std::string value_type; std::string value_raw; };
/// A single financial calendar event (earnings report, dividend, IPO, etc.).
struct CalendarEventInfo { std::string symbol; std::string market; std::string content; std::string counter_name; std::string date_type; std::string date; std::string chart_uid; std::vector<CalendarDataKv> data_kv; std::string event_type; std::string datetime; std::string icon; int32_t star; std::string id; std::string financial_market_time; std::string currency; std::string activity_type; };
/// Calendar events grouped by date.
struct CalendarDateGroup { std::string date; int32_t count; std::vector<CalendarEventInfo> infos; };
/// Response for finance_calendar — events grouped by date within the requested range.
struct CalendarEventsResponse { std::string date; std::vector<CalendarDateGroup> list; };

/// Financial calendar context — earnings, dividends, splits, IPOs, macro data.
class CalendarContext {
private: const lb_calendar_context_t* ctx_;
public:
  CalendarContext(); CalendarContext(const lb_calendar_context_t* ctx); CalendarContext(const CalendarContext&); CalendarContext(CalendarContext&&); ~CalendarContext(); CalendarContext& operator=(const CalendarContext&);
  /// Create a CalendarContext from a Config.
  static CalendarContext create(const Config& config);
  /// Get financial calendar events for the given date range. category: 0=Report,1=Dividend,2=Split,3=Ipo,4=MacroData,5=Closed.
  void finance_calendar(int32_t category, const std::string& start, const std::string& end, const std::string& market, AsyncCallback<CalendarContext, CalendarEventsResponse> callback) const;
};

} // namespace calendar
} // namespace longbridge
