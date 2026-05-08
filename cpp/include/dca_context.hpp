#pragma once
#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_dca_context_t lb_dca_context_t;

namespace longbridge {
namespace dca {

/// Dollar-cost averaging (DCA) plan management context.
class DCAContext {
private:
  const lb_dca_context_t* ctx_;

public:
  DCAContext();
  DCAContext(const lb_dca_context_t* ctx);
  DCAContext(const DCAContext& ctx);
  DCAContext(DCAContext&& ctx);
  ~DCAContext();
  DCAContext& operator=(const DCAContext& ctx);

  /// Create a DCAContext from a Config.
  static DCAContext create(const Config& config);

  /// List DCA plans filtered by status (0=Active, 1=Suspended, 2=Finished).
  void list(int32_t status, AsyncCallback<DCAContext, DcaList> callback) const;
  /// Get DCA statistics (counts, nearest plans, total invested/profit).
  void stats(AsyncCallback<DCAContext, DcaStats> callback) const;
  /// Check whether DCA is supported for the given securities.
  void check_support(const std::vector<std::string>& symbols,
                     AsyncCallback<DCAContext, DcaSupportList> callback) const;
  /// Pause a DCA plan by plan_id.
  void pause(const std::string& plan_id, AsyncCallback<DCAContext, void> callback) const;
  /// Resume a suspended DCA plan by plan_id.
  void resume(const std::string& plan_id, AsyncCallback<DCAContext, void> callback) const;
  /// Stop (permanently finish) a DCA plan by plan_id.
  void stop(const std::string& plan_id, AsyncCallback<DCAContext, void> callback) const;
  /// Calculate next projected trade date. frequency: 0=Daily,1=Weekly,2=Fortnightly,3=Monthly.
  /// Pass empty string for unused day_of_week; pass day_of_month=0 to omit.
  void calc_date(const std::string& symbol, int32_t frequency,
                 const std::string& day_of_week, uint32_t day_of_month,
                 AsyncCallback<DCAContext, DcaCalcDateResult> callback) const;
  /// Update advance reminder hours. hours must be "1", "6", or "12".
  void set_reminder(const std::string& hours, AsyncCallback<DCAContext, void> callback) const;
  /// Create a new DCA plan. frequency: 0=Daily,1=Weekly,2=Fortnightly,3=Monthly.
  /// Pass day_of_month=0 to omit.
  void create_dca(const std::string& symbol, const std::string& amount,
                  int32_t frequency, const std::string& day_of_week,
                  uint32_t day_of_month, bool allow_margin,
                  AsyncCallback<DCAContext, DcaCreateResult> callback) const;
  /// Update a DCA plan. Pass frequency=-1 to keep unchanged. allow_margin: 1=true, 0=false, -1=unchanged.
  void update_dca(const std::string& plan_id, const std::string& amount,
                  int32_t frequency, const std::string& day_of_week,
                  const std::string& day_of_month, int32_t allow_margin,
                  AsyncCallback<DCAContext, DcaCreateResult> callback) const;
  /// Get execution history for a DCA plan (page 1-based, limit = page size).
  void history(const std::string& plan_id, int32_t page, int32_t limit,
               AsyncCallback<DCAContext, DcaHistoryResponse> callback) const;
};

} // namespace dca
} // namespace longbridge
