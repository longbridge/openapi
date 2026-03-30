#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_asset_context_t lb_asset_context_t;

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

/// Asset context
class AssetContext
{
private:
  const lb_asset_context_t* ctx_;

public:
  AssetContext();
  AssetContext(const lb_asset_context_t* ctx);
  AssetContext(const AssetContext& ctx);
  AssetContext(AssetContext&& ctx);
  ~AssetContext();

  AssetContext& operator=(const AssetContext& ctx);

  static AssetContext create(const Config& config);

  /// Get statement data list
  void statements(
    int32_t statement_type,
    int32_t start_date,
    int32_t limit,
    AsyncCallback<AssetContext, std::vector<StatementItem>> callback) const;

  /// Get statement data download URL
  void statement_download_url(
    const std::string& file_key,
    AsyncCallback<AssetContext, StatementDownloadUrlResponse> callback)
    const;
};

} // namespace asset
} // namespace longbridge
