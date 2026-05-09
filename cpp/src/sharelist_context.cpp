#include "sharelist_context.hpp"
#include "longbridge.h"
#include "convert.hpp"

extern "C" {
const lb_sharelist_context_t* lb_sharelist_context_new(const lb_config_t*);
void lb_sharelist_context_retain(const lb_sharelist_context_t*);
void lb_sharelist_context_release(const lb_sharelist_context_t*);
void lb_sharelist_context_list(const lb_sharelist_context_t*, uint32_t, lb_async_callback_t, void*);
void lb_sharelist_context_detail(const lb_sharelist_context_t*, int64_t, lb_async_callback_t, void*);
void lb_sharelist_context_popular(const lb_sharelist_context_t*, uint32_t, lb_async_callback_t, void*);
void lb_sharelist_context_create(const lb_sharelist_context_t*, const char*, const char*, lb_async_callback_t, void*);
void lb_sharelist_context_delete(const lb_sharelist_context_t*, int64_t, lb_async_callback_t, void*);
void lb_sharelist_context_add_securities(const lb_sharelist_context_t*, int64_t, const char* const*, size_t, lb_async_callback_t, void*);
void lb_sharelist_context_remove_securities(const lb_sharelist_context_t*, int64_t, const char* const*, size_t, lb_async_callback_t, void*);
void lb_sharelist_context_sort_securities(const lb_sharelist_context_t*, int64_t, const char* const*, size_t, lb_async_callback_t, void*);
}

namespace longbridge {
namespace sharelist {

SharelistContext::SharelistContext() : ctx_(nullptr) {}
SharelistContext::SharelistContext(const lb_sharelist_context_t* ctx) { ctx_ = ctx; if(ctx_) lb_sharelist_context_retain(ctx_); }
SharelistContext::SharelistContext(const SharelistContext& ctx) { ctx_ = ctx.ctx_; if(ctx_) lb_sharelist_context_retain(ctx_); }
SharelistContext::SharelistContext(SharelistContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
SharelistContext::~SharelistContext() { if(ctx_) lb_sharelist_context_release(ctx_); }
SharelistContext& SharelistContext::operator=(const SharelistContext& ctx) { ctx_ = ctx.ctx_; if(ctx_) lb_sharelist_context_retain(ctx_); return *this; }
SharelistContext SharelistContext::create(const Config& config) { auto* ptr = lb_sharelist_context_new(config); SharelistContext ctx(ptr); if(ptr) lb_sharelist_context_release(ptr); return ctx; }

#define SL_LIST_CB(c_fn, ...)                                                     \
  c_fn(__VA_ARGS__,                                                               \
    [](auto res) {                                                                \
      auto cb = callback::get_async_callback<SharelistContext, SharelistList>(res->userdata); \
      SharelistContext fctx((const lb_sharelist_context_t*)res->ctx);             \
      Status status(res->error);                                                  \
      if (status) {                                                               \
        auto r = convert::convert((const lb_sharelist_list_t*)res->data);             \
        (*cb)(AsyncResult<SharelistContext, SharelistList>(fctx, std::move(status), &r)); \
      } else {                                                                    \
        (*cb)(AsyncResult<SharelistContext, SharelistList>(fctx, std::move(status), nullptr)); \
      }                                                                           \
    }, new AsyncCallback<SharelistContext, SharelistList>(callback))

void SharelistContext::list(uint32_t count, AsyncCallback<SharelistContext, SharelistList> callback) const {
    SL_LIST_CB(lb_sharelist_context_list, ctx_, count);
}

void SharelistContext::popular(uint32_t count, AsyncCallback<SharelistContext, SharelistList> callback) const {
    SL_LIST_CB(lb_sharelist_context_popular, ctx_, count);
}

void SharelistContext::detail(int64_t id, AsyncCallback<SharelistContext, SharelistDetail> callback) const {
    lb_sharelist_context_detail(ctx_, id,
        [](auto res) {
            auto cb = callback::get_async_callback<SharelistContext, SharelistDetail>(res->userdata);
            SharelistContext fctx((const lb_sharelist_context_t*)res->ctx);
            Status status(res->error);
            if (status) {
                auto r = convert::convert((const lb_sharelist_detail_t*)res->data);
                (*cb)(AsyncResult<SharelistContext, SharelistDetail>(fctx, std::move(status), &r));
            } else {
                (*cb)(AsyncResult<SharelistContext, SharelistDetail>(fctx, std::move(status), nullptr));
            }
        }, new AsyncCallback<SharelistContext, SharelistDetail>(callback));
}

#undef SL_LIST_CB

#define SL_VOID_CB(c_fn, ...) c_fn(__VA_ARGS__, [](auto res) { \
    auto cb = callback::get_async_callback<SharelistContext, void>(res->userdata); \
    (*cb)(AsyncResult<SharelistContext, void>(SharelistContext((const lb_sharelist_context_t*)res->ctx), Status(res->error), nullptr)); \
}, new AsyncCallback<SharelistContext, void>(callback))

void SharelistContext::create_sharelist(const std::string& name, const std::string& description,
                                        AsyncCallback<SharelistContext, void> callback) const {
    SL_VOID_CB(lb_sharelist_context_create, ctx_, name.c_str(), description.empty()?nullptr:description.c_str());
}

void SharelistContext::delete_sharelist(int64_t id, AsyncCallback<SharelistContext, void> callback) const {
    SL_VOID_CB(lb_sharelist_context_delete, ctx_, id);
}

void SharelistContext::add_securities(int64_t id, const std::vector<std::string>& symbols,
                                      AsyncCallback<SharelistContext, void> callback) const {
    std::vector<const char*> ptrs; for (auto& s : symbols) ptrs.push_back(s.c_str());
    SL_VOID_CB(lb_sharelist_context_add_securities, ctx_, id, ptrs.data(), ptrs.size());
}

void SharelistContext::remove_securities(int64_t id, const std::vector<std::string>& symbols,
                                         AsyncCallback<SharelistContext, void> callback) const {
    std::vector<const char*> ptrs; for (auto& s : symbols) ptrs.push_back(s.c_str());
    SL_VOID_CB(lb_sharelist_context_remove_securities, ctx_, id, ptrs.data(), ptrs.size());
}

void SharelistContext::sort_securities(int64_t id, const std::vector<std::string>& symbols,
                                       AsyncCallback<SharelistContext, void> callback) const {
    std::vector<const char*> ptrs; for (auto& s : symbols) ptrs.push_back(s.c_str());
    SL_VOID_CB(lb_sharelist_context_sort_securities, ctx_, id, ptrs.data(), ptrs.size());
}

#undef SL_VOID_CB

} // namespace sharelist
} // namespace longbridge
