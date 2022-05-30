from datetime import datetime, date, time
from decimal import Decimal
from typing import List, Optional, Type, Protocol


class Config:
    """
    Configuration options for Longbridge sdk

    Args:
        app_key: App Key
        app_secret: App Secret
        access_token: Access Token
        http_url: HTTP API url
        quote_ws_url: Websocket url for quote API
        trade_ws_url: Websocket url for trade API
    """

    def __init__(
        self,
        app_key: str,
        app_secret: str,
        access_token: str,
        http_url: str = "https://openapi.longbridgeapp.com",
        quote_ws_url: str = "wss://openapi-quote.longbridgeapp.com",
        trade_ws_url: str = "wss://openapi-trade.longbridgeapp.com",
    ) -> None: ...

    @classmethod
    def from_env(cls: Type) -> Config:
        """
        Create a new `Config` from the given environment variables

        # Variables

        - `LONGBRIDGE_APP_KEY` - App key
        - `LONGBRIDGE_APP_SECRET` - App secret
        - `LONGBRIDGE_ACCESS_TOKEN` - Access token
        - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url
        - `LONGBRIDGE_QUOTE_WS_URL` - Quote websocket endpoint url
        - `LONGBRIDGE_TRADE_WS_URL` - Trade websocket endpoint url
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


class QuoteHandler(Protocol):
    """
    Quote push message handler
    """

    def on_event(self, symbol: str, msg: PushQuote | PushDepth | PushBrokers | PushTrades) -> None:
        """
        Called when a new message is received
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

    outstanding_qty: int
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

    price: Decimal
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

    asks: List[Depth]
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
        One Days
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
    Trading session
    """


class MarketTradingSession:
    """
    Market trading session
    """

    market: Type[Market]
    """
    Market
    """

    trade_session: List[TradingSessionInfo]
    """
    Trading session
    """


class MarketTradingDays:
    trading_days: List[date]
    half_trading_days: List[date]


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


class QuoteContext:
    """
    Quote context

    Args:
        config: Configuration object
        handler: Push message handler
    """

    def __init__(
        self,
        config: Config,
        handler: Optional[QuoteHandler] = None,
    ) -> None: ...

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
                from longbridge.openapi import QuoteContext, Config, SubType

                class EventHandler:
                    def on_event(self, symbol: str, msg):
                        print(symbol, msg)

                config = Config.from_env()
                ctx = QuoteContext(config, EventHandler())

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote], is_first_push = True)
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

                from longbridge.openapi import QuoteContext, Config, SubType
                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                ctx.unsubscribe(["AAPL.US"], [SubType.Quote])
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

                from longbridge.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.static_info(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config

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

                from longbridge.openapi import QuoteContext, Config, Period, AdjustType

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.candlesticks("700.HK", Period.Day, 10, AdjustType.NoAdjust)
                print(resp)
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

                from longbridge.openapi import QuoteContext, Config

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
                from longbridge.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.option_chain_info_by_date("AAPL.US", date(2023, 1, 20))
                print(resp)
        """

    def warrant_issuers(self) -> List[IssuerInfo]:
        """
        Get warrant issuers

        Returns:
            Warrant issuers

        Examples:
            ::

                from longbridge.openapi import QuoteContext, Config

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.warrant_issuers()
                print(resp)
        """

    def trading_session(self) -> List[MarketTradingSession]:
        """
        Get trading session of the day

        Returns:
            Trading session of the day

        Examples:
            ::

                from longbridge.openapi import QuoteContext, Config

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
                from longbridge.openapi import QuoteContext, Config, Market

                config = Config.from_env()
                ctx = QuoteContext(config)

                resp = ctx.trading_days(Market.HK, date(2022, 1, 1), date(2022, 2, 1))
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
                from longbridge.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["HK.700", "AAPL.US"], [SubType.Quote], is_first_push = True)
                sleep(5)
                resp = ctx.realtime_quote(["HK.700", "AAPL.US"])
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
                from longbridge.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["HK.700", "AAPL.US"], [SubType.Depth], is_first_push = True)
                sleep(5)
                resp = ctx.realtime_depth("HK.700")
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
                from longbridge.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["HK.700", "AAPL.US"], [SubType.Brokers], is_first_push = True)
                sleep(5)
                resp = ctx.realtime_brokers("HK.700")
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
                from longbridge.openapi import QuoteContext, Config, SubType

                config = Config.from_env()
                ctx = QuoteContext(config)

                ctx.subscribe(["HK.700", "AAPL.US"], [SubType.Trade], is_first_push = False)
                sleep(5)
                resp = ctx.realtime_trades("HK.700", 10)
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

    quantity: Decimal
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

    quantity: str
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

    price: Decimal
    """
    Submitted price
    """

    executed_quantity: int
    """
    Executed quantity
    """

    executed_price: Decimal
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

    quantity: Decimal
    """
    Submitted quantity
    """

    executed_quantity: Decimal
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


class TradeHandler(Protocol):
    """
    Trade push message handler
    """

    def on_event(self, symbol: str, msg: PushOrderChanged) -> None:
        """
        Called when a new message is received
        """


class SubmitOrderResponse:
    """
    Response for withdraw order request
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
        Stock
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

    quantity: Decimal
    """
    The number of holdings
    """

    available_quality: Decimal
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


class TradeContext:
    """
    Trade context

    Args:
        config: Configuration object
        handler: Push message handler
    """

    def __init__(
        self,
        config: Config,
        handler: Optional[TradeHandler] = None,
    ) -> None: ...

    def subscribe(self, topics: List[str]) -> None:
        """
        Subscribe

        Args:
            topics: Topic list
        """

    def unsubscribe(self, topics: List[str]) -> None:
        """
        Unsubscribe

        Args:
            topics: Topic list
        """

    def history_executions(self, symbol: Optional[str] = None, start_at: Optional[date] = None, end_at: Optional[date] = None) -> List[Execution]:
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
                from longbridge.openapi import TradeContext, Config

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

                from longbridge.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.today_executions(symbol = "700.HK")
                print(resp)
        """

    def history_orders(self, symbol: Optional[str] = None, status: List[Type[OrderStatus]] = [], side: Optional[Type[OrderSide]] = None, market: Optional[Type[Market]] = None, start_at: Optional[date] = None, end_at: Optional[date] = None) -> List[Order]:
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
                from longbridge.openapi import TradeContext, Config, OrderStatus, OrderSide, Market

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

    def today_orders(self, symbol: Optional[str] = None, status: List[Type[OrderStatus]] = [], side: Optional[Type[OrderSide]] = None, market: Optional[Type[Market]] = None) -> List[Order]:
        """
        Get today orders

        Args:
            symbol: Filter by security code
            status: Filter by order status
            side: Filter by order side
            market: Filter by market type

        Returns:
            Order list

        Examples:
            ::

                from longbridge.openapi import TradeContext, Config, OrderStatus, OrderSide, Market

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

    def replace_order(self, order_id: str, quantity: Decimal, price: Optional[Decimal] = None, trigger_price: Optional[Decimal] = None, limit_offset: Optional[Decimal] = None, trailing_amount: Optional[Decimal] = None, trailing_percent: Optional[Decimal] = None, remark: Optional[str] = None) -> None:
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
                from longbridge.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                ctx.replace_order(
                    order_id = "709043056541253632",
                    quantity = Decimal("100"),
                    price = Decimal("100"),
                )
        """

    def submit_order(self, symbol: str, order_type: Type[OrderType], side: Type[OrderSide], submitted_quantity: Decimal, time_in_force: Type[TimeInForceType], submitted_price: Optional[Decimal] = None,  trigger_price: Optional[Decimal] = None, limit_offset: Optional[Decimal] = None, trailing_amount: Optional[Decimal] = None, trailing_percent: Optional[Decimal] = None, expire_date: Optional[date] = None,  outside_rth: Optional[Type[OutsideRTH]] = None,  remark: Optional[str] = None) -> SubmitOrderResponse:
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
                from longbridge.openapi import TradeContext, Config, OrderSide, OrderType, TimeInForceType

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.submit_order(
                    side = OrderSide.Buy,
                    symbol = "700.HK",
                    order_type = OrderType.LO,
                    submitted_price = Decimal("50"),
                    submitted_quantity = Decimal("200"),
                    time_in_force = TimeInForceType.Day,
                    remark = "Hello from Python SDK",
                )
                print(resp)
        """

    def withdraw_order(self, order_id: str) -> None:
        """
        Withdraw order

        Args:
            order_id: Order ID

        Examples:
            ::

                from longbridge.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                ctx.withdraw_order("709043056541253632")
        """

    def account_balance(self) -> List[AccountBalance]:
        """
        Get account balance

        Returns:
            Account list

        Examples:
            ::

                from longbridge.openapi import TradeContext, Config

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
                from longbridge.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.cash_flow(
                    start_at = datetime(2022, 5, 9),
                    end_at = datetime(2022, 5, 12),
                )
                print(resp)
        """

    def fund_positions(self, symbols: List[str] = []) -> FundPositionsResponse:
        """
        Get fund positions

        Args:
            symbols: Filter by fund codes

        Returns:
            Fund positions

        Examples:
            ::

                from longbridge.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.fund_positions()
                print(resp)
        """

    def stock_positions(self, symbols: List[str] = []) -> StockPositionsResponse:
        """
        Get stock positions

        Args:
            symbols: Filter by stock codes

        Returns:
            Stock positions

        Examples:
            ::

                from longbridge.openapi import TradeContext, Config

                config = Config.from_env()
                ctx = TradeContext(config)

                resp = ctx.stock_positions()
                print(resp)
        """
