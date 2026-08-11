#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_grid_context_t lb_grid_context_t;

namespace longbridge {
namespace grid {

/// Grid trading management context.
class GridContext
{
private:
  const lb_grid_context_t* ctx_;

public:
  GridContext();
  GridContext(const lb_grid_context_t* ctx);
  GridContext(const GridContext& ctx);
  GridContext(GridContext&& ctx);
  ~GridContext();

  GridContext& operator=(const GridContext& ctx);

  /// Create a GridContext from a Config.
  static GridContext create(const Config& config);

  /// Submit a grid trading order
  void submit(
    const SubmitGridOrderOptions& opts,
    AsyncCallback<GridContext, SubmitGridOrderResponse> callback) const;

  /// Replace (modify) a grid trading order
  void replace(const ReplaceGridOrderOptions& opts,
               AsyncCallback<GridContext, void> callback) const;

  /// Get grid trading orders (paged list)
  void list(
    const std::optional<GetGridOrdersOptions>& opts,
    AsyncCallback<GridContext, GridOrdersResponse> callback) const;

  /// Query grid trading orders by IDs
  void list_by_ids(
    const GetGridOrdersByIdsOptions& opts,
    AsyncCallback<GridContext, std::vector<GridOrder>> callback) const;

  /// Get grid trading order detail (and paged history)
  void detail(
    const GetGridOrderDetailOptions& opts,
    AsyncCallback<GridContext, GridOrderDetail> callback) const;

  /// Get grid trading trigger history
  void trigger_history(
    const GetGridTriggerHistoryOptions& opts,
    AsyncCallback<GridContext, GridTriggerHistoryResponse> callback) const;

  /// Cancel a grid trading order
  void cancel(const std::string& order_id,
              AsyncCallback<GridContext, void> callback) const;

  /// Suspend a grid trading order
  void suspend(const std::string& order_id,
               AsyncCallback<GridContext, void> callback) const;

  /// Restart a grid trading order
  void restart(const std::string& order_id,
               AsyncCallback<GridContext, void> callback) const;

  /// Submit the strategy risk-disclosure questionnaire record (grid trading
  /// compliance authorization).
  void submit_strategy_questionnaire(
    AsyncCallback<GridContext, void> callback) const;

  /// Get order info used by the grid order window (lot size, authorization
  /// flag, settlement currency, etc.).
  void order_info(
    const std::string& symbol,
    AsyncCallback<GridContext, GridOrderInfo> callback) const;
};

} // namespace grid
} // namespace longbridge
