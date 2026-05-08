#include "calendar_context.hpp"
#include "longbridge.h"

extern "C" {
const lb_calendar_context_t* lb_calendar_context_new(const lb_config_t* config);
void lb_calendar_context_retain(const lb_calendar_context_t* ctx);
void lb_calendar_context_release(const lb_calendar_context_t* ctx);
void lb_calendar_context_finance_calendar(const lb_calendar_context_t*, int32_t, const char*, const char*, const char*, lb_async_callback_t, void*);
}

namespace longbridge {
namespace calendar {

CalendarContext::CalendarContext() : ctx_(nullptr) {}
CalendarContext::CalendarContext(const lb_calendar_context_t* ctx) { ctx_ = ctx; if (ctx_) lb_calendar_context_retain(ctx_); }
CalendarContext::CalendarContext(const CalendarContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_calendar_context_retain(ctx_); }
CalendarContext::CalendarContext(CalendarContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
CalendarContext::~CalendarContext() { if (ctx_) lb_calendar_context_release(ctx_); }
CalendarContext& CalendarContext::operator=(const CalendarContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_calendar_context_retain(ctx_); return *this; }
CalendarContext CalendarContext::create(const Config& config) { auto* ptr = lb_calendar_context_new(config); CalendarContext ctx(ptr); if (ptr) lb_calendar_context_release(ptr); return ctx; }

void CalendarContext::finance_calendar(int32_t category, const std::string& start, const std::string& end, const std::string& market, AsyncCallback<CalendarContext, CalendarEventsResponse> callback) const {
  lb_calendar_context_finance_calendar(ctx_, category, start.c_str(), end.c_str(), market.empty() ? nullptr : market.c_str(),
    [](auto res) {
      auto cb = callback::get_async_callback<CalendarContext, CalendarEventsResponse>(res->userdata);
      CalendarContext fctx((const lb_calendar_context_t*)res->ctx);
      Status status(res->error);
      if (status) {
        auto* r = (const lb_calendar_events_response_t*)res->data;
        CalendarEventsResponse resp;
        resp.date = r->date;
        for (size_t i = 0; i < r->num_list; ++i) {
          CalendarDateGroup grp; grp.date = r->list[i].date; grp.count = r->list[i].count;
          for (size_t j = 0; j < r->list[i].num_infos; ++j) {
            const auto& info = r->list[i].infos[j];
            CalendarEventInfo ei; ei.symbol=info.symbol; ei.market=info.market; ei.content=info.content; ei.counter_name=info.counter_name; ei.date_type=info.date_type; ei.date=info.date; ei.chart_uid=info.chart_uid; ei.event_type=info.event_type; ei.datetime=info.datetime; ei.icon=info.icon; ei.star=info.star; ei.id=info.id; ei.financial_market_time=info.financial_market_time; ei.currency=info.currency; ei.activity_type=info.activity_type;
            for (size_t k = 0; k < info.num_data_kv; ++k) { CalendarDataKv kv; kv.key=info.data_kv[k].key; kv.value=info.data_kv[k].value; kv.value_type=info.data_kv[k].value_type; kv.value_raw=info.data_kv[k].value_raw; ei.data_kv.push_back(kv); }
            grp.infos.push_back(ei);
          }
          resp.list.push_back(grp);
        }
        (*cb)(AsyncResult<CalendarContext, CalendarEventsResponse>(fctx, std::move(status), &resp));
      } else { (*cb)(AsyncResult<CalendarContext, CalendarEventsResponse>(fctx, std::move(status), nullptr)); }
    }, new AsyncCallback<CalendarContext, CalendarEventsResponse>(callback));
}

} // namespace calendar
} // namespace longbridge
