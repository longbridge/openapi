#include "content_context.hpp"
#include "convert.hpp"
#include <algorithm>
#include <iterator>

namespace longbridge {
namespace content {

using longbridge::convert::convert;

ContentContext::ContentContext()
  : ctx_(nullptr)
{
}

ContentContext::ContentContext(const lb_content_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_) {
    lb_content_context_retain(ctx_);
  }
}

ContentContext::ContentContext(const ContentContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_content_context_retain(ctx_);
  }
}

ContentContext::ContentContext(ContentContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

ContentContext::~ContentContext()
{
  if (ctx_) {
    lb_content_context_release(ctx_);
  }
}

ContentContext&
ContentContext::operator=(const ContentContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_content_context_retain(ctx_);
  }
  return *this;
}

void
ContentContext::create(const Config& config,
                       AsyncCallback<ContentContext, void> callback)
{
  lb_content_context_new(
    config,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ContentContext, void>(res->userdata);
      auto* ctx_ptr = (lb_content_context_t*)res->ctx;
      ContentContext ctx(ctx_ptr);
      if (ctx_ptr) {
        lb_content_context_release(ctx_ptr);
      }
      (*callback_ptr)(
        AsyncResult<ContentContext, void>(ctx, Status(res->error), nullptr));
    },
    new AsyncCallback<ContentContext, void>(callback));
}

void
ContentContext::topics(
  const std::string& symbol,
  AsyncCallback<ContentContext, std::vector<TopicItem>> callback) const
{
  lb_content_context_topics(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ContentContext, std::vector<TopicItem>>(
          res->userdata);
      ContentContext ctx((const lb_content_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_topic_item_t*)res->data;
        std::vector<TopicItem> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<ContentContext, std::vector<TopicItem>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<ContentContext, std::vector<TopicItem>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<ContentContext, std::vector<TopicItem>>(callback));
}

void
ContentContext::news(
  const std::string& symbol,
  AsyncCallback<ContentContext, std::vector<NewsItem>> callback) const
{
  lb_content_context_news(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ContentContext, std::vector<NewsItem>>(
          res->userdata);
      ContentContext ctx((const lb_content_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_news_item_t*)res->data;
        std::vector<NewsItem> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<ContentContext, std::vector<NewsItem>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<ContentContext, std::vector<NewsItem>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<ContentContext, std::vector<NewsItem>>(callback));
}

} // namespace content
} // namespace longbridge
