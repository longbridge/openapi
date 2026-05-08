#pragma once
#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_sharelist_context_t lb_sharelist_context_t;

namespace longbridge {
namespace sharelist {

/// Community sharelist management context.
class SharelistContext {
private:
  const lb_sharelist_context_t* ctx_;

public:
  SharelistContext();
  SharelistContext(const lb_sharelist_context_t* ctx);
  SharelistContext(const SharelistContext& ctx);
  SharelistContext(SharelistContext&& ctx);
  ~SharelistContext();
  SharelistContext& operator=(const SharelistContext& ctx);

  /// Create a SharelistContext from a Config.
  static SharelistContext create(const Config& config);

  /// List the user's own and subscribed sharelists (up to count entries).
  void list(uint32_t count, AsyncCallback<SharelistContext, SharelistList> callback) const;
  /// Get sharelist detail (including constituent stocks) by ID.
  void detail(int64_t id, AsyncCallback<SharelistContext, SharelistDetail> callback) const;
  /// Get popular (trending) sharelists (up to count entries).
  void popular(uint32_t count, AsyncCallback<SharelistContext, SharelistList> callback) const;
  /// Create a new sharelist. description may be empty. Returns no data.
  void create_sharelist(const std::string& name, const std::string& description,
                        AsyncCallback<SharelistContext, void> callback) const;
  /// Delete a sharelist by ID.
  void delete_sharelist(int64_t id, AsyncCallback<SharelistContext, void> callback) const;
  /// Add securities (symbols) to a sharelist.
  void add_securities(int64_t id, const std::vector<std::string>& symbols,
                      AsyncCallback<SharelistContext, void> callback) const;
  /// Remove securities (symbols) from a sharelist.
  void remove_securities(int64_t id, const std::vector<std::string>& symbols,
                         AsyncCallback<SharelistContext, void> callback) const;
  /// Reorder securities in a sharelist.
  void sort_securities(int64_t id, const std::vector<std::string>& symbols,
                       AsyncCallback<SharelistContext, void> callback) const;
};

} // namespace sharelist
} // namespace longbridge
