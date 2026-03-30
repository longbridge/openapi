#include "asset_context.hpp"
#include <algorithm>
#include <iterator>

namespace longbridge {
namespace asset {

AssetContext::AssetContext()
  : ctx_(nullptr)
{
}

AssetContext::AssetContext(const lb_asset_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_) {
    lb_asset_context_retain(ctx_);
  }
}

AssetContext::AssetContext(const AssetContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_asset_context_retain(ctx_);
  }
}

AssetContext::AssetContext(AssetContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

AssetContext::~AssetContext()
{
  if (ctx_) {
    lb_asset_context_release(ctx_);
  }
}

AssetContext&
AssetContext::operator=(const AssetContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_asset_context_retain(ctx_);
  }
  return *this;
}

AssetContext
AssetContext::create(const Config& config)
{
  auto* ctx_ptr = lb_asset_context_new(config);
  AssetContext ctx(ctx_ptr);
  if (ctx_ptr) {
    lb_asset_context_release(ctx_ptr);
  }
  return ctx;
}

void
AssetContext::statements(
  int32_t statement_type,
  int32_t start_date,
  int32_t limit,
  AsyncCallback<AssetContext, std::vector<StatementItem>> callback) const
{
  lb_asset_context_statements(
    ctx_,
    statement_type,
    start_date,
    limit,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AssetContext,
                                     std::vector<StatementItem>>(
          res->userdata);
      AssetContext ctx((const lb_asset_context_t*)res->ctx);
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
          AsyncResult<AssetContext, std::vector<StatementItem>>(
            ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(
          AsyncResult<AssetContext, std::vector<StatementItem>>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AssetContext, std::vector<StatementItem>>(callback));
}

void
AssetContext::statement_download_url(
  const std::string& file_key,
  AsyncCallback<AssetContext, StatementDownloadUrlResponse> callback) const
{
  lb_asset_context_download_url(
    ctx_,
    file_key.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AssetContext,
                                     StatementDownloadUrlResponse>(
          res->userdata);
      AssetContext ctx((const lb_asset_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto resp = (const lb_statement_download_url_response_t*)res->data;
        StatementDownloadUrlResponse result;
        result.url = resp->url;

        (*callback_ptr)(
          AsyncResult<AssetContext, StatementDownloadUrlResponse>(
            ctx, std::move(status), &result));
      } else {
        (*callback_ptr)(
          AsyncResult<AssetContext, StatementDownloadUrlResponse>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AssetContext, StatementDownloadUrlResponse>(
      callback));
}

} // namespace asset
} // namespace longbridge
