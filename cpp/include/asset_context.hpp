#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_statement_context_t lb_statement_context_t;

namespace longbridge {
namespace asset {

/// Statement item
struct StatementItem
{
  /// Statement date (integer, e.g. 20250301)
  int32_t dt;
  /// File key
  std::string file_key;
};

/// Statement download URL response
struct StatementDownloadUrlResponse
{
  /// Presigned download URL
  std::string url;
};

/// Statement context
class StatementContext
{
private:
  const lb_statement_context_t* ctx_;

public:
  StatementContext();
  StatementContext(const lb_statement_context_t* ctx);
  StatementContext(const StatementContext& ctx);
  StatementContext(StatementContext&& ctx);
  ~StatementContext();

  StatementContext& operator=(const StatementContext& ctx);

  static StatementContext create(const Config& config);

  /// Get statement data list
  void statements(
    int32_t statement_type,
    int32_t start_date,
    int32_t limit,
    AsyncCallback<StatementContext, std::vector<StatementItem>> callback) const;

  /// Get statement data download URL
  void statement_download_url(
    const std::string& file_key,
    AsyncCallback<StatementContext, StatementDownloadUrlResponse> callback)
    const;
};

} // namespace asset
} // namespace longbridge
