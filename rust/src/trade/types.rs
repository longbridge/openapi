use num_enum::{FromPrimitive, IntoPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::{Display, EnumString};
use time::{Date, OffsetDateTime};

use crate::{serde_utils, Market};

/// Order type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
#[allow(clippy::upper_case_acronyms)]
pub enum OrderType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Limit Order
    #[strum(serialize = "LO")]
    LO,
    /// Enhanced Limit Order
    #[strum(serialize = "ELO")]
    ELO,
    /// Market Order
    #[strum(serialize = "MO")]
    MO,
    /// At-auction Order
    #[strum(serialize = "AO")]
    AO,
    /// At-auction Limit Order
    #[strum(serialize = "ALO")]
    ALO,
    /// Odd Lots
    #[strum(serialize = "ODD")]
    ODD,
    /// Limit If Touched
    #[strum(serialize = "LIT")]
    LIT,
    /// Market If Touched
    #[strum(serialize = "MIT")]
    MIT,
    /// Trailing Limit If Touched (Trailing Amount)
    #[strum(serialize = "TSLPAMT")]
    TSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    #[strum(serialize = "TSLPPCT")]
    TSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    #[strum(serialize = "TSMAMT")]
    TSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    #[strum(serialize = "TSMPCT")]
    TSMPCT,
}

/// Order status
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderStatus {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Not reported
    #[strum(serialize = "NotReported")]
    NotReported,
    /// Not reported (Replaced Order)
    #[strum(serialize = "ReplacedNotReported")]
    ReplacedNotReported,
    /// Not reported (Protected Order)
    #[strum(serialize = "ProtectedNotReported")]
    ProtectedNotReported,
    /// Not reported (Conditional Order)
    #[strum(serialize = "VarietiesNotReported")]
    VarietiesNotReported,
    /// Filled
    #[strum(serialize = "FilledStatus")]
    Filled,
    /// Wait To New
    #[strum(serialize = "WaitToNew")]
    WaitToNew,
    /// New
    #[strum(serialize = "NewStatus")]
    New,
    /// Wait To Replace
    #[strum(serialize = "WaitToReplace")]
    WaitToReplace,
    /// Pending Replace
    #[strum(serialize = "PendingReplaceStatus")]
    PendingReplace,
    /// Replaced
    #[strum(serialize = "ReplacedStatus")]
    Replaced,
    /// Partial Filled
    #[strum(serialize = "PartialFilledStatus")]
    PartialFilled,
    /// Wait To Cancel
    #[strum(serialize = "WaitToCancel")]
    WaitToCancel,
    /// Pending Cancel
    #[strum(serialize = "PendingCancelStatus")]
    PendingCancel,
    /// Rejected
    #[strum(serialize = "RejectedStatus")]
    Rejected,
    /// Canceled
    #[strum(serialize = "CanceledStatus")]
    Canceled,
    /// Expired
    #[strum(serialize = "ExpiredStatus")]
    Expired,
    /// Partial Withdrawal
    #[strum(serialize = "PartialWithdrawal")]
    PartialWithdrawal,
}

/// Execution
#[derive(Debug, Clone, Deserialize)]
pub struct Execution {
    /// Order ID
    pub order_id: String,
    /// Execution ID
    pub trade_id: String,
    /// Security code
    pub symbol: String,
    /// Trade done time
    #[serde(with = "serde_utils::timestamp")]
    pub trade_done_at: OffsetDateTime,
    /// Executed quantity
    #[serde(with = "serde_utils::int64_str")]
    pub quantity: i64,
    /// Executed price
    pub price: Decimal,
}

/// Order side
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderSide {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Buy
    #[strum(serialize = "Buy")]
    Buy,
    /// Sell
    #[strum(serialize = "Sell")]
    Sell,
}

/// Order trigger price type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TriggerPriceType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Limit If Touched
    #[strum(serialize = "LIT")]
    LimitIfTouched,
    /// Market If Touched
    #[strum(serialize = "MIT")]
    MarketIfTouched,
}

/// Order tag
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OrderTag {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Normal Order
    #[strum(serialize = "Normal")]
    Normal,
    /// Long term Order
    #[strum(serialize = "GTC")]
    LongTerm,
    /// Grey Order
    #[strum(serialize = "Grey")]
    Grey,
}

/// Time in force Type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TimeInForceType {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Day Order
    #[strum(serialize = "Day")]
    Day,
    /// Good Til Canceled Order
    #[strum(serialize = "GTC")]
    GoodTilCanceled,
    /// Good Til Date Order
    #[strum(serialize = "GTD")]
    GoodTilDate,
}

/// Trigger status
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum TriggerStatus {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Deactive
    #[strum(serialize = "DEACTIVE")]
    Deactive,
    /// Active
    #[strum(serialize = "ACTIVE")]
    Active,
    /// Released
    #[strum(serialize = "RELEASED")]
    Released,
}

/// Enable or disable outside regular trading hours
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum OutsideRTH {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// Regular trading hour only
    #[strum(serialize = "RTH_ONLY")]
    RTHOnly,
    /// Any time
    #[strum(serialize = "ANY_TIME")]
    AnyTime,
}

/// Order
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    /// Order ID
    pub order_id: String,
    /// Order status
    pub status: OrderStatus,
    /// Stock name
    pub stock_name: String,
    /// Submitted quantity
    #[serde(with = "serde_utils::int64_str")]
    pub quantity: i64,
    /// Executed quantity
    #[serde(with = "serde_utils::int64_str")]
    pub executed_quantity: i64,
    /// Submitted price
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub price: Option<Decimal>,
    /// Executed price
    #[serde(with = "serde_utils::decimal_opt_0_is_none")]
    pub executed_price: Option<Decimal>,
    /// Submitted time
    #[serde(with = "serde_utils::timestamp")]
    pub submitted_at: OffsetDateTime,
    /// Order side
    pub side: OrderSide,
    /// Security code
    pub symbol: String,
    /// Order type
    pub order_type: OrderType,
    /// Last done
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub last_done: Option<Decimal>,
    /// `LIT` / `MIT` Order Trigger Price
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trigger_price: Option<Decimal>,
    /// Rejected Message or remark
    pub msg: String,
    /// Order tag
    pub tag: OrderTag,
    /// Time in force type
    pub time_in_force: TimeInForceType,
    /// Long term order expire date
    #[serde(with = "serde_utils::date_opt")]
    pub expire_date: Option<Date>,
    /// Last updated time
    #[serde(with = "serde_utils::timestamp_opt")]
    pub updated_at: Option<OffsetDateTime>,
    /// Conditional order trigger time
    #[serde(with = "serde_utils::timestamp_opt")]
    pub trigger_at: Option<OffsetDateTime>,
    /// `TSMAMT` / `TSLPAMT` order trailing amount
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trailing_amount: Option<Decimal>,
    /// `TSMPCT` / `TSLPPCT` order trailing percent
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub trailing_percent: Option<Decimal>,
    /// `TSLPAMT` / `TSLPPCT` order limit offset amount
    #[serde(with = "serde_utils::decimal_opt_empty_is_none")]
    pub limit_offset: Option<Decimal>,
    /// Conditional order trigger status
    #[serde(with = "serde_utils::trigger_status")]
    pub trigger_status: Option<TriggerStatus>,
    /// Currency
    pub currency: String,
    /// Enable or disable outside regular trading hours
    #[serde(with = "serde_utils::outside_rth")]
    pub outside_rth: Option<OutsideRTH>,
}

/// Cash info
#[derive(Debug, Clone, Deserialize)]
pub struct CashInfo {
    /// Withdraw cash
    pub withdraw_cash: Decimal,
    /// Available cash
    pub available_cash: Decimal,
    /// Frozen cash
    pub frozen_cash: Decimal,
    /// Cash to be settled
    pub settling_cash: Decimal,
    /// Currency
    pub currency: String,
}

/// Account balance
#[derive(Debug, Clone, Deserialize)]
pub struct AccountBalance {
    /// Total cash
    pub total_cash: Decimal,
    /// Maximum financing amount
    pub max_finance_amount: Decimal,
    /// Remaining financing amount
    pub remaining_finance_amount: Decimal,
    /// Risk control level
    #[serde(with = "serde_utils::risk_level")]
    pub risk_level: i32,
    /// Margin call
    pub margin_call: Decimal,
    /// Currency
    pub currency: String,
    /// Cash details
    #[serde(default)]
    pub cash_infos: Vec<CashInfo>,
    /// Net assets
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub net_assets: Decimal,
    /// Initial margin
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub init_margin: Decimal,
    /// Maintenance margin
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub maintenance_margin: Decimal,
}

/// Balance type
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BalanceType {
    /// Unknown
    #[num_enum(default)]
    Unknown = 0,
    /// Cash
    Cash = 1,
    /// Stock
    Stock = 2,
    /// Fund
    Fund = 3,
}

impl Serialize for BalanceType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value: i32 = (*self).into();
        value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BalanceType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        Ok(BalanceType::from(value))
    }
}

/// Cash flow direction
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum CashFlowDirection {
    /// Unknown
    #[num_enum(default)]
    Unknown,
    /// Out
    Out = 1,
    /// In
    In = 2,
}

impl<'de> Deserialize<'de> for CashFlowDirection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        Ok(CashFlowDirection::from(value))
    }
}

/// Cash flow
#[derive(Debug, Clone, Deserialize)]
pub struct CashFlow {
    /// Cash flow name
    pub transaction_flow_name: String,
    /// Outflow direction
    pub direction: CashFlowDirection,
    /// Balance type
    pub business_type: BalanceType,
    /// Cash amount
    pub balance: Decimal,
    /// Cash currency
    pub currency: String,
    /// Business time
    #[serde(with = "serde_utils::timestamp")]
    pub business_time: OffsetDateTime,
    /// Associated Stock code information
    #[serde(with = "serde_utils::symbol_opt")]
    pub symbol: Option<String>,
    /// Cash flow description
    pub description: String,
}

/// Fund positions response
#[derive(Debug, Clone, Deserialize)]
pub struct FundPositionsResponse {
    /// Channels
    #[serde(rename = "list")]
    pub channels: Vec<FundPositionChannel>,
}

/// Fund position channel
#[derive(Debug, Clone, Deserialize)]
pub struct FundPositionChannel {
    /// Account type
    pub account_channel: String,

    /// Fund positions
    #[serde(default, rename = "fund_info")]
    pub positions: Vec<FundPosition>,
}

/// Fund position
#[derive(Debug, Clone, Deserialize)]
pub struct FundPosition {
    /// Fund ISIN code
    pub symbol: String,
    /// Current equity
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub current_net_asset_value: Decimal,
    /// Current equity time
    #[serde(with = "serde_utils::timestamp")]
    pub net_asset_value_day: OffsetDateTime,
    /// Fund name
    pub symbol_name: String,
    /// Currency
    pub currency: String,
    /// Net cost
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub cost_net_asset_value: Decimal,
    /// Holding units
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub holding_units: Decimal,
}

/// Stock positions response
#[derive(Debug, Clone, Deserialize)]
pub struct StockPositionsResponse {
    /// Channels
    #[serde(rename = "list")]
    pub channels: Vec<StockPositionChannel>,
}

/// Stock position channel
#[derive(Debug, Clone, Deserialize)]
pub struct StockPositionChannel {
    /// Account type
    pub account_channel: String,

    /// Stock positions
    #[serde(default, rename = "stock_info")]
    pub positions: Vec<StockPosition>,
}

/// Stock position
#[derive(Debug, Clone, Deserialize)]
pub struct StockPosition {
    /// Stock code
    pub symbol: String,
    /// Stock name
    pub symbol_name: String,
    /// The number of holdings
    #[serde(with = "serde_utils::int64_str")]
    pub quantity: i64,
    /// Available quantity
    #[serde(with = "serde_utils::int64_str")]
    pub available_quantity: i64,
    /// Currency
    pub currency: String,
    /// Cost Price(According to the client's choice of average purchase or
    /// diluted cost)
    #[serde(with = "serde_utils::decimal_empty_is_0")]
    pub cost_price: Decimal,
    /// Market
    pub market: Market,
}

/// Margin ratio
#[derive(Debug, Clone, Deserialize)]
pub struct MarginRatio {
    /// Initial margin ratio
    pub im_factor: Decimal,
    /// Maintain the initial margin ratio
    pub mm_factor: Decimal,
    /// Forced close-out margin ratio
    pub fm_factor: Decimal,
}

impl_serde_for_enum_string!(
    OrderType,
    OrderStatus,
    OrderSide,
    TriggerPriceType,
    OrderTag,
    TimeInForceType,
    TriggerStatus,
    OutsideRTH
);
impl_default_for_enum_string!(
    OrderType,
    OrderStatus,
    OrderSide,
    TriggerPriceType,
    OrderTag,
    TimeInForceType,
    TriggerStatus,
    OutsideRTH
);

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn fund_position_response() {
        let data = r#"
        {
            "list": [{
                "account_channel": "lb",
                "fund_info": [{
                    "symbol": "HK0000447943",
                    "symbol_name": "高腾亚洲收益基金",
                    "currency": "USD",
                    "holding_units": "5.000",
                    "current_net_asset_value": "0",
                    "cost_net_asset_value": "0.00",
                    "net_asset_value_day": "1649865600"
                }]
            }]
        }
        "#;

        let resp: FundPositionsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.channels.len(), 1);

        let channel = &resp.channels[0];
        assert_eq!(channel.account_channel, "lb");
        assert_eq!(channel.positions.len(), 1);

        let position = &channel.positions[0];
        assert_eq!(position.symbol, "HK0000447943");
        assert_eq!(position.symbol_name, "高腾亚洲收益基金");
        assert_eq!(position.currency, "USD");
        assert_eq!(position.current_net_asset_value, decimal!(0i32));
        assert_eq!(position.cost_net_asset_value, decimal!(0i32));
        assert_eq!(position.holding_units, decimal!(5i32));
        assert_eq!(position.net_asset_value_day, datetime!(2022-4-14 0:00 +8));
    }

    #[test]
    fn stock_position_response() {
        let data = r#"
        {
            "list": [
              {
                "account_channel": "lb",
                "stock_info": [
                  {
                    "symbol": "700.HK",
                    "symbol_name": "腾讯控股",
                    "currency": "HK",
                    "quantity": "650",
                    "available_quantity": "-450",
                    "cost_price": "457.53",
                    "market": "HK"
                  },
                  {
                    "symbol": "9991.HK",
                    "symbol_name": "宝尊电商-SW",
                    "currency": "HK",
                    "quantity": "200",
                    "available_quantity": "0",
                    "cost_price": "32.25",
                    "market": "HK"
                  }
                ]
              }
            ]
          }
        "#;

        let resp: StockPositionsResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.channels.len(), 1);

        let channel = &resp.channels[0];
        assert_eq!(channel.account_channel, "lb");
        assert_eq!(channel.positions.len(), 2);

        let position = &channel.positions[0];
        assert_eq!(position.symbol, "700.HK");
        assert_eq!(position.symbol_name, "腾讯控股");
        assert_eq!(position.currency, "HK");
        assert_eq!(position.quantity, 650);
        assert_eq!(position.available_quantity, -450);
        assert_eq!(position.cost_price, decimal!(457.53f32));
        assert_eq!(position.market, Market::HK);

        let position = &channel.positions[0];
        assert_eq!(position.symbol, "700.HK");
        assert_eq!(position.symbol_name, "腾讯控股");
        assert_eq!(position.currency, "HK");
        assert_eq!(position.quantity, 650);
        assert_eq!(position.available_quantity, -450);
        assert_eq!(position.cost_price, decimal!(457.53f32));
        assert_eq!(position.market, Market::HK);

        let position = &channel.positions[1];
        assert_eq!(position.symbol, "9991.HK");
        assert_eq!(position.symbol_name, "宝尊电商-SW");
        assert_eq!(position.currency, "HK");
        assert_eq!(position.quantity, 200);
        assert_eq!(position.available_quantity, 0);
        assert_eq!(position.cost_price, decimal!(32.25f32));
    }

    #[test]
    fn cash_flow() {
        let data = r#"
        {
            "list": [
              {
                "transaction_flow_name": "BuyContract-Stocks",
                "direction": 1,
                "balance": "-248.60",
                "currency": "USD",
                "business_type": 1,
                "business_time": "1621507957",
                "symbol": "AAPL.US",
                "description": "AAPL"
              },
              {
                "transaction_flow_name": "BuyContract-Stocks",
                "direction": 1,
                "balance": "-125.16",
                "currency": "USD",
                "business_type": 2,
                "business_time": "1621504824",
                "symbol": "AAPL.US",
                "description": "AAPL"
              }
            ]
          }
          "#;

        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<CashFlow>,
        }

        let resp: Response = serde_json::from_str(data).unwrap();
        assert_eq!(resp.list.len(), 2);

        let cashflow = &resp.list[0];
        assert_eq!(cashflow.transaction_flow_name, "BuyContract-Stocks");
        assert_eq!(cashflow.direction, CashFlowDirection::Out);
        assert_eq!(cashflow.balance, decimal!(-248.60f32));
        assert_eq!(cashflow.currency, "USD");
        assert_eq!(cashflow.business_type, BalanceType::Cash);
        assert_eq!(cashflow.business_time, datetime!(2021-05-20 18:52:37 +8));
        assert_eq!(cashflow.symbol.as_deref(), Some("AAPL.US"));
        assert_eq!(cashflow.description, "AAPL");

        let cashflow = &resp.list[1];
        assert_eq!(cashflow.transaction_flow_name, "BuyContract-Stocks");
        assert_eq!(cashflow.direction, CashFlowDirection::Out);
        assert_eq!(cashflow.balance, decimal!(-125.16f32));
        assert_eq!(cashflow.currency, "USD");
        assert_eq!(cashflow.business_type, BalanceType::Stock);
        assert_eq!(cashflow.business_time, datetime!(2021-05-20 18:00:24 +8));
        assert_eq!(cashflow.symbol.as_deref(), Some("AAPL.US"));
        assert_eq!(cashflow.description, "AAPL");
    }

    #[test]
    fn account_balance() {
        let data = r#"
        {
            "list": [
              {
                "total_cash": "1759070010.72",
                "max_finance_amount": "977582000",
                "remaining_finance_amount": "0",
                "risk_level": "1",
                "margin_call": "2598051051.50",
                "currency": "HKD",
                "cash_infos": [
                  {
                    "withdraw_cash": "97592.30",
                    "available_cash": "195902464.37",
                    "frozen_cash": "11579339.13",
                    "settling_cash": "207288537.81",
                    "currency": "HKD"
                  },
                  {
                    "withdraw_cash": "199893416.74",
                    "available_cash": "199893416.74",
                    "frozen_cash": "28723.76",
                    "settling_cash": "-276806.51",
                    "currency": "USD"
                  }
                ],
                "net_assets": "11111.12",
                "init_margin": "2222.23",
                "maintenance_margin": "3333.45"
              }
            ]
          }"#;

        #[derive(Debug, Deserialize)]
        struct Response {
            list: Vec<AccountBalance>,
        }

        let resp: Response = serde_json::from_str(data).unwrap();
        assert_eq!(resp.list.len(), 1);

        let balance = &resp.list[0];
        assert_eq!(balance.total_cash, "1759070010.72".parse().unwrap());
        assert_eq!(balance.max_finance_amount, "977582000".parse().unwrap());
        assert_eq!(balance.remaining_finance_amount, decimal!(0i32));
        assert_eq!(balance.risk_level, 1);
        assert_eq!(balance.margin_call, "2598051051.50".parse().unwrap());
        assert_eq!(balance.currency, "HKD");
        assert_eq!(balance.net_assets, "11111.12".parse().unwrap());
        assert_eq!(balance.init_margin, "2222.23".parse().unwrap());
        assert_eq!(balance.maintenance_margin, "3333.45".parse().unwrap());

        assert_eq!(balance.cash_infos.len(), 2);

        let cash_info = &balance.cash_infos[0];
        assert_eq!(cash_info.withdraw_cash, "97592.30".parse().unwrap());
        assert_eq!(cash_info.available_cash, "195902464.37".parse().unwrap());
        assert_eq!(cash_info.frozen_cash, "11579339.13".parse().unwrap());
        assert_eq!(cash_info.settling_cash, "207288537.81".parse().unwrap());
        assert_eq!(cash_info.currency, "HKD");

        let cash_info = &balance.cash_infos[1];
        assert_eq!(cash_info.withdraw_cash, "199893416.74".parse().unwrap());
        assert_eq!(cash_info.available_cash, "199893416.74".parse().unwrap());
        assert_eq!(cash_info.frozen_cash, "28723.76".parse().unwrap());
        assert_eq!(cash_info.settling_cash, "-276806.51".parse().unwrap());
        assert_eq!(cash_info.currency, "USD");
    }

    #[test]
    fn history_orders() {
        let data = r#"
        {
            "orders": [
              {
                "currency": "HKD",
                "executed_price": "0.000",
                "executed_quantity": "0",
                "expire_date": "",
                "last_done": "",
                "limit_offset": "",
                "msg": "",
                "order_id": "706388312699592704",
                "order_type": "ELO",
                "outside_rth": "UnknownOutsideRth",
                "price": "11.900",
                "quantity": "200",
                "side": "Buy",
                "status": "RejectedStatus",
                "stock_name": "Bank of East Asia Ltd/The",
                "submitted_at": "1651644897",
                "symbol": "23.HK",
                "tag": "Normal",
                "time_in_force": "Day",
                "trailing_amount": "",
                "trailing_percent": "",
                "trigger_at": "0",
                "trigger_price": "",
                "trigger_status": "NOT_USED",
                "updated_at": "1651644898"
              }
            ]
          }
        "#;

        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
        }

        let resp: Response = serde_json::from_str(data).unwrap();
        assert_eq!(resp.orders.len(), 1);

        let order = &resp.orders[0];
        assert_eq!(order.currency, "HKD");
        assert!(order.executed_price.is_none());
        assert_eq!(order.executed_quantity, 0);
        assert!(order.expire_date.is_none());
        assert!(order.last_done.is_none());
        assert!(order.limit_offset.is_none());
        assert_eq!(order.msg, "");
        assert_eq!(order.order_id, "706388312699592704");
        assert_eq!(order.order_type, OrderType::ELO);
        assert!(order.outside_rth.is_none());
        assert_eq!(order.price, Some("11.900".parse().unwrap()));
        assert_eq!(order.quantity, 200);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.status, OrderStatus::Rejected);
        assert_eq!(order.stock_name, "Bank of East Asia Ltd/The");
        assert_eq!(order.submitted_at, datetime!(2022-05-04 14:14:57 +8));
        assert_eq!(order.symbol, "23.HK");
        assert_eq!(order.tag, OrderTag::Normal);
        assert_eq!(order.time_in_force, TimeInForceType::Day);
        assert!(order.trailing_amount.is_none());
        assert!(order.trailing_percent.is_none());
        assert!(order.trigger_at.is_none());
        assert!(order.trigger_price.is_none());
        assert!(order.trigger_status.is_none());
        assert_eq!(order.updated_at, Some(datetime!(2022-05-04 14:14:58 +8)));
    }

    #[test]
    fn today_orders() {
        let data = r#"
        {
            "orders": [
              {
                "currency": "HKD",
                "executed_price": "0.000",
                "executed_quantity": "0",
                "expire_date": "",
                "last_done": "",
                "limit_offset": "",
                "msg": "",
                "order_id": "706388312699592704",
                "order_type": "ELO",
                "outside_rth": "UnknownOutsideRth",
                "price": "11.900",
                "quantity": "200",
                "side": "Buy",
                "status": "RejectedStatus",
                "stock_name": "Bank of East Asia Ltd/The",
                "submitted_at": "1651644897",
                "symbol": "23.HK",
                "tag": "Normal",
                "time_in_force": "Day",
                "trailing_amount": "",
                "trailing_percent": "",
                "trigger_at": "0",
                "trigger_price": "",
                "trigger_status": "NOT_USED",
                "updated_at": "1651644898"
              }
            ]
          }
        "#;

        #[derive(Deserialize)]
        struct Response {
            orders: Vec<Order>,
        }

        let resp: Response = serde_json::from_str(data).unwrap();
        assert_eq!(resp.orders.len(), 1);

        let order = &resp.orders[0];
        assert_eq!(order.currency, "HKD");
        assert!(order.executed_price.is_none());
        assert_eq!(order.executed_quantity, 0);
        assert!(order.expire_date.is_none());
        assert!(order.last_done.is_none());
        assert!(order.limit_offset.is_none());
        assert_eq!(order.msg, "");
        assert_eq!(order.order_id, "706388312699592704");
        assert_eq!(order.order_type, OrderType::ELO);
        assert!(order.outside_rth.is_none());
        assert_eq!(order.price, Some("11.900".parse().unwrap()));
        assert_eq!(order.quantity, 200);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.status, OrderStatus::Rejected);
        assert_eq!(order.stock_name, "Bank of East Asia Ltd/The");
        assert_eq!(order.submitted_at, datetime!(2022-05-04 14:14:57 +8));
        assert_eq!(order.symbol, "23.HK");
        assert_eq!(order.tag, OrderTag::Normal);
        assert_eq!(order.time_in_force, TimeInForceType::Day);
        assert!(order.trailing_amount.is_none());
        assert!(order.trailing_percent.is_none());
        assert!(order.trigger_at.is_none());
        assert!(order.trigger_price.is_none());
        assert!(order.trigger_status.is_none());
        assert_eq!(order.updated_at, Some(datetime!(2022-05-04 14:14:58 +8)));
    }

    #[test]
    fn history_executions() {
        let data = r#"
        {
            "has_more": false,
            "trades": [
              {
                "order_id": "693664675163312128",
                "price": "388",
                "quantity": "100",
                "symbol": "700.HK",
                "trade_done_at": "1648611351",
                "trade_id": "693664675163312128-1648611351433741210"
              }
            ]
          }
        "#;

        #[derive(Deserialize)]
        struct Response {
            trades: Vec<Execution>,
        }

        let resp: Response = serde_json::from_str(data).unwrap();
        assert_eq!(resp.trades.len(), 1);

        let execution = &resp.trades[0];
        assert_eq!(execution.order_id, "693664675163312128");
        assert_eq!(execution.price, "388".parse().unwrap());
        assert_eq!(execution.quantity, 100);
        assert_eq!(execution.symbol, "700.HK");
        assert_eq!(execution.trade_done_at, datetime!(2022-03-30 11:35:51 +8));
        assert_eq!(execution.trade_id, "693664675163312128-1648611351433741210");
    }
}
