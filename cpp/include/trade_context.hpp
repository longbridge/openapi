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
  TradeContext(const lb_trade_context_t* ctx);

  TradeContext(const TradeContext& ctx);

  ~TradeContext();

  static void create(const Config& config,
                     AsyncCallback<TradeContext, void> callback);

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
                    AsyncCallback<TradeContext, void> callback) const;

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
};

} // namespace trade
} // namespace longbridge