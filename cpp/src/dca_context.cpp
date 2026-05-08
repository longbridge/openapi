#include "dca_context.hpp"
#include "longbridge.h"
#include "convert.hpp"

extern "C" {
const lb_dca_context_t* lb_dca_context_new(const lb_config_t*);
void lb_dca_context_retain(const lb_dca_context_t*);
void lb_dca_context_release(const lb_dca_context_t*);
void lb_dca_context_list(const lb_dca_context_t*, int32_t, lb_async_callback_t, void*);
void lb_dca_context_stats(const lb_dca_context_t*, lb_async_callback_t, void*);
void lb_dca_context_check_support(const lb_dca_context_t*, const char* const*, size_t, lb_async_callback_t, void*);
void lb_dca_context_pause(const lb_dca_context_t*, const char*, lb_async_callback_t, void*);
void lb_dca_context_resume(const lb_dca_context_t*, const char*, lb_async_callback_t, void*);
void lb_dca_context_stop(const lb_dca_context_t*, const char*, lb_async_callback_t, void*);
void lb_dca_context_calc_date(const lb_dca_context_t*, const char*, int32_t, const char*, uint32_t, lb_async_callback_t, void*);
void lb_dca_context_create(const lb_dca_context_t*, const char*, const char*, int32_t, const char*, uint32_t, bool, lb_async_callback_t, void*);
void lb_dca_context_update(const lb_dca_context_t*, const char*, const char*, int32_t, const char*, const char*, int32_t, lb_async_callback_t, void*);
void lb_dca_context_set_reminder(const lb_dca_context_t*, const char*, lb_async_callback_t, void*);
void lb_dca_context_history(const lb_dca_context_t*, const char*, int32_t, int32_t, lb_async_callback_t, void*);
}

namespace longbridge {
namespace dca {

DCAContext::DCAContext() : ctx_(nullptr) {}
DCAContext::DCAContext(const lb_dca_context_t* ctx) { ctx_ = ctx; if(ctx_) lb_dca_context_retain(ctx_); }
DCAContext::DCAContext(const DCAContext& ctx) { ctx_ = ctx.ctx_; if(ctx_) lb_dca_context_retain(ctx_); }
DCAContext::DCAContext(DCAContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
DCAContext::~DCAContext() { if(ctx_) lb_dca_context_release(ctx_); }
DCAContext& DCAContext::operator=(const DCAContext& ctx) { ctx_ = ctx.ctx_; if(ctx_) lb_dca_context_retain(ctx_); return *this; }
DCAContext DCAContext::create(const Config& config) { auto* ptr = lb_dca_context_new(config); DCAContext ctx(ptr); if(ptr) lb_dca_context_release(ptr); return ctx; }

#define DCA_LIST_CB(c_fn, ...)                                                  \
  c_fn(__VA_ARGS__,                                                             \
    [](auto res) {                                                              \
      auto cb = callback::get_async_callback<DCAContext, DcaList>(res->userdata); \
      DCAContext fctx((const lb_dca_context_t*)res->ctx);                       \
      Status status(res->error);                                                \
      if (status) {                                                             \
        auto r = convert::convert((const lb_dca_list_t*)res->data);                 \
        (*cb)(AsyncResult<DCAContext, DcaList>(fctx, std::move(status), &r));  \
      } else {                                                                  \
        (*cb)(AsyncResult<DCAContext, DcaList>(fctx, std::move(status), nullptr)); \
      }                                                                         \
    }, new AsyncCallback<DCAContext, DcaList>(callback))

void DCAContext::list(int32_t status_val, AsyncCallback<DCAContext, DcaList> callback) const {
    DCA_LIST_CB(lb_dca_context_list, ctx_, status_val);
}

void DCAContext::stats(AsyncCallback<DCAContext, DcaStats> callback) const {
    lb_dca_context_stats(ctx_,
        [](auto res) {
            auto cb = callback::get_async_callback<DCAContext, DcaStats>(res->userdata);
            DCAContext fctx((const lb_dca_context_t*)res->ctx);
            Status status(res->error);
            if (status) {
                auto r = convert::convert((const lb_dca_stats_t*)res->data);
                (*cb)(AsyncResult<DCAContext, DcaStats>(fctx, std::move(status), &r));
            } else {
                (*cb)(AsyncResult<DCAContext, DcaStats>(fctx, std::move(status), nullptr));
            }
        }, new AsyncCallback<DCAContext, DcaStats>(callback));
}

void DCAContext::check_support(const std::vector<std::string>& symbols,
                               AsyncCallback<DCAContext, DcaSupportList> callback) const {
    std::vector<const char*> ptrs;
    for (auto& s : symbols) ptrs.push_back(s.c_str());
    auto* cb_ptr = new AsyncCallback<DCAContext, DcaSupportList>(callback);
    lb_dca_context_check_support(ctx_, ptrs.data(), ptrs.size(),
        [](auto res) {
            auto cb = callback::get_async_callback<DCAContext, DcaSupportList>(res->userdata);
            DCAContext fctx((const lb_dca_context_t*)res->ctx);
            Status status(res->error);
            if (status) {
                auto r = convert::convert((const lb_dca_support_list_t*)res->data);
                (*cb)(AsyncResult<DCAContext, DcaSupportList>(fctx, std::move(status), &r));
            } else {
                (*cb)(AsyncResult<DCAContext, DcaSupportList>(fctx, std::move(status), nullptr));
            }
        }, cb_ptr);
}

void DCAContext::pause(const std::string& plan_id, AsyncCallback<DCAContext, void> callback) const {
    lb_dca_context_pause(ctx_, plan_id.c_str(),
        [](auto res) {
            auto cb = callback::get_async_callback<DCAContext, void>(res->userdata);
            (*cb)(AsyncResult<DCAContext, void>(DCAContext((const lb_dca_context_t*)res->ctx), Status(res->error), nullptr));
        }, new AsyncCallback<DCAContext, void>(callback));
}

void DCAContext::resume(const std::string& plan_id, AsyncCallback<DCAContext, void> callback) const {
    lb_dca_context_resume(ctx_, plan_id.c_str(),
        [](auto res) {
            auto cb = callback::get_async_callback<DCAContext, void>(res->userdata);
            (*cb)(AsyncResult<DCAContext, void>(DCAContext((const lb_dca_context_t*)res->ctx), Status(res->error), nullptr));
        }, new AsyncCallback<DCAContext, void>(callback));
}

void DCAContext::stop(const std::string& plan_id, AsyncCallback<DCAContext, void> callback) const {
    lb_dca_context_stop(ctx_, plan_id.c_str(),
        [](auto res) {
            auto cb = callback::get_async_callback<DCAContext, void>(res->userdata);
            (*cb)(AsyncResult<DCAContext, void>(DCAContext((const lb_dca_context_t*)res->ctx), Status(res->error), nullptr));
        }, new AsyncCallback<DCAContext, void>(callback));
}

#undef DCA_LIST_CB

#define DCA_TYPED_CB(Resp, CType, c_fn, ...) c_fn(__VA_ARGS__, [](auto res) { \
  auto cb = callback::get_async_callback<DCAContext, Resp>(res->userdata); \
  DCAContext fctx((const lb_dca_context_t*)res->ctx); Status status(res->error); \
  if(status){auto r=convert::convert((const CType*)res->data);(*cb)(AsyncResult<DCAContext,Resp>(fctx,std::move(status),&r));} \
  else{(*cb)(AsyncResult<DCAContext,Resp>(fctx,std::move(status),nullptr));} \
}, new AsyncCallback<DCAContext,Resp>(callback))

void DCAContext::calc_date(const std::string& symbol, int32_t frequency,
                           const std::string& day_of_week, uint32_t day_of_month,
                           AsyncCallback<DCAContext, DcaCalcDateResult> callback) const {
  DCA_TYPED_CB(DcaCalcDateResult, lb_dca_calc_date_result_t, lb_dca_context_calc_date, ctx_, symbol.c_str(), frequency,
              day_of_week.empty()?nullptr:day_of_week.c_str(),
              day_of_month);
}

void DCAContext::set_reminder(const std::string& hours, AsyncCallback<DCAContext, void> callback) const {
  lb_dca_context_set_reminder(ctx_, hours.c_str(),
    [](auto res) {
      auto cb = callback::get_async_callback<DCAContext, void>(res->userdata);
      (*cb)(AsyncResult<DCAContext, void>(DCAContext((const lb_dca_context_t*)res->ctx), Status(res->error), nullptr));
    }, new AsyncCallback<DCAContext, void>(callback));
}

void DCAContext::create_dca(const std::string& symbol, const std::string& amount,
                            int32_t frequency, const std::string& day_of_week,
                            uint32_t day_of_month, bool allow_margin,
                            AsyncCallback<DCAContext, DcaCreateResult> callback) const {
    DCA_TYPED_CB(DcaCreateResult, lb_dca_create_result_t, lb_dca_context_create, ctx_, symbol.c_str(), amount.c_str(), frequency,
        day_of_week.empty()?nullptr:day_of_week.c_str(), day_of_month, allow_margin);
}

void DCAContext::update_dca(const std::string& plan_id, const std::string& amount,
                            int32_t frequency, const std::string& day_of_week,
                            const std::string& day_of_month, int32_t allow_margin,
                            AsyncCallback<DCAContext, DcaCreateResult> callback) const {
    DCA_TYPED_CB(DcaCreateResult, lb_dca_create_result_t, lb_dca_context_update, ctx_, plan_id.c_str(),
        amount.empty()?nullptr:amount.c_str(), frequency,
        day_of_week.empty()?nullptr:day_of_week.c_str(),
        day_of_month.empty()?nullptr:day_of_month.c_str(), allow_margin);
}

void DCAContext::history(const std::string& plan_id, int32_t page, int32_t limit,
                         AsyncCallback<DCAContext, DcaHistoryResponse> callback) const {
    DCA_TYPED_CB(DcaHistoryResponse, lb_dca_history_response_t, lb_dca_context_history,
        ctx_, plan_id.c_str(), page, limit);
}

#undef DCA_TYPED_CB

} // namespace dca
} // namespace longbridge
