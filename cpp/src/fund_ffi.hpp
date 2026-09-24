#pragma once

// FFI mirror structs for the fund module response types.
//
// The fund C FFI layer returns these `#[repr(C)]` structs through the generic
// `void* lb_async_result_t::data` pointer, so cbindgen does not emit them into
// `longbridge.h` (and the raw `CFundPosition` name additionally collides with
// the portfolio `CFundPosition` -> `lb_fund_position_t`). These declarations
// reproduce the exact layout of `c/src/fund_context/types.rs` so the C++
// converters can cast `res->data` and read the fields.
//
// Layout rules mirrored from the Rust `ToFFI` output:
//   * `CString`              -> `const char*`
//   * `CVec<CString>`        -> `const char* const*` + `uintptr_t num_*`
//   * `CVec<T>`              -> `const T*` + `uintptr_t num_*`
//   * nested struct          -> embedded by value
//   * i64/i32/bool           -> int64_t/int32_t/bool
// Field order matches the Rust `#[repr(C)]` declaration order exactly.

#include <cstdint>

namespace longbridge {
namespace fund {
namespace ffi {

struct CFundNavValue
{
  const char* change;
  const char* change_percent;
  const char* change_percent_format;
  const char* counter_id;
  const char* counter_name;
  const char* currency;
  const char* date_format;
  const char* isin;
  int64_t last_update_time;
  const char* value;
  const char* value_format;
};

struct CFundPerformancePoint
{
  const char* date;
  const char* last_done;
};

struct CHotFund
{
  int32_t asset_class;
  const char* asset_class_name;
  const char* counter_id;
  const char* currency;
  const char* earning_rate;
  const CFundPerformancePoint* fund_performances;
  uintptr_t num_fund_performances;
  const char* name;
  const char* purchase_amount;
  const char* recommendation_text;
  int32_t risk_level;
  const char* risk_level_name;
  const char* time_interval;
};

struct CFundBrief
{
  int32_t asset_class;
  const char* asset_class_name;
  const char* code;
  const char* counter_id;
  const char* currency;
  const char* description;
  const char* earning_rate;
  bool holding;
  const char* isin;
  const char* name;
  const char* product;
  const char* purchase_amount;
  const char* recommendation_text;
  int32_t risk_level;
  const char* risk_level_name;
  const char* time_interval;
  const char* unit_value;
};

struct CFundFilters
{
  const char* const* asset_class;
  uintptr_t num_asset_class;
  const char* const* company;
  uintptr_t num_company;
  const char* const* currency;
  uintptr_t num_currency;
  const char* const* industry_category_name;
  uintptr_t num_industry_category_name;
  const char* const* risk_level;
  uintptr_t num_risk_level;
};

struct CFundAssetAllocationItem
{
  const char* code;
  const char* counter_id;
  const char* name;
  const char* position_ratio;
};

struct CFundAssetAllocation
{
  int32_t asset_type;
  const CFundAssetAllocationItem* lists;
  uintptr_t num_lists;
  const char* report_date;
};

struct CFundDetail
{
  const char* additional_purchase_amount;
  int32_t affirm_day;
  const char* amount_affirm_day;
  CFundAssetAllocation asset_allocation;
  int32_t asset_class;
  const char* asset_class_name;
  const char* bill_purchase_rate;
  const char* channel;
  const char* close_period;
  const char* code;
  const char* currency;
  const char* cut_off_time;
  bool derivatives;
  int32_t done_day;
  const char* excess_return_fee;
  const char* gst_rate;
  const char* introduce;
  bool is_cash_plus;
  bool is_complex;
  bool is_new_cash_plus;
  bool is_yinghebao;
  const char* isin;
  const char* manage_rate;
  const char* manager;
  const char* min_hold_cash;
  const char* min_hold_share;
  const char* min_sell_share;
  const char* month_raise_day;
  const char* name;
  const char* nav_deadline;
  bool no_load;
  const char* open_date;
  const char* open_period;
  const char* product;
  const char* product_information_locals;
  const char* profile;
  int32_t purchasable;
  const char* purchase_affirm_day;
  const char* purchase_amount;
  const char* purchase_rate;
  int32_t rating;
  int32_t redeemable;
  const char* redemption_advance_day;
  const char* redemption_amount;
  const char* redemption_close_period_shows;
  const char* redemption_done_day;
  const char* redemption_open_day_shows;
  int32_t risk_level;
  const char* risk_level_name;
  int32_t verify_status;
  bool virtual_currency;
  const char* year_to_date_yield;
  int32_t ytd_yield_type;
};

struct CFundAnalysis
{
  int32_t actual_period;
  const char* cost_level;
  const char* return_ability;
  const char* risk_ability;
  const char* updated_at;
  const char* value_for_money;
  bool visible;
};

struct CFundAnalysisDetail
{
  int32_t actual_period;
  const int32_t* available_periods;
  uintptr_t num_available_periods;
  const char* cost_level;
  const char* return_ability;
  const char* risk_ability;
  const char* updated_at;
  const char* value_for_money;
  bool visible;
};

struct CFundTrendContrast
{
  const char* benchmark_name;
  const char* const* performances;
  uintptr_t num_performances;
};

struct CFundTrend
{
  int32_t actual_period;
  const int32_t* available_periods;
  uintptr_t num_available_periods;
  const char* const* category_average_performances;
  uintptr_t num_category_average_performances;
  CFundTrendContrast contrast_performances;
  const char* const* fund_performances;
  uintptr_t num_fund_performances;
};

struct CFundNamedContrast
{
  const char* name;
  const char* const* performances;
  uintptr_t num_performances;
};

struct CFundPerformanceComparison
{
  const CFundNamedContrast* contrast_performances;
  uintptr_t num_contrast_performances;
  const char* const* fund_performances;
  uintptr_t num_fund_performances;
};

struct CFundAnnualReturn
{
  const char* change_percent;
  int32_t year;
};

struct CFundQuarterlyReturn
{
  const char* change_percent;
  int32_t quarter;
  int32_t year;
};

struct CFundPerformance
{
  const char* annualized_return_five;
  const char* annualized_return_one;
  const char* annualized_return_ten;
  const char* annualized_return_three;
  const char* annualized_return_two;
  const char* counter_id;
  const char* fund_name;
  int32_t performance_rank_five_years;
  int32_t performance_rank_one_day;
  int32_t performance_rank_one_month;
  int32_t performance_rank_one_week;
  int32_t performance_rank_one_year;
  int32_t performance_rank_six_months;
  int32_t performance_rank_ten_years;
  int32_t performance_rank_three_months;
  int32_t performance_rank_three_years;
  int32_t performance_rank_two_years;
  int32_t performance_rank_ytd;
  const char* performance_return_five_years;
  const char* performance_return_one_day;
  const char* performance_return_one_month;
  const char* performance_return_one_week;
  const char* performance_return_one_year;
  const char* performance_return_six_months;
  const char* performance_return_ten_years;
  const char* performance_return_three_months;
  const char* performance_return_three_years;
  const char* performance_return_two_years;
  const char* performance_return_ytd;
  int32_t performance_total_five_years;
  int32_t performance_total_one_day;
  int32_t performance_total_one_month;
  int32_t performance_total_one_week;
  int32_t performance_total_one_year;
  int32_t performance_total_six_months;
  int32_t performance_total_ten_years;
  int32_t performance_total_three_months;
  int32_t performance_total_three_years;
  int32_t performance_total_two_years;
  int32_t performance_total_ytd;
  const char* seven_days_annualized;
  const char* ten_thousand_price;
  int64_t update_time;
};

struct CFundHolding
{
  const char* bond_type;
  const char* bond_type_name;
  const char* country_name;
  const char* holding_type;
  const char* industry_name;
  const char* market_value;
  const char* maturity_date;
  const char* name;
  const char* share_change;
  const char* share_change_percent;
  const char* shares;
  const char* weighting;
};

struct CFundHoldings
{
  const CFundHolding* holdings;
  uintptr_t num_holdings;
  const char* report_date;
  const char* weighting;
};

struct CFundStockHolding
{
  const char* code;
  const char* counter_id;
  const char* currency;
  const char* name;
  const char* position_ratio;
  const char* report_date;
};

struct CFundPosition
{
  const char* amount;
  const char* counter_id;
  const char* currency;
  const char* freeze_units;
  const char* holding_profit;
  const char* holding_units;
  const char* name;
  const char* recent_profit;
  int64_t recent_trading_day;
  const char* sum_recent_profit;
};

struct CFundPositions
{
  const char* account_channel;
  const CFundPosition* list;
  uintptr_t num_list;
  const char* pending_buy_orders;
  int64_t recent_trading_day;
  const char* sold_pending_credit_orders;
};

struct CFundDatedValue
{
  int64_t date;
  const char* value;
};

struct CFundUnitValue
{
  int64_t date;
  const char* day_increase_rate;
  const char* total_value;
  const char* unit_value;
};

struct CFundPositionDetailValues
{
  const char* amount;
  const char* currency;
  const char* holding_cost;
  const char* holding_profit;
  const char* holding_profit_rate;
  const char* holding_units;
  const char* holding_value;
  const char* pending_buy_value;
  const char* pending_sell_value;
  const char* profit_amount_accum_td;
  const char* profit_amount_accum_td_rate;
  const char* recent_profit;
  int64_t recent_tradingday;
  const char* recent_unit_value;
  const char* sold_pending_confirm_units;
};

struct CFundPositionDetail
{
  CFundPositionDetailValues detail_values;
  const CFundDatedValue* sum_profit;
  uintptr_t num_sum_profit;
  const CFundUnitValue* ut_value;
  uintptr_t num_ut_value;
};

struct CFundPositionPerformance
{
  const char* annualized_return_five;
  const char* annualized_return_one;
  const char* annualized_return_ten;
  const char* annualized_return_three;
  const char* annualized_return_two;
  const char* counter_id;
  const char* fund_name;
  const char* performance_return_five_years;
  const char* performance_return_one_day;
  const char* performance_return_one_month;
  const char* performance_return_one_week;
  const char* performance_return_one_year;
  const char* performance_return_six_months;
  const char* performance_return_ten_years;
  const char* performance_return_three_months;
  const char* performance_return_three_years;
  const char* performance_return_two_years;
  const char* performance_return_ytd;
  int64_t update_time;
};

struct CFundPositionProfits
{
  const char* currency;
  const CFundDatedValue* history_value;
  uintptr_t num_history_value;
  int64_t last_update_time;
  const char* sum_profit;
};

struct CFundPositionNav
{
  const char* change;
  const char* change_percent;
  const char* counter_id;
  const char* counter_name;
  int64_t last_update_time;
  const char* value;
};

struct CFundDividend
{
  const char* amount;
  const char* counter_id;
  const char* currency;
  int64_t date;
  const char* div_method;
  const char* name;
};

struct CFundDividends
{
  const char* currency;
  const CFundDividend* div_cash_infos;
  uintptr_t num_div_cash_infos;
  int64_t lastest_date;
  const char* total_div_cash;
};

struct CFundOrder
{
  const char* action;
  const char* amount;
  const char* counter_id;
  int64_t created_at;
  const char* currency;
  const char* fund_name;
  int64_t id;
  bool is_auto;
  const char* net_worth;
  const char* product_type;
  const char* state;
  const char* state_desc;
  const char* units;
};

struct CFundOrderKeyword
{
  const char* content;
  const char* group;
  const char* key;
  const char* line_strategy;
  const char* title;
};

struct CFundOrderStage
{
  const char* desc;
  const char* key;
  const char* link;
  const char* link_text;
  const char* progress;
  const char* stage;
};

struct CFundOrderInfo
{
  int64_t aaid;
  const char* account_channel;
  const char* action;
  const char* amount;
  const char* channel;
  const char* counter_id;
  int64_t created_at;
  const char* currency;
  const char* dividend_option;
  int64_t eq_at;
  const char* fee;
  const char* fund_name;
  const char* fund_source;
  const char* histories;
  int64_t id;
  const char* message;
  const char* net_worth;
  int64_t price_at;
  int64_t processed_at;
  const char* product_type;
  bool repurchaseable;
  const char* sale_proceeds;
  const char* sales_charge;
  const char* sales_price;
  const char* sales_unit;
  const char* state;
  const char* state_desc;
  int32_t status;
  int32_t status_ex;
  const char* t_description;
  const char* time_partition;
  const char* total_amount;
  int64_t transaction_at;
  const char* units;
  int64_t withdraw_at;
  bool withdrawable;
};

struct CFundOrderDetail
{
  const CFundOrderKeyword* keywords;
  uintptr_t num_keywords;
  CFundOrderInfo order;
  const CFundOrderStage* stages;
  uintptr_t num_stages;
};

struct CFundTransaction
{
  const char* amount;
  const char* category;
  int64_t created_at;
  const char* currency;
  const char* description;
  int64_t detail_created_at;
  const char* detail_type;
  int64_t done_at;
  const char* quantity_description;
  const char* redirect_page;
  const char* redirect_page_v2;
  const char* ref_no;
  const char* stock_quantity;
  const char* tx_type;
  const char* type_name;
};

struct CFundOrderValidation
{
  const char* auth_token;
  const char* eval_address;
  int32_t fund_risk_level;
  const char* msg;
  int32_t user_pi;
  int32_t user_risk_level;
};

struct CFundOrderSubmitResponse
{
  const char* action;
  const char* amount;
  const char* counter_id;
  int64_t created_at;
  const char* fund_name;
  int64_t id;
  const char* msg;
  int32_t status;
  const char* units;
};

} // namespace ffi
} // namespace fund
} // namespace longbridge
