#pragma once
#include <optional>

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_calendar_context_t lb_calendar_context_t;

namespace longbridge {
namespace calendar {

enum class CalendarCategory
{
  Report    = 0,
  Dividend  = 1,
  Split     = 2,
  Ipo       = 3,
  MacroData = 4,
  Closed    = 5,
};

/// Pagination direction for finance_calendar (the `next` query parameter).
enum class CalendarPageDirection
{
  Later   = 0,
  Earlier = 1,
};

/// Key-value metadata entry attached to a calendar event.
struct CalendarDataKv { std::string key; std::string value; std::string value_type; std::string value_raw; };
/// A single financial calendar event (earnings report, dividend, IPO, etc.).
struct CalendarEventInfo { std::string symbol; std::string market; std::string content; std::string counter_name; std::string date_type; std::string date; std::string chart_uid; std::vector<CalendarDataKv> data_kv; std::string event_type; std::string datetime; std::string icon; int32_t star; std::string id; std::string financial_market_time; std::string currency; std::string activity_type; };
/// Calendar events grouped by date.
struct CalendarDateGroup { std::string date; int32_t count; std::vector<CalendarEventInfo> infos; };
/// Response for finance_calendar — events grouped by date within the requested range.
/// Response for finance_calendar — events grouped by date within the requested range.
struct CalendarEventsResponse { std::string date; std::vector<CalendarDateGroup> list; std::string next_date; };

/// Financial calendar context — earnings, dividends, splits, IPOs, macro data.
class CalendarContext {
private: const lb_calendar_context_t* ctx_;
public:
  CalendarContext(); CalendarContext(const lb_calendar_context_t* ctx); CalendarContext(const CalendarContext&); CalendarContext(CalendarContext&&); ~CalendarContext(); CalendarContext& operator=(const CalendarContext&);
  /// Create a CalendarContext from a Config.
  static CalendarContext create(const Config& config);
  /// Get financial calendar events for the given date range.
  ///
  /// The endpoint paginates: the server caps each response (historically 10
  /// events per page unless a larger `count` is requested) and returns a
  /// `next_date` cursor. To page through the full window, request a larger
  /// `count`, or re-call with the returned `next_date` as `start`. `count`,
  /// `offset` and `next` are optional (pass `std::nullopt` to use the server
  /// default).
  void finance_calendar(CalendarCategory category, const std::string& start, const std::string& end, const std::string& market, std::optional<int32_t> count, std::optional<int32_t> offset, std::optional<CalendarPageDirection> next, AsyncCallback<CalendarContext, CalendarEventsResponse> callback) const;
};

} // namespace calendar
} // namespace longbridge
