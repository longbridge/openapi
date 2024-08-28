from datetime import datetime, date, time
from decimal import Decimal
from typing import Any, Callable, List, Mapping, Optional, Type, Protocol


class OpenApiException(Exception):
    """
    OpenAPI exception
    """

    code: Optional[int]
    """
    Error code
    """

    message: str
    """
    Error message
    """

    def __init__(self, code: int, message: str) -> None:
        ...


class HttpClient:
    """
    A HTTP client for longPort open api

    Args:
        http_url: HTTP API url
        app_key: App Key
        app_secret: App Secret
        access_token: Access Token
    """

    def __init__(self, http_url: str, app_key: str,
                 app_secret: str, access_token: str) -> None: ...

    @classmethod
    def from_env(cls: Type) -> HttpClient:
        """
        Create a new `HttpClient` from the given environment variables

        It first gets the environment variables from the `.env` file in the current directory.

        # Variables

        - `LONGPORT_HTTP_URL` - HTTP endpoint url
        - `LONGPORT_APP_KEY` - App key
        - `LONGPORT_APP_SECRET` - App secret
        - `LONGPORT_ACCESS_TOKEN` - Access token
        """

    def request(self, method: str, path: str, headers: Optional[dict[str, str]] = None, body: Optional[Any] = None) -> Any:
        """
        Performs a HTTP reqest

        Examples:
            ::

                from longport.openapi import HttpClient

                client = HttpClient(http_url, app_key,
                                    app_secret, access_token);

                # get
                resp = client.request("get", "/foo/bar");
                print(resp)

                # post
                client.request("get", "/foo/bar", { "foo": 1, "bar": 2 });
        """
        ...

class PushCandlestickMode:
    """
    Push candlestick mode
    """

    class Realtime(PushCandlestickMode):
        """
        Real-time
        """

    class Confirmed(PushCandlestickMode):
        """
        Confirmed
        """

class Config:
    """
    Configuration options for LongPort sdk

    Args:
        app_key: App Key
        app_secret: App Secret
        access_token: Access Token
        http_url: HTTP API url
        quote_ws_url: Websocket url for quote API
        trade_ws_url: Websocket url for trade API
        language: Language identifier
        enable_overnight: Enable overnight quote
        push_candlestick_mode: Push candlestick mode
    """

    def __init__(
        self,
        app_key: str,
        app_secret: str,
        access_token: str,
        http_url: Optional[str] = None,
        quote_ws_url: Optional[str] = None,
        trade_ws_url: Optional[str] = None,
        language: Optional[Type[Language]] = None,
        enable_overnight: bool = False,
        push_candlestick_mode: Type[PushCandlestickMode] = PushCandlestickMode.Realtime,
    ) -> None: ...

    @classmethod
    def from_env(cls: Type) -> Config:
        """
        Create a new `Config` from the given environment variables

        It first gets the environment variables from the `.env` file in the current directory.

        # Variables

        - `LONGPORT_APP_KEY` - App key
        - `LONGPORT_APP_SECRET` - App secret
        - `LONGPORT_ACCESS_TOKEN` - Access token
        - `LONGPORT_HTTP_URL` - HTTP endpoint url
        - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url
        - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url
        - `LONGPORT_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or `false` (Default: `false`)
        """

    def refresh_access_token(self, expired_at: Optional[datetime] = None) -> str:
        """
        Gets a new `access_token`

        Args:
            expired_at: The expiration time of the access token, defaults to `90` days.

        Returns:
            Access token
        """

class Language:
    """
    Language identifier
    """

    class ZH_CN(Language):
        """
        zh-CN
        """

    class ZH_HK(Language):
        """
        zh-HK
        """

    class EN(Language):
        """
        en
        """


class Market:
    """
    Market
    """

    class Unknown(Market):
        """
        Unknown
        """

    class US(Market):
        """
        US market
        """

    class HK(Market):
        """
        HK market
        """

    class CN(Market):
        """
        CN market
        """

    class SG(Market):
        """
        SG market
        """


class PushQuote:
    """
    Quote message
    """

    last_done: Decimal
    """
    Latest price
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    trade_session: Type[TradeSession]
    """
    Trade session
    """


class PushDepth:
    """
    Depth message
    """

    asks: List[Depth]
    """
    Ask depth
    """

    bids: List[Depth]
    """
    Bid depth
    """


class PushBrokers:
    """
    Brokers message
    """

    ask_brokers: List[Brokers]
    """
    Ask brokers
    """

    bid_brokers: List[Brokers]
    """
    Bid brokers
    """


class PushTrades:
    """
    Trades message
    """

    trades: List[Trade]
    """
    Trades data
    """


class PushCandlestick:
    """
    Candlestick updated event
    """

    period: Period
    """
    Period type
    """

    candlestick: Candlestick
    """
    Candlestick
    """


class SubType:
    """
    Subscription flags
    """

    class Quote(SubType):
        """
        Quote
        """

    class Depth(SubType):
        """
        Depth
        """

    class Brokers(SubType):
        """
        Broker
        """

    class Trade(SubType):
        """
        Trade
        """


class DerivativeType:
    """
    Derivative type
    """

    class Option(DerivativeType):
        """
        US stock options
        """

    class Warrant(DerivativeType):
        """
        HK warrants
        """


class SecurityBoard:
    """
    Security board
    """

    class Unknown(SecurityBoard):
        """
        Unknown
        """

    class USMain(SecurityBoard):
        """
        US Pink Board
        """

    class USPink(SecurityBoard):
        """
        US Pink Board
        """

    class USDJI(SecurityBoard):
        """
        Dow Jones Industrial Average
        """

    class USNSDQ(SecurityBoard):
        """
        Nasdsaq Index
        """

    class USSector(SecurityBoard):
        """
        US Industry Board
        """

    class USOption(SecurityBoard):
        """
        US Option
        """

    class USOptionS(SecurityBoard):
        """
        US Sepecial Option
        """

    class HKEquity(SecurityBoard):
        """
        Hong Kong Equity Securities
        """

    class HKPreIPO(SecurityBoard):
        """
        HK PreIPO Security
        """

    class HKWarrant(SecurityBoard):
        """
        HK Warrant
        """

    class HKHS(SecurityBoard):
        """
        Hang Seng Index
        """

    class HKSector(SecurityBoard):
        """
        HK Industry Board
        """

    class SHMainConnect(SecurityBoard):
        """
        SH Main Board(Connect)
        """

    class SHMainNonConnect(SecurityBoard):
        """
        SH Main Board(Non Connect)
        """

    class SHSTAR(SecurityBoard):
        """
        SH Science and Technology Innovation Board
        """

    class CNIX(SecurityBoard):
        """
        CN Index
        """

    class CNSector(SecurityBoard):
        """
        CN Industry Board
        """

    class SZMainConnect(SecurityBoard):
        """
        SZ Main Board(Connect)
        """

    class SZMainNonConnect(SecurityBoard):
        """
        SZ Main Board(Non Connect)
        """

    class SZGEMConnect(SecurityBoard):
        """
        SZ Gem Board(Connect)
        """

    class SZGEMNonConnect(SecurityBoard):
        """
        SZ Gem Board(Non Connect)
        """

    class SGMain(SecurityBoard):
        """
        SG Main Board
        """

    class STI(SecurityBoard):
        """
        Singapore Straits Index
        """

    class SGSector(SecurityBoard):
        """
        SG Industry Board
        """

class Security:
    """
    Security
    """

    symbol: str
    """
    Security code
    """

    name_cn: str
    """
    Security name (zh-CN)
    """

    name_en: str
    """
    Security name (en)
    """

    name_hk: str
    """
    Security name (zh-HK)
    """

class SecurityListCategory:
    """
    Security list category
    """

    class Overnight(SecurityListCategory):
        """
        Overnight
        """

class SecurityStaticInfo:
    """
    The basic information of securities
    """

    symbol: str
    """
    Security code
    """

    name_cn: str
    """
    Security name (zh-CN)
    """

    name_en: str
    """
    Security name (en)
    """

    name_hk: str
    """
    Security name (zh-HK)
    """

    exchange: str
    """
    Exchange which the security belongs to
    """

    currency: str
    """
    Trading currency
    """

    lot_size: int
    """
    Lot size
    """

    total_shares: int
    """
    Total shares
    """

    circulating_shares: int
    """
    Circulating shares
    """

    hk_shares: int
    """
    HK shares (only HK stocks)
    """

    eps: Decimal
    """
    Earnings per share
    """

    eps_ttm: Decimal
    """
    Earnings per share (TTM)
    """

    bps: Decimal
    """
    Net assets per share
    """

    dividend_yield: Decimal
    """
    Dividend yield
    """

    stock_derivatives: List[Type[DerivativeType]]
    """
    Types of supported derivatives
    """

    board: Type[SecurityBoard]
    """
    Board
    """


class TradeStatus:
    """
    Security Status
    """

    class Normal(TradeStatus):
        """
        Normal
        """

    class Halted(TradeStatus):
        """
        Suspension
        """

    class Delisted(TradeStatus):
        """
        Delisted
        """

    class Fuse(TradeStatus):
        """
        Fuse
        """

    class PrepareList(TradeStatus):
        """
        Prepare List
        """

    class CodeMoved(TradeStatus):
        """
        Code Moved
        """

    class ToBeOpened(TradeStatus):
        """
        To Be Opened
        """

    class SplitStockHalts(TradeStatus):
        """
        Split Stock Halts
        """

    class Expired(TradeStatus):
        """
        Expired
        """

    class WarrantPrepareList(TradeStatus):
        """
        Warrant To BeListed
        """

    class SuspendTrade(TradeStatus):
        """
        Suspend
        """


class PrePostQuote:
    """
    Quote of US pre/post market
    """

    last_done: Decimal
    """
    Latest price
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    prev_close: Decimal
    """
    Close of the last trade session
    """


class SecurityQuote:
    """
    Quote of securitity
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    prev_close: Decimal
    """
    Yesterday's close
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    pre_market_quote: Optional[PrePostQuote]
    """
    Quote of US pre market
    """

    post_market_quote: Optional[PrePostQuote]
    """
    Quote of US post market
    """

    overnight_quote: Optional[PrePostQuote]
    """
    Quote of US overnight market
    """


class OptionType:
    """
    Option type
    """

    class Unknown(OptionType):
        """
        Unknown
        """

    class American(OptionType):
        """
        American
        """

    class Europe(OptionType):
        """
        Europe
        """


class OptionDirection:
    """
    Option direction
    """

    class Unknown(OptionDirection):
        """
        Unknown
        """

    class Put(OptionDirection):
        """
        Put
        """

    class Call(OptionDirection):
        """
        Call
        """


class OptionQuote:
    """
    Quote of option
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    prev_close: Decimal
    """
    Yesterday's close
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    implied_volatility: Decimal
    """
    Implied volatility
    """

    open_interest: int
    """
    Number of open positions
    """

    expiry_date: date
    """
    Exprity date
    """

    strike_price: Decimal
    """
    Strike price
    """

    contract_multiplier: Decimal
    """
    Contract multiplier
    """

    contract_type: Type[OptionType]
    """
    Option type
    """

    contract_size: Decimal
    """
    Contract size
    """

    direction: Type[OptionDirection]
    """
    Option direction
    """

    historical_volatility: Decimal
    """
    Underlying security historical volatility of the option
    """

    underlying_symbol: str
    """
    Underlying security symbol of the option
    """


class WarrantType:
    """
    Warrant type
    """

    class Unknown(WarrantType):
        """
        Unknown
        """

    class Call(WarrantType):
        """
        Call
        """

    class Put(WarrantType):
        """
        Put
        """

    class Bull(WarrantType):
        """
        Bull
        """

    class Bear(WarrantType):
        """
        Bear
        """

    class Inline(WarrantType):
        """
        Inline
        """


class WarrantQuote:
    """
    Quote of warrant
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    prev_close: Decimal
    """
    Yesterday's close
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """

    implied_volatility: Decimal
    """
    Implied volatility
    """

    expiry_date: date
    """
    Exprity date
    """

    last_trade_date: date
    """
    Last tradalbe date
    """

    outstanding_ratio: Decimal
    """
    Outstanding ratio
    """

    outstanding_quantity: int
    """
    Outstanding quantity
    """

    conversion_ratio: Decimal
    """
    Conversion ratio
    """

    category: Type[WarrantType]
    """
    Warrant type
    """

    strike_price: Decimal
    """
    Strike price
    """

    upper_strike_price: Decimal
    """
    Upper bound price
    """

    lower_strike_price: Decimal
    """
    Lower bound price
    """

    call_price: Decimal
    """
    Call price
    """

    underlying_symbol: str
    """
    Underlying security symbol of the warrant
    """


class Depth:
    """
    Depth
    """

    position: int
    """
    Position
    """

    price: Optional[Decimal]
    """
    Price
    """

    volume: int
    """
    Volume
    """

    order_num: int
    """
    Number of orders
    """


class SecurityDepth:
    """
    Security depth
    """

    asks: List[Depth]
    """
    Ask depth
    """

    bids: List[Depth]
    """
    Bid depth
    """


class Brokers:
    """
    Brokers
    """

    position: int
    """
    Position
    """

    broker_ids: List[int]
    """
    Broker IDs
    """


class SecurityBrokers:
    """
    Security brokers
    """

    ask_brokers: List[Brokers]
    """
    Ask brokers
    """

    bid_brokers: List[Brokers]
    """
    Bid brokers
    """


class ParticipantInfo:
    """
    Participant info
    """

    broker_ids: List[int]
    """
    Broker IDs
    """

    name_cn: str
    """
    Participant name (zh-CN)
    """

    name_en: str
    """
    Participant name (en)
    """

    name_hk: str
    """
    Participant name (zh-HK)
    """


class TradeDirection:
    """
    Trade direction
    """

    class Neutral(TradeDirection):
        """
        Neutral
        """

    class Down(TradeDirection):
        """
        Down
        """

    class Up(TradeDirection):
        """
        Up
        """


class TradeSession:
    """
    Trade session
    """

    class Normal(TradeSession):
        """
        Trading
        """

    class Pre(TradeSession):
        """
        Pre-Trading
        """

    class Post(TradeSession):
        """
        Post-Trading
        """


class Trade:
    """
    Trade
    """

    price: Decimal
    """
    Price
    """

    volume: int
    """
    Volume
    """

    timestamp: datetime
    """
    Time of trading
    """

    trade_type: str
    """
    Trade type

    HK

    - `*` - Overseas trade
    - `D` - Odd-lot trade
    - `M` - Non-direct off-exchange trade
    - `P` - Late trade (Off-exchange previous day)
    - `U` - Auction trade
    - `X` - Direct off-exchange trade
    - `Y` - Automatch internalized
    - `<empty string>` - Automatch normal

    US

    - `<empty string>` - Regular sale
    - `A` - Acquisition
    - `B` - Bunched trade
    - `D` - Distribution
    - `F` - Intermarket sweep
    - `G` - Bunched sold trades
    - `H` - Price variation trade
    - `I` - Odd lot trade
    - `K` - Rule 155 trde(NYSE MKT)
    - `M` - Market center close price
    - `P` - Prior reference price
    - `Q` - Market center open price
    - `S` - Split trade
    - `V` - Contingent trade
    - `W` - Average price trade
    - `X` - Cross trade
    - `1` - Stopped stock(Regular trade)
    """

    direction: Type[TradeDirection]
    """
    Trade direction
    """

    trade_session: Type[TradeSession]
    """
    Trade session
    """


class IntradayLine:
    """
    Intraday line
    """

    price: Decimal
    """
    Close price of the minute
    """

    timestamp: datetime
    """
    Start time of the minute
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    avg_price: Decimal
    """
    Average price
    """


class Candlestick:
    """
    Candlestick
    """

    close: Decimal
    """
    Close price
    """

    open: Decimal
    """
    Open price
    """

    low: Decimal
    """
    Low price
    """

    high: Decimal
    """
    High price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    timestamp: datetime
    """
    Timestamp
    """


class AdjustType:
    """
    Candlestick adjustment type
    """

    class NoAdjust(AdjustType):
        """
        Actual
        """

    class ForwardAdjust(AdjustType):
        """
        Adjust forward
        """


class Period:
    """
    Candlestick period
    """

    class Unknown(Period):
        """
        Unknown
        """

    class Min_1(Period):
        """
        One Minute
        """

    class Min_5(Period):
        """
        Five Minutes
        """

    class Min_15(Period):
        """
        Fifteen Minutes
        """

    class Min_30(Period):
        """
        Thirty Minutes
        """

    class Min_60(Period):
        """
        Sixty Minutes
        """

    class Day(Period):
        """
        One Day
        """

    class Week(Period):
        """
        One Week
        """

    class Month(Period):
        """
        One Month
        """

    class Year(Period):
        """
        One Year
        """


class StrikePriceInfo:
    """
    Strike price info
    """

    price: Decimal
    """
    Strike price
    """

    call_symbol: str
    """
    Security code of call option
    """

    put_symbol: str
    """
    Security code of put option
    """

    standard: bool
    """
    Is standard
    """


class IssuerInfo:
    """
    Issuer info
    """

    issuer_id: int
    """
    Issuer ID
    """

    name_cn: str
    """
    Issuer name (zh-CN)
    """

    name_en: str
    """
    Issuer name (en)
    """

    name_hk: str
    """
    Issuer name (zh-HK)
    """

class WarrantStatus:
    """
    Warrant status
    """

    class Suspend(WarrantStatus):
        """
        Suspend
        """

    class PrepareList(WarrantStatus):
        """
        Prepare List
        """

    class Normal(WarrantStatus):
        """
        Normal
        """

class SortOrderType:
    """
    Sort order type
    """

    class Ascending(SortOrderType):
        """
        Ascending
        """

    class Descending(SortOrderType):
        """
        Descending
        """

class WarrantSortBy:
    """
    Warrant sort by
    """

    class LastDone(WarrantSortBy):
        """
        LastDone
        """

    class ChangeRate(WarrantSortBy):
        """
        Change rate
        """

    class ChangeValue(WarrantSortBy):
        """
        Change value
        """

    class Volume(WarrantSortBy):
        """
        Volume
        """

    class Turnover(WarrantSortBy):
        """
        Turnover
        """

    class ExpiryDate(WarrantSortBy):
        """
        Expiry date
        """

    class StrikePrice(WarrantSortBy):
        """
        Strike price
        """
    
    class UpperStrikePrice(WarrantSortBy):
        """
        Upper strike price
        """

    class LowerStrikePrice(WarrantSortBy):
        """
        Lower strike price
        """

    class OutstandingQuantity(WarrantSortBy):
        """
        Outstanding quantity
        """

    class OutstandingRatio(WarrantSortBy):
        """
        Outstanding ratio
        """

    class Premium(WarrantSortBy):
        """
        Premium
        """

    class ItmOtm(WarrantSortBy):
        """
        In/out of the bound
        """

    class ImpliedVolatility(WarrantSortBy):
        """
        Implied volatility
        """

    class Delta(WarrantSortBy):
        """
        Greek value delta
        """

    class CallPrice(WarrantSortBy):
        """
        Call price
        """

    class ToCallPrice(WarrantSortBy):
        """
        Price interval from the call price
        """

    class EffectiveLeverage(WarrantSortBy):
        """
        Effective leverage
        """

    class LeverageRatio(WarrantSortBy):
        """
        Leverage ratio
        """

    class ConversionRatio(WarrantSortBy):
        """
        Conversion ratio
        """

    class BalancePoint(WarrantSortBy):
        """
        Breakeven point
        """

    class Status(WarrantSortBy):
        """
        Status
        """

class FilterWarrantExpiryDate:
    """
    Filter warrant expiry date type
    """

    class LT_3(FilterWarrantExpiryDate):
        """
        Less than 3 months
        """

    class Between_3_6(FilterWarrantExpiryDate):
        """
        3 - 6 months
        """

    class Between_6_12(FilterWarrantExpiryDate):
        """
        6 - 12 months
        """

    class GT_12(FilterWarrantExpiryDate):
        """
        Greater than 12 months
        """

class FilterWarrantInOutBoundsType:
    """
    Filter warrant in/out of the bounds type
    """

    class In(FilterWarrantInOutBoundsType):
        """
        In bounds
        """

    class Out(FilterWarrantInOutBoundsType):
        """
        Out bounds
        """

class WarrantInfo:
    """
    Warrant info
    """

    symbol: str
    """
    Security code
    """

    warrant_type: Type[WarrantType]
    """
    Warrant type
    """

    name: str
    """
    Security name
    """

    last_done: Decimal
    """
    Latest price
    """

    change_rate: Decimal
    """
    Quote change rate
    """

    change_value: Decimal
    """
    Quote change
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    expiry_date: date
    """
    Expiry date
    """

    strike_price: Optional[Decimal]
    """
    Strike price
    """

    upper_strike_price: Optional[Decimal]
    """
    Upper strike price
    """

    lower_strike_price: Optional[Decimal]
    """
    Lower strike price
    """

    outstanding_qty: int
    """
    Outstanding quantity
    """

    outstanding_ratio: Decimal
    """
    Outstanding ratio
    """

    premium: Decimal
    """
    Premium
    """

    itm_otm: Optional[Decimal]
    """
    In/out of the bound
    """

    implied_volatility: Optional[Decimal]
    """
    Implied volatility
    """

    delta: Optional[Decimal]
    """
    Greek value delta
    """

    call_price: Optional[Decimal]
    """
    Call price
    """

    to_call_price: Optional[Decimal]
    """
    Price interval from the call price
    """

    effective_leverage: Optional[Decimal]
    """
    Effective leverage
    """

    leverage_ratio: Decimal
    """
    Leverage ratio
    """

    conversion_ratio: Optional[Decimal]
    """
    Conversion ratio
    """

    balance_point: Optional[Decimal]
    """
    Breakeven point
    """

    status: Type[WarrantStatus]
    """
    Status
    """

class TradingSessionInfo:
    """
    The information of trading session
    """

    begin_time: time
    """
    Being trading time
    """

    end_time: time
    """
    End trading time
    """

    trade_session: Type[TradeSession]
    """
    Trading sessions
    """


class MarketTradingSession:
    """
    Market trading session
    """

    market: Type[Market]
    """
    Market
    """

    trade_sessions: List[TradingSessionInfo]
    """
    Trading session
    """


class MarketTradingDays:
    trading_days: List[date]
    half_trading_days: List[date]


class CapitalFlowLine:
    """
    Capital flow line
    """

    inflow: Decimal
    """
    Inflow capital data
    """

    timestamp: datetime
    """
    Time
    """


class CapitalDistribution:
    """
    Capital distribution
    """

    large: Decimal
    """
    Large order
    """

    medium: Decimal
    """
    Medium order
    """

    small: Decimal
    """
    Small order
    """


class CapitalDistributionResponse:
    """
    Capital distribution response
    """

    timestamp: datetime
    """
    Time
    """

    capital_in: CapitalDistribution
    """
    Inflow capital data
    """

    capital_out: CapitalDistribution
    """
    Outflow capital data
    """


class WatchlistSecurity:
    """
    Watchlist security
    """

    symbol: str
    """
    Security symbol
    """

    market: Market
    """
    Market
    """

    name: str
    """
    Security name
    """

    watched_price: Optional[Decimal]
    """
    Watched price
    """

    watched_at: datetime
    """
    Watched time
    """


class WatchlistGroup:
    id: int
    """
    Group id
    """

    name: str
    """
    Group name
    """

    securities: List[WatchlistSecurity]
    """
    Securities
    """


class SecuritiesUpdateMode:
    """
    Securities update mode
    """

    class Add(SecuritiesUpdateMode):
        """
        Add securities
        """

    class Remove(SecuritiesUpdateMode):
        """
        Remove securities
        """

    class Replace(SecuritiesUpdateMode):
        """
        Replace securities
        """


class RealtimeQuote:
    """
    Real-time quote
    """

    symbol: str
    """
    Security code
    """

    last_done: Decimal
    """
    Latest price
    """

    open: Decimal
    """
    Open
    """

    high: Decimal
    """
    High
    """

    low: Decimal
    """
    Low
    """

    timestamp: datetime
    """
    Time of latest price
    """

    volume: int
    """
    Volume
    """

    turnover: Decimal
    """
    Turnover
    """

    trade_status: Type[TradeStatus]
    """
    Security trading status
    """


class Subscription:
    """
    Subscription
    """

    symbol: str
    """
    Security code
    """

    sub_types: List[Type[SubType]]
    """
    Subscription types
    """

    candlesticks: List[Type[Period]]
    """
    Candlesticks
    """


class CalcIndex:
    """
    Calc index
    """

    class LastDone(CalcIndex):
        """
        Latest price
        """

    class ChangeValue(CalcIndex):
        """
        Change value
        """

    class ChangeRate(CalcIndex):
        """
        Change rate
        """

    class Volume(CalcIndex):
        """
        Volume
        """

    class Turnover(CalcIndex):
        """
        Turnover
        """

    class YtdChangeRate(CalcIndex):
        """
        Year-to-date change ratio
        """

    class TurnoverRate(CalcIndex):
        """
        Turnover rate
        """

    class TotalMarketValue(CalcIndex):
        """
        Total market value
        """

    class CapitalFlow(CalcIndex):
        """
        Capital flow
        """

    class Amplitude(CalcIndex):
        """
        Amplitude
        """

    class VolumeRatio(CalcIndex):
        """
        Volume ratio
        """

    class PeTtmRatio(CalcIndex):
        """
        PE (TTM)
        """

    class PbRatio(CalcIndex):
        """
        PB
        """

    class DividendRatioTtm(CalcIndex):
        """
        Dividend ratio (TTM)
        """

    class FiveDayChangeRate(CalcIndex):
        """
        Five days change ratio
        """

    class TenDayChangeRate(CalcIndex):
        """
        Ten days change ratio
        """

    class HalfYearChangeRate(CalcIndex):
        """
        Half year change ratio
        """

    class FiveMinutesChangeRate(CalcIndex):
        """
        Five minutes change ratio
        """

    class ExpiryDate(CalcIndex):
        """
        Expiry date
        """

    class StrikePrice(CalcIndex):
        """
        Strike price
        """

    class UpperStrikePrice(CalcIndex):
        """
        Upper bound price
        """

    class LowerStrikePrice(CalcIndex):
        """
        Lower bound price
        """

    class OutstandingQty(CalcIndex):
        """
        Outstanding quantity
        """

    class OutstandingRatio(CalcIndex):
        """
        Outstanding ratio
        """

    class Premium(CalcIndex):
        """
        Premium
        """

    class ItmOtm(CalcIndex):
        """
        In/out of the bound
        """

    class ImpliedVolatility(CalcIndex):
        """
        Implied volatility
        """

    class WarrantDelta(CalcIndex):
        """
        Warrant delta
        """

    class CallPrice(CalcIndex):
        """
        Call price
        """

    class ToCallPrice(CalcIndex):
        """
        Price interval from the call price
        """

    class EffectiveLeverage(CalcIndex):
        """
        Effective leverage
        """

    class LeverageRatio(CalcIndex):
        """
        Leverage ratio
        """

    class ConversionRatio(CalcIndex):
        """
        Conversion ratio
        """

    class BalancePoint(CalcIndex):
        """
        Breakeven point
        """

    class OpenInterest(CalcIndex):
        """
        Open interest
        """

    class Delta(CalcIndex):
        """
        Delta
        """

    class Gamma(CalcIndex):
        """
        Gamma
        """

    class Theta(CalcIndex):
        """
        Theta
        """

    class Vega(CalcIndex):
        """
        Vega
        """

    class Rho(CalcIndex):
        """
        Rho
        """


class SecurityCalcIndex:
    """
    Security calc index response
    """

    symbol: str
    """
    Security symbol
    """

    last_done: Optional[Decimal]
    """
    Latest price
    """

    change_value: Optional[Decimal]
    """
    Change value
    """

    change_value: Optional[Decimal]
    """
    Change value
    """

    change_rate: Optional[Decimal]
    """
    Change ratio
    """

    volume: Optional[int]
    """
    Volume
    """

    turnover: Optional[Decimal]
    """
    Turnover
    """

    ytd_change_rate: Optional[Decimal]
    """
    Year-to-date change ratio
    """

    turnover_rate: Optional[Decimal]
    """
    turnover_rate
    """

    total_market_value: Optional[Decimal]
    """
    Total market value
    """

    capital_flow: Optional[Decimal]
    """
    Capital flow
    """

    amplitude: Optional[Decimal]
    """
    Amplitude
    """

    volume_ratio: Optional[Decimal]
    """
    Volume ratio
    """

    pe_ttm_ratio: Optional[Decimal]
    """
    PE (TTM)
    """

    pb_ratio: Optional[Decimal]
    """
    PB
    """

    dividend_ratio_ttm: Optional[Decimal]
    """
    Dividend ratio (TTM)
    """

    five_day_change_rate: Optional[Decimal]
    """
    Five days change ratio
    """

    ten_day_change_rate: Optional[Decimal]
    """
    Ten days change ratio
    """

    half_year_change_rate: Optional[Decimal]
    """
    Half year change ratio
    """

    five_minutes_change_rate: Optional[Decimal]
    """
    Five minutes change ratio
    """

    expiry_date: Optional[date]
    """
    Expiry date
    """

    strike_price: Optional[Decimal]
    """
    Strike price
    """

    upper_strike_price: Optional[Decimal]
    """
    Upper bound price
    """

    lower_strike_price: Optional[Decimal]
    """
    Lower bound price
    """

    outstanding_qty: Optional[int]
    """
    Outstanding quantity
    """

    outstanding_ratio: Optional[Decimal]
    """
    Outstanding ratio
    """

    premium: Optional[Decimal]
    """
    Premium
    """

    itm_otm: Optional[Decimal]
    """
    In/out of the bound
    """

    implied_volatility: Optional[Decimal]
    """
    Implied volatility
    """

    warrant_delta: Optional[Decimal]
    """
    Warrant delta
    """

    call_price: Optional[Decimal]
    """
    Call price
    """

    to_call_price: Optional[Decimal]
    """
    Price interval from the call price
    """

    effective_leverage: Optional[Decimal]
    """
    Effective leverage
    """

    leverage_ratio: Optional[Decimal]
    """
    Leverage ratio
    """

    conversion_ratio: Optional[Decimal]
    """
    Conversion ratio
    """

    balance_point: Optional[Decimal]
    """
    Breakeven point
    """

    open_interest: Optional[int]
    """
    Open interest
    """

    delta: Optional[Decimal]
    """
    Delta
    """

    gamma: Optional[Decimal]
    """
    Gamma
    """

    theta: Optional[Decimal]
    """
    Theta
    """

    vega: Optional[Decimal]
    """
    Vega
    """

    rho: Optional[Decimal]
    """
    Rho
    """


class QuoteContext:
    """
    Quote context

    Args:
        config: Configuration object
    """

    def __init__(self, config: Config) -> None: ...

    def member_id(self) -> int:
        """
        Returns the member ID
        """
    
    def quote_level(self) -> str:
        """
        Returns the quote level
        """

    def set_on_quote(self, callback: Callable[[str, PushQuote], None]) -> None:
        """
        Set quote callback, after receiving the quote data push, it will call back to this function.
        """

    def set_on_depth(self, callback: Callable[[str, PushDepth], None]) -> None:
        """
        Set depth callback, after receiving the depth data push, it will call back to this function.
        """

    def set_on_brokers(self, callback: Callable[[str, PushBrokers], None]) -> None:
        """
        Set brokers callback, after receiving the brokers data push, it will call back to this function.
        """

    def set_on_trades(self, callback: Callable[[str, PushTrades], None]) -> None:
        """
        Set trades callback, after receiving the trades data push, it will call back to this function.
        """

    def set_on_candlestick(self, callback: Callable[[str, PushCandlestick], None]) -> None:
        """
        Set candlestick callback, after receiving the candlestick updated event, it will call back to this function.
        """

    def subscribe(self, symbols: List[str], sub_types: List[Type[SubType]], is_first_push: bool = False) -> None:
        """
        Subscribe

        Args:
            symbols: Security codes
            sub_types: Subscribe types
            is_first_push: Whether to perform a data push immediately after subscribing. (trade not supported)

        Examples:
            ::

                from time import sleep
                from longport.openapi import QuoteContext, Config, SubType, PushQuote

                def on_quote(symbol: str, event: PushQuote):
                    print(symbol, event)

                config = Config.from_env()
                ctx = QuoteContext(config)
                ctx.set_on_quote(on_quote)

                ctx.subscribe(["700.HK", "AAPL.US"], [
                              SubType.Quote], is_first_push = True)
                sleep(30)
        """

    def unsubscribe(self, symbols: List[str], sub_types: List[Type[SubType]]) -> None:
        """
        Unsubscribe

        Args:
            symbols: Security codes
            sub_types: Subscribe types

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, SubType
                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                ctx.unsubscribe(["AAPL.US"], [SubType.Quote])
        """

    def subscribe_candlesticks(self, symbol: str, period: Type[Period]) -> None:
        """
        Subscribe security candlesticks

        Args:
            symbol: Security code
            period: Period type

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, PushCandlestick
                config = Config.from_env()
                ctx = QuoteContext(config)

                def on_candlestick(symbol: str, event: PushCandlestick):
                    print(symbol, event)

                ctx.set_on_candlestick(on_candlestick)
                ctx.subscribe_candlesticks("700.HK", Period.Min_1)
                sleep(30)
        """

    def unsubscribe_candlesticks(self, symbol: str, period: Type[Period]) -> None:
        """
        Subscribe security candlesticks

        Args:
            symbol: Security code
            period: Period type
        """

    def subscriptions(self) -> List[Subscription]:
        """
        Get subscription information

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, SubType
                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                resp = ctx.subscriptions()
                print(resp)
        """

    def static_info(self, symbols: List[str]) -> List[SecurityStaticInfo]:
        """
        Get basic information of securities

        Args:
            symbols: Security codes

        Returns:
            Security info list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.static_info(
                    ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
                print(resp)
        """

    def quote(self, symbols: List[str]) -> List[SecurityQuote]:
        """
        Get quote of securities

        Args:
            symbols: Security codes

        Returns:
            Security quote list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
                print(resp)
        """

    def option_quote(self, symbols: List[str]) -> List[OptionQuote]:
        """
        Get quote of option securities

        Args:
            symbols: Security codes

        Returns:
            Option quote list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.option_quote(["AAPL230317P160000.US"])
                print(resp)
        """

    def warrant_quote(self, symbols: List[str]) -> List[WarrantQuote]:
        """
        Get quote of warrant securities

        Args:
            symbols: Security codes

        Returns:
            Warrant quote list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.warrant_quote(["21125.HK"])
                print(resp)
        """

    def depth(self, symbol: str) -> SecurityDepth:
        """
        Get security depth

        Args:
            symbol: Security code

        Returns:
            Security depth

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.depth("700.HK")
                print(resp)
        """

    def brokers(self, symbol: str) -> SecurityBrokers:
        """
        Get security brokers

        Args:
            symbol: Security code

        Returns:
            Security brokers

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.brokers("700.HK")
                print(resp)
        """

    def participants(self) -> List[ParticipantInfo]:
        """
        Get participants

        Returns:
            Participants

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.participants()
                print(resp)
        """

    def trades(self, symbol: str, count: int) -> List[Trade]:
        """
        Get security trades

        Args:
            symbol: Security code
            count: Count of trades (Maximum is `1000`)

        Returns:
            Trades

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.trades("700.HK", 10)
                print(resp)
        """

    def intraday(self, symbol: str) -> List[IntradayLine]:
        """
        Get security intraday lines

        Args:
            symbol: Security code

        Returns:
            Intraday lines

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.intraday("700.HK")
                print(resp)
        """

    def candlesticks(self, symbol: str, period: Type[Period], count: int, adjust_type: Type[AdjustType]) -> List[Candlestick]:
        """
        Get security candlesticks

        Args:
            symbol: Security code
            period: Candlestick period
            count: Count of cancdlestick (Maximum is `1000`)
            adjust_type: Adjustment type

        Returns:
            Candlesticks

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, Period, AdjustType

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.candlesticks(
                    "700.HK", Period.Day, 10, AdjustType.NoAdjust)
                print(resp)
        """

    def history_candlesticks_by_offset(self, symbol: str, period: Type[Period], adjust_type: Type[AdjustType], forward: bool, time: datetime, count: int) -> List[Candlestick]:
        """
        Get security history candlesticks by offset

        Args:
            symbol: Security code
            period: Period type
            adjust_type: Adjust type
            forward: If `True`, query the latest from the specified time
            time: Datetime
            count: Count of candlesticks
        """

    def history_candlesticks_by_date(self, symbol: str, period: Type[Period], adjust_type: Type[AdjustType], start: Optional[date], end: Optional[date]) -> List[Candlestick]:
        """
        Get security history candlesticks by date

        Args:
            symbol: Security code
            period: Period type
            adjust_type: Adjust type
            start: Start date
            end: End date
        """

    def option_chain_expiry_date_list(self, symbol: str) -> List[date]:
        """
        Get option chain expiry date list

        Args:
            symbol: Security code

        Returns:
            Option chain expiry date list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.option_chain_expiry_date_list("AAPL.US")
                print(resp)
        """

    def option_chain_info_by_date(self, symbol: str, expiry_date: date) -> List[StrikePriceInfo]:
        """
        Get option chain info by date

        Args:
            symbol: Security code
            expiry_date: Expiry date

        Returns:
            Option chain info

        Examples:
            ::

                from datetime import date
                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.option_chain_info_by_date(
                    "AAPL.US", date(2023, 1, 20))
                print(resp)
        """

    def warrant_issuers(self) -> List[IssuerInfo]:
        """
        Get warrant issuers

        Returns:
            Warrant issuers

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.warrant_issuers()
                print(resp)
        """

    def warrant_list(self, symbol: str, sort_by: Type[WarrantSortBy], sort_order: Type[SortOrderType], warrant_type: Optional[List[Type[WarrantType]]] = None, issuer: Optional[List[int]] = None, expiry_date: Optional[List[Type[FilterWarrantExpiryDate]]] = None, price_type: Optional[List[Type[FilterWarrantInOutBoundsType]]] = None, status: Optional[List[Type[WarrantStatus]]] = None) -> List[WarrantInfo]:
        """
        Get warrant list

        Args:
            symbol: Security code
            sort_by: Sort by field
            sort_order: Sort order
            warrant_type: Filter by warrant type
            issuer: Filter by issuer
            expiry_date: Filter by expiry date
            price_type: Filter by price type
            status: Filter by status

        Returns:
            Warrant list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, WarrantSortBy, SortOrderType

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.warrant_list("700.HK", WarrantSortBy.LastDone, SortOrderType.Ascending)
                print(resp)
        """

    def trading_session(self) -> List[MarketTradingSession]:
        """
        Get trading session of the day

        Returns:
            Trading session of the day

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.trading_session()
                print(resp)
        """

    def trading_days(self, market: Type[Market], begin: date, end: date) -> MarketTradingDays:
        """
        Get trading session of the day

        The interval must be less than one month, and only the most recent year is supported.

        Args:
            market: Market
            begin: Begin date
            end: End date

        Returns:
            Trading days

        Examples:
            ::

                from datetime import date
                from longport.openapi import QuoteContext, Config, Market

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.trading_days(
                    Market.HK, date(2022, 1, 1), date(2022, 2, 1))
                print(resp)
        """

    def capital_flow(self, symbol: str) -> List[CapitalFlowLine]:
        """
        Get capital flow intraday

        Args:
            symbol: Security code

        Returns:
            Capital flow list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.capital_flow("700.HK")
                print(resp)
        """

    def capital_distribution(self, symbol: str) -> CapitalDistributionResponse:
        """
        Get capital distribution

        Args:
            symbol: Security code

        Returns:
            Capital distribution

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.capital_distribution("700.HK")
                print(resp)
        """

    def calc_indexes(self, symbols: List[str], indexes: List[Type[CalcIndex]]) -> List[SecurityCalcIndex]:
        """
        Get calc indexes

        Args:
            symbols: Security codes
            indexes: Calc indexes

        Returns:
            Calc indexes of the symbols

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, CalcIndex

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.calc_indexes(["700.HK", "APPL.US"], [CalcIndex.LastDone, CalcIndex.ChangeRate])
                print(resp)
        """

    def watchlist(self) -> List[WatchlistGroup]:
        """
        Get watch list

        Returns:
            Watch list groups

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.watchlist()
                print(resp)
        """

    def create_watchlist_group(self, name: str, securities: Optional[List[str]] = None) -> int:
        """
        Create watchlist group

        Args:
            name: Group name
            securities: Securities

        Returns:
            Group ID

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)
                group_id = ctx.create_watchlist_group(name = "Watchlist1", securities = ["700.HK", "AAPL.US"])
                print(group_id)
        """

    def delete_watchlist_group(self, id: int, purge: bool = False):
        """
        Delete watchlist group

        Args:
            id: Group ID
            purge: Move securities in this group to the default group

        Examples:
            ::

                from longport.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)
                ctx.delete_watchlist_group(10086)
        """

    def update_watchlist_group(self, id: int, name: Optional[str] = None, securities: Optional[List[str]] = None, mode: Optional[Type[SecuritiesUpdateMode]] = None):
        """
        Update watchlist group

        Args:
            id: Group ID
            name: Group name
            securities: Securities

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, SecuritiesUpdateMode

                config = Config.from_env()
                ctx = QuoteContext(config)
                ctx.update_watchlist_group(10086, name = "Watchlist2", securities = ["700.HK", "AAPL.US"], SecuritiesUpdateMode.Replace)
        """

    def security_list(self, market: Type[Market], category: Type[SecurityListCategory]) -> List[Security]:
        """
        Get security list

        Args:
            market: Market
            category: Security list category

        Returns:
            Security list

        Examples:
            ::

                from longport.openapi import QuoteContext, Config, Market, SecurityListCategory

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.security_list(Market.HK, SecurityListCategory.Overnight)
                print(resp)
        """

    def realtime_quote(self, symbols: List[str]) -> List[RealtimeQuote]:
        """
        Get real-time quote

        Get real-time quotes of the subscribed symbols, it always returns the data in the local storage.

        Args:
            symbols: Security codes

        Returns:
            Quote list

        Examples:
            ::

                from time import sleep
                from longport.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [
                              SubType.Quote], is_first_push = True)
                sleep(5)
                resp = ctx.realtime_quote(["700.HK", "AAPL.US"])
                print(resp)
        """

    def realtime_depth(self, symbol: str) -> SecurityDepth:
        """
        Get real-time depth

        Get real-time depth of the subscribed symbols, it always returns the data in the local storage.

        Args:
            symbol: Security code

        Returns:
            Security depth

        Examples:
            ::

                from time import sleep
                from longport.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [
                              SubType.Depth], is_first_push = True)
                sleep(5)
                resp = ctx.realtime_depth("700.HK")
                print(resp)
        """

    def realtime_brokers(self, symbol: str) -> SecurityBrokers:
        """
        Get real-time brokers

        Get real-time brokers of the subscribed symbols, it always returns the data in the local storage.

        Args:
            symbol: Security code

        Returns:
            Security brokers

        Examples:
            ::

                from time import sleep
                from longport.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [
                              SubType.Brokers], is_first_push = True)
                sleep(5)
                resp = ctx.realtime_brokers("700.HK")
                print(resp)
        """

    def realtime_trades(self, symbol: str, count: int) -> List[Trade]:
        """
        Get real-time trades

        Get real-time trades of the subscribed symbols, it always returns the data in the local storage.

        Args:
            symbol: Security code
            count: Count of trades

        Returns:
            Security trades

        Examples:
            ::

                from time import sleep
                from longport.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [
                              SubType.Trade], is_first_push = False)
                sleep(5)
                resp = ctx.realtime_trades("700.HK", 10)
                print(resp)
        """

    def realtime_candlesticks(self, symbol: str, period: Type[Period], count: int) -> List[Candlestick]:
        """
        Get real-time candlesticks

        Get Get real-time candlesticks of the subscribed symbols, it always returns the data in the local storage.

        Args:
            symbol: Security code
            period: Period type
            count: Count of candlesticks

        Returns:
            Security candlesticks

        Examples:
            ::

                from time import sleep
                from longport.openapi import QuoteContext, Config, Period

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe_candlesticks("AAPL.US", Period.Min_1)
                sleep(5)
                resp = ctx.realtime_candlesticks("AAPL.US", Period.Min_1, 10)
                print(resp)
        """


class OrderSide:
    """
    Order side
    """

    class Unknown(OrderSide):
        """
        Unknown
        """

    class Buy(OrderSide):
        """
        Buy
        """

    class Sell(OrderSide):
        """
        Sell
        """


class OrderType:
    """
    Order type
    """

    class Unknown(OrderType):
        """
        Unknown
        """

    class LO(OrderType):
        """
        Limit Order
        """

    class ELO(OrderType):
        """
        Enhanced Limit Order
        """

    class MO(OrderType):
        """
        Market Order
        """

    class AO(OrderType):
        """
        At-auction Order
        """

    class ALO(OrderType):
        """
        At-auction Limit Order
        """

    class ODD(OrderType):
        """
        Odd Lots
        """

    class LIT(OrderType):
        """
        Limit If Touched
        """

    class MIT(OrderType):
        """
        Market If Touched
        """

    class TSLPAMT(OrderType):
        """
        Trailing Limit If Touched (Trailing Amount)
        """

    class TSLPPCT(OrderType):
        """
        Trailing Limit If Touched (Trailing Percent)
        """

    class TSMAMT(OrderType):
        """
        Trailing Market If Touched (Trailing Amount)
        """

    class TSMPCT(OrderType):
        """
        Trailing Market If Touched (Trailing Percent)
        """

    class SLO(OrderType):
        """
        Special Limit Order
        """


class OrderStatus:
    """
    Order status
    """

    class Unknown(OrderStatus):
        """
        Unknown
        """

    class NotReported(OrderStatus):
        """
        Not reported
        """

    class ReplacedNotReported(OrderStatus):
        """
        Not reported (Replaced Order)
        """

    class ProtectedNotReported(OrderStatus):
        """
        Not reported (Protected Order)
        """

    class VarietiesNotReported(OrderStatus):
        """
        Not reported (Conditional Order)
        """

    class Filled(OrderStatus):
        """
        Filled
        """

    class WaitToNew(OrderStatus):
        """
        Wait To New
        """

    class New(OrderStatus):
        """
        New
        """

    class WaitToReplace(OrderStatus):
        """
        Wait To Replace
        """

    class PendingReplace(OrderStatus):
        """
        Pending Replace
        """

    class Replaced(OrderStatus):
        """
        Replaced
        """

    class PartialFilled(OrderStatus):
        """
        Partial Filled
        """

    class WaitToCancel(OrderStatus):
        """
        Wait To Cancel
        """

    class PendingCancel(OrderStatus):
        """
        Pending Cancel
        """

    class Rejected(OrderStatus):
        """
        Rejected
        """

    class Canceled(OrderStatus):
        """
        Canceled
        """

    class Expired(OrderStatus):
        """
        ExpiredStatus
        """

    class PartialWithdrawal(OrderStatus):
        """
        PartialWithdrawal
        """


class OrderTag:
    """
    Order tag
    """

    class Unknown(OrderTag):
        """
        Unknown
        """

    class Normal(OrderTag):
        """
        Normal Order
        """

    class LongTerm(OrderTag):
        """
        Long term Order
        """

    class Grey(OrderTag):
        """
        Grey Order
        """

    class MarginCall(OrderTag):
        """
        Force Selling
        """

    class Offline(OrderTag):
        """
        OTC
        """

    class Creditor(OrderTag):
        """
        Option Exercise Long
        """

    class Debtor(OrderTag):
        """
        Option Exercise Short
        """

    class NonExercise(OrderTag):
        """
        Wavier Of Option Exercise
        """

    class AllocatedSub(OrderTag):
        """
        Trade Allocation
        """


class TriggerStatus:
    """
    Trigger status
    """

    class Unknown(TriggerStatus):
        """
        Unknown
        """

    class Deactive(TriggerStatus):
        """
        Deactive
        """

    class Active(TriggerStatus):
        """
        Active
        """

    class Released(TriggerStatus):
        """
        Released
        """


class Execution:
    """
    Execution
    """

    order_id: str
    """
    Order ID
    """

    trade_id: str
    """
    Execution ID
    """

    symbol: str
    """
    Security code
    """

    trade_done_at: datetime
    """
    Trade done time
    """

    quantity: int
    """
    Executed quantity
    """

    price: Decimal
    """
    Executed price
    """


class PushOrderChanged:
    """
    Order changed message
    """

    side: Type[OrderSide]
    """
    Order side
    """

    stock_name: str
    """
    Stock name
    """

    submitted_quantity: int
    """
    Submitted quantity
    """

    symbol: str
    """
    Order symbol
    """

    order_type: Type[OrderType]
    """
    Order type
    """

    submitted_price: Decimal
    """
    Submitted price
    """

    executed_quantity: int
    """
    Executed quantity
    """

    executed_price: Optional[Decimal]
    """
    Executed price
    """

    order_id: str
    """
    Order ID
    """

    currency: str
    """
    Currency
    """

    status: Type[OrderStatus]
    """
    Order status
    """

    submitted_at: datetime
    """
    Submitted time
    """

    updated_at: datetime
    """
    Last updated time
    """

    trigger_price: Optional[Decimal]
    """
    Order trigger price
    """

    msg: str
    """
    Rejected message or remark
    """

    tag: Type[OrderTag]
    """
    Order tag
    """

    trigger_status: Optional[Type[TriggerStatus]]
    """
    Conditional order trigger status
    """

    trigger_at: Optional[datetime]
    """
    Conditional order trigger time
    """

    trailing_amount: Optional[Decimal]
    """
    Trailing amount
    """

    trailing_percent: Optional[Decimal]
    """
    Trailing percent
    """

    limit_offset: Optional[Decimal]
    """
    Limit offset amount
    """

    account_no: str
    """
    Account no
    """

    last_share: Optional[Decimal]
    """
    Last share
    """

    last_price: Optional[Decimal]
    """
    Last price
    """

    remark: str
    """
    Remark message
    """


class TimeInForceType:
    """
    Time in force type
    """

    class Unknown(TimeInForceType):
        """
        Unknown
        """

    class Day(TimeInForceType):
        """
        Day Order
        """

    class GoodTilCanceled(TimeInForceType):
        """
        Good Til Canceled Order
        """

    class GoodTilDate(TimeInForceType):
        """
        Good Til Date Order
        """


class OutsideRTH:
    """
    Enable or disable outside regular trading hours
    """

    class Unknown(OutsideRTH):
        """
        Unknown
        """

    class RTHOnly(OutsideRTH):
        """
        Regular trading hour only
        """

    class AnyTime(OutsideRTH):
        """
        Any time
        """

    class Overnight(OutsideRTH):
        """
        Overnight
        """

class Order:
    """
    Order
    """

    order_id: str
    """
    Order ID
    """

    status: Type[OrderStatus]
    """
    Order status
    """

    stock_name: str
    """
    Stock name
    """

    quantity: int
    """
    Submitted quantity
    """

    executed_quantity: int
    """
    Executed quantity
    """

    price: Optional[Decimal]
    """
    Submitted price
    """

    executed_price: Optional[Decimal]
    """
    Executed price
    """

    submitted_at: datetime
    """
    Submitted time
    """

    side: Type[OrderSide]
    """
    Order side
    """

    symbol: str
    """
    Security code
    """

    order_type: Type[OrderType]
    """
    Order type
    """

    last_done: Optional[Decimal]
    """
    Last done
    """

    trigger_price: Optional[Decimal]
    """
    `LIT` / `MIT` Order Trigger Price
    """

    msg: str
    """
    Rejected Message or remark
    """

    tag: Type[OrderTag]
    """
    Order tag
    """

    time_in_force: Type[TimeInForceType]
    """
    Time in force type
    """

    expire_date: Optional[date]
    """
    Long term order expire date
    """

    updated_at: Optional[datetime]
    """
    Last updated time
    """

    trigger_at: Optional[datetime]
    """
    Conditional order trigger time
    """

    trailing_amount: Optional[Decimal]
    """
    `TSMAMT` / `TSLPAMT` order trailing amount
    """

    trailing_percent: Optional[Decimal]
    """
    `TSMPCT` / `TSLPPCT` order trailing percent
    """

    limit_offset: Optional[Decimal]
    """
    `TSLPAMT` / `TSLPPCT` order limit offset amount
    """

    trigger_status: Optional[Type[TriggerStatus]]
    """
    Conditional order trigger status
    """

    currency: str
    """
    Currency
    """

    outside_rth: Optional[Type[OutsideRTH]]
    """
    Enable or disable outside regular trading hours
    """

    remark: str
    """
    Remark
    """


class CommissionFreeStatus:
    """
    Commission-free Status
    """

    class Unknown(CommissionFreeStatus):
        """
        Unknown
        """

    class None_(CommissionFreeStatus):
        """
        None
        """

    class Calculated(CommissionFreeStatus):
        """
        Commission-free amount to be calculated
        """

    class Pending(CommissionFreeStatus):
        """
        Pending commission-free
        """

    class Ready(CommissionFreeStatus):
        """
        Commission-free applied
        """


class DeductionStatus:
    """
    Deduction status
    """

    class Unknown(DeductionStatus):
        """
        Unknown
        """

    class None_(DeductionStatus):
        """
        None
        """

    class NoData(DeductionStatus):
        """
        Settled with no data
        """

    class Pending(DeductionStatus):
        """
        Settled and pending distribution
        """

    class Done(DeductionStatus):
        """
        Settled and distributed
        """


class ChargeCategoryCode:
    """
    Charge category code
    """

    class Unknown(ChargeCategoryCode):
        """
        Unknown
        """

    class Broker(ChargeCategoryCode):
        """
        Broker
        """

    class Third(ChargeCategoryCode):
        """
        Third
        """


class OrderHistoryDetail:
    """
    Order history detail
    """

    price: Decimal
    """
    Executed price for executed orders, submitted price for expired, canceled, rejected orders, etc.
    """

    quantity: int
    """
    Executed quantity for executed orders, remaining quantity for expired, canceled, rejected orders, etc.
    """

    status: Type[OrderStatus]
    """
    Order status
    """

    msg: str
    """
    Execution or error message
    """

    time: datetime
    """
    Occurrence time
    """


class OrderChargeFee:
    """
    Order charge fee
    """

    code: str
    """
    Charge code
    """

    name: str
    """
    Charge name
    """

    amount: Decimal
    """
    Charge amount
    """

    currency: str
    """
    Charge currency
    """


class OrderChargeItem:
    """
    Order charge item
    """

    code: Type[ChargeCategoryCode]
    """
    Charge category code
    """

    name: str
    """
    Charge category name
    """

    fees: List[OrderChargeFee]
    """
    Charge details
    """


class OrderChargeDetail:
    """
    Order charge detail
    """

    total_amount: Decimal
    """
    Total charges amount
    """

    currency: str
    """
    Settlement currency
    """

    items: List[OrderChargeItem]
    """
    Order charge items
    """


class OrderDetail:
    """
    Order detail
    """

    order_id: str
    """
    Order ID
    """

    status: Type[OrderStatus]
    """
    Order status
    """

    stock_name: str
    """
    Stock name
    """

    quantity: int
    """
    Submitted quantity
    """

    executed_quantity: int
    """
    Executed quantity
    """

    price: Optional[Decimal]
    """
    Submitted price
    """

    executed_price: Optional[Decimal]
    """
    Executed price
    """

    submitted_at: datetime
    """
    Submitted time
    """

    side: Type[OrderSide]
    """
    Order side
    """

    symbol: str
    """
    Security code
    """

    order_type: Type[OrderType]
    """
    Order type
    """

    last_done: Optional[Decimal]
    """
    Last done
    """

    trigger_price: Optional[Decimal]
    """
    `LIT` / `MIT` Order Trigger Price
    """

    msg: str
    """
    Rejected Message or remark
    """

    tag: Type[OrderTag]
    """
    Order tag
    """

    time_in_force: Type[TimeInForceType]
    """
    Time in force type
    """

    expire_date: Optional[date]
    """
    Long term order expire date
    """

    updated_at: Optional[datetime]
    """
    Last updated time
    """

    trigger_at: Optional[datetime]
    """
    Conditional order trigger time
    """

    trailing_amount: Optional[Decimal]
    """
    `TSMAMT` / `TSLPAMT` order trailing amount
    """

    trailing_percent: Optional[Decimal]
    """
    `TSMPCT` / `TSLPPCT` order trailing percent
    """

    limit_offset: Optional[Decimal]
    """
    `TSLPAMT` / `TSLPPCT` order limit offset amount
    """

    trigger_status: Optional[Type[TriggerStatus]]
    """
    Conditional order trigger status
    """

    currency: str
    """
    Currency
    """

    outside_rth: Optional[Type[OutsideRTH]]
    """
    Enable or disable outside regular trading hours
    """

    remark: str
    """
    Remark
    """

    free_status: Type[CommissionFreeStatus]
    """
    Commission-free Status
    """

    free_amount: Optional[Decimal]
    """
    Commission-free amount
    """

    free_currency: Optional[str]
    """
    Commission-free currency
    """

    deductions_status: Type[DeductionStatus]
    """
    Deduction status
    """

    deductions_amount: Optional[Decimal]
    """
    Deduction amount
    """

    deductions_currency: Optional[str]
    """
    Deduction currency
    """

    platform_deducted_status: Type[DeductionStatus]
    """
    Platform fee deduction status
    """

    platform_deducted_amount: Optional[Decimal]
    """
    Platform deduction amount
    """

    platform_deducted_currency: Optional[str]
    """
    Platform deduction currency
    """

    history: List[OrderHistoryDetail]
    """
    Order history details
    """

    charge_detail: OrderChargeDetail
    """
    Order charges
    """


class SubmitOrderResponse:
    """
    Response for submit order request
    """

    order_id: str
    """
    Order id
    """


class CashInfo:
    """
    CashInfo
    """

    withdraw_cash: Decimal
    """
    Withdraw cash
    """
    available_cash: Decimal
    """
    Available cash
    """
    frozen_cash: Decimal
    """
    Frozen cash
    """
    settling_cash: Decimal
    """
    Cash to be settled
    """
    currency: str
    """
    Currency
    """


class AccountBalance:
    """
    Account balance
    """

    total_cash: Decimal
    """
    Total cash
    """
    max_finance_amount: Decimal
    """
    Maximum financing amount
    """
    remaining_finance_amount: Decimal
    """
    Remaining financing amount
    """
    risk_level: int
    """
    Risk control level
    """
    margin_call: Decimal
    """
    Margin call
    """
    currency: str
    """
    Currency
    """
    cash_infos: List[CashInfo]
    """
    Cash details
    """

    net_assets: Decimal
    """
    Net assets
    """

    init_margin: Decimal
    """
    Initial margin
    """

    maintenance_margin: Decimal
    """
    Maintenance margin
    """

    buy_power: Decimal
    """
    Buy power
    """


class BalanceType:
    class Unknown(BalanceType):
        ...

    class Cash(BalanceType):
        ...

    class Stock(BalanceType):
        ...

    class Fund(BalanceType):
        ...


class CashFlowDirection:
    """
    Cash flow direction
    """

    class Unknown(CashFlowDirection):
        """
        Unknown
        """
        ...

    class Out(CashFlowDirection):
        """
        Out
        """
        ...

    class In(CashFlowDirection):
        """
        In
        """
        ...


class CashFlow:
    """
    Cash flow
    """

    transaction_flow_name: str
    """
    Cash flow name
    """

    direction: Type[CashFlowDirection]
    """
    Outflow direction
    """

    business_type: Type[BalanceType]
    """
    Balance type
    """

    balance: Decimal
    """
    Cash amount
    """

    currency: str
    """
    Cash currency
    """

    business_time: datetime
    """
    Business time
    """

    symbol: Optional[str]
    """
    Associated Stock code information
    """

    description: str
    """
    Cash flow description
    """


class FundPosition:
    """
    Fund position
    """

    symbol: str
    """
    Fund ISIN code
    """

    current_net_asset_value: Decimal
    """
    Current equity
    """

    net_asset_value_day: datetime
    """
    Current equity PyDecimal
    """

    symbol_name: str
    """
    Fund name
    """

    currency: str
    """
    Currency
    """

    cost_net_asset_value: Decimal
    """
    Net cost
    """

    holding_units: Decimal
    """
    Holding units
    """


class FundPositionChannel:
    """
    Fund position channel
    """

    account_channel: str
    """
    Account type
    """

    positions: List[FundPosition]
    """
    Fund positions
    """


class FundPositionsResponse:
    """
    Fund positions response
    """

    channels: List[FundPositionChannel]
    """
    Channels
    """


class StockPosition:
    """
    Stock position
    """

    symbol: str
    """
    Stock code
    """

    symbol_name: str
    """
    Stock name
    """

    quantity: int
    """
    The number of holdings
    """

    available_quantity: int
    """
    Available quantity
    """

    currency: str
    """
    Currency
    """

    cost_price: Decimal
    """
    Cost Price(According to the client's choice of average purchase or diluted cost)
    """

    market: Market
    """
    Market
    """

    init_quantity: Optional[int]
    """
    Initial position before market opening
    """


class StockPositionChannel:
    """
    Stock position channel
    """

    account_channel: str
    """
    Account type
    """

    positions: List[StockPosition]
    """
    Stock positions
    """


class StockPositionsResponse:
    """
    Stock positions response
    """

    channels: List[StockPositionChannel]
    """
    Channels
    """


class TopicType:
    """
    Topic type
    """

    class Private(TopicType):
        """
        Private notification for trade
        """
        ...


class MarginRatio:
    """
    Margin ratio
    """

    im_factor: Decimal
    """
    Initial margin ratio
    """

    mm_factor: Decimal
    """
    Maintain the initial margin ratio
    """

    fm_factor: Decimal
    """
    Forced close-out margin ratio
    """


class EstimateMaxPurchaseQuantityResponse:
    """
    Response for estimate maximum purchase quantity
    """

    cash_max_qty: int
    """
    Cash available quantity
    """

    margin_max_qty: int
    """
    Margin available quantity
    """


class TradeContext:
    """
    Trade context

    Args:
        config: Configuration object
    """

    def __init__(self, config: Config) -> None: ...

    def set_on_order_changed(self, callback: Callable[[PushOrderChanged], None]) -> None:
        """
        Set order changed callback, after receiving the order changed event, it will call back to this function.
        """

    def subscribe(self, topics: List[Type[TopicType]]) -> None:
        """
        Subscribe

        Args:
            topics: Topic list

        Examples:
            ::

                from time import sleep
                from decimal import Decimal
                from longport.openapi import TradeContext, Config, OrderSide, OrderType, TimeInForceType, PushOrderChanged, TopicType


                def on_order_changed(event: PushOrderChanged):
                    print(event)


                config = Config.from_env()
                ctx = TradeContext(config)
                ctx.set_on_order_changed(on_order_changed)
                ctx.subscribe([TopicType.Private])

                resp = ctx.submit_order(
                    side=OrderSide.Buy,
                    symbol="700.HK",
                    order_type=OrderType.LO,
                    submitted_price=Decimal("50"),
                    submitted_quantity=200,
                    time_in_force=TimeInForceType.Day,
                    remark="Hello from Python SDK",
                )
                print(resp)
                sleep(5)  # waiting for push event
        """

    def unsubscribe(self, topics: List[str]) -> None:
        """
        Unsubscribe

        Args:
            topics: Topic list
        """

    def history_executions(self, symbol: Optional[str] = None, start_at: Optional[datetime] = None, end_at: Optional[datetime] = None) -> List[Execution]:
        """
        Get history executions

        Args:
            symbol: Filter by security code, example: `700.HK`, `AAPL.US`
            start_at: Start time
            end_at: End time

        Returns:
            Execution list

        Examples:
            ::

                from datetime import datetime
                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.history_executions(
                    symbol = "700.HK",
                    start_at = datetime(2022, 5, 9),
                    end_at = datetime(2022, 5, 12),
                )
                print(resp)
        """

    def today_executions(self, symbol: Optional[str] = None, order_id: Optional[str] = None) -> List[Execution]:
        """
        Get today executions

        Args:
            symbol: Filter by security code
            order_id: Filter by Order ID

        Returns:
            Execution list

        Examples:
            ::

                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.today_executions(symbol = "700.HK")
                print(resp)
        """

    def history_orders(self, symbol: Optional[str] = None, status: Optional[List[Type[OrderStatus]]] = None, side: Optional[Type[OrderSide]] = None, market: Optional[Type[Market]] = None, start_at: Optional[datetime] = None, end_at: Optional[datetime] = None) -> List[Order]:
        """
        Get history orders

        Args:
            symbol: Filter by security code
            status: Filter by order status
            side: Filter by order side
            market: Filter by market type
            start_at: Start time
            end_at: End time

        Returns:
            Order list

        Examples:
            ::

                from datetime import datetime
                from longport.openapi import TradeContext, Config, OrderStatus, OrderSide, Market

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.history_orders(
                    symbol = "700.HK",
                    status = [OrderStatus.Filled, OrderStatus.New],
                    side = OrderSide.Buy,
                    market = Market.HK,
                    start_at = datetime(2022, 5, 9),
                    end_at = datetime(2022, 5, 12),
                )
                print(resp)
        """

    def today_orders(self, symbol: Optional[str] = None, status: Optional[List[Type[OrderStatus]]] = None, side: Optional[Type[OrderSide]] = None, market: Optional[Type[Market]] = None, order_id: Optional[str] = None) -> List[Order]:
        """
        Get today orders

        Args:
            symbol: Filter by security code
            status: Filter by order status
            side: Filter by order side
            market: Filter by market type
            order_id: Filter by order id

        Returns:
            Order list

        Examples:
            ::

                from longport.openapi import TradeContext, Config, OrderStatus, OrderSide, Market

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.today_orders(
                    symbol = "700.HK",
                    status = [OrderStatus.Filled, OrderStatus.New],
                    side = OrderSide.Buy,
                    market = Market.HK,
                )
                print(resp)
        """

    def replace_order(self, order_id: str, quantity: int, price: Optional[Decimal] = None, trigger_price: Optional[Decimal] = None, limit_offset: Optional[Decimal] = None, trailing_amount: Optional[Decimal] = None, trailing_percent: Optional[Decimal] = None, remark: Optional[str] = None) -> None:
        """
        Replace order

        Args:
            quantity: Replaced quantity
            price: Replaced price
            trigger_price: Trigger price (`LIT` / `MIT` Order Required)
            limit_offset: Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
            trailing_amount: Trailing amount (`TSLPAMT` / `TSMAMT` Required)
            trailing_percent: Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
            remark: Remark (Maximum 64 characters)

        Examples:
            ::

                from decimal import Decimal
                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                ctx.replace_order(
                    order_id = "709043056541253632",
                    quantity = 100,
                    price = Decimal("100"),
                )
        """

    def submit_order(self, symbol: str, order_type: Type[OrderType], side: Type[OrderSide], submitted_quantity: int, time_in_force: Type[TimeInForceType], submitted_price: Optional[Decimal] = None,  trigger_price: Optional[Decimal] = None, limit_offset: Optional[Decimal] = None, trailing_amount: Optional[Decimal] = None, trailing_percent: Optional[Decimal] = None, expire_date: Optional[date] = None,  outside_rth: Optional[Type[OutsideRTH]] = None,  remark: Optional[str] = None) -> SubmitOrderResponse:
        """
        Submit order

        Args:
            symbol: Security code
            order_type: Order type
            side: Order Side
            submitted_quantity: Submitted quantity
            time_in_force: Time in force type
            submitted_price: Submitted price
            trigger_price: Trigger price (`LIT` / `MIT` Required)
            limit_offset: Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
            trailing_amount: Trailing amount (`TSLPAMT` / `TSMAMT` Required)
            trailing_percent: Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
            expire_date: Long term order expire date (Required when `time_in_force` is `GoodTilDate`)
            outside_rth: Enable or disable outside regular trading hours
            remark: Remark (Maximum 64 characters)

        Returns:
            Response

        Examples:
            ::

                from decimal import Decimal
                from longport.openapi import TradeContext, Config, OrderSide, OrderType, TimeInForceType

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.submit_order(
                    side = OrderSide.Buy,
                    symbol = "700.HK",
                    order_type = OrderType.LO,
                    submitted_price = Decimal("50"),
                    submitted_quantity = 200,
                    time_in_force = TimeInForceType.Day,
                    remark = "Hello from Python SDK",
                )
                print(resp)
        """

    def cancel_order(self, order_id: str) -> None:
        """
        Cancel order

        Args:
            order_id: Order ID

        Examples:
            ::

                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                ctx.cancel_order("709043056541253632")
        """

    def account_balance(self, currency: Optional[str] = None) -> List[AccountBalance]:
        """
        Get account balance

        Args:
            currency: Currency

        Returns:
            Account list

        Examples:
            ::

                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.account_balance()
                print(resp)
        """

    def cash_flow(self, start_at: datetime, end_at: datetime, business_type: Optional[Type[BalanceType]] = None, symbol: Optional[str] = None, page: Optional[int] = None, size: Optional[int] = None) -> List[CashFlow]:
        """
        Get cash flow

        Args:
            start_at: Start time
            end_at: End time
            business_type: Balance type
            symbol: Target security code
            page: Start page (Default: 1)
            size: Page size (Default: 50)

        Returns:
            Cash flow list

        Examples:
            ::

                from datetime import datetime
                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.cash_flow(
                    start_at = datetime(2022, 5, 9),
                    end_at = datetime(2022, 5, 12),
                )
                print(resp)
        """

    def fund_positions(self, symbols: Optional[List[str]] = None) -> FundPositionsResponse:
        """
        Get fund positions

        Args:
            symbols: Filter by fund codes

        Returns:
            Fund positions

        Examples:
            ::

                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.fund_positions()
                print(resp)
        """

    def stock_positions(self, symbols: Optional[List[str]] = None) -> StockPositionsResponse:
        """
        Get stock positions

        Args:
            symbols: Filter by stock codes

        Returns:
            Stock positions

        Examples:
            ::

                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.stock_positions()
                print(resp)
        """

    def margin_ratio(self, symbol: str) -> MarginRatio:
        """
        Get margin ratio

        Args:
            symbol: Security symbol

        Returns:
            Margin ratio

        Examples:
            ::

                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.margin_ratio("700.HK")
                print(resp)
        """

    def order_detail(self, order_id: str) -> OrderDetail:
        """
        Get order detail

        Args:
            order id: Order id

        Returns:
            Order detail

        Examples:
            ::

                from longport.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.order_detail("701276261045858304")
                print(resp)
        """

    def estimate_max_purchase_quantity(self, symbol: str, order_type: Type[OrderType], side: Type[OrderSide], price: Optional[Decimal] = None, currency: Optional[str] = None, order_id: Optional[str] = None) -> EstimateMaxPurchaseQuantityResponse:
        """
        Estimating the maximum purchase quantity for Hong Kong and US stocks, warrants, and options

        Args:
            symbol: Security symbol
            order_type: Order type
            side: Order side
            price: Estimated order price,
            currency: Settlement currency
            order_id: Order ID, required when estimating the maximum purchase quantity for a modified order

        Returns:
            Response

        Examples:
            ::

                from longport.openapi import TradeContext, Config, OrderType, OrderSide

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.estimate_max_purchase_quantity(
                    symbol = "700.HK",
                    order_type = OrderType.LO,
                    side = OrderSide.Buy,
                )
                print(resp)
        """
