#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"
#include <optional>
#include <string>
#include <vector>

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

/// Verification status of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class BankCardStatus
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Not verified (server code 0)
  Unverified,
  /// Under review (server code 1)
  UnderReview,
  /// Verified (server code 2)
  Verified,
};

/// Witness (in-person / video verification) state of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class BankCardWitnessState
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Witnessing not required (server code 1)
  NotRequired,
  /// Witnessing pending (server code 2)
  Pending,
  /// Witnessed (server code 3)
  Witnessed,
};

/// Binding state of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class BankCardBindState
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Binding in progress (server code 0)
  Binding,
  /// Bound (server code 1)
  Bound,
  /// Binding failed (server code 2)
  Failed,
};

/// Status of a bank-card authorization or of an eDDA application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class AuthorizationStatus
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Not authorized (server code 0)
  Unauthorized,
  /// Authorized (server code 1)
  Authorized,
  /// Authorization in progress (server code 2)
  Authorizing,
  /// Authorization failed (server code 3)
  Failed,
};

/// State of a stock transfer-in application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class StockTransferInState
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Submitted (server code 0)
  Submitted,
  /// Cancelled (server code 1)
  Cancelled,
  /// Completed (server code 3)
  Completed,
  /// Processing (server code 5)
  Processing,
  /// Partially completed (server code 7)
  PartiallyCompleted,
  /// Failed (server code 8)
  Failed,
};

/// State of a stock transfer-out application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class StockTransferOutState
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Submitted (the only state that can still be cancelled) (server code 0)
  Submitted,
  /// Processing (server code 1)
  Processing,
  /// Executing (server code 2)
  Executing,
  /// Completed (server code 3)
  Completed,
  /// Cancelled (server code 4)
  Cancelled,
  /// Partially completed (server code 5)
  PartiallyCompleted,
  /// Failed (server code 6)
  Failed,
  /// Rejected (server code 7)
  Rejected,
};

/// State of one stock inside a stock transfer application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class TransferStockState
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Processing (server code 0)
  Processing,
  /// Succeeded (server code 1)
  Succeeded,
  /// Failed (server code 2)
  Failed,
};

/// Type of a reward record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class RewardType
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Cash reward (server code 0)
  Cash,
  /// Gift (prize) (server code 1)
  Gift,
  /// Coupon (server code 2)
  Coupon,
  /// Task reward (server code 3)
  Task,
};

/// Status of a reward record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class RewardStatus
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Upcoming (server code 0)
  Upcoming,
  /// Cancelled (server code 1)
  Cancelled,
  /// The user exited the activity (server code 2)
  ExitedActivity,
  /// Not claimed yet (server code 8)
  Unclaimed,
  /// Being issued (server code 10)
  Issuing,
  /// Issued (server code 12)
  Issued,
  /// Issuing failed (server code 13)
  Failed,
  /// Exchanged (server code 16)
  Exchanged,
  /// Expired (server code 17)
  Expired,
  /// Refunded (server code 18)
  Refunded,
};

/// Delivery status of an issued prize
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class RewardDeliverStatus
{
  /// Unknown: an undocumented code, or no delivery status (never sent to the server)
  Unknown,
  /// Not activated (server code 0)
  NotActivated,
  /// Activated, or shipping (server code 1)
  ActivatedOrShipping,
  /// Consumed (server code 2)
  Consumed,
  /// E-voucher activating (server code 5)
  EVoucherActivating,
  /// E-voucher expired before activation (server code 6)
  EVoucherExpired,
  /// E-voucher invalid (server code 7)
  EVoucherInvalid,
  /// Shipping address missing (server code 8)
  AddressMissing,
  /// Shipping address filled (server code 9)
  AddressFilled,
  /// Shipping (server code 10)
  Shipping,
};

/// Status of a coupon
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class CouponStatus
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Unused (server code 0)
  Unused,
  /// Used (server code 1)
  Used,
  /// Expired (server code 2)
  Expired,
  /// Being redeemed (server code 3)
  Redeeming,
  /// In use (server code 4)
  InUse,
  /// Processing (server code 5)
  Processing,
};

/// State of a deposit record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class DepositState
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Processing (server code 0)
  Processing,
  /// Credited to the account (server code 1)
  Credited,
  /// Failed (server code 2)
  Failed,
  /// Cancelled (server code 3)
  Cancelled,
  /// Invalidated (server code 4)
  Invalidated,
  /// Rejected (server code 5)
  Rejected,
};

/// State of a withdrawal record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class WithdrawalState
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Submitted (server code 0)
  Submitted,
  /// Processing (server code 1)
  Processing,
  /// Rejected (server code 2)
  Rejected,
  /// Cancelled (server code 3)
  Cancelled,
  /// Remitted (server code 4)
  Remitted,
  /// Failed (server code 5)
  Failed,
  /// Refunded (server code 6)
  Refunded,
  /// Approved, awaiting payout (server code 7)
  ApprovedAwaitingPayout,
  /// Large-amount review (server code 8)
  LargeAmountReview,
  /// Change pending approval (server code 9)
  ChangePendingApproval,
  /// Deletion pending approval (server code 10)
  DeletionPendingApproval,
  /// Deleted (server code 11)
  Deleted,
  /// Rejection pending approval (server code 12)
  RejectionPendingApproval,
  /// Awaiting confirmation (server code 15)
  AwaitingConfirmation,
};

/// Which orders of a coupon to list
enum class CouponOrderType
{
  /// Orders the coupon is locked to (in progress)
  Locked,
  /// Orders whose rebate has been settled
  Settled,
};

/// An authorization attached to a bank card (eDDA, bank securities account, FPS, ...)
struct BankCardAuthorization
{
  /// Authorization flow: `hk_edda` / `hk_sdda` (eDDA), `hk_bank_sec`, `hk_fps`, ...
  std::string flow_id;
  /// Display name of the authorization
  std::string name;
  /// Authorized currency
  std::string currency;
  /// Authorization status
  AuthorizationStatus status;
  /// Failure reason (empty unless the authorization failed)
  std::string message;
  /// Whether this authorization is supported for the card
  bool is_support;
  /// Maximum amount per debit (null when not set)
  std::optional<Decimal> max_amount;
  /// Whether the per-debit amount is unlimited
  bool max_amount_unlimited;
  /// Maximum number of debits per day (null when not set)
  std::optional<int64_t> daily_max_count;
  /// Whether the daily debit count is unlimited
  bool daily_max_count_unlimited;
  /// Expiry date of the authorization as a unix timestamp (null when not set
  /// and for a long-term authorization: the server's 9999-12-31 "never
  /// expires" sentinel is null; see `long_term_valid`)
  std::optional<int64_t> expiry_date;
  /// Whether the authorization never expires
  bool long_term_valid;
  /// Whether submitted documents are under review
  bool doc_reviewing;
  /// Whether the "no prior deposit" check is under review
  bool no_prior_deposit_reviewing;
};

/// A linked bank card (holder name, address and ID documents are not modelled)
struct BankCard
{
  /// Bank card ID (pass to edda_applications)
  std::string id;
  /// Bank ID
  std::string bank_id;
  /// Bank name
  std::string bank;
  /// Bank name in English
  std::string bank_en;
  /// Bank code
  std::string bank_code;
  /// Bank account number, as returned by the server (not masked)
  std::string account;
  /// Account type
  std::string account_type;
  /// Country or region of the bank (e.g. `Hong Kong`; not a currency)
  std::string country;
  /// Region code
  std::string region;
  /// Region name
  std::string region_name;
  /// SWIFT code
  std::string swift_code;
  /// Remark
  std::string remark;
  /// Verification status
  BankCardStatus status;
  /// Witness state
  BankCardWitnessState witness_state;
  /// Whether the card supports witnessing
  bool support_witness;
  /// Binding state
  BankCardBindState bind_state;
  /// Binding hint text
  std::string bind_tip;
  /// Withdrawal fee text, as returned by the server
  std::string withdraw_fee;
  /// Time the card was added as a unix timestamp (null when not set)
  std::optional<int64_t> created_at;
  /// Authorizations attached to the card
  std::vector<BankCardAuthorization> auth_list;
};

/// An eDDA authorization application of a bank card
struct EddaApplication
{
  /// Application ID
  std::string id;
  /// Application status
  AuthorizationStatus status;
  /// Failure reason (empty unless the application failed)
  std::string message;
};

/// One stock in a stock transfer application
struct TransferStock
{
  /// Stock code
  std::string code;
  /// Stock name
  std::string name;
  /// Stock name in English
  std::string name_en;
  /// Market
  std::string market;
  /// Counter ID
  std::string counter_id;
  /// Quantity (null when not set)
  std::optional<Decimal> amount;
  /// Currency
  std::string currency;
  /// Cost price (null when not set)
  std::optional<Decimal> cost;
  /// State code of this stock in the transfer
  TransferStockState state_code;
  /// Position side
  std::string position_side;
};

/// A stock transfer-in application (another broker -> this account)
struct StockTransferIn
{
  /// Record ID
  std::string id;
  /// Order ID
  std::string order_id;
  /// Market
  std::string market;
  /// Name of the other broker
  std::string broker;
  /// ID of the other broker
  std::string broker_id;
  /// English name of the other broker
  std::string broker_en;
  /// Broker code of the other broker
  std::string broker_code;
  /// Account number at the other broker, as returned by the server (not masked)
  std::string account;
  /// State text, as returned by the server
  std::string state;
  /// State code
  StockTransferInState state_code;
  /// App-side state: `noticed` means the user tapped "I have notified the
  /// broker" in the app (the user's own confirmation, not a Longbridge
  /// action); `default` otherwise
  std::string app_state;
  /// Whether the application can still be cancelled or changed
  bool can_modify;
  /// Created time as a unix timestamp (null when not set)
  std::optional<int64_t> created_at;
  /// Updated time as a unix timestamp (null when not set)
  std::optional<int64_t> updated_at;
  /// Processing start time as a unix timestamp (null when not set)
  std::optional<int64_t> process_at;
  /// Execution time as a unix timestamp (null when not executed)
  std::optional<int64_t> exec_at;
  /// Remark
  std::string remark;
  /// Reason (e.g. for a failure)
  std::string reason;
  /// Stocks in the application
  std::vector<TransferStock> stocks;
};

/// Response for the stock transfer-in list
struct StockTransferInResponse
{
  /// Records
  std::vector<StockTransferIn> records;
  /// Total number of records
  int64_t total;
};

/// A stock transfer-out application (this account -> another broker)
struct StockTransferOut
{
  /// Record ID
  std::string id;
  /// Order ID
  std::string order_id;
  /// Market
  std::string market;
  /// Name of the other broker
  std::string broker;
  /// ID of the other broker
  std::string broker_id;
  /// English name of the other broker
  std::string broker_en;
  /// Broker code of the other broker
  std::string broker_code;
  /// Account number at the other broker, as returned by the server (not masked)
  std::string account;
  /// State text, as returned by the server
  std::string state;
  /// State code (only `Submitted` applications can be cancelled)
  StockTransferOutState state_code;
  /// App-side state: `noticed` means the user tapped "I have notified the
  /// broker" in the app (the user's own confirmation, not a Longbridge
  /// action); `default` otherwise
  std::string app_state;
  /// Created time as a unix timestamp (null when not set)
  std::optional<int64_t> created_at;
  /// Updated time as a unix timestamp (null when not set)
  std::optional<int64_t> updated_at;
  /// Processing start time as a unix timestamp (null when not set)
  std::optional<int64_t> process_at;
  /// Execution time as a unix timestamp (null when not executed)
  std::optional<int64_t> exec_at;
  /// Time the other broker agreed as a unix timestamp (null when not set)
  std::optional<int64_t> agree_at;
  /// Time the application was refused as a unix timestamp (null when not set)
  std::optional<int64_t> refuse_at;
  /// Time the application was cancelled as a unix timestamp (null when not set)
  std::optional<int64_t> cancel_at;
  /// Remark
  std::string remark;
  /// Reason (e.g. for a rejection)
  std::string reason;
  /// Stocks in the application
  std::vector<TransferStock> stocks;
};

/// A reward record from the reward center (prize secrets and shipping contacts are not modelled)
struct RewardRecord
{
  /// Reward type
  RewardType reward_type;
  /// Record ID
  std::string record_id;
  /// Title of the activity that granted the reward
  std::string activity_title;
  /// Reward title
  std::string title;
  /// Reward description
  std::string description;
  /// Coupon type (coupon rewards), e.g. `commission`
  std::string card_type;
  /// Coupon status (coupon rewards; `CouponStatus::Unknown` otherwise)
  CouponStatus card_status;
  /// Prize type code (gift rewards; `0` = physical goods), null when not set
  std::optional<int32_t> prize_type;
  /// Amount (null when not set)
  std::optional<Decimal> amount;
  /// Currency
  std::string currency;
  /// Reward status
  RewardStatus status;
  /// Delivery status (issued gift rewards only)
  RewardDeliverStatus deliver_status;
  /// Exit status code, as returned by the server
  int32_t exit_status;
  /// Time the reward was sent as a unix timestamp (null when not set)
  std::optional<int64_t> send_time;
  /// Start time as a unix timestamp (null when not set)
  std::optional<int64_t> start_time;
  /// End time as a unix timestamp (null when not set)
  std::optional<int64_t> end_time;
  /// Effective time as a unix timestamp (null when not set)
  std::optional<int64_t> effective_time;
  /// Deadline to claim the reward as a unix timestamp (null when not set)
  std::optional<int64_t> receive_deadline;
};

/// Response for the reward record list
struct RewardRecordsResponse
{
  /// Reward records
  std::vector<RewardRecord> list;
  /// Total number of records
  int64_t total;
};

/// Commission terms of a commission coupon
struct CouponCommission
{
  /// Markets the coupon applies to
  std::vector<std::string> markets;
  /// Rebate rate, as returned by the server
  std::string rebate_rate;
  /// Instrument types the coupon applies to: `0` stocks, `1` options, `2` warrants, `3` ETF, `4` LOF
  std::vector<int32_t> bz_types;
  /// Remaining commission balance (null when not set)
  std::optional<Decimal> commission_balance;
  /// Commission duration, as returned by the server
  std::string commission_duration;
  /// Whether the coupon is unlimited
  bool unlimited;
  /// Commission expiry time as a unix timestamp (null when not set)
  std::optional<int64_t> commission_expired_time;
  /// Number of times used
  int64_t usage_count;
  /// Number of uses left
  int64_t lave_usage_count;
  /// Settlement model, as returned by the server
  std::string settlement_model;
};

/// A coupon (reward card)
struct Coupon
{
  /// Coupon ID (32 hex characters; pass to coupon_orders)
  std::string card_id;
  /// Coupon type, e.g. `commission`, `platform_fee_deduct`, `stock`, `quote`, `cash`
  std::string card_type;
  /// Title
  std::string title;
  /// Secondary title
  std::string title_sec;
  /// Description
  std::string description;
  /// Terms and details
  std::string detail;
  /// Status
  CouponStatus card_status;
  /// Start of validity as a unix timestamp (null when not set)
  std::optional<int64_t> start_time;
  /// End of validity as a unix timestamp (null when not set or when the
  /// coupon never expires; see `no_expiry`)
  std::optional<int64_t> end_time;
  /// Whether the coupon never expires (the server's end time is in or after
  /// 2100)
  bool no_expiry;
  /// Activation time as a unix timestamp (null when not activated)
  std::optional<int64_t> activate_time;
  /// Amount (null when not set)
  std::optional<Decimal> amount;
  /// Currency
  std::string currency;
  /// Trading sessions the coupon applies to, as display text
  std::string available_trade_session;
  /// Commission terms (commission coupons only; empty otherwise)
  CouponCommission commission;
};

/// Response for the coupon list
struct CouponsResponse
{
  /// Pinned coupons shown above the list (repeated on every page)
  std::vector<Coupon> top_list;
  /// Coupons on this page
  std::vector<Coupon> list;
  /// Pinned coupons shown below the list (repeated on every page)
  std::vector<Coupon> bottom_list;
  /// Total number of coupons
  int64_t total;
  /// Number of expired coupons
  int64_t expired_count;
};

/// An order a coupon was applied to
struct CouponOrder
{
  /// Order ID
  std::string order_id;
  /// State text, as returned by the server
  std::string state;
  /// State value, as returned by the server (e.g. `FilledStatus`)
  std::string state_val;
  /// Executed price (null when not set)
  std::optional<Decimal> executed_price;
  /// Executed quantity (null when not set)
  std::optional<Decimal> executed_quantity;
  /// Currency
  std::string currency;
  /// Settlement currency
  std::string settlement_currency;
  /// Rebate amount (null when not set)
  std::optional<Decimal> rebate;
  /// Rebate date, as returned by the server
  std::string rebate_dt;
};

/// A deposit record (receipt images are not modelled)
struct Deposit
{
  /// Record ID
  std::string id;
  /// Created time as a unix timestamp (null when not set)
  std::optional<int64_t> created_at;
  /// Updated time as a unix timestamp (null when not set)
  std::optional<int64_t> updated_at;
  /// Currency
  std::string currency;
  /// Notified (applied) amount (null when not set)
  std::optional<Decimal> amount;
  /// Received amount (`0` = not received yet; null when not set)
  std::optional<Decimal> receivable_amount;
  /// State text, as returned by the server
  std::string state;
  /// State code
  DepositState state_code;
  /// Sub-state code, as returned by the server
  int32_t sub_state_code;
  /// Deposit method ID
  std::string bank_operation_type_id;
  /// Deposit method name
  std::string bank_operation_type_name;
  /// Reference number
  std::string reference_no;
  /// Whether the deposit can still be cancelled
  bool can_cancel;
  /// SWIFT code of the paying bank
  std::string user_bank_swift_code;
};

/// A withdrawal record
struct Withdrawal
{
  /// Record ID
  std::string id;
  /// Order ID
  std::string order_id;
  /// Created time as a unix timestamp (null when not set)
  std::optional<int64_t> created_at;
  /// Currency
  std::string currency;
  /// Applied amount (null when not set)
  std::optional<Decimal> amount;
  /// Fee (null when not set)
  std::optional<Decimal> fee;
  /// Amount paid out (null when not set)
  std::optional<Decimal> settlement_amount;
  /// State code
  WithdrawalState state_code;
  /// Raw state, as returned by the server
  std::string raw_state;
  /// State display text, as returned by the server
  std::string state_description;
  /// Receiving bank account number, as returned by the server (not masked)
  std::string bank_account;
  /// Receiving bank account, as displayed in the app
  std::string display_bank_account;
  /// SWIFT code of the receiving bank
  std::string user_bank_swift_code;
  /// Account channel
  std::string account_channel;
};

/// Response for the withdrawal list
struct WithdrawalsResponse
{
  /// Withdrawal records
  std::vector<Withdrawal> list;
  /// Total number of records
  int64_t total;
};


/// Options for the stock transfer-in list
struct GetStockTransfersInOptions
{
  /// Market filter: `HK`, `US`, `CN` or `SG`
  std::optional<std::string> market;
  /// State filter (`StockTransferInState::Unknown` is ignored); sent to the server as the
  /// state's server code
  std::optional<StockTransferInState> state;
  /// Only applications created at or after this unix timestamp
  std::optional<int64_t> start;
  /// Only applications created at or before this unix timestamp
  std::optional<int64_t> end;
  /// Page number (default 1)
  std::optional<int32_t> page;
  /// Page size (default 20, max 50)
  std::optional<int32_t> size;
};

/// Options for the stock transfer-out list
struct GetStockTransfersOutOptions
{
  /// Market filter: `HK`, `US`, `CN` or `SG`
  std::optional<std::string> market;
  /// State filter (`StockTransferOutState::Unknown` is ignored); sent to the server as the
  /// state's server code
  std::optional<StockTransferOutState> state;
  /// Only applications created at or after this unix timestamp
  std::optional<int64_t> start;
  /// Only applications created at or before this unix timestamp
  std::optional<int64_t> end;
  /// Page number (default 1)
  std::optional<int32_t> page;
  /// Page size (default 20, max 50)
  std::optional<int32_t> size;
};

/// Options for the reward record list
struct GetRewardRecordsOptions
{
  /// Page number (default 1)
  std::optional<int32_t> page;
  /// Page size (max 50)
  std::optional<int32_t> page_size;
  /// Reward type filter (default: all types; `RewardType::Unknown` is
  /// ignored)
  std::optional<RewardType> reward_type;
};

/// Options for the coupon list
struct GetCouponsOptions
{
  /// Page number (default 1)
  std::optional<int32_t> page;
  /// Page size (max 50)
  std::optional<int32_t> page_size;
  /// List expired coupons instead of the ones that have not expired
  bool expired = false;
  /// Coupon type filter, e.g. `commission` (default: all types)
  std::optional<std::string> card_type;
};

/// Options for the deposit list
struct GetDepositsOptions
{
  /// Page number (default 1)
  std::optional<int32_t> page;
  /// Page size
  std::optional<int32_t> size;
  /// State filter
  std::optional<std::vector<DepositState>> states;
  /// Currency filter, e.g. `HKD`
  std::optional<std::vector<std::string>> currencies;
  /// Only deposits applied at or after this unix timestamp
  std::optional<int64_t> start;
  /// Only deposits applied at or before this unix timestamp
  std::optional<int64_t> end;
};

/// Options for the withdrawal list
struct GetWithdrawalsOptions
{
  /// Page number (default 1)
  std::optional<int32_t> page;
  /// Page size
  std::optional<int32_t> size;
  /// Only withdrawals applied at or after this unix timestamp
  std::optional<int64_t> start;
  /// Only withdrawals applied at or before this unix timestamp
  std::optional<int64_t> end;
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

  /// Get the linked bank cards, with their verification, binding and
  /// authorization status (AP accounts only)
  void bank_cards(
    AsyncCallback<AssetContext, std::vector<BankCard>> callback) const;

  /// Get the eDDA authorization applications of a bank card in one currency
  /// (AP accounts only)
  void edda_applications(
    const std::string& card_id,
    const std::string& currency,
    AsyncCallback<AssetContext, std::vector<EddaApplication>> callback) const;

  /// Get stock transfer-in applications (AP accounts only)
  void stock_transfers_in(
    const std::optional<GetStockTransfersInOptions>& opts,
    AsyncCallback<AssetContext, StockTransferInResponse> callback) const;

  /// Get stock transfer-out applications (AP accounts only)
  void stock_transfers_out(
    const std::optional<GetStockTransfersOutOptions>& opts,
    AsyncCallback<AssetContext, std::vector<StockTransferOut>> callback) const;

  /// Get reward records from the reward center (AP accounts only). The route
  /// has no date filter: page through the records and filter on their times.
  void reward_records(
    const std::optional<GetRewardRecordsOptions>& opts,
    AsyncCallback<AssetContext, RewardRecordsResponse> callback) const;

  /// Get coupons (AP accounts only). By default lists the coupons that have
  /// not expired.
  void coupons(const std::optional<GetCouponsOptions>& opts,
               AsyncCallback<AssetContext, CouponsResponse> callback) const;

  /// Get the orders a coupon was applied to (AP accounts only)
  void coupon_orders(
    const std::string& card_id,
    CouponOrderType order_type,
    AsyncCallback<AssetContext, std::vector<CouponOrder>> callback) const;

  /// Get deposit records (AP accounts only)
  void deposits(const std::optional<GetDepositsOptions>& opts,
                AsyncCallback<AssetContext, std::vector<Deposit>> callback) const;

  /// Get withdrawal records (AP accounts only)
  void withdrawals(
    const std::optional<GetWithdrawalsOptions>& opts,
    AsyncCallback<AssetContext, WithdrawalsResponse> callback) const;
};

} // namespace asset
} // namespace longbridge
