#pragma once
#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_alert_context_t lb_alert_context_t;

namespace longbridge {
namespace alert {

/// Price alert management context.
class AlertContext {
private:
  const lb_alert_context_t* ctx_;

public:
  AlertContext();
  AlertContext(const lb_alert_context_t* ctx);
  AlertContext(const AlertContext& ctx);
  AlertContext(AlertContext&& ctx);
  ~AlertContext();
  AlertContext& operator=(const AlertContext& ctx);

  /// Create an AlertContext from a Config.
  static AlertContext create(const Config& config);

  /// List all price alerts grouped by security.
  void list(AsyncCallback<AlertContext, AlertList> callback) const;
  /// Add a price alert. condition: 1=price_rise,2=price_fall,3=pct_rise,4=pct_fall. frequency: 1=daily,2=every_time,3=once.
  void add(const std::string& symbol, int32_t condition,
           const std::string& trigger_value, int32_t frequency,
           AsyncCallback<AlertContext, void> callback) const;
  /// Enable a price alert by alert_id.
  void enable(const std::string& alert_id,
              AsyncCallback<AlertContext, void> callback) const;
  /// Disable a price alert by alert_id.
  void disable(const std::string& alert_id,
               AsyncCallback<AlertContext, void> callback) const;
};

} // namespace alert
} // namespace longbridge
