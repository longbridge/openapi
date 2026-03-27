#include "asset_context.hpp"
#include <algorithm>
#include <iterator>

namespace longbridge {
namespace asset {

StatementContext::StatementContext()
  : ctx_(nullptr)
{
}

StatementContext::StatementContext(const lb_statement_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_) {
    lb_statement_context_retain(ctx_);
  }
}

StatementContext::StatementContext(const StatementContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_statement_context_retain(ctx_);
  }
}

StatementContext::StatementContext(StatementContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

StatementContext::~StatementContext()
{
  if (ctx_) {
    lb_statement_context_release(ctx_);
  }
}

StatementContext&
StatementContext::operator=(const StatementContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_statement_context_retain(ctx_);
  }
  return *this;
}

StatementContext
StatementContext::create(const Config& config)
{
  auto* ctx_ptr = lb_statement_context_new(config);
  StatementContext ctx(ctx_ptr);
  if (ctx_ptr) {
    lb_statement_context_release(ctx_ptr);
  }
  return ctx;
}

void
StatementContext::statements(
  int32_t statement_type,
  int32_t start_date,
  int32_t limit,
  AsyncCallback<StatementContext, std::vector<StatementItem>> callback) const
{
  lb_statement_context_statements(
    ctx_,
    statement_type,
    start_date,
    limit,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<StatementContext,
                                     std::vector<StatementItem>>(
          res->userdata);
      StatementContext ctx((const lb_statement_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_statement_item_t*)res->data;
        std::vector<StatementItem> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](const auto& row) {
                         StatementItem item;
                         item.dt = row.dt;
                         item.file_key = row.file_key;
                         return item;
                       });

        (*callback_ptr)(
          AsyncResult<StatementContext, std::vector<StatementItem>>(
            ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(
          AsyncResult<StatementContext, std::vector<StatementItem>>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<StatementContext, std::vector<StatementItem>>(callback));
}

void
StatementContext::statement_download_url(
  const std::string& file_key,
  AsyncCallback<StatementContext, StatementDownloadUrlResponse> callback) const
{
  lb_statement_context_download_url(
    ctx_,
    file_key.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<StatementContext,
                                     StatementDownloadUrlResponse>(
          res->userdata);
      StatementContext ctx((const lb_statement_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto resp = (const lb_statement_download_url_response_t*)res->data;
        StatementDownloadUrlResponse result;
        result.url = resp->url;

        (*callback_ptr)(
          AsyncResult<StatementContext, StatementDownloadUrlResponse>(
            ctx, std::move(status), &result));
      } else {
        (*callback_ptr)(
          AsyncResult<StatementContext, StatementDownloadUrlResponse>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<StatementContext, StatementDownloadUrlResponse>(
      callback));
}

} // namespace asset
} // namespace longbridge
