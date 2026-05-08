#include "alert_context.hpp"
#include "longbridge.h"
#include "convert.hpp"

extern "C" {
const lb_alert_context_t* lb_alert_context_new(const lb_config_t*);
void lb_alert_context_retain(const lb_alert_context_t*);
void lb_alert_context_release(const lb_alert_context_t*);
void lb_alert_context_list(const lb_alert_context_t*, lb_async_callback_t, void*);
void lb_alert_context_add(const lb_alert_context_t*, const char*, int32_t, const char*, int32_t, lb_async_callback_t, void*);
void lb_alert_context_enable(const lb_alert_context_t*, const char*, lb_async_callback_t, void*);
void lb_alert_context_disable(const lb_alert_context_t*, const char*, lb_async_callback_t, void*);
}

namespace longbridge {
namespace alert {

AlertContext::AlertContext() : ctx_(nullptr) {}
AlertContext::AlertContext(const lb_alert_context_t* ctx) { ctx_ = ctx; if(ctx_) lb_alert_context_retain(ctx_); }
AlertContext::AlertContext(const AlertContext& ctx) { ctx_ = ctx.ctx_; if(ctx_) lb_alert_context_retain(ctx_); }
AlertContext::AlertContext(AlertContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
AlertContext::~AlertContext() { if(ctx_) lb_alert_context_release(ctx_); }
AlertContext& AlertContext::operator=(const AlertContext& ctx) { ctx_ = ctx.ctx_; if(ctx_) lb_alert_context_retain(ctx_); return *this; }
AlertContext AlertContext::create(const Config& config) { auto* ptr = lb_alert_context_new(config); AlertContext ctx(ptr); if(ptr) lb_alert_context_release(ptr); return ctx; }

void AlertContext::list(AsyncCallback<AlertContext, AlertList> callback) const {
    lb_alert_context_list(ctx_,
        [](auto res) {
            auto cb = callback::get_async_callback<AlertContext, AlertList>(res->userdata);
            AlertContext fctx((const lb_alert_context_t*)res->ctx);
            Status status(res->error);
            if (status) {
                auto r = convert::convert((const lb_alert_list_t*)res->data);
                (*cb)(AsyncResult<AlertContext, AlertList>(fctx, std::move(status), &r));
            } else {
                (*cb)(AsyncResult<AlertContext, AlertList>(fctx, std::move(status), nullptr));
            }
        }, new AsyncCallback<AlertContext, AlertList>(callback));
}

void AlertContext::add(const std::string& symbol, int32_t condition,
                       const std::string& trigger_value, int32_t frequency,
                       AsyncCallback<AlertContext, void> callback) const {
    lb_alert_context_add(ctx_, symbol.c_str(), condition, trigger_value.c_str(), frequency,
        [](auto res) {
            auto cb = callback::get_async_callback<AlertContext, void>(res->userdata);
            AlertContext fctx((const lb_alert_context_t*)res->ctx);
            Status status(res->error);
            (*cb)(AsyncResult<AlertContext, void>(fctx, std::move(status), nullptr));
        }, new AsyncCallback<AlertContext, void>(callback));
}

void AlertContext::enable(const std::string& alert_id,
                          AsyncCallback<AlertContext, void> callback) const {
    lb_alert_context_enable(ctx_, alert_id.c_str(),
        [](auto res) {
            auto cb = callback::get_async_callback<AlertContext, void>(res->userdata);
            AlertContext fctx((const lb_alert_context_t*)res->ctx);
            Status status(res->error);
            (*cb)(AsyncResult<AlertContext, void>(fctx, std::move(status), nullptr));
        }, new AsyncCallback<AlertContext, void>(callback));
}

void AlertContext::disable(const std::string& alert_id,
                           AsyncCallback<AlertContext, void> callback) const {
    lb_alert_context_disable(ctx_, alert_id.c_str(),
        [](auto res) {
            auto cb = callback::get_async_callback<AlertContext, void>(res->userdata);
            AlertContext fctx((const lb_alert_context_t*)res->ctx);
            Status status(res->error);
            (*cb)(AsyncResult<AlertContext, void>(fctx, std::move(status), nullptr));
        }, new AsyncCallback<AlertContext, void>(callback));
}

} // namespace alert
} // namespace longbridge
