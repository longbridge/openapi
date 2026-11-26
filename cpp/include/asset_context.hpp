#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"
#include <map>
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

/// Instrument type a coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class CouponTargetType
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Stocks (server code 0, `StockTrade`)
  Stock,
  /// US stock options (server code 1, `UsStockOptions`)
  UsOption,
  /// HK warrants (server code 2, `HkStockWarrant`)
  HkWarrant,
  /// ETF (server code 3)
  Etf,
  /// LOF (server code 4)
  Lof,
};

/// IPO subscription type an IPO commission coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class IpoCouponTargetType
{
  /// Unknown: unset or an undocumented code (never sent to the server)
  Unknown,
  /// Financing (margin) IPO subscription (server code 1, `FinancingIPO`)
  Financing,
  /// Cash IPO subscription (server code 2, `CashIPO`)
  Cash,
};

/// Kind of commission coupon (the server's `unlimited` code). The meanings are the ones the Longbridge app gives the codes; the proto names are given in brackets.
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class CommissionUnlimitedType
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Time-limited free commission (`Proportionate`, server code 0)
  Proportionate,
  /// Commission rebate (`Full`, server code 1)
  Full,
  /// Commission deduction (`Lifetime`, server code 2)
  Lifetime,
  /// Per-use card with a fixed amount per use: `amount / usage_count` (`ByAmountPer`, server code 6)
  ByAmountPer,
  /// Per-use discount card, capped at `amount_limit` per use (`ByRatePer`, server code 7)
  ByRatePer,
};

/// Investor type a quote coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
enum class QuoteInvestorType
{
  /// Unknown: an undocumented code (never sent to the server)
  Unknown,
  /// Any investor (server code 0, `INVESTOR_TYPE_ANY`)
  Any,
  /// Non-professional investors (server code 1, `INVESTOR_TYPE_NON_PROFESSIONAL`)
  NonProfessional,
  /// Professional investors (server code 2, `INVESTOR_TYPE_PROFESSIONAL`)
  Professional,
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
  /// The server's numeric code of `status`, also when it is a code
  /// this SDK does not know (`status` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> status_raw;
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
  /// The server's numeric code of `status`, also when it is a code
  /// this SDK does not know (`status` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> status_raw;
  /// Witness state
  BankCardWitnessState witness_state;
  /// The server's numeric code of `witness_state`, also when it is a code
  /// this SDK does not know (`witness_state` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> witness_state_raw;
  /// Whether the card supports witnessing
  bool support_witness;
  /// Binding state
  BankCardBindState bind_state;
  /// The server's numeric code of `bind_state`, also when it is a code
  /// this SDK does not know (`bind_state` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> bind_state_raw;
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
  /// Authorization flow: `hk_edda` (authorized at the bank) or `hk_sdda`
  /// (authorized in the app)
  std::string flow_id;
  /// Display name of the authorization
  std::string name;
  /// Authorized currency
  std::string currency;
  /// Application status
  AuthorizationStatus status;
  /// The server's numeric code of `status`, also when it is a code
  /// this SDK does not know (`status` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> status_raw;
  /// Failure reason (empty unless the application failed)
  std::string message;
  /// Time the application was filed as a unix timestamp (null when not set)
  std::optional<int64_t> created_at;
  /// Time the application was last updated as a unix timestamp (null when
  /// not set)
  std::optional<int64_t> updated_at;
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
  /// The server's numeric code of `state_code`, also when it is a code
  /// this SDK does not know (`state_code` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> state_code_raw;
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
  /// The server's numeric code of `state_code`, also when it is a code
  /// this SDK does not know (`state_code` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> state_code_raw;
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
  /// The server's numeric code of `state_code`, also when it is a code
  /// this SDK does not know (`state_code` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> state_code_raw;
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

/// A reward record from the reward center (prize redemption secrets `coupon_secret`, cash card codes `cash_card_code` and shipping contacts are not modelled)
struct RewardRecord
{
  /// Reward type: named after the first detail object the server sent (`cash`, `card`, `prize`, in that order), else the server's `reward_type` code
  RewardType reward_type;
  /// The server's numeric code of `reward_type`, also when it is a code
  /// this SDK does not know (`reward_type` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> reward_type_raw;
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
  /// The server's numeric code of `card_status`, also when it is a code
  /// this SDK does not know (`card_status` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> card_status_raw;
  /// Prize type code (gift rewards; `0` = physical goods), null when not set
  std::optional<int32_t> prize_type;
  /// Amount (null when not set)
  std::optional<Decimal> amount;
  /// Currency
  std::string currency;
  /// Reward status
  RewardStatus status;
  /// The server's numeric code of `status`, also when it is a code
  /// this SDK does not know (`status` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> status_raw;
  /// Delivery status (issued gift rewards only)
  RewardDeliverStatus deliver_status;
  /// The server's numeric code of `deliver_status` (e.g. 11 where `deliver_status` reads `AddressFilled`), also when it is a code
  /// this SDK does not know (`deliver_status` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> deliver_status_raw;
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

/// Commission terms of a commission coupon (`commission`)
struct CouponCommission
{
  /// Markets the coupon applies to
  std::vector<std::string> markets;
  /// Rebate rate, as returned by the server
  std::string rebate_rate;
  /// Instrument types the coupon applies to
  std::vector<CouponTargetType> bz_types;
  /// The server's numeric codes of `bz_types`, in order, including codes this
  /// SDK does not know (which are `Unknown` in `bz_types`)
  std::vector<int32_t> bz_types_raw;
  /// Remaining commission balance (null when not set)
  std::optional<Decimal> commission_balance;
  /// Commission duration in seconds (the app reads 200 years or more as lifetime free commission)
  int64_t commission_duration;
  /// Kind of commission coupon (the server's `unlimited` code)
  CommissionUnlimitedType unlimited;
  /// The server's numeric code of `unlimited`, also when it is a code
  /// this SDK does not know (`unlimited` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> unlimited_raw;
  /// Commission expiry time as a unix timestamp (null when not set)
  std::optional<int64_t> commission_expired_time;
  /// Number of times used
  int64_t usage_count;
  /// Number of uses left
  int64_t lave_usage_count;
  /// Settlement model, as returned by the server
  std::string settlement_model;
  /// Amount cap per use of a per-use discount card (`ByRatePer`), as returned by the server
  int64_t amount_limit;
  /// Whether the coupon can be combined with other coupons
  bool superposition;
};

/// Terms of an AI benefit coupon (`ai_benefit`)
struct CouponAiBenefit
{
  /// Length of the benefit in days
  int32_t benefit_days;
  /// Start of the benefit as a unix timestamp (null when not set)
  std::optional<int64_t> benefit_start;
  /// End of the benefit as a unix timestamp (null when not set)
  std::optional<int64_t> benefit_end;
  /// Benefit tag, as returned by the server
  std::string benefit_tag;
};

/// Terms of a cash coupon (`cash`)
struct CouponCash
{
  /// Receive duration, as returned by the server (its unit and meaning are not confirmed)
  int64_t receive_duration;
};

/// A store of a coffee coupon
struct CouponCoffeeStore
{
  /// Store ID
  int64_t store_id;
  /// Store name
  std::string store_name;
};

/// Terms of a coffee coupon (`coffee_coupon`)
struct CouponCoffee
{
  /// Coupon channel, as returned by the server
  std::string coupon_channel;
  /// Coupon ID, as returned by the server
  std::string coupon_id;
  /// Voucher template code, e.g. `coffee-50` (not a personal redemption code)
  std::string coupon_no;
  /// Whether the coupon can be used at every store
  bool is_available_all_store;
  /// Period type (e.g. `day`), as returned by the server
  std::string period_type;
  /// ID of the store the coupon was settled at (`0` when not settled)
  int64_t settled_store_id;
  /// IDs of the stores the coupon can be used at
  std::vector<int64_t> store_id_list;
  /// Stores the coupon can be used at
  std::vector<CouponCoffeeStore> store_list;
  /// Threshold, as returned by the server
  int64_t threshold;
  /// Use limit (`total` or `period`), as returned by the server
  std::string use_limit;
};

/// A counter (security) of a crypto coupon
struct CouponCryptoCounter
{
  /// Counter ID
  std::string counter_id;
};

/// Fallback of a crypto coupon
struct CouponCryptoFallback
{
  /// Whether the fallback is enabled
  bool enabled;
  /// Counter ID
  std::string counter_id;
};

/// Terms of a crypto coupon (`crypto`)
struct CouponCrypto
{
  /// Fallback
  CouponCryptoFallback fallback;
  /// Optional counters
  std::vector<CouponCryptoCounter> optional_counters;
};

/// Terms of a finance interest coupon (`finance_interest`). The meaning of these fields is not confirmed; they are returned as the server sends them.
struct CouponFinanceInterest
{
  /// Discount mode, as returned by the server
  int32_t discount_mode;
  /// Discount rate (null when not set)
  std::optional<Decimal> discount_rate;
  /// Whether the coupon can be combined with other coupons
  bool superposition;
};

/// Terms of a fund deduction coupon (`fund_deduct`)
struct CouponFundDeduct
{
  /// Maximum risk level, as returned by the server
  int32_t max_risk_level;
  /// Minimum order amount, as returned by the server
  std::string min_order_amount;
  /// Order types, as returned by the server
  std::vector<std::string> order_types;
  /// Underlying type, as returned by the server
  std::string underlying_type;
  /// Underlying values, as returned by the server
  std::vector<std::string> underlying_values;
  /// Underlying names, as returned by the server
  std::vector<std::string> underlying_names;
};

/// Terms of a quote (market data) or fundamental-data coupon (`quote` /
/// `fundamental`)
struct CouponQuote
{
  /// Current end time as a unix timestamp (null when not set)
  std::optional<int64_t> current_end_time;
  /// Whether the coupon is inapplicable, as returned by the server
  bool inapplicable;
  /// Investor type the coupon applies to
  QuoteInvestorType investor_type;
  /// The server's numeric code of `investor_type`, also when it is a code
  /// this SDK does not know (`investor_type` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> investor_type_raw;
  /// Whether the coupon is activated manually
  bool manual_activate;
  /// Markets
  std::vector<std::string> markets;
  /// Investor type of the member, as returned by the server
  int32_t member_investor_type;
  /// Paired SPU number, as returned by the server
  std::string pair_spu_no;
  /// Quote duration in seconds (the app reads 200 years or more as lifetime)
  int64_t quote_duration;
  /// Quote market
  std::string quote_market;
  /// Quote tags
  std::vector<std::string> quote_tags;
  /// Whether the coupon can be combined with other coupons
  bool superposition;
};

/// Terms of an IPO coupon (`ipo`)
struct CouponIpo
{
  /// Subscription ID
  std::string subscription_id;
};

/// Terms of an IPO commission coupon (`ipo_commission`)
struct CouponIpoCommission
{
  /// IPO subscription types the coupon applies to
  std::vector<IpoCouponTargetType> bz_types;
  /// The server's numeric codes of `bz_types`, in order, including codes this
  /// SDK does not know (which are `Unknown` in `bz_types`)
  std::vector<int32_t> bz_types_raw;
  /// Counter ID
  std::string counter_id;
  /// Counter name
  std::string counter_name;
  /// Financing limit, as returned by the server
  std::string financing_limit;
};

/// Terms of an IPO interest coupon (`ipo_interest`)
struct CouponIpoInterest
{
  /// Annual interest discount rate, as returned by the server (the app computes the daily discount as rate × financed amount / 365)
  std::string discount_rate;
};

/// Terms of a platform fee deduction coupon (`platform_fee_deduct`)
struct CouponPlatformFeeDeduct
{
  /// Instrument types the coupon applies to
  std::vector<CouponTargetType> bz_types;
  /// The server's numeric codes of `bz_types`, in order, including codes this
  /// SDK does not know (which are `Unknown` in `bz_types`)
  std::vector<int32_t> bz_types_raw;
  /// Counter ID
  std::string counter_id;
  /// Counter name
  std::string counter_name;
  /// Markets the coupon applies to
  std::vector<std::string> markets;
  /// Number of times used
  int64_t usage_count;
  /// Number of uses left
  int64_t lave_usage_count;
  /// Whether the coupon can be combined with other coupons
  bool superposition;
};

/// An optional counter of a stock coupon
struct CouponStockCounter
{
  /// Counter ID
  std::string counter_id;
  /// Shares, as returned by the server
  std::string shares;
};

/// Fallback of a stock coupon
struct CouponStockFallback
{
  /// Whether the fallback is enabled
  bool enabled;
  /// Fallback type (the server's `type`), as returned by the server
  std::string fallback_type;
  /// Counter ID
  std::string counter_id;
  /// Shares, as returned by the server
  std::string shares;
};

/// Terms of a stock coupon (`stock`)
struct CouponStock
{
  /// Counter ID
  std::string counter_id;
  /// Counter name
  std::string counter_name;
  /// Amount (null when not set)
  std::optional<Decimal> amount;
  /// Redemption type (`by_card_value` or `shares_number`), as returned by the server
  std::string redemption_type;
  /// Delivery way (`card_exchange` or `account_transfer`), as returned by the server
  std::string send_way;
  /// Optional counters
  std::vector<CouponStockCounter> optional_counters;
  /// Fallback
  CouponStockFallback fallback;
};

/// Terms of a stock fragment coupon (`stock_fragment`)
struct CouponStockFragment
{
  /// Counter ID
  std::string counter_id;
  /// Counter name
  std::string counter_name;
  /// Market
  std::string market;
  /// Markets
  std::vector<std::string> markets;
  /// Instrument type the coupon applies to
  CouponTargetType underlying_type;
  /// The server's numeric code of `underlying_type`, also when it is a code
  /// this SDK does not know (`underlying_type` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> underlying_type_raw;
  /// Amounts, as returned by the server
  std::map<std::string, std::string> amounts;
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
  /// The server's numeric code of `card_status`, also when it is a code
  /// this SDK does not know (`card_status` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> card_status_raw;
  /// Start of validity as a unix timestamp (null when not set)
  std::optional<int64_t> start_time;
  /// End of validity (none when not set or when the coupon never expires; see `no_expiry`) as a unix timestamp (null when not set)
  std::optional<int64_t> end_time;
  /// Whether the coupon never expires (the server's end time is in or after 2100)
  bool no_expiry;
  /// Activation time (none when not activated) as a unix timestamp (null when not set)
  std::optional<int64_t> activate_time;
  /// Amount (null when not set)
  std::optional<Decimal> amount;
  /// Currency
  std::string currency;
  /// Trading sessions the coupon applies to, as display text
  std::string available_trade_session;
  /// Trading sessions the coupon applies to, as returned by the server
  std::vector<std::string> trade_sessions;
  /// Subscription ID, as returned by the server
  std::string subscription_id;
  /// Terms of an AI benefit coupon (null for other coupon types)
  std::optional<CouponAiBenefit> ai_benefit;
  /// Terms of a cash coupon (null for other coupon types)
  std::optional<CouponCash> cash;
  /// Terms of a coffee coupon (null for other coupon types)
  std::optional<CouponCoffee> coffee_coupon;
  /// Terms of a commission coupon (null for other coupon types)
  std::optional<CouponCommission> commission;
  /// Terms of a crypto coupon (null for other coupon types)
  std::optional<CouponCrypto> crypto;
  /// Terms of a finance interest coupon (null for other coupon types)
  std::optional<CouponFinanceInterest> finance_interest;
  /// Terms of a fund deduction coupon (null for other coupon types)
  std::optional<CouponFundDeduct> fund_deduct;
  /// Terms of a fundamental-data coupon (null for other coupon types)
  std::optional<CouponQuote> fundamental;
  /// Terms of an IPO coupon (null for other coupon types)
  std::optional<CouponIpo> ipo;
  /// Terms of an IPO commission coupon (null for other coupon types)
  std::optional<CouponIpoCommission> ipo_commission;
  /// Terms of an IPO interest coupon (null for other coupon types)
  std::optional<CouponIpoInterest> ipo_interest;
  /// Terms of a platform fee deduction coupon (null for other coupon types)
  std::optional<CouponPlatformFeeDeduct> platform_fee_deduct;
  /// Terms of a quote (market data) coupon (null for other coupon types)
  std::optional<CouponQuote> quote;
  /// Terms of a stock coupon (null for other coupon types)
  std::optional<CouponStock> stock;
  /// Terms of a stock fragment coupon (null for other coupon types)
  std::optional<CouponStockFragment> stock_fragment;
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
  /// The server's numeric code of `state_code`, also when it is a code
  /// this SDK does not know (`state_code` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> state_code_raw;
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
  /// Number of receipts uploaded with the deposit
  int64_t voucher_count;
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
  /// The server's numeric code of `state_code`, also when it is a code
  /// this SDK does not know (`state_code` is then `Unknown`); null when the
  /// server sent none
  std::optional<int32_t> state_code_raw;
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
  /// Page size (max 20; larger values are sent as 20, because the deposit list returns a short 10-record page for them)
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
