#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"
#include <optional>
#include <string>

typedef struct lb_forex_context_t lb_forex_context_t;

namespace longbridge {
namespace forex {

/// Forex (currency exchange) channel context.
class ForexContext
{
private:
  const lb_forex_context_t* ctx_;

public:
  ForexContext();
  ForexContext(const lb_forex_context_t* ctx);
  ForexContext(const ForexContext& ctx);
  ForexContext(ForexContext&& ctx);
  ~ForexContext();

  ForexContext& operator=(const ForexContext& ctx);

  /// Create a ForexContext from a Config.
  static ForexContext create(const Config& config);

  /// Get a forex quote.
  ///
  /// `from` / `to` are ISO 4217 currency codes. Provide at most one of
  /// `amount` / `target_amount`.
  void quote(const std::string& from,
             const std::string& to,
             const std::optional<Decimal>& amount,
             const std::optional<Decimal>& target_amount,
             AsyncCallback<ForexContext, ForexQuote> callback) const;

  /// Submit a forex order.
  ///
  /// Success only means the order was accepted; conversion is asynchronous —
  /// poll `order` with the same `client_order_id` for the final state.
  void submit_order(const std::string& quote_id,
                    const std::string& client_order_id,
                    AsyncCallback<ForexContext, void> callback) const;

  /// Query a forex order by `client_order_id`.
  void order(const std::string& client_order_id,
             AsyncCallback<ForexContext, ForexOrderDetail> callback) const;
};

} // namespace forex
} // namespace longbridge
