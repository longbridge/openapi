#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "push.hpp"
#include "types.hpp"

typedef struct lb_trade_context_t lb_trade_context_t;

namespace longbridge {
namespace trade {

/// Trade context
class TradeContext
{
private:
  const lb_trade_context_t* ctx_;

public:
  TradeContext();
  TradeContext(const lb_trade_context_t* ctx);
  TradeContext(const TradeContext& ctx);
  TradeContext(TradeContext&& ctx);
  ~TradeContext();

  TradeContext& operator=(const TradeContext& ctx);

  size_t ref_count() const;

  static TradeContext create(const Config& config);

  /// Subscribe
  void subscribe(const std::vector<TopicType>& topics,
                 AsyncCallback<TradeContext, void> callback) const;

  /// Unsubscribe
  void unsubscribe(const std::vector<TopicType>& topics,
                   AsyncCallback<TradeContext, void> callback) const;

  /// Set order changed callback, after receiving the order changed event, it
  /// will call back to this function.
  void set_on_order_changed(
    PushCallback<TradeContext, PushOrderChanged> callback) const;

  /// Get history executions
  void history_executions(
    const std::optional<GetHistoryExecutionsOptions>& opts,
    AsyncCallback<TradeContext, std::vector<Execution>> callback) const;

  /// Get today executions
  void today_executions(
    const std::optional<GetTodayExecutionsOptions>& opts,
    AsyncCallback<TradeContext, std::vector<Execution>> callback) const;

  // TODO: temporarily disabled — restore when API is available
  /*
  /// Get all executions
  void all_executions(
    const std::optional<GetAllExecutionsOptions>& opts,
    AsyncCallback<TradeContext, AllExecutionsResponse> callback) const;
  */

  /// Get history orders
  void history_orders(
    const std::optional<GetHistoryOrdersOptions>& opts,
    AsyncCallback<TradeContext, std::vector<Order>> callback) const;

  /// Get history orders
  void today_orders(
    const std::optional<GetTodayOrdersOptions>& opts,
    AsyncCallback<TradeContext, std::vector<Order>> callback) const;

  /// Replace order
  void replace_order(const ReplaceOrderOptions& opts,
                     AsyncCallback<TradeContext, void> callback) const;

  /// Submit order
  void submit_order(
    const SubmitOrderOptions& opts,
    AsyncCallback<TradeContext, SubmitOrderResponse> callback) const;

  /// Cancel order
  void cancel_order(const std::string& order_id,
                    AsyncCallback<TradeContext, void> callback,
                    bool is_attached = false) const;

  /// Get account balance with currency
  void account_balance(
    const std::string& currency,
    AsyncCallback<TradeContext, std::vector<AccountBalance>> callback) const;

  /// Get account balance
  void account_balance(
    AsyncCallback<TradeContext, std::vector<AccountBalance>> callback) const;

  /// Get cash flow
  void account_balance(
    const GetCashFlowOptions& opts,
    AsyncCallback<TradeContext, std::vector<CashFlow>> callback) const;

  /// Get fund positions
  void fund_positions(
    const std::optional<GetFundPositionsOptions>& opts,
    AsyncCallback<TradeContext, FundPositionsResponse> callback) const;

  /// Get stock positions
  void stock_positions(
    const std::optional<GetStockPositionsOptions>& opts,
    AsyncCallback<TradeContext, StockPositionsResponse> callback) const;

  /// Get margin ratio
  void margin_ratio(const std::string& symbol,
                    AsyncCallback<TradeContext, MarginRatio> callback) const;

  /// Get order detail
  void order_detail(const std::string& order_id,
                    AsyncCallback<TradeContext, OrderDetail> callback) const;

  /// Get order detail with attached orders
  void order_detail_attached(
    const std::string& order_id,
    AsyncCallback<TradeContext, OrderDetail> callback) const;

  /// Estimating the maximum purchase quantity for Hong Kong and US stocks,
  /// warrants, and options
  void estimate_max_purchase_quantity(
    const EstimateMaxPurchaseQuantityOptions& opts,
    AsyncCallback<TradeContext, EstimateMaxPurchaseQuantityResponse> callback)
    const;

  /// Set grid order changed callback, after receiving the grid order changed
  /// event, it will call back to this function.
  void set_on_grid_order_changed(
    PushCallback<TradeContext, PushGridOrderChanged> callback) const;

  /// Submit a grid trading order
  void submit_grid_order(
    const SubmitGridOrderOptions& opts,
    AsyncCallback<TradeContext, SubmitGridOrderResponse> callback) const;

  /// Replace (modify) a grid trading order
  void replace_grid_order(const ReplaceGridOrderOptions& opts,
                          AsyncCallback<TradeContext, void> callback) const;

  /// Get grid trading orders (paged list)
  void grid_orders(
    const std::optional<GetGridOrdersOptions>& opts,
    AsyncCallback<TradeContext, GridOrdersResponse> callback) const;

  /// Query grid trading orders by IDs
  void grid_orders_by_ids(
    const GetGridOrdersByIdsOptions& opts,
    AsyncCallback<TradeContext, std::vector<GridOrder>> callback) const;

  /// Get grid trading order detail (and paged history)
  void grid_order_detail(
    const GetGridOrderDetailOptions& opts,
    AsyncCallback<TradeContext, GridOrderDetail> callback) const;

  /// Get grid trading trigger history
  void grid_trigger_history(
    const GetGridTriggerHistoryOptions& opts,
    AsyncCallback<TradeContext, GridTriggerHistoryResponse> callback) const;

  /// Cancel a grid trading order
  void cancel_grid_order(const std::string& order_id,
                         AsyncCallback<TradeContext, void> callback) const;

  /// Suspend a grid trading order
  void suspend_grid_order(const std::string& order_id,
                          AsyncCallback<TradeContext, void> callback) const;

  /// Restart a grid trading order
  void restart_grid_order(const std::string& order_id,
                          AsyncCallback<TradeContext, void> callback) const;

  /// Submit the strategy risk-disclosure questionnaire record (grid trading
  /// compliance authorization).
  void submit_strategy_questionnaire(
    AsyncCallback<TradeContext, void> callback) const;

  /// Get order info used by the grid order window (lot size, authorization
  /// flag, settlement currency, etc.).
  void grid_order_info(
    const std::string& symbol,
    AsyncCallback<TradeContext, GridOrderInfo> callback) const;
};

} // namespace trade
} // namespace longbridge