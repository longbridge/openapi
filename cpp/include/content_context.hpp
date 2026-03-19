#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_content_context_t lb_content_context_t;

namespace longbridge {
namespace content {

/// Content context
class ContentContext
{
private:
  const lb_content_context_t* ctx_;

public:
  ContentContext();
  ContentContext(const lb_content_context_t* ctx);
  ContentContext(const ContentContext& ctx);
  ContentContext(ContentContext&& ctx);
  ~ContentContext();

  ContentContext& operator=(const ContentContext& ctx);

  static void create(const Config& config,
                     AsyncCallback<ContentContext, void> callback);

  /// Get discussion topics list for a symbol
  void topics(const std::string& symbol,
              AsyncCallback<ContentContext, std::vector<TopicItem>> callback) const;

  /// Get news list for a symbol
  void news(const std::string& symbol,
            AsyncCallback<ContentContext, std::vector<NewsItem>> callback) const;
};

} // namespace content
} // namespace longbridge
