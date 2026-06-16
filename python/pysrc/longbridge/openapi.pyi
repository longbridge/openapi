from datetime import date, datetime, time
from decimal import Decimal
from typing import Any, Awaitable, Callable, Coroutine, List, Optional, Type

class ErrorKind:
    """
    Error kind
    """

    class Http(ErrorKind):
        """
        HTTP error
        """

    class OpenApi(ErrorKind):
        """
        OpenApi error
        """

    class Other(ErrorKind):
        """
        Other error
        """

class OpenApiException(Exception):
    """
    OpenAPI exception
    """

    kind: ErrorKind
    """
    Error kind
    """

    code: int
    """
    Error code
    """

    message: str
    """
    Error message
    """

    def __init__(self, code: int, message: str) -> None: ...

class HttpClient:
    """
    A HTTP client for Longbridge OpenAPI.
    """

    @staticmethod
    def from_apikey(
        app_key: str,
        app_secret: str,
        access_token: str,
        http_url: Optional[str] = None,
    ) -> HttpClient:
        """
        Create a new ``HttpClient`` using API Key authentication.

        ``LONGBRIDGE_HTTP_URL`` is read from the environment automatically.
        Passing ``http_url`` overrides that value.

        Args:
            app_key: App Key
            app_secret: App Secret
            access_token: Access Token
            http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL``
                from env if omitted; falls back to
                ``https://openapi.longbridge.com``)
        """

    @classmethod
    def from_apikey_env(cls: Type[HttpClient]) -> HttpClient:
        """
        Create a new ``HttpClient`` from environment variables (API Key
        authentication).

        Variables:

        - ``LONGBRIDGE_HTTP_URL`` - HTTP endpoint url
        - ``LONGBRIDGE_APP_KEY`` - App key
        - ``LONGBRIDGE_APP_SECRET`` - App secret
        - ``LONGBRIDGE_ACCESS_TOKEN`` - Access token
        """

    @classmethod
    def from_oauth(
        cls: Type[HttpClient],
        oauth: OAuth,
        http_url: Optional[str] = None,
    ) -> HttpClient:
        """
        Create a new ``HttpClient`` from an OAuth handle.

        ``LONGBRIDGE_HTTP_URL`` is read from the environment automatically.
        Passing ``http_url`` overrides that value.

        Args:
            oauth: :class:`OAuth` handle from :meth:`OAuthBuilder.build` or
                :meth:`OAuthBuilder.build_async`
            http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL``
                from env if omitted; falls back to
                ``https://openapi.longbridge.com``)
        """

    def request(
        self,
        method: str,
        path: str,
        headers: Optional[dict[str, str]] = None,
        body: Optional[Any] = None,
    ) -> Any:
        """
        Performs a HTTP reqest

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, HttpClient

                oauth = OAuthBuilder("your-client-id").build(lambda url: print("Visit:", url))
                client = HttpClient.from_oauth(oauth)

                # get
                resp = client.request("get", "/foo/bar")
                print(resp)

                # post
                client.request("post", "/foo/bar", body={ "foo": 1, "bar": 2 })
        """
        ...

    def request_async(
        self,
        method: str,
        path: str,
        headers: Optional[dict[str, str]] = None,
        body: Optional[Any] = None,
    ) -> Awaitable[Any]:
        """
        Performs an async HTTP request. Returns an awaitable; must be awaited inside asyncio.

        Args:
            method: HTTP method (e.g. "get", "post").
            path: Request path (e.g. "/v1/trade/execution/today").
            headers: Optional request headers.
            body: Optional JSON-serializable request body.

        Returns:
            An awaitable that resolves to the response body (same as sync request).

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, HttpClient

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    http_cli = HttpClient.from_oauth(oauth)
                    resp = await http_cli.request_async(
                        "get",
                        "/v1/trade/execution/today",
                    )
                    print(resp)

                asyncio.run(main())
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

class OAuth:
    """
    OAuth 2.0 client handle for Longbridge OpenAPI.

    Obtain an instance via :meth:`OAuthBuilder.build` (blocking) or
    :meth:`AsyncOAuthBuilder.build` (async).  Pass it to
    :meth:`Config.from_oauth` or :meth:`HttpClient.from_oauth`.
    """

class OAuthBuilder:
    """
    Builder for the OAuth 2.0 authorization flow.

    Args:
        client_id: OAuth 2.0 client ID from the Longbridge developer portal
        callback_port: TCP port for the local callback server (default 60355).
            Must match one of the redirect URIs registered for the client.

    Example (blocking)::

        from longbridge.openapi import OAuthBuilder, Config

        oauth = OAuthBuilder("your-client-id").build(
            lambda url: print("Open:", url)
        )
        config = Config.from_oauth(oauth)

    Example (async)::

        import asyncio
        from longbridge.openapi import OAuthBuilder, Config

        async def main():
            oauth = await OAuthBuilder("your-client-id").build_async(
                lambda url: print("Open:", url)
            )
            config = Config.from_oauth(oauth)

        asyncio.run(main())
    """

    def __init__(self, client_id: str, callback_port: Optional[int] = None) -> None: ...
    def build(self, on_open_url: Callable[[str], None]) -> OAuth:
        """
        Build an OAuth 2.0 client (blocking).

        If a valid token is already cached on disk
        (``~/.longbridge/openapi/tokens/<client_id>``) it is reused;
        otherwise the browser authorization flow is started and
        ``on_open_url`` is called with the authorization URL.

        Args:
            on_open_url: Callable that receives the authorization URL as a
                string.

        Returns:
            :class:`OAuth` handle
        """

    async def build_async(self, on_open_url: Callable[[str], None]) -> OAuth:
        """
        Build an OAuth 2.0 client (async).

        If a valid token is already cached on disk
        (``~/.longbridge/openapi/tokens/<client_id>``) it is reused;
        otherwise the browser authorization flow is started and
        ``on_open_url`` is called with the authorization URL.

        Args:
            on_open_url: Callable that receives the authorization URL as a
                string.

        Returns:
            Awaitable resolving to an :class:`OAuth` handle
        """

class Config:
    """
    Configuration options for Longbridge SDK

    Args:
        app_key: App Key
        app_secret: App Secret
        access_token: Access Token
        http_url: HTTP API url (default: ``https://openapi.longbridge.com``)
        quote_ws_url: Websocket url for quote API
        trade_ws_url: Websocket url for trade API
        language: Language identifier (default: ``Language.EN``)
        enable_overnight: Enable overnight quote (default: ``False``)
        push_candlestick_mode: Push candlestick mode
        enable_print_quote_packages: Print opened quote packages on connect
            (default: ``True``)
        log_path: Path for log files (default: no logs)
    """

    @staticmethod
    def from_apikey(
        app_key: str,
        app_secret: str,
        access_token: str,
        http_url: Optional[str] = None,
        quote_ws_url: Optional[str] = None,
        trade_ws_url: Optional[str] = None,
        language: Optional[Type[Language]] = None,
        enable_overnight: bool = False,
        push_candlestick_mode: Type[PushCandlestickMode] = PushCandlestickMode.Realtime,
        enable_print_quote_packages: bool = True,
        log_path: Optional[str] = None,
        enable_papertrading: bool = False,
    ) -> Config:
        """
        Create a new ``Config`` using API Key authentication.

        Optional environment variables are read automatically
        (``LONGBRIDGE_HTTP_URL``, ``LONGBRIDGE_LANGUAGE``,
        ``LONGBRIDGE_QUOTE_WS_URL``, ``LONGBRIDGE_TRADE_WS_URL``,
        ``LONGBRIDGE_ENABLE_OVERNIGHT``, ``LONGBRIDGE_PUSH_CANDLESTICK_MODE``,
        ``LONGBRIDGE_PRINT_QUOTE_PACKAGES``, ``LONGBRIDGE_LOG_PATH``,
        ``LONGBRIDGE_PAPERTRADING``).
        Any explicit parameter overrides the corresponding env variable.

        Args:
            app_key: App Key
            app_secret: App Secret
            access_token: Access Token
            http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL``
                from env if omitted)
            quote_ws_url: Quote WS url override (reads
                ``LONGBRIDGE_QUOTE_WS_URL`` from env if omitted)
            trade_ws_url: Trade WS url override (reads
                ``LONGBRIDGE_TRADE_WS_URL`` from env if omitted)
            language: Language identifier override (reads
                ``LONGBRIDGE_LANGUAGE`` from env if omitted)
            enable_overnight: Enable overnight quote (default: ``False``)
            push_candlestick_mode: Push candlestick mode
            enable_print_quote_packages: Print opened quote packages on
                connect (default: ``True``)
            log_path: Path for log files (default: no logs)
            enable_papertrading: When ``True``, all API calls target the
                paper trading (simulation) environment.  The server validates
                the token: if it belongs to a real-money account the server
                returns an error.  Default: ``False`` (no restriction imposed
                by server).
        """

    @classmethod
    def from_apikey_env(cls: Type[Config]) -> Config:
        """
        Create a new ``Config`` from environment variables (API Key
        authentication).

        It first gets the environment variables from the ``.env`` file in the
        current directory.

        Variables:

        - ``LONGBRIDGE_APP_KEY`` - App key
        - ``LONGBRIDGE_APP_SECRET`` - App secret
        - ``LONGBRIDGE_ACCESS_TOKEN`` - Access token
        - ``LONGBRIDGE_LANGUAGE`` - ``zh-CN``, ``zh-HK`` or ``en``
          (Default: ``en``)
        - ``LONGBRIDGE_HTTP_URL`` - HTTP endpoint url
        - ``LONGBRIDGE_QUOTE_WS_URL`` - Quote websocket endpoint url
        - ``LONGBRIDGE_TRADE_WS_URL`` - Trade websocket endpoint url
        - ``LONGBRIDGE_ENABLE_OVERNIGHT`` - ``true`` or ``false``
          (Default: ``false``)
        - ``LONGBRIDGE_PUSH_CANDLESTICK_MODE`` - ``realtime`` or ``confirmed``
          (Default: ``realtime``)
        - ``LONGBRIDGE_PRINT_QUOTE_PACKAGES`` - ``true`` or ``false``
          (Default: ``true``)
        - ``LONGBRIDGE_LOG_PATH`` - Log file directory (Default: no logs)
        """

    @classmethod
    def from_oauth(
        cls: Type[Config],
        oauth: OAuth,
        http_url: Optional[str] = None,
        quote_ws_url: Optional[str] = None,
        trade_ws_url: Optional[str] = None,
        language: Optional[Type[Language]] = None,
        enable_overnight: Optional[bool] = None,
        push_candlestick_mode: Optional[Type[PushCandlestickMode]] = None,
        enable_print_quote_packages: Optional[bool] = None,
        log_path: Optional[str] = None,
        enable_papertrading: Optional[bool] = None,
    ) -> Config:
        """
        Create a new ``Config`` for OAuth 2.0 authentication.

        OAuth 2.0 is the recommended authentication method — no app_secret or
        HMAC signatures required.

        Optional environment variables are read automatically
        (``LONGBRIDGE_HTTP_URL``, ``LONGBRIDGE_LANGUAGE``,
        ``LONGBRIDGE_QUOTE_WS_URL``, ``LONGBRIDGE_TRADE_WS_URL``,
        ``LONGBRIDGE_ENABLE_OVERNIGHT``, ``LONGBRIDGE_PUSH_CANDLESTICK_MODE``,
        ``LONGBRIDGE_PRINT_QUOTE_PACKAGES``, ``LONGBRIDGE_LOG_PATH``,
        ``LONGBRIDGE_PAPERTRADING``).
        Any explicit parameter overrides the corresponding env variable.

        Args:
            oauth: :class:`OAuth` handle from :meth:`OAuthBuilder.build` or
                :meth:`AsyncOAuthBuilder.build`
            http_url: HTTP API url override (reads ``LONGBRIDGE_HTTP_URL``
                from env if omitted)
            quote_ws_url: Quote WS url override (reads
                ``LONGBRIDGE_QUOTE_WS_URL`` from env if omitted)
            trade_ws_url: Trade WS url override (reads
                ``LONGBRIDGE_TRADE_WS_URL`` from env if omitted)
            language: Language identifier override (reads
                ``LONGBRIDGE_LANGUAGE`` from env if omitted)
            enable_overnight: Enable overnight quote (optional)
            push_candlestick_mode: Push candlestick mode (optional)
            enable_print_quote_packages: Print opened quote packages on
                connect (optional)
            log_path: Path for log files (optional)
            enable_papertrading: When ``True``, all API calls target the
                paper trading (simulation) environment.  The server validates
                the token: if it belongs to a real-money account the server
                returns an error.  Default: ``None`` (no restriction imposed
                by server).

        Returns:
            Config object
        """

    def refresh_access_token(
        self,
        expired_at: Optional[datetime] = None,
    ) -> str:
        """
        Gets a new ``access_token``.

        This method is only available when using **Legacy API Key**
        authentication (i.e. :meth:`Config.from_apikey`). It is not supported
        for OAuth 2.0 mode.

        Args:
            expired_at: The expiration time of the access token (default: 90
                days from now).

        Returns:
            New access token string
        """

    async def refresh_access_token_async(
        self,
        expired_at: Optional[datetime] = None,
    ) -> str:
        """
        Async version of :meth:`Config.refresh_access_token`. Returns an
        awaitable; must be awaited inside asyncio.

        This method is only available when using **Legacy API Key**
        authentication (i.e. :meth:`Config.from_apikey`). It is not supported
        for OAuth 2.0 mode.

        Args:
            expired_at: The expiration time of the access token (default: 90
                days from now).

        Returns:
            New access token string
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

    class Crypto(Market):
        """
        Crypto market
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

    current_volume: int
    """
    Increase volume between pushes
    """

    current_turnover: Decimal
    """
    Increase turnover between pushes
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

    is_confirmed: bool
    """
    Is confirmed
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

    class SPXIndex(SecurityBoard):
        """
        S&P 500 Index
        """

    class VIXIndex(SecurityBoard):
        """
        CBOE Volatility Index
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
    Dividend (per share), **not** the dividend yield (ratio).
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

    class Suspend(TradeStatus):
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

    class Intraday(TradeSession):
        """
        Intraday
        """

    class Pre(TradeSession):
        """
        Pre-Market
        """

    class Post(TradeSession):
        """
        Post-Market
        """

    class Overnight(TradeSession):
        """
        Overnight
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

    trade_session: TradeSession
    """
    Trade session
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

    class Min_2(Period):
        """
        Two Minutes
        """

    class Min_3(Period):
        """
        Three Minutes
        """

    class Min_5(Period):
        """
        Five Minutes
        """

    class Min_10(Period):
        """
        Ten Minutes
        """

    class Min_15(Period):
        """
        Fifteen Minutes
        """

    class Min_20(Period):
        """
        Twenty Minutes
        """

    class Min_30(Period):
        """
        Thirty Minutes
        """

    class Min_45(Period):
        """
        Forty-Five Minutes
        """

    class Min_60(Period):
        """
        Sixty Minutes
        """

    class Min_120(Period):
        """
        Two Hours
        """

    class Min_180(Period):
        """
        Three Hours
        """

    class Min_240(Period):
        """
        Four Hours
        """

    class Day(Period):
        """
        Daily
        """

    class Week(Period):
        """
        Weekly
        """

    class Month(Period):
        """
        Monthly
        """

    class Quarter(Period):
        """
        Quarterly
        """

    class Year(Period):
        """
        Yearly
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

class PinnedMode:
    """Pinned mode for watchlist securities."""

    class Add(PinnedMode):
        """Pin (add) securities to the top of the group"""

    class Remove(PinnedMode):
        """Unpin (remove) securities from the top of the group"""

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

class QuotePackageDetail:
    """
    Quote package detail
    """

    key: str
    """
    Key
    """

    name: str
    """
    Name
    """

    description: str
    """
    Description
    """

    start_at: datetime
    """
    Start time
    """

    end_at: datetime
    """
    End time
    """

class TradeSessions:
    """
    Trade sessions
    """

    class Intraday(TradeSessions):
        """
        Intraday
        """

    class All(TradeSessions):
        """
        All
        """

class MarketTemperature:
    """
    Market temperature
    """

    temperature: int
    """
    Temperature value
    """

    description: str
    """
    Temperature description
    """

    valuation: int
    """
    Market valuation
    """

    sentiment: int
    """
    Market sentiment
    """

    timestamp: datetime
    """
    Time
    """

class Granularity:
    """
    Data granularity
    """

    class Unknown(Granularity):
        """
        Unknown
        """

    class Daily(Granularity):
        """
        Daily
        """

    class Weekly(Granularity):
        """
        Weekly
        """

    class Monthly(Granularity):
        """
        Monthly
        """

class HistoryMarketTemperatureResponse:
    """
    History market temperature response
    """

    granularity: Type[Granularity]
    """
    Granularity
    """

    records: List[MarketTemperature]
    """
    Records
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

    def quote_package_details(self) -> List[QuotePackageDetail]:
        """
        Returns the quote package details
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

    def set_on_candlestick(
        self, callback: Callable[[str, PushCandlestick], None]
    ) -> None:
        """
        Set candlestick callback, after receiving the candlestick updated event, it will call back to this function.
        """

    def subscribe(self, symbols: List[str], sub_types: List[Type[SubType]]) -> None:
        """
        Subscribe

        Args:
            symbols: Security codes
            sub_types: Subscribe types

        Examples:
            ::

                from time import sleep
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SubType, PushQuote

                def on_quote(symbol: str, event: PushQuote):
                    print(symbol, event)

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)
                ctx.set_on_quote(on_quote)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SubType
                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                ctx.unsubscribe(["AAPL.US"], [SubType.Quote])
        """

    def subscribe_candlesticks(
        self,
        symbol: str,
        period: Type[Period],
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> List[Candlestick]:
        """
        Subscribe security candlesticks

        Args:
            symbol: Security code
            period: Period type
            trade_sessions: Trade sessions

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, PushCandlestick, TradeSessions
                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                def on_candlestick(symbol: str, event: PushCandlestick):
                    print(symbol, event)

                ctx.set_on_candlestick(on_candlestick)
                ctx.subscribe_candlesticks("700.HK", Period.Min_1, TradeSessions.Intraday)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SubType
                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.trades("700.HK", 10)
                print(resp)
        """

    def intraday(
        self, symbol: str, trade_sessions: Type[TradeSessions] = TradeSessions.Intraday
    ) -> List[IntradayLine]:
        """
        Get security intraday lines

        Args:
            symbol: Security code
            trade_sessions: Trade sessions

        Returns:
            Intraday lines

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, TradeSessions

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.intraday("700.HK", TradeSessions.Intraday)
                print(resp)
        """

    def candlesticks(
        self,
        symbol: str,
        period: Type[Period],
        count: int,
        adjust_type: Type[AdjustType],
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> List[Candlestick]:
        """
        Get security candlesticks

        Args:
            symbol: Security code
            period: Candlestick period
            count: Count of cancdlestick (Maximum is `1000`)
            adjust_type: Adjustment type
            trade_sessions: Trade sessions

        Returns:
            Candlesticks

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, Period, AdjustType, TradeSessions

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.candlesticks(
                    "700.HK", Period.Day, 10, AdjustType.NoAdjust, TradeSessions.Intraday)
                print(resp)
        """

    def history_candlesticks_by_offset(
        self,
        symbol: str,
        period: Type[Period],
        adjust_type: Type[AdjustType],
        forward: bool,
        count: int,
        time: Optional[datetime] = None,
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> List[Candlestick]:
        """
        Get security history candlesticks by offset

        Args:
            symbol: Security code
            period: Period type
            adjust_type: Adjust type
            forward: If `True`, query the latest from the specified time
            count: Count of candlesticks
            time: Datetime
            trade_sessions: Trade sessions
        """

    def history_candlesticks_by_date(
        self,
        symbol: str,
        period: Type[Period],
        adjust_type: Type[AdjustType],
        start: Optional[date],
        end: Optional[date],
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> List[Candlestick]:
        """
        Get security history candlesticks by date

        Args:
            symbol: Security code
            period: Period type
            adjust_type: Adjust type
            start: Start date
            end: End date
            trade_sessions: Trade sessions
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.option_chain_expiry_date_list("AAPL.US")
                print(resp)
        """

    def option_chain_info_by_date(
        self, symbol: str, expiry_date: date
    ) -> List[StrikePriceInfo]:
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
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.warrant_issuers()
                print(resp)
        """

    def warrant_list(
        self,
        symbol: str,
        sort_by: Type[WarrantSortBy],
        sort_order: Type[SortOrderType],
        warrant_type: Optional[List[Type[WarrantType]]] = None,
        issuer: Optional[List[int]] = None,
        expiry_date: Optional[List[Type[FilterWarrantExpiryDate]]] = None,
        price_type: Optional[List[Type[FilterWarrantInOutBoundsType]]] = None,
        status: Optional[List[Type[WarrantStatus]]] = None,
    ) -> List[WarrantInfo]:
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, WarrantSortBy, SortOrderType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.trading_session()
                print(resp)
        """

    def trading_days(
        self, market: Type[Market], begin: date, end: date
    ) -> MarketTradingDays:
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
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, Market

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.capital_distribution("700.HK")
                print(resp)
        """

    def calc_indexes(
        self, symbols: List[str], indexes: List[Type[CalcIndex]]
    ) -> List[SecurityCalcIndex]:
        """
        Get calc indexes

        Args:
            symbols: Security codes
            indexes: Calc indexes

        Returns:
            Calc indexes of the symbols

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, CalcIndex

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.watchlist()
                print(resp)
        """

    def create_watchlist_group(
        self, name: str, securities: Optional[List[str]] = None
    ) -> int:
        """
        Create watchlist group

        Args:
            name: Group name
            securities: Securities

        Returns:
            Group ID

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)
                ctx.delete_watchlist_group(10086)
        """

    def update_watchlist_group(
        self,
        id: int,
        name: Optional[str] = None,
        securities: Optional[List[str]] = None,
        mode: Optional[Type[SecuritiesUpdateMode]] = None,
    ):
        """
        Update watchlist group

        Args:
            id: Group ID
            name: Group name
            securities: Securities

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SecuritiesUpdateMode

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)
                ctx.update_watchlist_group(10086, name = "Watchlist2", securities = ["700.HK", "AAPL.US"], SecuritiesUpdateMode.Replace)
        """

    def update_pinned(
        self,
        mode: Type[PinnedMode],
        symbols: List[str],
    ) -> None:
        """
        Pin or unpin watchlist securities.

        Args:
            mode: :class:`PinnedMode.Add` to pin, :class:`PinnedMode.Remove` to unpin
            symbols: List of security symbols to pin/unpin
        """

    def security_list(
        self,
        market: Type[Market],
        category: Optional[Type[SecurityListCategory]] = None,
    ) -> List[Security]:
        """
        Get security list

        Args:
            market: Market
            category: Security list category

        Returns:
            Security list

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, Market, SecurityListCategory

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.security_list(Market.HK, SecurityListCategory.Overnight)
                print(resp)
        """

    def market_temperature(self, market: Type[Market]) -> MarketTemperature:
        """
        Get current market temperature

        Args:
            market: Market

        Returns:
            Market temperature

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, Market

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.market_temperature(Market.HK)
                print(resp)
        """

    def history_market_temperature(
        self, market: Type[Market], start_date: date, end_date: date
    ) -> HistoryMarketTemperatureResponse:
        """
        Get historical market temperature

        Args:
            market: Market
            start_date: Start date
            end_date: End date

        Returns:
            History market temperature

        Examples:
            ::

                from datetime import date
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, Market

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                resp = ctx.history_market_temperature(Market.HK, date(2023, 1, 1), date(2023, 1, 31))
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
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SubType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
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
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SubType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Depth])
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
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SubType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Brokers])
                sleep(5)
                resp = ctx.realtime_brokers("700.HK")
                print(resp)
        """

    def realtime_trades(self, symbol: str, count: int = 500) -> List[Trade]:
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
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, SubType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Trade])
                sleep(5)
                resp = ctx.realtime_trades("700.HK", 10)
                print(resp)
        """

    def realtime_candlesticks(
        self, symbol: str, period: Type[Period], count: int = 500
    ) -> List[Candlestick]:
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
                from longbridge.openapi import OAuthBuilder, QuoteContext, Config, Period

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = QuoteContext(config)

                ctx.subscribe_candlesticks("AAPL.US", Period.Min_1)
                sleep(5)
                resp = ctx.realtime_candlesticks("AAPL.US", Period.Min_1, 10)
                print(resp)
        """

    def short_positions(
        self, symbol: str, count: int = 20
    ) -> ShortPositionsResponse:
        """
        Get short interest / position data for a US or HK security.

        Market is inferred from the symbol suffix: ``.HK`` → HK endpoint,
        otherwise US endpoint.

        Args:
            symbol: Security code (e.g. ``"700.HK"`` or ``"AAPL.US"``)
            count: Number of records (1–100, default 20)

        Returns:
            :class:`ShortPositionsResponse` with raw JSON data
        """

    def short_trades(
        self, symbol: str, count: int = 20
    ) -> ShortTradesResponse:
        """
        Get short trade records for a HK or US security.

        Market is inferred from the symbol suffix: ``.HK`` → HK endpoint,
        otherwise US endpoint.

        Args:
            symbol: Security code
            count: Number of records (1–100, default 20)

        Returns:
            :class:`ShortTradesResponse` with raw JSON data
        """

    def us_crypto_overview(self, symbol: str) -> "USCryptoOverview":
        """Get US cryptocurrency market overview. US token required.

        Args:
            symbol: Trading-pair symbol, e.g. ``"BTCUSD.BKKT"``

        Returns:
            :class:`USCryptoOverview`
        """
        ...

class AsyncQuoteContext:
    """
    Async quote context for use with asyncio. Create via `AsyncQuoteContext.create(config)` and await inside asyncio.
    Callbacks (set_on_quote, set_on_depth, etc.) are set the same way as the sync QuoteContext; all I/O methods return awaitables.
    """

    @classmethod
    def create(
        cls: Type[AsyncQuoteContext], config: Config, loop_: Optional[Any] = None
    ) -> AsyncQuoteContext:
        """
        Create an async quote context.

        Args:
            config: Configuration object.
            loop_: Optional event loop; pass asyncio.get_running_loop() when using async callbacks.

        Returns:
            AsyncQuoteContext instance.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, Config, AsyncQuoteContext

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.quote(["700.HK", "AAPL.US"])
                    print(resp)

                asyncio.run(main())
        """
        ...

    async def member_id(self) -> int:
        """Returns the member ID."""
        ...

    async def quote_level(self) -> str:
        """Returns the quote level."""
        ...

    async def quote_package_details(self) -> List[QuotePackageDetail]:
        """Returns the quote package details."""
        ...

    def set_on_quote(
        self,
        callback: Callable[[str, PushQuote], None]
        | Callable[[str, PushQuote], Coroutine[Any, Any, None]],
    ) -> None:
        """Set quote callback; called when quote push is received. Callback may be sync or async (async is scheduled on the event loop)."""
        ...

    def set_on_depth(
        self,
        callback: Callable[[str, PushDepth], None]
        | Callable[[str, PushDepth], Coroutine[Any, Any, None]],
    ) -> None:
        """Set depth callback; called when depth push is received. Callback may be sync or async (async is scheduled on the event loop)."""
        ...

    def set_on_brokers(
        self,
        callback: Callable[[str, PushBrokers], None]
        | Callable[[str, PushBrokers], Coroutine[Any, Any, None]],
    ) -> None:
        """Set brokers callback; called when brokers push is received. Callback may be sync or async (async is scheduled on the event loop)."""
        ...

    def set_on_trades(
        self,
        callback: Callable[[str, PushTrades], None]
        | Callable[[str, PushTrades], Coroutine[Any, Any, None]],
    ) -> None:
        """Set trades callback; called when trades push is received. Callback may be sync or async (async is scheduled on the event loop)."""
        ...

    def set_on_candlestick(
        self,
        callback: Callable[[str, PushCandlestick], None]
        | Callable[[str, PushCandlestick], Coroutine[Any, Any, None]],
    ) -> None:
        """Set candlestick callback; called when candlestick push is received. Callback may be sync or async (async is scheduled on the event loop)."""
        ...

    def subscribe(
        self, symbols: List[str], sub_types: List[Type[SubType]]
    ) -> Awaitable[None]:
        """
        Subscribe to symbols and sub types. Returns an awaitable; must be awaited in asyncio.

        Args:
            symbols: Security codes.
            sub_types: Subscribe types.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SubType, PushQuote

                def on_quote(symbol: str, event: PushQuote):
                    print(symbol, event)

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    ctx.set_on_quote(on_quote)
                    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                    await asyncio.sleep(30)

                asyncio.run(main())
        """
        ...

    def unsubscribe(
        self, symbols: List[str], sub_types: List[Type[SubType]]
    ) -> Awaitable[None]:
        """
        Unsubscribe from symbols and sub types. Returns an awaitable.

        Args:
            symbols: Security codes.
            sub_types: Subscribe types.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SubType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                    await ctx.unsubscribe(["AAPL.US"], [SubType.Quote])

                asyncio.run(main())
        """
        ...

    def subscribe_candlesticks(
        self,
        symbol: str,
        period: Type[Period],
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> Awaitable[List[Candlestick]]:
        """
        Subscribe security candlesticks. Returns an awaitable that resolves to initial candlesticks.

        Args:
            symbol: Security code.
            period: Period type.
            trade_sessions: Trade sessions.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, (
                    AsyncQuoteContext,
                    Config,
                    Period,
                    PushCandlestick,
                    TradeSessions,
                )

                def on_candlestick(symbol: str, event: PushCandlestick):
                    print(symbol, event)

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    ctx.set_on_candlestick(on_candlestick)
                    await ctx.subscribe_candlesticks(
                        "700.HK",
                        Period.Min_1,
                        TradeSessions.Intraday,
                    )
                    await asyncio.sleep(30)

                asyncio.run(main())
        """
        ...

    def unsubscribe_candlesticks(
        self, symbol: str, period: Type[Period]
    ) -> Awaitable[None]:
        """
        Unsubscribe security candlesticks. Returns an awaitable.

        Args:
            symbol: Security code.
            period: Period type.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, (
                    AsyncQuoteContext,
                    Config,
                    Period,
                    TradeSessions,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe_candlesticks(
                        "700.HK",
                        Period.Min_1,
                        TradeSessions.Intraday,
                    )
                    await ctx.unsubscribe_candlesticks("700.HK", Period.Min_1)

                asyncio.run(main())
        """
        ...

    def subscriptions(self) -> Awaitable[List[Subscription]]:
        """
        Get subscription information. Returns an awaitable that resolves to subscription list.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SubType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                    resp = await ctx.subscriptions()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def static_info(self, symbols: List[str]) -> Awaitable[List[SecurityStaticInfo]]:
        """
        Get basic information of securities. Returns an awaitable that resolves to security info list.

        Args:
            symbols: Security codes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.static_info(
                        ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def quote(self, symbols: List[str]) -> Awaitable[List[SecurityQuote]]:
        """
        Get quote of securities. Returns an awaitable that resolves to security quote list.

        Args:
            symbols: Security codes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
                    print(resp)

                asyncio.run(main())
        """
        ...

    def option_quote(self, symbols: List[str]) -> Awaitable[List[OptionQuote]]:
        """
        Get quote of option securities. Returns an awaitable that resolves to option quote list.

        Args:
            symbols: Security codes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.option_quote(["AAPL230317P160000.US"])
                    print(resp)

                asyncio.run(main())
        """
        ...

    def warrant_quote(self, symbols: List[str]) -> Awaitable[List[WarrantQuote]]:
        """
        Get quote of warrant securities. Returns an awaitable that resolves to warrant quote list.

        Args:
            symbols: Security codes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.warrant_quote(["21125.HK"])
                    print(resp)

                asyncio.run(main())
        """
        ...

    def depth(self, symbol: str) -> Awaitable[SecurityDepth]:
        """
        Get security depth. Returns an awaitable that resolves to security depth.

        Args:
            symbol: Security code.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.depth("700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def brokers(self, symbol: str) -> Awaitable[SecurityBrokers]:
        """
        Get security brokers. Returns an awaitable that resolves to security brokers.

        Args:
            symbol: Security code.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.brokers("700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def participants(self) -> Awaitable[List[ParticipantInfo]]:
        """
        Get participants. Returns an awaitable that resolves to participant list.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.participants()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def trades(self, symbol: str, count: int) -> Awaitable[List[Trade]]:
        """
        Get security trades. Returns an awaitable that resolves to trades list (max count 1000).

        Args:
            symbol: Security code.
            count: Count of trades.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.trades("700.HK", 10)
                    print(resp)

                asyncio.run(main())
        """
        ...

    def intraday(
        self, symbol: str, trade_sessions: Type[TradeSessions] = TradeSessions.Intraday
    ) -> Awaitable[List[IntradayLine]]:
        """
        Get security intraday lines. Returns an awaitable that resolves to intraday line list.

        Args:
            symbol: Security code.
            trade_sessions: Trade sessions.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, TradeSessions

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.intraday("700.HK", TradeSessions.Intraday)
                    print(resp)

                asyncio.run(main())
        """
        ...

    def candlesticks(
        self,
        symbol: str,
        period: Type[Period],
        count: int,
        adjust_type: Type[AdjustType],
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> Awaitable[List[Candlestick]]:
        """
        Get security candlesticks. Returns an awaitable that resolves to candlesticks list (max count 1000).

        Args:
            symbol: Security code.
            period: Candlestick period.
            count: Count of candlesticks.
            adjust_type: Adjustment type.
            trade_sessions: Trade sessions.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, (
                    AsyncQuoteContext,
                    Config,
                    Period,
                    AdjustType,
                    TradeSessions,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.candlesticks(
                        "700.HK",
                        Period.Day,
                        10,
                        AdjustType.NoAdjust,
                        TradeSessions.Intraday,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def history_candlesticks_by_offset(
        self,
        symbol: str,
        period: Type[Period],
        adjust_type: Type[AdjustType],
        forward: bool,
        count: int,
        time: Optional[datetime] = None,
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> Awaitable[List[Candlestick]]:
        """
        Get security history candlesticks by offset. Returns an awaitable that resolves to candlesticks list.

        Args:
            symbol: Security code.
            period: Period type.
            adjust_type: Adjust type.
            forward: If True, query the latest from the specified time.
            count: Count of candlesticks.
            time: Datetime.
            trade_sessions: Trade sessions.

        Examples:
            ::

                import asyncio
                import datetime
                from longbridge.openapi import OAuthBuilder, (
                    AsyncQuoteContext,
                    Config,
                    Period,
                    AdjustType,
                    TradeSessions,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.history_candlesticks_by_offset(
                        "700.HK",
                        Period.Day,
                        AdjustType.NoAdjust,
                        False,
                        10,
                        datetime.datetime(2023, 8, 18),
                        TradeSessions.Intraday,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def history_candlesticks_by_date(
        self,
        symbol: str,
        period: Type[Period],
        adjust_type: Type[AdjustType],
        start: Optional[date],
        end: Optional[date],
        trade_sessions: Type[TradeSessions] = TradeSessions.Intraday,
    ) -> Awaitable[List[Candlestick]]:
        """
        Get security history candlesticks by date. Returns an awaitable that resolves to candlesticks list.

        Args:
            symbol: Security code.
            period: Period type.
            adjust_type: Adjust type.
            start: Start date.
            end: End date.
            trade_sessions: Trade sessions.

        Examples:
            ::

                import asyncio
                import datetime
                from longbridge.openapi import OAuthBuilder, (
                    AsyncQuoteContext,
                    Config,
                    Period,
                    AdjustType,
                    TradeSessions,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.history_candlesticks_by_date(
                        "700.HK",
                        Period.Day,
                        AdjustType.NoAdjust,
                        datetime.date(2022, 5, 5),
                        datetime.date(2022, 6, 23),
                        TradeSessions.Intraday,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def option_chain_expiry_date_list(self, symbol: str) -> Awaitable[List[date]]:
        """
        Get option chain expiry date list. Returns an awaitable that resolves to date list.

        Args:
            symbol: Security code.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.option_chain_expiry_date_list("AAPL.US")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def option_chain_info_by_date(
        self, symbol: str, expiry_date: date
    ) -> Awaitable[List[StrikePriceInfo]]:
        """
        Get option chain info by date. Returns an awaitable that resolves to strike price info list.

        Args:
            symbol: Security code.
            expiry_date: Expiry date.

        Examples:
            ::

                import asyncio
                from datetime import date
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.option_chain_info_by_date(
                        "AAPL.US",
                        date(2023, 1, 20),
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def warrant_issuers(self) -> Awaitable[List[IssuerInfo]]:
        """
        Get warrant issuers. Returns an awaitable that resolves to issuer list.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.warrant_issuers()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def warrant_list(
        self,
        symbol: str,
        sort_by: Type[WarrantSortBy],
        sort_order: Type[SortOrderType],
        warrant_type: Optional[List[Type[WarrantType]]] = None,
        issuer: Optional[List[int]] = None,
        expiry_date: Optional[List[Type[FilterWarrantExpiryDate]]] = None,
        price_type: Optional[List[Type[FilterWarrantInOutBoundsType]]] = None,
        status: Optional[List[Type[WarrantStatus]]] = None,
    ) -> Awaitable[List[WarrantInfo]]:
        """
        Get warrant list with optional filters. Returns an awaitable that resolves to warrant info list.

        Args:
            symbol: Security code.
            sort_by: Sort by field.
            sort_order: Sort order.
            warrant_type: Filter by warrant type.
            issuer: Filter by issuer.
            expiry_date: Filter by expiry date.
            price_type: Filter by price type.
            status: Filter by status.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, (
                    AsyncQuoteContext,
                    Config,
                    WarrantSortBy,
                    SortOrderType,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.warrant_list(
                        "700.HK",
                        WarrantSortBy.LastDone,
                        SortOrderType.Ascending,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def trading_session(self) -> Awaitable[List[MarketTradingSession]]:
        """
        Get trading session of the day. Returns an awaitable that resolves to market trading session list.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.trading_session()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def trading_days(
        self, market: Type[Market], begin: date, end: date
    ) -> Awaitable[MarketTradingDays]:
        """
        Get trading days in the given market and date range. Returns an awaitable (interval must be less than one month).

        Args:
            market: Market.
            begin: Begin date.
            end: End date.

        Examples:
            ::

                import asyncio
                from datetime import date
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, Market

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.trading_days(
                        Market.HK,
                        date(2022, 1, 1),
                        date(2022, 2, 1),
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def capital_flow(self, symbol: str) -> Awaitable[List[CapitalFlowLine]]:
        """
        Get capital flow intraday. Returns an awaitable that resolves to capital flow line list.

        Args:
            symbol: Security code.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.capital_flow("700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def capital_distribution(
        self, symbol: str
    ) -> Awaitable[CapitalDistributionResponse]:
        """
        Get capital distribution. Returns an awaitable that resolves to capital distribution response.

        Args:
            symbol: Security code.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.capital_distribution("700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def calc_indexes(
        self, symbols: List[str], indexes: List[Type[CalcIndex]]
    ) -> Awaitable[List[SecurityCalcIndex]]:
        """
        Get calc indexes for symbols. Returns an awaitable that resolves to security calc index list.

        Args:
            symbols: Security codes.
            indexes: Calc indexes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, CalcIndex

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.calc_indexes(
                        ["700.HK", "APPL.US"],
                        [CalcIndex.LastDone, CalcIndex.ChangeRate],
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def watchlist(self) -> Awaitable[List[WatchlistGroup]]:
        """
        Get watch list. Returns an awaitable that resolves to watchlist group list.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.watchlist()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def create_watchlist_group(
        self, name: str, securities: Optional[List[str]] = None
    ) -> Awaitable[int]:
        """
        Create watchlist group. Returns an awaitable that resolves to group ID.

        Args:
            name: Group name.
            securities: Securities.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    group_id = await ctx.create_watchlist_group(
                        name="Watchlist1",
                        securities=["700.HK", "AAPL.US"],
                    )
                    print(group_id)

                asyncio.run(main())
        """
        ...

    def delete_watchlist_group(self, id: int, purge: bool = False) -> Awaitable[None]:
        """
        Delete watchlist group. Returns an awaitable.

        Args:
            id: Group ID.
            purge: Move securities in this group to the default group.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.delete_watchlist_group(10086)

                asyncio.run(main())
        """
        ...

    def update_watchlist_group(
        self,
        id: int,
        name: Optional[str] = None,
        securities: Optional[List[str]] = None,
        mode: Optional[Type[SecuritiesUpdateMode]] = None,
    ) -> Awaitable[None]:
        """
        Update watchlist group. Returns an awaitable.

        Args:
            id: Group ID.
            name: Group name.
            securities: Securities.
            mode: Securities update mode.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SecuritiesUpdateMode

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.update_watchlist_group(
                        10086,
                        name="Watchlist2",
                        securities=["700.HK", "AAPL.US"],
                        mode=SecuritiesUpdateMode.Replace,
                    )

                asyncio.run(main())
        """
        ...

    def update_pinned(
        self,
        mode: Type[PinnedMode],
        symbols: List[str],
    ) -> Awaitable[None]:
        """
        Pin or unpin watchlist securities. Returns an awaitable.

        Args:
            mode: :class:`PinnedMode.Add` to pin, :class:`PinnedMode.Remove` to unpin
            symbols: List of security symbols to pin/unpin
        """
        ...

    def security_list(
        self,
        market: Type[Market],
        category: Optional[Type[SecurityListCategory]] = None,
    ) -> Awaitable[List[Security]]:
        """
        Get security list. Returns an awaitable that resolves to security list.

        Args:
            market: Market.
            category: Security list category.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, Market, SecurityListCategory

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.security_list(
                        Market.HK,
                        SecurityListCategory.Overnight,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def market_temperature(self, market: Type[Market]) -> Awaitable[MarketTemperature]:
        """
        Get current market temperature. Returns an awaitable that resolves to market temperature.

        Args:
            market: Market.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, Market

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.market_temperature(Market.HK)
                    print(resp)

                asyncio.run(main())
        """
        ...

    def history_market_temperature(
        self, market: Type[Market], start_date: date, end_date: date
    ) -> Awaitable[HistoryMarketTemperatureResponse]:
        """
        Get historical market temperature. Returns an awaitable that resolves to history market temperature response.

        Args:
            market: Market.
            start_date: Start date.
            end_date: End date.

        Examples:
            ::

                import asyncio
                import datetime
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, Market

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    resp = await ctx.history_market_temperature(
                        Market.HK,
                        datetime.date(2023, 1, 1),
                        datetime.date(2023, 1, 31),
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def realtime_quote(self, symbols: List[str]) -> Awaitable[List[RealtimeQuote]]:
        """
        Get real-time quote of subscribed symbols from local storage. Returns an awaitable that resolves to realtime quote list.

        Args:
            symbols: Security codes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SubType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
                    await asyncio.sleep(5)
                    resp = await ctx.realtime_quote(["700.HK", "AAPL.US"])
                    print(resp)

                asyncio.run(main())
        """
        ...

    def realtime_depth(self, symbol: str) -> Awaitable[SecurityDepth]:
        """
        Get real-time depth of subscribed symbol from local storage. Returns an awaitable that resolves to security depth.

        Args:
            symbol: Security code.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SubType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Depth])
                    await asyncio.sleep(5)
                    resp = await ctx.realtime_depth("700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def realtime_brokers(self, symbol: str) -> Awaitable[SecurityBrokers]:
        """
        Get real-time brokers of subscribed symbol from local storage. Returns an awaitable that resolves to security brokers.

        Args:
            symbol: Security code.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SubType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Brokers])
                    await asyncio.sleep(5)
                    resp = await ctx.realtime_brokers("700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def realtime_trades(self, symbol: str, count: int = 500) -> Awaitable[List[Trade]]:
        """
        Get real-time trades of subscribed symbol from local storage. Returns an awaitable that resolves to trade list.

        Args:
            symbol: Security code.
            count: Count of trades.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, SubType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Trade])
                    await asyncio.sleep(5)
                    resp = await ctx.realtime_trades("700.HK", 10)
                    print(resp)

                asyncio.run(main())
        """
        ...

    def realtime_candlesticks(
        self, symbol: str, period: Type[Period], count: int = 500
    ) -> Awaitable[List[Candlestick]]:
        """
        Get real-time candlesticks of subscribed symbol from local storage. Returns an awaitable that resolves to candlestick list.

        Args:
            symbol: Security code.
            period: Period type.
            count: Count of candlesticks.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncQuoteContext, Config, Period

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncQuoteContext.create(config)
                    await ctx.subscribe_candlesticks(
                        "AAPL.US",
                        Period.Min_1,
                    )
                    await asyncio.sleep(5)
                    resp = await ctx.realtime_candlesticks(
                        "AAPL.US",
                        Period.Min_1,
                        10,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def short_positions(
        self, symbol: str, count: int = 20
    ) -> Awaitable[ShortPositionsResponse]:
        """
        Get short interest / position data for a US or HK security. Returns awaitable.

        Market is inferred from the symbol suffix: ``.HK`` → HK endpoint,
        otherwise US endpoint.

        Args:
            symbol: Security code
            count: Number of records (1–100, default 20)

        Returns:
            Awaitable resolving to :class:`ShortPositionsResponse`
        """
        ...

    def short_trades(
        self, symbol: str, count: int = 20
    ) -> Awaitable[ShortTradesResponse]:
        """
        Get short trade records for a HK or US security. Returns awaitable.

        Market is inferred from the symbol suffix: ``.HK`` → HK endpoint,
        otherwise US endpoint.

        Args:
            symbol: Security code
            count: Number of records (1–100, default 20)

        Returns:
            Awaitable resolving to :class:`ShortTradesResponse`
        """
        ...

    def us_crypto_overview(self, symbol: str) -> "Awaitable[USCryptoOverview]":
        """Get US cryptocurrency market overview. US token required. Returns awaitable.

        Args:
            symbol: Trading-pair symbol, e.g. ``"BTCUSD.BKKT"``

        Returns:
            Awaitable resolving to :class:`USCryptoOverview`
        """
        ...

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

class AllExecutionsResponse:
    """
    Response for get all executions request
    """

    has_more: bool
    """
    Has more records
    """

    trades: List[Execution]
    """
    Execution list
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

    submitted_quantity: Decimal
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

    executed_quantity: Decimal
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

    class OptionPreMarket(OutsideRTH):
        """
        Overnight option
        """

class AttachedOrderType:
    """
    Attached order type
    """

    class ProfitTaker(AttachedOrderType):
        """
        Take profit
        """

    class StopLoss(AttachedOrderType):
        """
        Stop loss
        """

    class Bracket(AttachedOrderType):
        """
        Bracket order
        """

class AttachedOrderDetail:
    """
    Attached order detail
    """

    order_id: str
    """
    Order ID
    """

    attached_type_display: int
    """
    Attached type display (1=take-profit, 2=stop-loss)
    """

    trigger_price: Optional[Decimal]
    """
    Trigger price
    """

    quantity: Decimal
    """
    Quantity
    """

    executed_qty: Decimal
    """
    Executed quantity
    """

    status: Type[OrderStatus]
    """
    Order status
    """

    updated_at: datetime
    """
    Last updated time
    """

    withdrawn: bool
    """
    Withdrawn
    """

    gtd: Optional[date]
    """
    Good till date
    """

    time_in_force: Type[TimeInForceType]
    """
    Time in force type
    """

    counter_id: str
    """
    Counter ID
    """

    trigger_status: int
    """
    Trigger status
    """

    executed_amount: Decimal
    """
    Executed amount
    """

    tag: int
    """
    Tag
    """

    submitted_at: datetime
    """
    Submitted time
    """

    executed_price: Decimal
    """
    Executed price
    """

    force_only_rth: Optional[Type[OutsideRTH]]
    """
    Enable or disable outside regular trading hours (force only)
    """

    reviewed: bool
    """
    Reviewed
    """

    activate_order_type: Type[OrderType]
    """
    Activate order type
    """

    activate_rth: Optional[Type[OutsideRTH]]
    """
    Activate RTH
    """

    submit_price: Optional[Decimal]
    """
    Submit price
    """

class SubmitAttachedParams:
    """
    Submit attached order parameters
    """

    def __init__(
        self,
        attached_order_type: Type[AttachedOrderType],
        *,
        profit_taker_price: Optional[Decimal] = None,
        stop_loss_price: Optional[Decimal] = None,
        time_in_force: Optional[Type[TimeInForceType]] = None,
        expire_time: Optional[int] = None,
        activate_order_type: Optional[Type[OrderType]] = None,
        profit_taker_submit_price: Optional[Decimal] = None,
        stop_loss_submit_price: Optional[Decimal] = None,
        activate_rth: Optional[Type[OutsideRTH]] = None,
    ) -> None: ...

class ReplaceAttachedParams:
    """
    Replace attached order parameters
    """

    def __init__(
        self,
        attached_order_type: Type[AttachedOrderType],
        *,
        profit_taker_price: Optional[Decimal] = None,
        stop_loss_price: Optional[Decimal] = None,
        time_in_force: Optional[Type[TimeInForceType]] = None,
        expire_time: Optional[int] = None,
        activate_order_type: Optional[Type[OrderType]] = None,
        profit_taker_submit_price: Optional[Decimal] = None,
        stop_loss_submit_price: Optional[Decimal] = None,
        activate_rth: Optional[Type[OutsideRTH]] = None,
        profit_taker_id: Optional[int] = None,
        stop_loss_id: Optional[int] = None,
        cancel_all_attached: Optional[bool] = None,
        main_id: Optional[int] = None,
        quantity: Optional[Decimal] = None,
        market_price: Optional[Decimal] = None,
    ) -> None: ...

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

    limit_depth_level: Optional[int]
    """
    Limit depth level
    """

    trigger_count: Optional[int]
    """
    Trigger count
    """

    monitor_price: Optional[Decimal]
    """
    Monitor price
    """

    remark: str
    """
    Remark
    """

    attached_orders: List[AttachedOrderDetail]
    """
    Attached orders
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

    quantity: Decimal
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

    limit_depth_level: Optional[int]
    """
    Limit depth level
    """

    trigger_count: Optional[int]
    """
    Trigger count
    """

    monitor_price: Optional[Decimal]
    """
    Monitor price
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

    attached_orders: List[AttachedOrderDetail]
    """
    Attached orders
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

class FrozenTransactionFee:
    currency: str
    """
    Currency
    """

    frozen_transaction_fee: Decimal
    """
    Frozen transaction fee
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

    frozen_transaction_fees: List[FrozenTransactionFee]
    """
    Frozen transaction fees
    """

class BalanceType:
    class Unknown(BalanceType): ...
    class Cash(BalanceType): ...
    class Stock(BalanceType): ...
    class Fund(BalanceType): ...

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

    quantity: Decimal
    """
    The number of holdings
    """

    available_quantity: Decimal
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

    init_quantity: Optional[Decimal]
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

    cash_max_qty: Decimal
    """
    Cash available quantity
    """

    margin_max_qty: Decimal
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
    def set_on_order_changed(
        self, callback: Callable[[PushOrderChanged], None]
    ) -> None:
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
                from longbridge.openapi import OAuthBuilder, TradeContext, Config, OrderSide, OrderType, TimeInForceType, PushOrderChanged, TopicType


                def on_order_changed(event: PushOrderChanged):
                    print(event)


                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)
                ctx.set_on_order_changed(on_order_changed)
                ctx.subscribe([TopicType.Private])

                resp = ctx.submit_order(
                    side = OrderSide.Buy,
                    symbol = "700.HK",
                    order_type = OrderType.LO,
                    submitted_price = Decimal(50),
                    submitted_quantity = Decimal(200),
                    time_in_force = TimeInForceType.Day,
                    remark = "Hello from Python SDK",
                )
                print(resp)
                sleep(5)  # waiting for push event
        """

    def unsubscribe(self, topics: List[Type[TopicType]]) -> None:
        """
        Unsubscribe

        Args:
            topics: Topic list
        """

    def history_executions(
        self,
        symbol: Optional[str] = None,
        start_at: Optional[datetime] = None,
        end_at: Optional[datetime] = None,
    ) -> List[Execution]:
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
                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.history_executions(
                    symbol = "700.HK",
                    start_at = datetime(2022, 5, 9),
                    end_at = datetime(2022, 5, 12),
                )
                print(resp)
        """

    def today_executions(
        self, symbol: Optional[str] = None, order_id: Optional[str] = None
    ) -> List[Execution]:
        """
        Get today executions

        Args:
            symbol: Filter by security code
            order_id: Filter by Order ID

        Returns:
            Execution list

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.today_executions(symbol = "700.HK")
                print(resp)
        """

    # TODO: temporarily disabled — restore when API is available
    # def all_executions(
    #     self,
    #     symbol: Optional[str] = None,
    #     order_id: Optional[str] = None,
    #     start_at: Optional[datetime] = None,
    #     end_at: Optional[datetime] = None,
    #     page: Optional[int] = None,
    # ) -> AllExecutionsResponse: ...

    def history_orders(
        self,
        symbol: Optional[str] = None,
        status: Optional[List[Type[OrderStatus]]] = None,
        side: Optional[Type[OrderSide]] = None,
        market: Optional[Type[Market]] = None,
        start_at: Optional[datetime] = None,
        end_at: Optional[datetime] = None,
    ) -> List[Order]:
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
                from longbridge.openapi import OAuthBuilder, TradeContext, Config, OrderStatus, OrderSide, Market

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

    def today_orders(
        self,
        symbol: Optional[str] = None,
        status: Optional[List[Type[OrderStatus]]] = None,
        side: Optional[Type[OrderSide]] = None,
        market: Optional[Type[Market]] = None,
        order_id: Optional[str] = None,
        is_attached: bool = False,
    ) -> List[Order]:
        """
        Get today orders

        Args:
            symbol: Filter by security code
            status: Filter by order status
            side: Filter by order side
            market: Filter by market type
            order_id: Filter by order id
            is_attached: Include attached orders

        Returns:
            Order list

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, TradeContext, Config, OrderStatus, OrderSide, Market

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.today_orders(
                    symbol = "700.HK",
                    status = [OrderStatus.Filled, OrderStatus.New],
                    side = OrderSide.Buy,
                    market = Market.HK,
                )
                print(resp)
        """

    def replace_order(
        self,
        order_id: str,
        quantity: Decimal,
        price: Optional[Decimal] = None,
        trigger_price: Optional[Decimal] = None,
        limit_offset: Optional[Decimal] = None,
        trailing_amount: Optional[Decimal] = None,
        trailing_percent: Optional[Decimal] = None,
        limit_depth_level: Optional[int] = None,
        trigger_count: Optional[int] = None,
        monitor_price: Optional[Decimal] = None,
        remark: Optional[str] = None,
        attached_params: Optional[ReplaceAttachedParams] = None,
    ) -> None:
        """
        Replace order

        Args:
            quantity: Replaced quantity
            price: Replaced price
            trigger_price: Trigger price (`LIT` / `MIT` Order Required)
            limit_offset: Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
            trailing_amount: Trailing amount (`TSLPAMT` / `TSMAMT` Required)
            trailing_percent: Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
            limit_depth_level: Limit depth level
            trigger_count: Trigger count
            monitor_price: Monitor price
            remark: Remark (Maximum 64 characters)

        Examples:
            ::

                from decimal import Decimal
                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                ctx.replace_order(
                    order_id = "709043056541253632",
                    quantity = Decimal(100),
                    price = Decimal(100),
                )
        """

    def submit_order(
        self,
        symbol: str,
        order_type: Type[OrderType],
        side: Type[OrderSide],
        submitted_quantity: Decimal,
        time_in_force: Type[TimeInForceType],
        submitted_price: Optional[Decimal] = None,
        trigger_price: Optional[Decimal] = None,
        limit_offset: Optional[Decimal] = None,
        trailing_amount: Optional[Decimal] = None,
        trailing_percent: Optional[Decimal] = None,
        expire_date: Optional[date] = None,
        outside_rth: Optional[Type[OutsideRTH]] = None,
        limit_depth_level: Optional[int] = None,
        trigger_count: Optional[int] = None,
        monitor_price: Optional[Decimal] = None,
        remark: Optional[str] = None,
        client_request_id: Optional[str] = None,
        attached_params: Optional[SubmitAttachedParams] = None,
    ) -> SubmitOrderResponse:
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
            limit_depth_level: Limit depth level
            trigger_count: Trigger count
            monitor_price: Monitor price
            remark: Remark (Maximum 64 characters)
            client_request_id: Idempotent request ID. If not specified, idempotency control is skipped. The server caches this ID for 10 minutes to prevent duplicate orders.
            attached_params: Attached order parameters

        Returns:
            Response

        Examples:
            ::

                from decimal import Decimal
                from longbridge.openapi import OAuthBuilder, TradeContext, Config, OrderSide, OrderType, TimeInForceType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.submit_order(
                    side = OrderSide.Buy,
                    symbol = "700.HK",
                    order_type = OrderType.LO,
                    submitted_price = Decimal(50),
                    submitted_quantity = Decimal(200),
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

                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.account_balance()
                print(resp)
        """

    def cash_flow(
        self,
        start_at: datetime,
        end_at: datetime,
        business_type: Optional[Type[BalanceType]] = None,
        symbol: Optional[str] = None,
        page: Optional[int] = None,
        size: Optional[int] = None,
    ) -> List[CashFlow]:
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
                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.cash_flow(
                    start_at = datetime(2022, 5, 9),
                    end_at = datetime(2022, 5, 12),
                )
                print(resp)
        """

    def fund_positions(
        self, symbols: Optional[List[str]] = None
    ) -> FundPositionsResponse:
        """
        Get fund positions

        Args:
            symbols: Filter by fund codes

        Returns:
            Fund positions

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.fund_positions()
                print(resp)
        """

    def stock_positions(
        self, symbols: Optional[List[str]] = None
    ) -> StockPositionsResponse:
        """
        Get stock positions

        Args:
            symbols: Filter by stock codes

        Returns:
            Stock positions

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
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

                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.margin_ratio("700.HK")
                print(resp)
        """

    def order_detail(self, order_id: str, is_attached: bool = False) -> OrderDetail:
        """
        Get order detail

        Args:
            order_id: Order id
            is_attached: Include attached orders

        Returns:
            Order detail

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, TradeContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.order_detail("701276261045858304")
                print(resp)
        """

    def estimate_max_purchase_quantity(
        self,
        symbol: str,
        order_type: Type[OrderType],
        side: Type[OrderSide],
        price: Optional[Decimal] = None,
        currency: Optional[str] = None,
        order_id: Optional[str] = None,
        fractional_shares: bool = False,
    ) -> EstimateMaxPurchaseQuantityResponse:
        """
        Estimating the maximum purchase quantity for Hong Kong and US stocks, warrants, and options

        Args:
            symbol: Security symbol
            order_type: Order type
            side: Order side
            price: Estimated order price,
            currency: Settlement currency
            order_id: Order ID, required when estimating the maximum purchase quantity for a modified order
            fractional_shares: Get the maximum fractional share buying power

        Returns:
            Response

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, TradeContext, Config, OrderType, OrderSide

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = TradeContext(config)

                resp = ctx.estimate_max_purchase_quantity(
                    symbol = "700.HK",
                    order_type = OrderType.LO,
                    side = OrderSide.Buy,
                )
                print(resp)
        """

    def us_query_orders(
        self,
        symbol: Optional[str] = None,
        action: int = 0,
        start_at: int = 0,
        end_at: int = 0,
        query_type: int = 0,
        page: int = 1,
        limit: int = 20,
    ) -> str:
        """Query US order list (paginated). Returns JSON string. US token required.

        Args:
            symbol: Optional symbol filter, e.g. ``"AAPL.US"``
            action: Direction filter: 0=all, 1=buy, 2=sell
            start_at: Start timestamp (unix seconds); 0 = last 90 days
            end_at: End timestamp (unix seconds); 0 = now
            query_type: 0=all (incl. Rejected), 1=pending, 2=history (filled only)
            page: Page number, 1-based
            limit: Page size

        Returns:
            JSON string with order list
        """
        ...

    def us_order_detail(self, order_id: str) -> "USOrderDetailResponse":
        """Get US order detail. US token required.

        Args:
            order_id: Order ID

        Returns:
            :class:`USOrderDetailResponse`
        """
        ...

    def us_asset_overview(self) -> "USAssetOverview":
        """Get US account asset overview. US token required.

        Returns:
            :class:`USAssetOverview` with stock, option, and crypto positions
            plus purchasing power
        """
        ...

    def us_realized_pl(self, currency: str, category: Optional[str] = None) -> "USRealizedPL":
        """Get realized P&L for the US account. US token required.

        Args:
            currency: Currency (e.g. ``"USD"``)
            category: Asset category filter: ``"ALL"``, ``"STOCK"``,
                ``"OPTION"``, or ``"CRYPTO"``; ``None`` for all

        Returns:
            :class:`USRealizedPL`
        """
        ...

class AsyncTradeContext:
    """
    Async trade context for use with asyncio. Create via `AsyncTradeContext.create(config)` and await inside asyncio.
    Callbacks (set_on_order_changed) are set the same way as the sync TradeContext; all I/O methods return awaitables.
    """

    @classmethod
    def create(
        cls: Type[AsyncTradeContext], config: Config, loop_: Optional[Any] = None
    ) -> AsyncTradeContext:
        """
        Create an async trade context.

        Args:
            config: Configuration object.
            loop_: Optional event loop; pass asyncio.get_running_loop() when using async callbacks.

        Returns:
            AsyncTradeContext instance.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, Config, AsyncTradeContext

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.today_orders()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def set_on_order_changed(
        self,
        callback: Callable[[PushOrderChanged], None]
        | Callable[[PushOrderChanged], Coroutine[Any, Any, None]],
    ) -> None:
        """Set order changed callback; called when order changed event is received. Callback may be sync or async (async is scheduled on the event loop)."""
        ...

    def subscribe(self, topics: List[Type[TopicType]]) -> Awaitable[None]:
        """
        Subscribe to topics (e.g. TopicType.Private). Returns an awaitable; must be awaited in asyncio.

        Args:
            topics: Topic list.

        Examples:
            ::

                import asyncio
                from decimal import Decimal
                from longbridge.openapi import OAuthBuilder, (
                    AsyncTradeContext,
                    Config,
                    OrderSide,
                    OrderType,
                    TimeInForceType,
                    PushOrderChanged,
                    TopicType,
                )

                def on_order_changed(event: PushOrderChanged):
                    print(event)

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    ctx.set_on_order_changed(on_order_changed)
                    await ctx.subscribe([TopicType.Private])
                    resp = await ctx.submit_order(
                        symbol="700.HK",
                        order_type=OrderType.LO,
                        side=OrderSide.Buy,
                        submitted_quantity=Decimal(200),
                        time_in_force=TimeInForceType.Day,
                        submitted_price=Decimal(50),
                        remark="Hello from Python SDK",
                    )
                    print(resp)
                    await asyncio.sleep(5)

                asyncio.run(main())
        """
        ...

    def unsubscribe(self, topics: List[Type[TopicType]]) -> Awaitable[None]:
        """
        Unsubscribe from topics. Returns an awaitable.

        Args:
            topics: Topic list.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config, TopicType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    await ctx.subscribe([TopicType.Private])
                    await ctx.unsubscribe([TopicType.Private])

                asyncio.run(main())
        """
        ...

    def history_executions(
        self,
        symbol: Optional[str] = None,
        start_at: Optional[datetime] = None,
        end_at: Optional[datetime] = None,
    ) -> Awaitable[List[Execution]]:
        """
        Get history executions. Optional filters: symbol, start_at, end_at. Returns an awaitable that resolves to execution list.

        Args:
            symbol: Filter by security code.
            start_at: Start time.
            end_at: End time.

        Examples:
            ::

                import asyncio
                import datetime
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.history_executions(
                        symbol="700.HK",
                        start_at=datetime.datetime(2022, 5, 9),
                        end_at=datetime.datetime(2022, 5, 12),
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def today_executions(
        self, symbol: Optional[str] = None, order_id: Optional[str] = None
    ) -> Awaitable[List[Execution]]:
        """
        Get today executions. Optional filters: symbol, order_id. Returns an awaitable that resolves to execution list.

        Args:
            symbol: Filter by security code.
            order_id: Filter by order ID.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.today_executions(symbol="700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    # TODO: temporarily disabled — restore when API is available
    # def all_executions(
    #     self,
    #     symbol: Optional[str] = None,
    #     order_id: Optional[str] = None,
    #     start_at: Optional[datetime] = None,
    #     end_at: Optional[datetime] = None,
    #     page: Optional[int] = None,
    # ) -> Awaitable[AllExecutionsResponse]: ...

    def history_orders(
        self,
        symbol: Optional[str] = None,
        status: Optional[List[Type[OrderStatus]]] = None,
        side: Optional[Type[OrderSide]] = None,
        market: Optional[Type[Market]] = None,
        start_at: Optional[datetime] = None,
        end_at: Optional[datetime] = None,
    ) -> Awaitable[List[Order]]:
        """
        Get history orders with optional filters. Returns an awaitable that resolves to order list.

        Args:
            symbol: Filter by security code.
            status: Filter by order status.
            side: Filter by order side.
            market: Filter by market type.
            start_at: Start time.
            end_at: End time.

        Examples:
            ::

                import asyncio
                import datetime
                from longbridge.openapi import OAuthBuilder, (
                    AsyncTradeContext,
                    Config,
                    OrderStatus,
                    OrderSide,
                    Market,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.history_orders(
                        symbol="700.HK",
                        status=[OrderStatus.Filled, OrderStatus.New],
                        side=OrderSide.Buy,
                        market=Market.HK,
                        start_at=datetime.datetime(2022, 5, 9),
                        end_at=datetime.datetime(2022, 5, 12),
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def today_orders(
        self,
        symbol: Optional[str] = None,
        status: Optional[List[Type[OrderStatus]]] = None,
        side: Optional[Type[OrderSide]] = None,
        market: Optional[Type[Market]] = None,
        order_id: Optional[str] = None,
        is_attached: bool = False,
    ) -> Awaitable[List[Order]]:
        """
        Get today orders with optional filters. Returns an awaitable that resolves to order list.

        Args:
            symbol: Filter by security code.
            status: Filter by order status.
            side: Filter by order side.
            market: Filter by market type.
            order_id: Filter by order ID.
            is_attached: Include attached orders.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, (
                    AsyncTradeContext,
                    Config,
                    OrderStatus,
                    OrderSide,
                    Market,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.today_orders(
                        symbol="700.HK",
                        status=[OrderStatus.Filled, OrderStatus.New],
                        side=OrderSide.Buy,
                        market=Market.HK,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def replace_order(
        self,
        order_id: str,
        quantity: Decimal,
        price: Optional[Decimal] = None,
        trigger_price: Optional[Decimal] = None,
        limit_offset: Optional[Decimal] = None,
        trailing_amount: Optional[Decimal] = None,
        trailing_percent: Optional[Decimal] = None,
        limit_depth_level: Optional[int] = None,
        trigger_count: Optional[int] = None,
        monitor_price: Optional[Decimal] = None,
        remark: Optional[str] = None,
        attached_params: Optional[ReplaceAttachedParams] = None,
    ) -> Awaitable[None]:
        """
        Replace order. Returns an awaitable. Same parameters as sync TradeContext.replace_order.

        Args:
            order_id: Order ID.
            quantity: Replaced quantity.
            price: Replaced price.
            trigger_price: Trigger price (LIT/MIT order).
            limit_offset: Limit offset amount (TSLPAMT/TSLPPCT).
            trailing_amount: Trailing amount (TSLPAMT/TSMAMT).
            trailing_percent: Trailing percent (TSLPPCT/TSMPCT).
            limit_depth_level: Limit depth level.
            trigger_count: Trigger count.
            monitor_price: Monitor price.
            remark: Remark (max 64 characters).
            attached_params: Attached order parameters.

        Examples:
            ::

                import asyncio
                from decimal import Decimal
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    await ctx.replace_order(
                        order_id="709043056541253632",
                        quantity=Decimal(100),
                        price=Decimal(100),
                    )

                asyncio.run(main())
        """
        ...

    def submit_order(
        self,
        symbol: str,
        order_type: Type[OrderType],
        side: Type[OrderSide],
        submitted_quantity: Decimal,
        time_in_force: Type[TimeInForceType],
        submitted_price: Optional[Decimal] = None,
        trigger_price: Optional[Decimal] = None,
        limit_offset: Optional[Decimal] = None,
        trailing_amount: Optional[Decimal] = None,
        trailing_percent: Optional[Decimal] = None,
        expire_date: Optional[date] = None,
        outside_rth: Optional[Type[OutsideRTH]] = None,
        limit_depth_level: Optional[int] = None,
        trigger_count: Optional[int] = None,
        monitor_price: Optional[Decimal] = None,
        remark: Optional[str] = None,
        client_request_id: Optional[str] = None,
        attached_params: Optional[SubmitAttachedParams] = None,
    ) -> Awaitable[SubmitOrderResponse]:
        """
        Submit order. Returns an awaitable that resolves to SubmitOrderResponse. Same parameters as sync TradeContext.submit_order.

        Args:
            symbol: Security code.
            order_type: Order type.
            side: Order side.
            submitted_quantity: Submitted quantity.
            time_in_force: Time in force type.
            submitted_price: Submitted price.
            trigger_price: Trigger price (LIT/MIT).
            limit_offset: Limit offset amount (TSLPAMT/TSLPPCT).
            trailing_amount: Trailing amount (TSLPAMT/TSMAMT).
            trailing_percent: Trailing percent (TSLPPCT/TSMPCT).
            expire_date: Long term order expire date (required when time_in_force is GoodTilDate).
            outside_rth: Enable or disable outside regular trading hours.
            limit_depth_level: Limit depth level.
            trigger_count: Trigger count.
            monitor_price: Monitor price.
            remark: Remark (max 64 characters).
            client_request_id: Idempotent request ID. If not specified, idempotency control is skipped. The server caches this ID for 10 minutes to prevent duplicate orders.
            attached_params: Attached order parameters.

        Examples:
            ::

                import asyncio
                from decimal import Decimal
                from longbridge.openapi import OAuthBuilder, (
                    AsyncTradeContext,
                    Config,
                    OrderSide,
                    OrderType,
                    TimeInForceType,
                )

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.submit_order(
                        symbol="700.HK",
                        order_type=OrderType.LO,
                        side=OrderSide.Buy,
                        submitted_quantity=Decimal(500),
                        time_in_force=TimeInForceType.Day,
                        submitted_price=Decimal(50),
                        remark="Hello from Python SDK",
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def cancel_order(self, order_id: str) -> Awaitable[None]:
        """
        Cancel order by order_id. Returns an awaitable.

        Args:
            order_id: Order ID.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    await ctx.cancel_order("709043056541253632")

                asyncio.run(main())
        """
        ...

    def account_balance(
        self, currency: Optional[str] = None
    ) -> Awaitable[List[AccountBalance]]:
        """
        Get account balance. Optional currency filter. Returns an awaitable that resolves to account balance list.

        Args:
            currency: Currency filter.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.account_balance()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def cash_flow(
        self,
        start_at: datetime,
        end_at: datetime,
        business_type: Optional[Type[BalanceType]] = None,
        symbol: Optional[str] = None,
        page: Optional[int] = None,
        size: Optional[int] = None,
    ) -> Awaitable[List[CashFlow]]:
        """
        Get cash flow. Required: start_at, end_at. Optional: business_type, symbol, page, size. Returns an awaitable that resolves to cash flow list.

        Args:
            start_at: Start time.
            end_at: End time.
            business_type: Balance type.
            symbol: Target security code.
            page: Start page (default 1).
            size: Page size (default 50).

        Examples:
            ::

                import asyncio
                import datetime
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.cash_flow(
                        start_at=datetime.datetime(2022, 5, 9),
                        end_at=datetime.datetime(2022, 5, 12),
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def fund_positions(
        self, symbols: Optional[List[str]] = None
    ) -> Awaitable[FundPositionsResponse]:
        """
        Get fund positions. Optional filter: symbols. Returns an awaitable that resolves to fund positions response.

        Args:
            symbols: Filter by fund codes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.fund_positions()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def stock_positions(
        self, symbols: Optional[List[str]] = None
    ) -> Awaitable[StockPositionsResponse]:
        """
        Get stock positions. Optional filter: symbols. Returns an awaitable that resolves to stock positions response.

        Args:
            symbols: Filter by stock codes.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.stock_positions()
                    print(resp)

                asyncio.run(main())
        """
        ...

    def margin_ratio(self, symbol: str) -> Awaitable[MarginRatio]:
        """
        Get margin ratio for symbol. Returns an awaitable that resolves to margin ratio.

        Args:
            symbol: Security symbol.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.margin_ratio("700.HK")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def order_detail(self, order_id: str, is_attached: bool = False) -> Awaitable[OrderDetail]:
        """
        Get order detail by order_id. Returns an awaitable that resolves to order detail.

        Args:
            order_id: Order ID.
            is_attached: Include attached orders.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.order_detail("701276261045858304")
                    print(resp)

                asyncio.run(main())
        """
        ...

    def estimate_max_purchase_quantity(
        self,
        symbol: str,
        order_type: Type[OrderType],
        side: Type[OrderSide],
        price: Optional[Decimal] = None,
        currency: Optional[str] = None,
        order_id: Optional[str] = None,
        fractional_shares: bool = False,
    ) -> Awaitable[EstimateMaxPurchaseQuantityResponse]:
        """
        Estimate maximum purchase quantity. Returns an awaitable that resolves to estimate response. order_id required when estimating for replace order.

        Args:
            symbol: Security symbol.
            order_type: Order type.
            side: Order side.
            price: Estimated order price.
            currency: Settlement currency.
            order_id: Order ID (required when estimating for replace order).
            fractional_shares: Get maximum fractional share buying power.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncTradeContext, Config, OrderType, OrderSide

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncTradeContext.create(config)
                    resp = await ctx.estimate_max_purchase_quantity(
                        symbol="700.HK",
                        order_type=OrderType.LO,
                        side=OrderSide.Buy,
                    )
                    print(resp)

                asyncio.run(main())
        """
        ...

    def us_query_orders(
        self,
        symbol: Optional[str] = None,
        action: int = 0,
        start_at: int = 0,
        end_at: int = 0,
        query_type: int = 0,
        page: int = 1,
        limit: int = 20,
    ) -> "Awaitable[str]":
        """Query US order list (paginated). Returns awaitable JSON string. US token required.

        Args:
            symbol: Optional symbol filter, e.g. ``"AAPL.US"``
            action: Direction filter: 0=all, 1=buy, 2=sell
            start_at: Start timestamp (unix seconds); 0 = last 90 days
            end_at: End timestamp (unix seconds); 0 = now
            query_type: 0=all (incl. Rejected), 1=pending, 2=history (filled only)
            page: Page number, 1-based
            limit: Page size

        Returns:
            Awaitable resolving to JSON string with order list
        """
        ...

    def us_order_detail(self, order_id: str) -> "Awaitable[USOrderDetailResponse]":
        """Get US order detail (async). US token required.

        Args:
            order_id: Order ID

        Returns:
            Awaitable[:class:`USOrderDetailResponse`]
        """
        ...

    def us_asset_overview(self) -> "Awaitable[USAssetOverview]":
        """Get US account asset overview. US token required. Returns awaitable.

        Returns:
            Awaitable resolving to :class:`USAssetOverview`
        """
        ...

    def us_realized_pl(self, currency: str, category: Optional[str] = None) -> "Awaitable[USRealizedPL]":
        """Get realized P&L for the US account. US token required. Returns awaitable.

        Args:
            currency: Currency (e.g. ``"USD"``)
            category: Asset category filter: ``"ALL"``, ``"STOCK"``,
                ``"OPTION"``, or ``"CRYPTO"``; ``None`` for all

        Returns:
            Awaitable resolving to :class:`USRealizedPL`
        """
        ...

class StatementType:
    """
    Statement type
    """

    class Daily(StatementType):
        """
        Daily statement
        """

    class Monthly(StatementType):
        """
        Monthly statement
        """

class StatementItem:
    """
    Statement item
    """

    dt: int
    """
    Statement date (integer, e.g. 20250301)
    """

    file_key: str
    """
    File key used to request the download URL
    """

class GetStatementListResponse:
    """
    Response for get statement list
    """

    list: List[StatementItem]
    """
    List of statement items
    """

class GetStatementResponse:
    """
    Response for get statement download URL
    """

    url: str
    """
    Presigned download URL
    """

class AssetContext:
    """
    Asset context

    Args:
        config: Configuration object
    """

    def __init__(self, config: Config) -> None: ...
    def statements(
        self, statement_type: Type[StatementType], start_date: int = 1, limit: int = 20
    ) -> GetStatementListResponse:
        """
        Get statement data list

        Args:
            statement_type: Statement type (StatementType.Daily or StatementType.Monthly)
            start_date: Start date for pagination (default 1)
            limit: Number of results (default 20)

        Returns:
            Statement list response

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, AssetContext, Config, StatementType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = AssetContext(config)

                resp = ctx.statements(StatementType.Daily)
                for item in resp.list:
                    print(item.dt, item.file_key)
        """
        ...

    def statement_download_url(self, file_key: str) -> GetStatementResponse:
        """
        Get statement data download URL

        Args:
            file_key: File key obtained from the statements list

        Returns:
            Statement download URL response

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, AssetContext, Config, StatementType

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = AssetContext(config)

                resp = ctx.statements(StatementType.Daily)
                if resp.list:
                    url_resp = ctx.statement_download_url(resp.list[0].file_key)
                    print(url_resp.url)
        """
        ...

class AsyncAssetContext:
    """
    Async asset context for use with asyncio. Create via `AsyncAssetContext.create(config)` and await inside asyncio.
    All I/O methods return awaitables.
    """

    @classmethod
    def create(cls: Type[AsyncAssetContext], config: Config) -> AsyncAssetContext:
        """
        Create an async asset context.

        Args:
            config: Configuration object.

        Returns:
            AsyncAssetContext instance.

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, Config, AsyncAssetContext, StatementType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncAssetContext.create(config)
                    resp = await ctx.statements(StatementType.Daily)
                    for item in resp.list:
                        print(item.dt, item.file_key)

                asyncio.run(main())
        """
        ...

    def statements(
        self, statement_type: Type[StatementType], start_date: int = 1, limit: int = 20
    ) -> Awaitable[GetStatementListResponse]:
        """
        Get statement data list. Returns an awaitable.

        Args:
            statement_type: Statement type (StatementType.Daily or StatementType.Monthly)
            start_date: Start date for pagination (default 1)
            limit: Number of results (default 20)

        Returns:
            Statement list response

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, Config, AsyncAssetContext, StatementType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncAssetContext.create(config)
                    resp = await ctx.statements(StatementType.Daily, limit=5)
                    for item in resp.list:
                        print(item.dt, item.file_key)

                asyncio.run(main())
        """
        ...

    def statement_download_url(self, file_key: str) -> Awaitable[GetStatementResponse]:
        """
        Get statement data download URL. Returns an awaitable.

        Args:
            file_key: File key obtained from the statements list

        Returns:
            Statement download URL response

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, Config, AsyncAssetContext, StatementType

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncAssetContext.create(config)
                    resp = await ctx.statements(StatementType.Daily)
                    if resp.list:
                        url_resp = await ctx.statement_download_url(resp.list[0].file_key)
                        print(url_resp.url)

                asyncio.run(main())
        """
        ...

class TopicAuthor:
    """
    Topic author
    """

    member_id: str
    """
    Member ID
    """
    name: str
    """
    Display name
    """
    avatar: str
    """
    Avatar URL
    """

class TopicImage:
    """
    Topic image
    """

    url: str
    """
    Original image URL
    """
    sm: str
    """
    Small thumbnail URL
    """
    lg: str
    """
    Large image URL
    """

class OwnedTopic:
    """
    Topic created by the current authenticated user
    """

    id: str
    """
    Topic ID
    """
    title: str
    """
    Title
    """
    description: str
    """
    Plain text excerpt
    """
    body: str
    """
    Markdown body
    """
    author: TopicAuthor
    """
    Author
    """
    tickers: List[str]
    """
    Related stock tickers
    """
    hashtags: List[str]
    """
    Hashtag names
    """
    images: List[TopicImage]
    """
    Images
    """
    likes_count: int
    """
    Likes count
    """
    comments_count: int
    """
    Comments count
    """
    views_count: int
    """
    Views count
    """
    shares_count: int
    """
    Shares count
    """
    topic_type: str
    """
    Content type: "article" or "post"
    """
    detail_url: str
    """
    URL to the full topic page
    """
    created_at: datetime
    """
    Created time
    """
    updated_at: datetime
    """
    Updated time
    """

class TopicReply:
    """
    A reply on a topic
    """

    id: str
    """
    Reply ID
    """
    topic_id: str
    """
    Topic ID this reply belongs to
    """
    body: str
    """
    Reply body (plain text)
    """
    reply_to_id: str
    """
    ID of the parent reply ("0" means top-level)
    """
    author: TopicAuthor
    """
    Author info
    """
    images: List[TopicImage]
    """
    Attached images
    """
    likes_count: int
    """
    Likes count
    """
    comments_count: int
    """
    Nested replies count
    """
    created_at: datetime
    """
    Created time
    """

class ContentContext:
    """
    Content context

    Args:
        config: Configuration object
    """

    def __init__(self, config: Config) -> None: ...
    def my_topics(
        self,
        page: Optional[int] = None,
        size: Optional[int] = None,
        topic_type: Optional[str] = None,
    ) -> List[OwnedTopic]:
        """
        Get topics created by the current authenticated user

        Args:
            page: Page number (default 1)
            size: Page size (default 50, range 1-500)
            topic_type: Filter by type: "article" or "post"; empty returns all

        Returns:
            List of owned topics

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, ContentContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = ContentContext(config)
                topics = ctx.my_topics(size=20)
                for t in topics:
                    print(t.id, t.title)
        """
        ...

    def create_topic(
        self,
        title: str,
        body: str,
        topic_type: Optional[str] = None,
        tickers: Optional[List[str]] = None,
        hashtags: Optional[List[str]] = None,
    ) -> str:
        """
        Create a new community topic

        Args:
            title: Topic title (required for "article"; optional for "post")
            body: Topic body (plain text for "post", Markdown for "article")
            topic_type: "post" (default) or "article"
            tickers: Associated stock symbols, e.g. ["700.HK"], max 10
            hashtags: Hashtag names, max 5

        Returns:
            The new topic ID

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, ContentContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = ContentContext(config)
                topic_id = ctx.create_topic(
                    title="My Article",
                    body="Hello world",
                    topic_type="article",
                    tickers=["700.HK"],
                )
                print(topic_id)
        """
        ...

    def topics(self, symbol: str) -> List[TopicItem]:
        """
        Get discussion topics list for a symbol

        Args:
            symbol: Security symbol, e.g. "700.HK"

        Returns:
            List of topic items

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, ContentContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = ContentContext(config)
                topics = ctx.topics("700.HK")
                for t in topics:
                    print(t.id, t.title)
        """
        ...

    def news(self, symbol: str) -> List[NewsItem]:
        """
        Get news list for a symbol

        Args:
            symbol: Security symbol, e.g. "700.HK"

        Returns:
            List of news items

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, ContentContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = ContentContext(config)
                news = ctx.news("700.HK")
                for n in news:
                    print(n.id, n.title)
        """
        ...

    def topic_detail(self, id: str) -> OwnedTopic:
        """
        Get full details of a topic by its ID

        Args:
            id: Topic ID

        Returns:
            Full topic detail

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, ContentContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = ContentContext(config)
                topic = ctx.topic_detail("123456")
                print(topic.title, topic.body)
        """
        ...

    def list_topic_replies(
        self,
        topic_id: str,
        page: Optional[int] = None,
        size: Optional[int] = None,
    ) -> List[TopicReply]:
        """
        List replies on a topic

        Args:
            topic_id: Topic ID
            page: Page number (default 1)
            size: Page size (default 20, range 1-50)

        Returns:
            List of topic replies

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, ContentContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = ContentContext(config)
                replies = ctx.list_topic_replies("123456")
                for r in replies:
                    print(r.id, r.body)
        """
        ...

    def create_topic_reply(
        self,
        topic_id: str,
        body: str,
        reply_to_id: Optional[str] = None,
    ) -> TopicReply:
        """
        Post a reply to a community topic

        Args:
            topic_id: Topic ID
            body: Reply body (plain text only)
            reply_to_id: ID of the parent reply to nest under; empty or "0" for top-level

        Returns:
            The created reply

        Examples:
            ::

                from longbridge.openapi import OAuthBuilder, ContentContext, Config

                oauth = OAuthBuilder("your-client-id").build(
                    lambda url: print("Visit:", url)
                )
                config = Config.from_oauth(oauth)
                ctx = ContentContext(config)
                reply = ctx.create_topic_reply("123456", "Great post!")
                print(reply.id)
        """
        ...

class AsyncContentContext:
    """
    Async content context. Create via ``AsyncContentContext.create(config)`` and
    await inside asyncio. All I/O methods return awaitables.
    """

    @classmethod
    def create(cls, config: Config) -> AsyncContentContext: ...
    async def my_topics(
        self,
        page: Optional[int] = None,
        size: Optional[int] = None,
        topic_type: Optional[str] = None,
    ) -> List[OwnedTopic]:
        """
        Get topics created by the current authenticated user

        Args:
            page: Page number (default 1)
            size: Page size (default 50, range 1-500)
            topic_type: Filter by type: "article" or "post"; empty returns all

        Returns:
            List of owned topics

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncContentContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncContentContext.create(config)
                    topics = await ctx.my_topics(size=20)
                    for t in topics:
                        print(t.id, t.title)

                asyncio.run(main())
        """
        ...

    async def create_topic(
        self,
        title: str,
        body: str,
        topic_type: Optional[str] = None,
        tickers: Optional[List[str]] = None,
        hashtags: Optional[List[str]] = None,
    ) -> str:
        """
        Create a new community topic

        Args:
            title: Topic title (required for "article"; optional for "post")
            body: Topic body (plain text for "post", Markdown for "article")
            topic_type: "post" (default) or "article"
            tickers: Associated stock symbols, e.g. ["700.HK"], max 10
            hashtags: Hashtag names, max 5

        Returns:
            The new topic ID

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncContentContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncContentContext.create(config)
                    topic_id = await ctx.create_topic(
                        title="My Article",
                        body="Hello world",
                        topic_type="article",
                        tickers=["700.HK"],
                    )
                    print(topic_id)

                asyncio.run(main())
        """
        ...

    async def topics(self, symbol: str) -> List[TopicItem]:
        """
        Get discussion topics list for a symbol

        Args:
            symbol: Security symbol, e.g. "700.HK"

        Returns:
            List of topic items

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncContentContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncContentContext.create(config)
                    topics = await ctx.topics("700.HK")
                    for t in topics:
                        print(t.id, t.title)

                asyncio.run(main())
        """
        ...

    async def news(self, symbol: str) -> List[NewsItem]:
        """
        Get news list for a symbol

        Args:
            symbol: Security symbol, e.g. "700.HK"

        Returns:
            List of news items

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncContentContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncContentContext.create(config)
                    news = await ctx.news("700.HK")
                    for n in news:
                        print(n.id, n.title)

                asyncio.run(main())
        """
        ...

    async def topic_detail(self, id: str) -> OwnedTopic:
        """
        Get full details of a topic by its ID

        Args:
            id: Topic ID

        Returns:
            Full topic detail

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncContentContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncContentContext.create(config)
                    topic = await ctx.topic_detail("123456")
                    print(topic.title, topic.body)

                asyncio.run(main())
        """
        ...

    async def list_topic_replies(
        self,
        topic_id: str,
        page: Optional[int] = None,
        size: Optional[int] = None,
    ) -> List[TopicReply]:
        """
        List replies on a topic

        Args:
            topic_id: Topic ID
            page: Page number (default 1)
            size: Page size (default 20, range 1-50)

        Returns:
            List of topic replies

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncContentContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncContentContext.create(config)
                    replies = await ctx.list_topic_replies("123456")
                    for r in replies:
                        print(r.id, r.body)

                asyncio.run(main())
        """
        ...

    async def create_topic_reply(
        self,
        topic_id: str,
        body: str,
        reply_to_id: Optional[str] = None,
    ) -> TopicReply:
        """
        Post a reply to a community topic

        Args:
            topic_id: Topic ID
            body: Reply body (plain text only)
            reply_to_id: ID of the parent reply to nest under; empty or "0" for top-level

        Returns:
            The created reply

        Examples:
            ::

                import asyncio
                from longbridge.openapi import OAuthBuilder, AsyncContentContext, Config

                async def main():
                    oauth = await OAuthBuilder("your-client-id").build_async(
                        lambda url: print("Visit:", url)
                    )
                    config = Config.from_oauth(oauth)
                    ctx = AsyncContentContext.create(config)
                    reply = await ctx.create_topic_reply("123456", "Great post!")
                    print(reply.id)

                asyncio.run(main())
        """
        ...

# ── FundamentalContext ────────────────────────────────────────────

class FinancialReports:
    """
    Financial reports response.

    ``list`` contains raw nested data keyed by report kind
    (``"IS"``, ``"BS"``, ``"CF"``).
    """

    list: object
    """Raw financial data dict (IS/BS/CF indicators)"""


class DividendItem:
    """One dividend or distribution event."""

    symbol: str
    """Security symbol, e.g. ``"700.HK"``"""
    id: str
    """Internal record ID"""
    desc: str
    """Human-readable description, e.g. ``"每股派息 5.3 HKD"``"""
    record_date: str
    """Record / book-close date"""
    ex_date: str
    """Ex-dividend date"""
    payment_date: str
    """Payment date"""


class DividendList:
    """Dividend history response."""

    list: list[DividendItem]
    """List of dividend events"""


class RatingEvaluate:
    """Analyst rating distribution counts."""

    buy: int
    """Number of Buy ratings"""
    over: int
    """Number of Strong Buy / Outperform ratings"""
    hold: int
    """Number of Hold ratings"""
    under: int
    """Number of Underperform ratings"""
    sell: int
    """Number of Sell ratings"""
    no_opinion: int
    """Number of No Opinion ratings"""
    total: int
    """Total analyst count"""
    start_date: str
    """Window start (unix timestamp string)"""
    end_date: str
    """Window end (unix timestamp string)"""


class RatingTarget:
    """Analyst target price range."""

    highest_price: str
    """Highest price target"""
    lowest_price: str
    """Lowest price target"""
    prev_close: str
    """Previous close price"""
    start_date: str
    """Window start"""
    end_date: str
    """Window end"""


class InstitutionRatingLatest:
    """Latest analyst rating snapshot."""

    evaluate: RatingEvaluate
    """Rating distribution counts"""
    target: RatingTarget
    """Target price range"""
    industry_id: int
    """Industry classification ID"""
    industry_name: str
    """Industry name"""
    industry_rank: int
    """Rank within the industry (1 = highest)"""
    industry_total: int
    """Total securities in the industry"""
    industry_mean: int
    """Mean analyst count in the industry"""
    industry_median: int
    """Median analyst count in the industry"""


class RatingSummaryEvaluate:
    """Simplified rating distribution for consensus summary."""

    buy: int
    """Number of Buy ratings"""
    date: str
    """Date of the update"""
    hold: int
    """Number of Hold ratings"""
    sell: int
    """Number of Sell ratings"""
    strong_buy: int
    """Number of Strong Buy ratings"""
    under: int
    """Number of Underperform ratings"""


class InstitutionRecommend:
    """Institutional analyst recommendation."""

    class Unknown(InstitutionRecommend): ...
    """Unknown"""
    class StrongBuy(InstitutionRecommend): ...
    """Strong buy"""
    class Buy(InstitutionRecommend): ...
    """Buy"""
    class Hold(InstitutionRecommend): ...
    """Hold"""
    class Sell(InstitutionRecommend): ...
    """Sell"""
    class StrongSell(InstitutionRecommend): ...
    """Strong sell"""
    class Underperform(InstitutionRecommend): ...
    """Underperform"""
    class NoOpinion(InstitutionRecommend): ...
    """No opinion"""


class InstitutionRatingSummary:
    """Analyst consensus summary."""

    ccy_symbol: str
    """Currency symbol, e.g. ``"HK$"``"""
    change: str
    """Change vs previous period"""
    evaluate: RatingSummaryEvaluate
    """Simplified rating distribution"""
    recommend: InstitutionRecommend
    """Consensus recommendation"""
    target: str
    """Consensus target price"""
    updated_at: str
    """Last updated display string"""


class InstitutionRating:
    """Combined analyst rating response (latest + consensus summary)."""

    latest: InstitutionRatingLatest
    """Latest rating snapshot"""
    summary: InstitutionRatingSummary
    """Consensus summary"""


class InstitutionRatingDetailEvaluateItem:
    """One weekly rating distribution snapshot."""

    buy: int
    """Number of Buy ratings"""
    date: str
    """Date in ``"2021/05/14"`` format"""
    hold: int
    """Number of Hold ratings"""
    sell: int
    """Number of Sell ratings"""
    strong_buy: int
    """Number of Strong Buy / Outperform ratings"""
    no_opinion: int
    """Number of No Opinion ratings"""
    under: int
    """Number of Underperform ratings"""


class InstitutionRatingDetailEvaluate:
    """Historical rating distribution time-series."""

    list: list[InstitutionRatingDetailEvaluateItem]
    """Weekly rating distribution snapshots"""


class InstitutionRatingDetailTargetItem:
    """One weekly target price snapshot."""

    avg_target: str
    """Average target price"""
    date: str
    """Date string"""
    max_target: str
    """Highest target price"""
    min_target: str
    """Lowest target price"""
    meet: bool
    """Whether the stock price reached the target"""
    price: str
    """Actual stock price at this date"""
    timestamp: str
    """Unix timestamp string"""


class InstitutionRatingDetailTarget:
    """Historical target price time-series."""

    data_percent: str | None
    """Prediction accuracy ratio, e.g. ``"0.9934"`` (may be ``None``)"""
    prediction_accuracy: str
    """Overall prediction accuracy percentage"""
    updated_at: str
    """Last updated display string"""
    list: list[InstitutionRatingDetailTargetItem]
    """Weekly target price snapshots"""


class InstitutionRatingDetail:
    """Historical analyst rating detail response."""

    ccy_symbol: str
    """Currency symbol"""
    evaluate: InstitutionRatingDetailEvaluate
    """Historical rating distribution time-series"""
    target: InstitutionRatingDetailTarget
    """Historical target price time-series"""


class ForecastEpsItem:
    """One EPS forecast snapshot."""

    forecast_eps_median: str
    """Median EPS estimate"""
    forecast_eps_mean: str
    """Mean EPS estimate"""
    forecast_eps_lowest: str
    """Lowest EPS estimate"""
    forecast_eps_highest: str
    """Highest EPS estimate"""
    institution_total: int
    """Total number of forecasting institutions"""
    institution_up: int
    """Institutions that raised their estimate"""
    institution_down: int
    """Institutions that lowered their estimate"""
    forecast_start_date: datetime
    """Forecast window start"""
    forecast_end_date: datetime
    """Forecast window end"""


class ForecastEps:
    """EPS forecast response."""

    items: list[ForecastEpsItem]
    """EPS forecast snapshots"""


class ConsensusDetail:
    """Consensus estimate for one financial metric."""

    key: str
    """Metric key, e.g. ``"revenue"``"""
    name: str
    """Display name"""
    description: str
    """Metric description"""
    actual: str
    """Actual reported value (empty if not yet released)"""
    estimate: str
    """Consensus estimate value"""
    comp_value: str
    """Actual minus estimate"""
    comp_desc: str
    """Beat/miss description"""
    comp: str
    """Comparison result code"""
    is_released: bool
    """Whether actual results have been published"""


class ConsensusReport:
    """Consensus report for one fiscal period."""

    fiscal_year: int
    """Fiscal year, e.g. ``2025``"""
    fiscal_period: str
    """Fiscal period code"""
    period_text: str
    """Human-readable period label, e.g. ``"Q4 FY2025"``"""
    details: list[ConsensusDetail]
    """Per-metric consensus details"""


class FinancialConsensus:
    """Financial consensus estimates response."""

    list: list[ConsensusReport]
    """Per-period consensus reports"""
    current_index: int
    """Index of the most recently released period"""
    currency: str
    """Reporting currency"""
    opt_periods: list[str]
    """Available period types"""
    current_period: str
    """Currently returned period type"""


class ValuationPoint:
    """One valuation data point."""

    timestamp: datetime
    """Date of the data point"""
    value: str
    """Metric value"""


class ValuationMetricData:
    """Historical time-series for one valuation metric."""

    desc: str
    """Human-readable description with current value and percentile"""
    high: str
    """Historical high"""
    low: str
    """Historical low"""
    median: str
    """Historical median"""
    list: list[ValuationPoint]
    """Historical data points"""


class ValuationMetricsData:
    """Container for valuation metrics."""

    pe: ValuationMetricData | None
    """Price-to-Earnings ratio history"""
    pb: ValuationMetricData | None
    """Price-to-Book ratio history"""
    ps: ValuationMetricData | None
    """Price-to-Sales ratio history"""
    dvd_yld: ValuationMetricData | None
    """Dividend yield history"""


class ValuationData:
    """Valuation metrics response."""

    metrics: ValuationMetricsData
    """Valuation metrics (PE / PB / PS / dividend yield)"""


class ValuationHistoryMetric:
    """Historical data for one valuation metric."""

    desc: str
    """Human-readable description"""
    high: str
    """Historical high over the period"""
    low: str
    """Historical low over the period"""
    median: str
    """Historical median over the period"""
    list: list[ValuationPoint]
    """Historical data points"""


class ValuationHistoryMetrics:
    """Historical valuation metrics container."""

    pe: ValuationHistoryMetric | None
    """Price-to-Earnings history"""
    pb: ValuationHistoryMetric | None
    """Price-to-Book history"""
    ps: ValuationHistoryMetric | None
    """Price-to-Sales history"""


class ValuationHistoryData:
    """Historical valuation data container."""

    metrics: ValuationHistoryMetrics
    """Historical metrics"""


class ValuationHistoryResponse:
    """Historical valuation response."""

    history: ValuationHistoryData
    """Historical valuation data"""


class IndustryValuationHistory:
    """Historical valuation snapshot for one peer security."""

    date: str
    """Unix timestamp string"""
    pe: str
    """Price-to-Earnings ratio"""
    pb: str
    """Price-to-Book ratio"""
    ps: str
    """Price-to-Sales ratio"""


class IndustryValuationItem:
    """Valuation data for one peer security."""

    symbol: str
    """Security symbol"""
    name: str
    """Company name"""
    currency: str
    """Reporting currency"""
    assets: str
    """Total assets"""
    bps: str
    """Book value per share"""
    eps: str
    """Earnings per share"""
    dps: str
    """Dividends per share"""
    div_yld: str
    """Dividend yield"""
    div_payout_ratio: str
    """Dividend payout ratio"""
    five_y_avg_dps: str
    """5-year average dividends per share"""
    pe: str
    """Current PE ratio"""
    history: list[IndustryValuationHistory]
    """Historical PE/PB/PS snapshots"""


class IndustryValuationList:
    """Industry peer valuation comparison response."""

    list: list[IndustryValuationItem]
    """List of peer securities with valuation data"""


class ValuationDist:
    """Distribution statistics for one valuation metric."""

    low: str
    """Minimum value in the industry"""
    high: str
    """Maximum value in the industry"""
    median: str
    """Median value in the industry"""
    value: str
    """Current value of the queried security"""
    ranking: str
    """Percentile ranking (0–1 range)"""
    rank_index: str
    """Ordinal rank index"""
    rank_total: str
    """Total securities in the industry"""


class IndustryValuationDist:
    """Industry valuation distribution response."""

    pe: ValuationDist | None
    """PE ratio distribution"""
    pb: ValuationDist | None
    """PB ratio distribution"""
    ps: ValuationDist | None
    """PS ratio distribution"""


class CompanyOverview:
    """Company overview response."""

    name: str
    """Short name, e.g. ``"腾讯控股"``"""
    company_name: str
    """Full legal name"""
    founded: str
    """Founding date"""
    listing_date: str
    """Listing date"""
    market: str
    """Primary listing market display name"""
    region: str
    """Market region code, e.g. ``"HK"``"""
    address: str
    """Registered address"""
    office_address: str
    """Principal office address"""
    website: str
    """Company website"""
    issue_price: str
    """IPO issue price"""
    shares_offered: str
    """Number of shares offered at IPO"""
    chairman: str
    """Chairman name"""
    secretary: str
    """Company secretary"""
    audit_inst: str
    """Auditing institution"""
    category: str
    """Company classification category"""
    year_end: str
    """Fiscal year end"""
    employees: str
    """Number of employees"""
    phone: str
    """Phone number"""
    fax: str
    """Fax number"""
    email: str
    """Investor relations email"""
    legal_repr: str
    """Legal representative"""
    manager: str
    """CEO / Managing Director"""
    ticker: str
    """Exchange ticker code"""
    icon: str
    """Logo icon URL"""
    profile: str
    """Business profile / description"""
    sector: int
    """Industry sector code"""


class Professional:
    """One executive / board member."""

    id: str
    """Internal wiki person ID"""
    name: str
    """Full name"""
    name_zhcn: str
    """Full name in Simplified Chinese"""
    name_en: str
    """Full name in English"""
    title: str
    """Job title"""
    biography: str
    """Biography text"""
    photo: str
    """Photo URL"""
    wiki_url: str
    """Wiki profile URL"""


class ExecutiveGroup:
    """Executives for one security."""

    symbol: str
    """Security symbol"""
    forward_url: str
    """Link to company wiki page"""
    total: int
    """Total number of executives"""
    professionals: list[Professional]
    """Individual executive entries"""


class ExecutiveList:
    """Executive list response."""

    professional_list: list[ExecutiveGroup]
    """Groups of executives per security"""


class ShareholderStock:
    """A security in an institutional shareholder's cross-holdings."""

    symbol: str
    """Security symbol of the cross-held stock"""
    code: str
    """Ticker code"""
    market: str
    """Market"""
    chg: str
    """Day change percentage"""


class Shareholder:
    """One major shareholder."""

    shareholder_id: str
    """Internal shareholder ID"""
    shareholder_name: str
    """Shareholder name"""
    institution_type: str
    """Institution type"""
    percent_of_shares: str
    """Percentage of shares held"""
    shares_changed: str
    """Change in shares held"""
    report_date: str
    """Most recent filing date"""
    stocks: list[ShareholderStock]
    """Other securities held by this shareholder (cross-holdings)"""


class ShareholderList:
    """Shareholder list response."""

    shareholder_list: list[Shareholder]
    """List of major shareholders"""
    forward_url: str
    """Link to full shareholder page"""
    total: int
    """Total number returned"""


class FundHolder:
    """A fund or ETF that holds the queried security."""

    code: str
    """Fund/ETF ticker code"""
    symbol: str
    """Fund/ETF symbol"""
    currency: str
    """Reporting currency"""
    name: str
    """Fund/ETF full name"""
    position_ratio: str
    """Position ratio percentage string"""
    report_date: str
    """Report date"""


class FundHolders:
    """Fund/ETF holders response."""

    lists: list[FundHolder]
    """Funds and ETFs holding the queried security"""


class CorpActionLive:
    """Live stream associated with a corporate action."""

    id: str
    """Live stream ID"""
    status: str
    """Status: ``"1"``=preview, ``"2"``=live, ``"3"``=ended, ``"4"``=replay"""
    started_at: str
    """Start time"""
    name: str
    """Stream title"""
    icon: str
    """Icon URL"""


class CorpActionItem:
    """One corporate action event."""

    id: str
    """Internal event ID"""
    date: str
    """Date in YYYYMMDD format"""
    date_str: str
    """Short display date"""
    date_type: str
    """Date type label"""
    date_zone: str
    """Time zone description"""
    act_type: str
    """Event category"""
    act_desc: str
    """Human-readable event description"""
    action: str
    """Machine-readable action code"""
    recent: bool
    """Whether this is a recent event"""
    is_delay: bool
    """Whether publication was delayed"""
    delay_content: str
    """Delay announcement content"""
    live: CorpActionLive | None
    """Associated live stream (if any)"""


class CorpActions:
    """Corporate actions response."""

    items: list[CorpActionItem]
    """Corporate action events"""


class InvestSecurity:
    """A security in which the company has an investment stake."""

    company_id: str
    """Internal company ID"""
    company_name: str
    """Company name"""
    company_name_en: str
    """Company name in English"""
    company_name_zhcn: str
    """Company name in Simplified Chinese"""
    symbol: str
    """Security symbol"""
    currency: str
    """Reporting currency"""
    percent_of_shares: str
    """Percentage of shares held"""
    shares_rank: str
    """Shareholder rank"""
    shares_value: str
    """Market value of the holding"""


class InvestRelations:
    """Investor relations response."""

    forward_url: str
    """Link to investor relations page"""
    invest_securities: list[InvestSecurity]
    """Securities in which the company has a stake"""


class OperatingIndicator:
    """One financial indicator from an operating report."""

    field_name: str
    """Field name key, e.g. ``"operating_revenue"``"""
    indicator_name: str
    """Display name"""
    indicator_value: str
    """Formatted value, e.g. ``"8217 亿"``"""
    yoy: str
    """Year-over-year change as decimal string"""


class OperatingFinancial:
    """Key financial metrics from an operating report."""

    code: str
    """Ticker code"""
    symbol: str
    """Symbol in CODE.MARKET format (e.g. ``AAPL.US``)"""
    currency: str
    """Reporting currency"""
    name: str
    """Company name"""
    region: str
    """Market region"""
    report: str
    """Report period code"""
    indicators: list[OperatingIndicator]
    """Financial indicators"""


class OperatingItem:
    """One operating summary report (annual / quarterly)."""

    id: str
    """Internal report ID"""
    report: str
    """Report period code, e.g. ``"af"`` (annual)"""
    title: str
    """Report title"""
    txt: str
    """Management discussion text"""
    latest: bool
    """Whether this is the most recent report"""
    web_url: str
    """URL to the full community report page"""
    financial: OperatingFinancial
    """Key financial metrics"""


class OperatingList:
    """Operating metrics response."""

    list: list[OperatingItem]
    """Operating summary reports"""


class RecentBuybacks:
    """TTM (trailing twelve months) buyback summary."""

    currency: str
    """Reporting currency"""
    net_buyback_ttm: str
    """Net buyback amount TTM"""
    net_buyback_yield_ttm: str
    """Net buyback yield TTM"""


class BuybackHistoryItem:
    """Historical annual buyback data item."""

    fiscal_year: str
    """Fiscal year label, e.g. ``"FY2024"``"""
    fiscal_year_range: str
    """Fiscal year date range string"""
    net_buyback: str
    """Net buyback amount"""
    net_buyback_yield: str
    """Net buyback yield"""
    net_buyback_growth_rate: str
    """Year-over-year net buyback growth rate"""
    currency: str
    """Reporting currency"""


class BuybackRatios:
    """Buyback payout and cash-flow ratios."""

    net_buyback_payout_ratio: str
    """Net buyback payout ratio"""
    net_buyback_to_cashflow_ratio: str
    """Net buyback to free cash-flow ratio"""


class BuybackData:
    """Response for :meth:`FundamentalContext.buyback`."""

    recent_buybacks: "RecentBuybacks | None"
    """Most recent TTM buyback summary"""
    buyback_history: list[BuybackHistoryItem]
    """Historical annual buyback data"""
    buyback_ratios: list[BuybackRatios]
    """Buyback payout and cash-flow ratios"""


class StockRatings:
    """
    Response for :meth:`FundamentalContext.ratings`.

    The ``ratings_json`` field contains the full nested ratings structure as a
    JSON string (too complex to type fully).
    """

    style_txt_name: str
    """Style display name"""
    scale_txt_name: str
    """Scale display name"""
    report_period_txt: str
    """Report period display text"""
    multi_score: str
    """Composite score (string representation)"""
    multi_letter: str
    """Composite score letter grade"""
    multi_score_change: int
    """Score change vs previous period"""
    industry_name: str
    """Industry name"""
    industry_rank: int
    """Industry rank"""
    ratings_json: str
    """Full ratings array as a JSON string"""


class FinancialReportKind:
    """Financial report kind."""

    class IncomeStatement(FinancialReportKind): ...
    """Income statement (IS)"""
    class BalanceSheet(FinancialReportKind): ...
    """Balance sheet (BS)"""
    class CashFlow(FinancialReportKind): ...
    """Cash flow statement (CF)"""
    class All(FinancialReportKind): ...
    """All statements (default)"""


class FinancialReportPeriod:
    """Financial report period type."""

    class Annual(FinancialReportPeriod): ...
    """Annual report (af)"""
    class SemiAnnual(FinancialReportPeriod): ...
    """Semi-annual report (saf)"""
    class Q1(FinancialReportPeriod): ...
    """Q1 report"""
    class Q2(FinancialReportPeriod): ...
    """Q2 report"""
    class Q3(FinancialReportPeriod): ...
    """Q3 report"""
    class QuarterlyFull(FinancialReportPeriod): ...
    """Full quarterly report (qf)"""
    class ThreeQ(FinancialReportPeriod): ...
    """Three-quarter report (3q, first three quarters)"""


class FundamentalContext:
    """
    Fundamental data context.

    Provides access to financial reports, analyst ratings, dividends,
    valuation metrics, company overview, and more.

    Examples:
        ::

            from longbridge.openapi import Config, FundamentalContext, FinancialReportKind

            config = Config.from_env()
            ctx = FundamentalContext(config)

            overview = ctx.company("700.HK")
            print(overview.name, overview.employees)

            dividends = ctx.dividend("700.HK")
            for d in dividends.list:
                print(d.desc, d.payment_date)
    """

    def __init__(self, config: "Config") -> None:
        """Create a FundamentalContext."""
        ...

    def financial_report(
        self,
        symbol: str,
        kind: "FinancialReportKind" = ...,
        period: "FinancialReportPeriod | None" = None,
    ) -> "FinancialReports":
        """
        Get financial reports.

        Args:
            symbol: Security symbol, e.g. ``"700.HK"``
            kind: Report kind (default ``All``)
            period: Report period (``None`` means not specified)

        Returns:
            Financial reports response
        """
        ...

    def institution_rating(self, symbol: str) -> "InstitutionRating":
        """
        Get analyst ratings (latest snapshot + consensus summary).

        Args:
            symbol: Security symbol

        Returns:
            Combined analyst rating response
        """
        ...

    def institution_rating_detail(self, symbol: str) -> "InstitutionRatingDetail":
        """Get historical analyst rating details."""
        ...

    def dividend(self, symbol: str) -> "DividendList":
        """Get dividend history."""
        ...

    def dividend_detail(self, symbol: str) -> "DividendList":
        """Get detailed dividend information."""
        ...

    def forecast_eps(self, symbol: str) -> "ForecastEps":
        """Get EPS forecasts."""
        ...

    def consensus(self, symbol: str) -> "FinancialConsensus":
        """Get financial consensus estimates."""
        ...

    def valuation(self, symbol: str) -> "ValuationData":
        """Get valuation metrics (PE / PB / PS / dividend yield)."""
        ...

    def valuation_history(self, symbol: str) -> "ValuationHistoryResponse":
        """Get historical valuation data."""
        ...

    def industry_valuation(self, symbol: str) -> "IndustryValuationList":
        """Get industry peer valuation comparison."""
        ...

    def industry_valuation_dist(self, symbol: str) -> "IndustryValuationDist":
        """Get industry valuation distribution."""
        ...

    def company(self, symbol: str) -> "CompanyOverview":
        """Get company overview."""
        ...

    def executive(self, symbol: str) -> "ExecutiveList":
        """Get executive and board member information."""
        ...

    def shareholder(self, symbol: str) -> "ShareholderList":
        """Get major shareholders."""
        ...

    def fund_holder(self, symbol: str) -> "FundHolders":
        """Get funds and ETFs that hold the security."""
        ...

    def corp_action(self, symbol: str) -> "CorpActions":
        """Get corporate actions (dividends, splits, buybacks, etc.)."""
        ...

    def invest_relation(self, symbol: str) -> "InvestRelations":
        """Get investor relations / investment holdings."""
        ...

    def operating(self, symbol: str) -> "OperatingList":
        """Get operating metrics and financial report summaries."""
        ...

    def buyback(self, symbol: str) -> "BuybackData":
        """
        Get buyback data for a security.

        Args:
            symbol: Security symbol, e.g. ``"AAPL.US"``

        Returns:
            :class:`BuybackData`
        """
        ...

    def ratings(self, symbol: str) -> "StockRatings":
        """
        Get stock ratings for a security.

        Args:
            symbol: Security symbol, e.g. ``"AAPL.US"``

        Returns:
            :class:`StockRatings`
        """
        ...

    def shareholder_top(self, symbol: str) -> "ShareholderTopResponse":
        """
        Get ranked list of top shareholders.

        Args:
            symbol: Security symbol

        Returns:
            :class:`ShareholderTopResponse` with raw JSON data
        """
        ...

    def shareholder_detail(
        self, symbol: str, object_id: int
    ) -> "ShareholderDetailResponse":
        """
        Get holding history and detail for one shareholder.

        Args:
            symbol: Security symbol
            object_id: Shareholder object ID

        Returns:
            :class:`ShareholderDetailResponse` with raw JSON data
        """
        ...

    def valuation_comparison(
        self,
        symbol: str,
        currency: str,
        comparison_symbols: Optional[List[str]] = None,
    ) -> "ValuationComparisonResponse":
        """
        Get valuation comparison between a security and optional peers.

        Args:
            symbol: Security symbol
            currency: Currency code (e.g. ``"USD"``)
            comparison_symbols: Optional list of peer symbols

        Returns:
            :class:`ValuationComparisonResponse` with raw JSON data
        """
        ...

    def etf_asset_allocation(self, symbol: str) -> "AssetAllocationResponse":
        """
        Get ETF asset allocation (holdings / regional / asset class / industry).

        Args:
            symbol: ETF security code (e.g. ``"QQQ.US"``)

        Returns:
            :class:`AssetAllocationResponse` with allocation groups
        """
        ...

    def macroeconomic_indicators(
        self,
        offset: int | None = None,
        limit: int | None = None,
    ) -> list["MacroeconomicIndicator"]:
        """
        List macroeconomic indicators.

        Args:
            offset: Pagination offset (default 0)
            limit: Page size (default 100, max 1000)

        Returns:
            List of :class:`MacroeconomicIndicator`
        """
        ...

    def macroeconomic(
        self,
        indicator_code: str,
        start_date: str | None = None,
        end_date: str | None = None,
        limit: int | None = None,
    ) -> "MacroeconomicResponse":
        """
        Get historical data for a macroeconomic indicator.

        Args:
            indicator_code: External vendor code from ``macroeconomic_indicators``
            start_date: Start date in ``"YYYY-MM-DD"`` format (optional)
            end_date: End date in ``"YYYY-MM-DD"`` format (optional)
            limit: Max records to return (default 100, max 100)

        Returns:
            :class:`MacroeconomicResponse`
        """
        ...

    def us_company_overview(self, symbol: str) -> "USCompanyOverview":
        """Get US company overview. US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``

        Returns:
            :class:`USCompanyOverview`
        """
        ...

    def us_valuation_overview(self, symbol: str) -> "USValuationOverview":
        """Get US valuation overview. US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``

        Returns:
            :class:`USValuationOverview`
        """
        ...

    def us_financial_overview(self, symbol: str, report: str) -> "USFinancialOverview":
        """Get US financial overview (revenue, net income, EPS, cash flow). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            report: ``"annual"`` or ``"quarterly"``

        Returns:
            :class:`USFinancialOverview`
        """
        ...

    def us_financial_statement(self, symbol: str, kind: str, report: str) -> "USFinancialStatement":
        """Get US financial statement detail (IS/BS/CF). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            kind: Statement kind: ``"IS"`` (income), ``"BS"`` (balance sheet), ``"CF"`` (cash flow)
            report: Period: ``"q1"`` (Q1), ``"qf"`` (quarterly), ``"saf"`` (semi-annual), ``"3q"`` (Q3), ``"af"`` (annual)

        Returns:
            :class:`USFinancialStatement`
        """
        ...

    def us_key_financial_metrics(self, symbol: str, report: str) -> "USKeyFinancialMetrics":
        """Get US key financial metrics (ROE, margins, debt ratio). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            report: Period: ``"q1"`` (Q1), ``"qf"`` (quarterly), ``"saf"`` (semi-annual), ``"3q"`` (Q3), ``"af"`` (annual)

        Returns:
            :class:`USKeyFinancialMetrics`
        """
        ...

    def us_analyst_consensus(self, symbol: str, report: str) -> "USAnalystConsensus":
        """Get US analyst consensus estimates (EPS, revenue forecasts). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            report: Period: ``"q1"`` (Q1), ``"qf"`` (quarterly), ``"saf"`` (semi-annual), ``"3q"`` (Q3), ``"af"`` (annual)

        Returns:
            :class:`USAnalystConsensus`
        """
        ...

    def us_etf_dividend_info(self, symbol: str) -> "USETFDividendInfo":
        """Get US ETF dividend history. US token required.

        Args:
            symbol: ETF symbol, e.g. ``"SPY.US"``

        Returns:
            :class:`USETFDividendInfo`
        """
        ...

    def us_company_dividends(self, symbol: str) -> "USCompanyDividends":
        """Get US company historical dividends. US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``

        Returns:
            :class:`USCompanyDividends`
        """
        ...

    def us_etf_files(self, symbol: str, size: Optional[int] = None) -> "USETFFilesResponse":
        """Get US ETF document list. US token required.

        Args:
            symbol: ETF symbol, e.g. ``"SPY.US"``
            size: Number of files to return; ``None`` returns all

        Returns:
            :class:`USETFFilesResponse`
        """
        ...


class AsyncFundamentalContext:
    """Async fundamental data context for use with asyncio. All I/O methods return awaitables."""

    def __init__(self, config: "Config") -> None:
        """Create an AsyncFundamentalContext."""
        ...

    def us_company_overview(self, symbol: str) -> "Awaitable[USCompanyOverview]":
        """Get US company overview. US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``

        Returns:
            Awaitable[:class:`USCompanyOverview`]
        """
        ...

    def us_valuation_overview(self, symbol: str) -> "Awaitable[USValuationOverview]":
        """Get US valuation overview. US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``

        Returns:
            Awaitable[:class:`USValuationOverview`]
        """
        ...

    def us_financial_overview(self, symbol: str, report: str) -> "Awaitable[USFinancialOverview]":
        """Get US financial overview (revenue, net income, EPS, cash flow). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            report: ``"annual"`` or ``"quarterly"``

        Returns:
            Awaitable[:class:`USFinancialOverview`]
        """
        ...

    def us_financial_statement(self, symbol: str, kind: str, report: str) -> "Awaitable[USFinancialStatement]":
        """Get US financial statement detail (IS/BS/CF). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            kind: Statement kind: ``"IS"`` (income), ``"BS"`` (balance sheet), ``"CF"`` (cash flow)
            report: Period: ``"q1"`` (Q1), ``"qf"`` (quarterly), ``"saf"`` (semi-annual), ``"3q"`` (Q3), ``"af"`` (annual)

        Returns:
            Awaitable[:class:`USFinancialStatement`]
        """
        ...

    def us_key_financial_metrics(self, symbol: str, report: str) -> "Awaitable[USKeyFinancialMetrics]":
        """Get US key financial metrics (ROE, margins, debt ratio). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            report: Period: ``"q1"`` (Q1), ``"qf"`` (quarterly), ``"saf"`` (semi-annual), ``"3q"`` (Q3), ``"af"`` (annual)

        Returns:
            Awaitable[:class:`USKeyFinancialMetrics`]
        """
        ...

    def us_analyst_consensus(self, symbol: str, report: str) -> "Awaitable[USAnalystConsensus]":
        """Get US analyst consensus estimates (EPS, revenue forecasts). US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``
            report: Period: ``"q1"`` (Q1), ``"qf"`` (quarterly), ``"saf"`` (semi-annual), ``"3q"`` (Q3), ``"af"`` (annual)

        Returns:
            Awaitable[:class:`USAnalystConsensus`]
        """
        ...

    def us_etf_dividend_info(self, symbol: str) -> "Awaitable[USETFDividendInfo]":
        """Get US ETF dividend history. US token required.

        Args:
            symbol: ETF symbol, e.g. ``"SPY.US"``

        Returns:
            Awaitable[:class:`USETFDividendInfo`]
        """
        ...

    def us_company_dividends(self, symbol: str) -> "Awaitable[USCompanyDividends]":
        """Get US company historical dividends. US token required.

        Args:
            symbol: Symbol, e.g. ``"AAPL.US"``

        Returns:
            Awaitable[:class:`USCompanyDividends`]
        """
        ...

    def us_etf_files(self, symbol: str, size: Optional[int] = None) -> "Awaitable[USETFFilesResponse]":
        """Get US ETF document list. US token required.

        Args:
            symbol: ETF symbol, e.g. ``"SPY.US"``
            size: Number of files to return; ``None`` returns all

        Returns:
            Awaitable[:class:`USETFFilesResponse`]
        """
        ...


# ── FundamentalContext new response types ─────────────────────────

class ShareholderTopResponse:
    """Top-shareholder list response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw top-shareholder data (JSON object / list)"""


class ShareholderDetailResponse:
    """Shareholder detail response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw shareholder detail data (JSON object / list)"""


class ValuationHistoryPoint:
    """One historical valuation data point."""

    date: str
    """Date (RFC 3339, converted from Unix timestamp)"""
    pe: str
    """P/E ratio"""
    pb: str
    """P/B ratio"""
    ps: str
    """P/S ratio"""


class ValuationComparisonItem:
    """One security's valuation comparison item."""

    symbol: str
    """Symbol (e.g. ``"AAPL.US"``)"""
    name: str
    """Security name"""
    currency: str
    """Currency"""
    market_value: str
    """Market capitalisation"""
    price_close: str
    """Latest closing price"""
    pe: str
    """P/E ratio"""
    pb: str
    """P/B ratio"""
    ps: str
    """P/S ratio"""
    roe: str
    """Return on equity"""
    eps: str
    """Earnings per share"""
    bps: str
    """Book value per share"""
    dps: str
    """Dividends per share"""
    div_yld: str
    """Dividend yield"""
    assets: str
    """Total assets"""
    history: List[ValuationHistoryPoint]
    """Historical valuation points"""


class ValuationComparisonResponse:
    """Valuation comparison response."""

    list: List[ValuationComparisonItem]
    """Valuation comparison items"""


class ElementType:
    """ETF asset allocation element type."""

    class Unknown(ElementType):
        """Unknown"""

    class Holdings(ElementType):
        """Holdings"""

    class Regional(ElementType):
        """Regional"""

    class AssetClass(ElementType):
        """Asset class"""

    class Industry(ElementType):
        """Industry"""


class HoldingDetail:
    """Holding detail of an ETF asset allocation element (holdings only)."""

    industry_id: str
    """Industry ID"""
    industry_name: str
    """Industry name"""
    index: str
    """Index counter ID (e.g. ``BK/US/CP99000``)"""
    index_name: str
    """Index name"""
    holding_type: str
    """Holding type (e.g. ``E`` for stock)"""
    holding_type_name: str
    """Holding type name"""


class AssetAllocationItem:
    """One element of an ETF asset allocation group."""

    name: str
    """Element name"""
    code: str
    """Security code (holdings only, e.g. ``NVDA``)"""
    position_ratio: str
    """Position ratio (e.g. ``0.0861114``)"""
    symbol: str
    """Security symbol (holdings only, e.g. ``NVDA.US``)"""
    name_locales: dict[str, str]
    """Localized names (locale → name)"""
    holding_detail: Optional[HoldingDetail]
    """Holding detail (holdings only)"""


class AssetAllocationGroup:
    """One ETF asset allocation group (grouped by element type)."""

    report_date: str
    """Report date (e.g. ``20260601``)"""
    asset_type: Type[ElementType]
    """Element type of this group"""
    lists: list[AssetAllocationItem]
    """Elements"""


class AssetAllocationResponse:
    """ETF asset allocation response."""

    info: list[AssetAllocationGroup]
    """Asset allocation groups"""


class MultiLanguageText:
    """Localized text in simplified Chinese, traditional Chinese, and English."""

    english: str
    simplified_chinese: str
    traditional_chinese: str


class MacroeconomicIndicator:
    """Metadata for one macroeconomic indicator."""

    indicator_code: str
    """External vendor code (input to macroeconomic)"""
    source_org: str
    country: str
    name: str
    adjustment_factor: str
    periodicity: str
    """Release periodicity (e.g. monthly / quarterly)"""
    category: str
    describe: str
    importance: int
    start_date: datetime | None
    """Start date of data coverage"""


class Macroeconomic:
    """One historical data point for a macroeconomic indicator."""

    period: str
    """Statistical period (e.g. 2024-Q1, 2024-03)"""
    release_at: datetime | None
    actual_value: str
    previous_value: str
    forecast_value: str
    revised_value: str
    next_release_at: datetime | None
    unit: str
    unit_prefix: str


class MacroeconomicResponse:
    """Response for macroeconomic."""

    info: MacroeconomicIndicator
    data: list[Macroeconomic]


# ── MarketContext ─────────────────────────────────────────────────

class MarketTimeItem:
    """Trading status for one market."""

    market: Market
    """Market"""
    trade_status: int
    """Raw market trade status code. See the market status definition for the complete code table."""
    timestamp: str
    """Current market time (unix timestamp string)"""
    delay_trade_status: int
    """Delayed-quote market trade status code"""
    delay_timestamp: str
    """Delayed-quote market time (unix timestamp string)"""
    sub_status: int
    """Sub-status code"""
    delay_sub_status: int
    """Delayed-quote sub-status code"""


class MarketStatusResponse:
    """Market trading status response."""

    market_time: list[MarketTimeItem]
    """Per-market trading status items"""


class BrokerHoldingEntry:
    """One broker entry in a top-holding list."""

    name: str
    """Broker name"""
    parti_number: str
    """Participant number / broker code"""
    chg: str
    """Net change in shares held"""
    strong: bool
    """Whether this is a strengthening broker"""


class BrokerHoldingTop:
    """Top broker holdings response."""

    buy: list[BrokerHoldingEntry]
    """Top buying brokers"""
    sell: list[BrokerHoldingEntry]
    """Top selling brokers"""
    updated_at: str
    """Last updated string"""


class BrokerHoldingChanges:
    """Broker holding changes over multiple periods."""

    value: str
    """Current value"""
    chg_1: str
    """1-day change"""
    chg_5: str
    """5-day change"""
    chg_20: str
    """20-day change"""
    chg_60: str
    """60-day change"""


class BrokerHoldingDetailItem:
    """One broker's full holding detail."""

    name: str
    """Broker name"""
    parti_number: str
    """Participant number"""
    ratio: BrokerHoldingChanges
    """Holding ratio changes over various periods"""
    shares: BrokerHoldingChanges
    """Share count changes over various periods"""
    strong: bool
    """Whether this is a strengthening broker"""


class BrokerHoldingDetail:
    """Full broker holding detail response."""

    list: list[BrokerHoldingDetailItem]
    """Full broker list"""
    updated_at: str
    """Last updated string"""


class BrokerHoldingDailyItem:
    """One day's broker holding record."""

    date: str
    """Date in ``"2026.05.05"`` format"""
    holding: str
    """Total shares held"""
    ratio: str
    """Holding ratio"""
    chg: str
    """Daily change"""


class BrokerHoldingDailyHistory:
    """Daily broker holding history response."""

    list: list[BrokerHoldingDailyItem]
    """Daily records"""


class AhPremiumKline:
    """One A/H premium data point."""

    aprice: str
    """A-share price"""
    apreclose: str
    """A-share previous close"""
    hprice: str
    """H-share price"""
    hpreclose: str
    """H-share previous close"""
    currency_rate: str
    """CNY/HKD exchange rate"""
    ahpremium_rate: str
    """A/H premium rate (negative = H-share at premium)"""
    price_spread: str
    """Price spread"""
    timestamp: datetime
    """Data point timestamp"""


class AhPremiumKlines:
    """A/H premium K-line response."""

    klines: list[AhPremiumKline]
    """K-line data points"""


class AhPremiumIntraday:
    """A/H premium intraday response."""

    klines: list[AhPremiumKline]
    """Intraday data points"""


class TradePriceLevel:
    """Trade volume at one price level."""

    buy_amount: str
    """Buy volume at this price"""
    neutral_amount: str
    """Neutral (unknown direction) volume"""
    price: str
    """Price level"""
    sell_amount: str
    """Sell volume at this price"""


class TradeStatistics:
    """Summary trade statistics."""

    avgprice: str
    """Volume-weighted average price"""
    buy: str
    """Total buy volume (shares)"""
    neutral: str
    """Total neutral / unknown-direction volume"""
    preclose: str
    """Previous close price"""
    sell: str
    """Total sell volume (shares)"""
    timestamp: str
    """Data timestamp (unix timestamp string)"""
    total_amount: str
    """Total trading volume (shares)"""
    trade_date: list[str]
    """Unix timestamps for the last 5 trading days"""
    trades_count: str
    """Total number of trades"""


class TradeStatsResponse:
    """Trade statistics response."""

    statistics: TradeStatistics
    """Summary statistics"""
    trades: list[TradePriceLevel]
    """Per-price-level breakdown"""


class AnomalyItem:
    """One market anomaly event."""

    symbol: str
    """Security symbol"""
    name: str
    """Security name"""
    alert_name: str
    """Anomaly type name, e.g. ``"大宗交易"``"""
    alert_time: int
    """Time of the anomaly (unix timestamp in milliseconds)"""
    change_values: list[str]
    """Change value strings"""
    emotion: int
    """Sentiment direction: 1=positive/up, 2=negative/down"""


class AnomalyResponse:
    """Market anomaly response."""

    all_off: bool
    """Whether anomaly alerts are globally disabled"""
    changes: list[AnomalyItem]
    """List of market anomaly events"""


class ConstituentStock:
    """One constituent stock of an index."""

    symbol: str
    """Security symbol"""
    name: str
    """Security name"""
    last_done: str
    """Latest price"""
    prev_close: str
    """Previous close"""
    inflow: str
    """Net capital inflow today"""
    balance: str
    """Turnover amount"""
    amount: str
    """Trading volume (shares)"""
    total_shares: str
    """Total shares outstanding"""
    tags: list[str]
    """Tags, e.g. ``["领涨龙头"]``"""
    intro: str
    """Brief description"""
    market: str
    """Market, e.g. ``"HK"``"""
    circulating_shares: str
    """Circulating shares"""
    delay: bool
    """Whether this is a delayed quote"""
    chg: str
    """Day change percentage"""
    trade_status: int
    """Raw trade status code"""


class IndexConstituents:
    """Index constituents response."""

    fall_num: int
    """Number of constituent stocks that fell today"""
    flat_num: int
    """Number of constituent stocks unchanged today"""
    rise_num: int
    """Number of constituent stocks that rose today"""
    stocks: list[ConstituentStock]
    """Constituent stock details"""


class BrokerHoldingPeriod:
    """Broker holding lookback period."""

    class Rct1(BrokerHoldingPeriod): ...
    """1-day change"""
    class Rct5(BrokerHoldingPeriod): ...
    """5-day change"""
    class Rct20(BrokerHoldingPeriod): ...
    """20-day change"""
    class Rct60(BrokerHoldingPeriod): ...
    """60-day change"""


class AhPremiumPeriod:
    """A/H premium K-line period."""

    class Min1(AhPremiumPeriod): ...
    """1-minute"""
    class Min5(AhPremiumPeriod): ...
    """5-minute"""
    class Min15(AhPremiumPeriod): ...
    """15-minute"""
    class Min30(AhPremiumPeriod): ...
    """30-minute"""
    class Min60(AhPremiumPeriod): ...
    """60-minute"""
    class Day(AhPremiumPeriod): ...
    """Daily (default)"""
    class Week(AhPremiumPeriod): ...
    """Weekly"""
    class Month(AhPremiumPeriod): ...
    """Monthly"""
    class Year(AhPremiumPeriod): ...
    """Yearly"""


class MarketContext:
    """
    Market data context.

    Provides broker holdings, A/H premium, trade statistics,
    market anomaly alerts, and index constituents.

    Examples:
        ::

            from longbridge.openapi import Config, MarketContext

            config = Config.from_env()
            ctx = MarketContext(config)
            status = ctx.market_status()
            for item in status.market_time:
                print(item.market, item.trade_status)
    """

    def __init__(self, config: "Config") -> None:
        """Create a MarketContext."""
        ...

    def market_status(self) -> "MarketStatusResponse":
        """Get current trading status for all markets."""
        ...

    def broker_holding(
        self,
        symbol: str,
        period: "BrokerHoldingPeriod" = ...,
    ) -> "BrokerHoldingTop":
        """
        Get top broker holdings (buy/sell leaders) for a security.

        Args:
            symbol: Security symbol
            period: Lookback period (default ``Rct1``)
        """
        ...

    def broker_holding_detail(self, symbol: str) -> "BrokerHoldingDetail":
        """Get full broker holding details for a security."""
        ...

    def broker_holding_daily(
        self, symbol: str, broker_id: str
    ) -> "BrokerHoldingDailyHistory":
        """
        Get daily holding history for a specific broker.

        Args:
            symbol: Security symbol
            broker_id: Broker participant number, e.g. ``"B01451"``
        """
        ...

    def ah_premium(
        self,
        symbol: str,
        period: "AhPremiumPeriod" = ...,
        count: int = 100,
    ) -> "AhPremiumKlines":
        """
        Get A/H premium K-line data for a dual-listed security.

        Args:
            symbol: H-share symbol, e.g. ``"2318.HK"``
            period: K-line period (default ``Day``)
            count: Number of K-lines to return
        """
        ...

    def ah_premium_intraday(self, symbol: str) -> "AhPremiumIntraday":
        """Get A/H premium intraday data for a dual-listed security."""
        ...

    def trade_stats(self, symbol: str) -> "TradeStatsResponse":
        """Get buy/sell/neutral trade statistics for a security."""
        ...

    def anomaly(self, market: str) -> "AnomalyResponse":
        """
        Get market anomaly alerts (unusual price/volume events).

        Args:
            market: Market code: ``"HK"``, ``"US"``, ``"CN"``, ``"SG"``
        """
        ...

    def constituent(self, symbol: str) -> "IndexConstituents":
        """
        Get constituent stocks for an index.

        Args:
            symbol: Index symbol, e.g. ``"HSI.HK"``
        """
        ...

    def top_movers(
        self,
        markets: List[str],
        sort: int = 0,
        date: Optional[str] = None,
        limit: int = 20,
    ) -> "TopMoversResponse":
        """
        Get top movers (stocks with unusual price movements) across one or more markets.

        Args:
            markets: List of market codes, e.g. ``["HK", "US"]``
            sort: Sort order (0=ascending, 1=descending)
            date: Optional date filter (``"YYYY-MM-DD"``)
            limit: Max records to return

        Returns:
            :class:`TopMoversResponse` with raw JSON data
        """
        ...

    def rank_categories(self) -> "RankCategoriesResponse":
        """
        Get all available rank category keys and labels.

        Returns:
            :class:`RankCategoriesResponse` with raw JSON data
        """
        ...

    def rank_list(
        self, key: str, need_article: bool = False
    ) -> "RankListResponse":
        """
        Get a ranked list of securities for the given category key.

        Args:
            key: Category key from :meth:`rank_categories`
            need_article: Whether to include article content

        Returns:
            :class:`RankListResponse` with raw JSON data
        """
        ...


# ── MarketContext new response types ──────────────────────────────

class TopMoversStock:
    """Stock information within a top-movers event."""

    symbol: str
    """Symbol (e.g. ``"NVDA.US"``)"""
    code: str
    """Ticker code"""
    name: str
    """Security name"""
    full_name: str
    """Full name"""
    change: str
    """Price change (decimal ratio)"""
    last_done: str
    """Latest price"""
    market: str
    """Market code"""
    labels: List[str]
    """Labels / tags"""
    logo: str
    """Logo URL"""


class TopMoversEvent:
    """One top-movers event entry."""

    timestamp: str
    """Event time (RFC 3339)"""
    alert_reason: str
    """Alert reason description"""
    alert_type: int
    """Alert type code"""
    stock: TopMoversStock
    """Stock information"""
    post: object
    """Associated news post (raw JSON object)"""


class TopMoversResponse:
    """Top movers response."""

    events: List[TopMoversEvent]
    """Top-mover events"""
    next_params: object
    """Pagination cursor for next page (raw JSON object)"""


class RankCategoriesResponse:
    """Rank categories response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw rank categories data (JSON object / list)"""


class RankListItem:
    """One ranked security item."""

    symbol: str
    """Symbol (e.g. ``"MU.US"``)"""
    code: str
    """Ticker code"""
    name: str
    """Security name"""
    last_done: str
    """Latest price"""
    chg: str
    """Price change ratio"""
    change: str
    """Absolute price change"""
    inflow: str
    """Net inflow"""
    market_cap: str
    """Market cap"""
    industry: str
    """Industry name"""
    pre_post_price: str
    """Pre/post market price"""
    pre_post_chg: str
    """Pre/post market change"""
    amplitude: str
    """Amplitude"""
    five_day_chg: str
    """5-day change"""
    turnover_rate: str
    """Turnover rate"""
    volume_rate: str
    """Volume ratio"""
    pb_ttm: str
    """P/B ratio (TTM)"""


class RankListResponse:
    """Rank list response."""

    bmp: bool
    """Whether delayed / BMP data"""
    lists: List[RankListItem]
    """Ranked security items"""


# ── ScreenerContext ───────────────────────────────────────────────

class ScreenerCondition:
    """A filter condition for :meth:`ScreenerContext.screener_search` Mode B."""

    key: str
    """Indicator key without ``filter_`` prefix, e.g. ``"pettm"``, ``"roe"``, ``"macd_day"``"""
    min: str
    """Lower bound (empty string = no lower bound)"""
    max: str
    """Upper bound (empty string = no upper bound)"""
    tech_values: str
    """Technical indicator params as JSON string. Use ``"{}"`` for fundamental indicators.
    Example: ``'{"category": "goldenfork", "period": "day"}'``"""

    def __init__(
        self,
        key: str,
        min: str = "",
        max: str = "",
        tech_values: str = "{}",
    ) -> None: ...


class ScreenerRecommendStrategiesResponse:
    """Recommended screener strategies response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw recommended strategies data (JSON object / list)"""


class ScreenerUserStrategiesResponse:
    """User screener strategies response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw user strategies data (JSON object / list)"""


class ScreenerStrategyResponse:
    """Single screener strategy response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw strategy detail data (JSON object / list)"""


class ScreenerSearchResponse:
    """Screener search results response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw search results data (JSON object / list)"""


class ScreenerIndicatorsResponse:
    """Screener indicator definitions response. ``data`` is a Python dict/list from JSON."""

    data: object
    """Raw indicator definitions data (JSON object / list)"""


class ScreenerContext:
    """Stock screener context — strategies, search, and indicators."""

    def __init__(self, config: Config) -> None: ...

    def screener_recommend_strategies(self, market: str) -> ScreenerRecommendStrategiesResponse:
        """Get preset built-in screener strategies."""
        ...

    def screener_user_strategies(self, market: str) -> ScreenerUserStrategiesResponse:
        """Get the current user's saved screener strategies."""
        ...

    def screener_strategy(self, id: int) -> ScreenerStrategyResponse:
        """Get detail for one screener strategy by ID."""
        ...

    def screener_search(
        self,
        market: str,
        strategy_id: Optional[int] = None,
        conditions: List["ScreenerCondition"] = [],
        show: List[str] = [],
        page: int = 0,
        size: int = 20,
    ) -> ScreenerSearchResponse:
        """Search / screen securities using a strategy or custom conditions.

        When *strategy_id* is given (Mode A), the strategy is fetched from the AI
        endpoint and its filters are forwarded to the search request.  The
        ``market`` is taken from the strategy response.

        When *strategy_id* is ``None`` (Mode B), *conditions* must be provided as
        :class:`ScreenerCondition` objects and *market* is used directly.

        ``filter_`` is stripped from every ``items[].indicators[].key`` in the
        response before it is returned.
        """
        ...

    def screener_indicators(self) -> ScreenerIndicatorsResponse:
        """Get all available screener indicator definitions."""
        ...


class AsyncScreenerContext:
    """Async screener context for use with asyncio."""

    @classmethod
    def create(cls, config: Config) -> "AsyncScreenerContext": ...

    def screener_recommend_strategies(
        self,
        market: str,
    ) -> Awaitable[ScreenerRecommendStrategiesResponse]:
        """Get preset built-in screener strategies. Returns awaitable."""
        ...

    def screener_user_strategies(
        self,
        market: str,
    ) -> Awaitable[ScreenerUserStrategiesResponse]:
        """Get the current user's saved screener strategies. Returns awaitable."""
        ...

    def screener_strategy(
        self, id: int
    ) -> Awaitable[ScreenerStrategyResponse]:
        """Get detail for one screener strategy by ID. Returns awaitable."""
        ...

    def screener_search(
        self,
        market: str,
        strategy_id: Optional[int] = None,
        conditions: List["ScreenerCondition"] = [],
        show: List[str] = [],
        page: int = 0,
        size: int = 20,
    ) -> Awaitable[ScreenerSearchResponse]:
        """Search / screen securities using a strategy or custom conditions.
        Returns awaitable.

        When *strategy_id* is given (Mode A), the strategy is fetched from the AI
        endpoint and its filters are forwarded to the search request.  The
        ``market`` is taken from the strategy response.

        When *strategy_id* is ``None`` (Mode B), *conditions* must be provided as
        :class:`ScreenerCondition` objects and *market* is used directly.

        ``filter_`` is stripped from every ``items[].indicators[].key`` in the
        response before it is returned.
        """
        ...

    def screener_indicators(self) -> Awaitable[ScreenerIndicatorsResponse]:
        """Get all available screener indicator definitions. Returns awaitable."""
        ...


# ── CalendarContext ───────────────────────────────────────────────

class CalendarDataKv:
    """One key-value data pair in a calendar event."""

    key: str
    """Key (may be empty)"""
    value: str
    """Formatted display value"""
    value_type: str
    """Value type code, e.g. ``"estimate_eps"``"""
    value_raw: str
    """Raw numeric value"""


class CalendarEventInfo:
    """One financial calendar event."""

    symbol: str
    """Security symbol"""
    market: str
    """Market, e.g. ``"HK"``"""
    content: str
    """Event content description"""
    counter_name: str
    """Security name"""
    date_type: str
    """Date type label, e.g. ``"盘前"``"""
    date: str
    """Event date string, e.g. ``"2025.05.02"``"""
    chart_uid: str
    """Chart UID (may be empty)"""
    data_kv: list[CalendarDataKv]
    """Structured data key-value pairs"""
    event_type: str
    """Event type code, e.g. ``"financial"``"""
    datetime: str
    """Event datetime (unix timestamp string)"""
    icon: str
    """Icon URL"""
    star: int
    """Importance star rating (0–3)"""
    id: str
    """Internal event ID"""
    financial_market_time: str
    """Financial market session time string"""
    currency: str
    """Currency"""
    activity_type: str
    """Activity type code"""


class CalendarDateGroup:
    """Events for one calendar date."""

    date: str
    """Date string, e.g. ``"2025-05-02"``"""
    count: int
    """Total event count for this date"""
    infos: list[CalendarEventInfo]
    """Event details"""


class CalendarEventsResponse:
    """Finance calendar response."""

    date: str
    """Start date of the query window"""
    list: list[CalendarDateGroup]
    """Per-day event groups"""
    next_date: str
    """Pagination cursor; pass as start to fetch the next page, empty when there are no more pages"""


class CalendarCategory:
    """Calendar event category."""

    class Report(CalendarCategory): ...
    """Earnings reports"""
    class Dividend(CalendarCategory): ...
    """Dividend events"""
    class Split(CalendarCategory): ...
    """Stock splits"""
    class Ipo(CalendarCategory): ...
    """IPOs"""
    class MacroData(CalendarCategory): ...
    """Macro-economic data releases"""
    class Closed(CalendarCategory): ...
    """Market closure days"""
    class Meeting(CalendarCategory): ...
    """Shareholder / analyst meetings"""
    class Merge(CalendarCategory): ...
    """Stock consolidations / mergers"""


class CalendarContext:
    """
    Financial calendar context.

    Examples:
        ::

            from longbridge.openapi import Config, CalendarContext, CalendarCategory

            config = Config.from_env()
            ctx = CalendarContext(config)
            resp = ctx.finance_calendar(
                CalendarCategory.Report, "2025-05-01", "2025-05-31", "HK"
            )
            for group in resp.list:
                print(group.date, group.count)
    """

    def __init__(self, config: "Config") -> None:
        """Create a CalendarContext."""
        ...

    def finance_calendar(
        self,
        category: "CalendarCategory",
        start: str,
        end: str,
        market: str | None = None,
    ) -> "CalendarEventsResponse":
        """
        Get financial calendar events.

        Args:
            category: Event category
            start: Start date in ``YYYY-MM-DD`` format
            end: End date in ``YYYY-MM-DD`` format
            market: Optional market filter, e.g. ``"HK"``
        """
        ...


# ── PortfolioContext ──────────────────────────────────────────────

class ExchangeRate:
    """One currency exchange rate."""

    average_rate: float
    """Average rate (base_currency per other_currency)"""
    base_currency: str
    """Base currency, e.g. ``"USD"``"""
    bid_rate: float
    """Bid rate"""
    offer_rate: float
    """Offer rate"""
    other_currency: str
    """Other currency, e.g. ``"HKD"``"""


class ExchangeRates:
    """Exchange rates response."""

    exchanges: list[ExchangeRate]
    """List of exchange rates"""


class AssetType:
    """Asset class category."""

    class Unknown(AssetType): ...
    """Unknown"""
    class Stock(AssetType): ...
    """Stock"""
    class Fund(AssetType): ...
    """Fund"""
    class Crypto(AssetType): ...
    """Crypto"""


class FlowDirection:
    """Trade flow direction."""

    class Unknown(FlowDirection): ...
    """Unknown"""
    class Buy(FlowDirection): ...
    """Buy"""
    class Sell(FlowDirection): ...
    """Sell"""


class ProfitSummaryInfo:
    """P&L summary for one asset category."""

    asset_type: AssetType
    """Asset type"""
    profit_max: str
    """Security with the maximum profit"""
    profit_max_name: str
    """Name of the max-profit security"""
    loss_max: str
    """Security with the maximum loss"""
    loss_max_name: str
    """Name of the max-loss security"""


class ProfitSummaryBreakdown:
    """P&L breakdown by asset type."""

    stock: str
    """Stock P&L"""
    fund: str
    """Fund P&L"""
    crypto: str
    """Crypto P&L"""
    mmf: str
    """Money market fund P&L"""
    other: str
    """Other P&L"""
    cumulative_transaction_amount: str
    """Cumulative transaction amount"""
    trade_order_num: str
    """Total number of orders"""
    trade_stock_num: str
    """Total number of traded securities"""
    ipo: str
    """IPO P&L"""
    ipo_hit: int
    """IPO hits"""
    ipo_subscription: int
    """IPO subscriptions"""
    summary_info: list[ProfitSummaryInfo]
    """Per-category summary"""


class ProfitAnalysisSummary:
    """Account-level P&L summary."""

    currency: str
    """Account currency"""
    current_total_asset: str
    """Current total asset value"""
    start_date: str
    """Query start date"""
    end_date: str
    """Query end date"""
    start_time: str
    """Start time (unix timestamp string)"""
    end_time: str
    """End time (unix timestamp string)"""
    ending_asset_value: str
    """Ending asset value"""
    initial_asset_value: str
    """Initial asset value"""
    invest_amount: str
    """Total invested amount"""
    is_traded: bool
    """Whether any trades occurred"""
    sum_profit: str
    """Total profit/loss"""
    sum_profit_rate: str
    """Total profit/loss rate"""
    profits: ProfitSummaryBreakdown
    """Per-asset-type breakdown"""


class ProfitAnalysisItem:
    """P&L for one security."""

    name: str
    """Security name"""
    market: str
    """Market"""
    is_holding: bool
    """Whether still holding"""
    profit: str
    """Profit/loss amount"""
    profit_rate: str
    """Profit/loss rate"""
    clearance_times: int
    """Number of completed trades"""
    item_type: AssetType
    """Asset type"""
    currency: str
    """Currency"""
    symbol: str
    """Security symbol"""
    holding_period: str
    """Holding period display string"""
    security_code: str
    """Ticker code"""
    isin: str
    """ISIN (for funds)"""
    underlying_profit: str
    """Underlying stock P&L"""
    derivatives_profit: str
    """Derivatives P&L"""
    order_profit: str
    """P&L in order currency"""


class ProfitAnalysisSublist:
    """Per-security P&L breakdown."""

    start: str
    """Start time (unix timestamp string)"""
    end: str
    """End time (unix timestamp string)"""
    start_date: str
    """Start date string"""
    end_date: str
    """End date string"""
    updated_at: str
    """Last updated time"""
    updated_date: str
    """Last updated date"""
    items: list[ProfitAnalysisItem]
    """Per-security items"""


class ProfitAnalysis:
    """Combined portfolio P&L analysis response."""

    summary: ProfitAnalysisSummary
    """Account-level summary"""
    sublist: ProfitAnalysisSublist
    """Per-security breakdown"""


class ProfitDetailEntry:
    """One P&L detail line item."""

    describe: str
    """Description"""
    amount: str
    """Amount"""


class ProfitDetails:
    """Detailed P&L breakdown for one asset class."""

    holding_value: str
    """Current holding market value"""
    profit: str
    """Total profit/loss"""
    cumulative_credited_amount: str
    """Cumulative credited amount"""
    credited_details: list[ProfitDetailEntry]
    """Credit detail entries"""
    cumulative_debited_amount: str
    """Cumulative debited amount"""
    debited_details: list[ProfitDetailEntry]
    """Debit detail entries"""
    cumulative_fee_amount: str
    """Cumulative fee amount"""
    fee_details: list[ProfitDetailEntry]
    """Fee detail entries"""
    short_holding_value: str
    """Short position holding value"""
    long_holding_value: str
    """Long position holding value"""
    holding_value_at_beginning: str
    """Opening position market value at period start"""
    holding_value_at_ending: str
    """Closing position market value at period end"""


class ProfitAnalysisDetail:
    """P&L detail for one security."""

    profit: str
    """Total profit/loss"""
    underlying_details: ProfitDetails
    """Underlying stock P&L details"""
    derivative_pnl_details: ProfitDetails
    """Derivative P&L details"""
    name: str
    """Security name"""
    updated_at: str
    """Last updated time"""
    updated_date: str
    """Last updated date"""
    currency: str
    """Currency"""
    default_tag: int
    """Default detail tab: 0=underlying, 1=derivative"""
    start: int
    """Query start time (unix timestamp)"""
    end: int
    """Query end time (unix timestamp)"""
    start_date: str
    """Query start date"""
    end_date: str
    """Query end date"""


class PortfolioContext:
    """
    Portfolio analytics context.

    Examples:
        ::

            from longbridge.openapi import Config, PortfolioContext

            config = Config.from_env()
            ctx = PortfolioContext(config)
            rates = ctx.exchange_rate()
            for r in rates.exchanges:
                print(r.base_currency, r.other_currency, r.average_rate)
    """

    def __init__(self, config: "Config") -> None:
        """Create a PortfolioContext."""
        ...

    def exchange_rate(self) -> "ExchangeRates":
        """Get exchange rates for supported currencies."""
        ...

    def profit_analysis(
        self,
        start: str | None = None,
        end: str | None = None,
    ) -> "ProfitAnalysis":
        """
        Get portfolio P&L analysis (summary + per-security breakdown).

        Args:
            start: Optional start date in ``YYYY-MM-DD`` format
            end: Optional end date in ``YYYY-MM-DD`` format
        """
        ...

    def profit_analysis_detail(
        self,
        symbol: str,
        start: str | None = None,
        end: str | None = None,
    ) -> "ProfitAnalysisDetail":
        """
        Get P&L detail for a specific security.

        Args:
            symbol: Security symbol, e.g. ``"700.HK"``
            start: Optional start date
            end: Optional end date
        """
        ...

    def profit_analysis_by_market(
        self,
        page: int = 1,
        size: int = 20,
        market: str | None = None,
        start: str | None = None,
        end: str | None = None,
        currency: str | None = None,
    ) -> "ProfitAnalysisByMarket":
        """
        Get P&L grouped by market with per-security breakdown.

        Args:
            page: Page number (1-based, default 1)
            size: Page size (default 20)
            market: Optional market filter, e.g. ``"HK"`` or ``"US"``
            start: Optional start date in ``YYYY-MM-DD`` format
            end: Optional end date in ``YYYY-MM-DD`` format
            currency: Optional currency filter
        """
        ...

    def profit_analysis_flows(
        self,
        symbol: str,
        page: int,
        size: int,
        derivative: bool,
        start: str | None = None,
        end: str | None = None,
    ) -> "ProfitAnalysisFlows":
        """
        Get paginated P&L flow records for a security.

        Args:
            symbol: Security symbol, e.g. ``"700.HK"``
            page: Page number (1-based)
            size: Page size
            derivative: Whether to include derivative flows
            start: Optional start date in ``YYYY-MM-DD`` format
            end: Optional end date in ``YYYY-MM-DD`` format
        """
        ...


class ProfitAnalysisByMarketItem:
    """One security entry in a by-market P&L response."""

    code: str
    """Security symbol (ticker code)"""
    name: str
    """Security name"""
    market: str
    """Market, e.g. ``"HK"`` or ``"US"``"""
    profit: str
    """Profit/loss amount"""


class ProfitAnalysisByMarket:
    """Response for :meth:`PortfolioContext.profit_analysis_by_market`."""

    profit: str
    """Total P&L across all returned items"""
    has_more: bool
    """Whether more pages are available"""
    stock_items: list[ProfitAnalysisByMarketItem]
    """Per-security P&L items"""


class FlowItem:
    """One profit-analysis flow record."""

    executed_date: str
    """Execution date string, e.g. ``"2024-01-15"``"""
    executed_timestamp: str
    """Execution timestamp (string representation)"""
    code: str
    """Security code / ticker"""
    direction: FlowDirection
    """Direction of the flow"""
    executed_quantity: str
    """Executed quantity"""
    executed_price: str
    """Executed price"""
    executed_cost: str
    """Executed cost"""
    describe: str
    """Human-readable description"""


class ProfitAnalysisFlows:
    """Response for :meth:`PortfolioContext.profit_analysis_flows`."""

    flows_list: list[FlowItem]
    """Paginated list of flow items"""
    has_more: bool
    """Whether there are more pages"""


# ── AlertContext ──────────────────────────────────────────────────

class AlertItem:
    """One price alert."""

    id: str
    """Alert ID"""
    indicator_id: str
    """Condition: ``"1"``=price_rise, ``"2"``=price_fall, ``"3"``=pct_rise, ``"4"``=pct_fall"""
    enabled: bool
    """Whether the alert is active"""
    frequency: int
    """Frequency: 1=daily, 2=every_time, 3=once"""
    scope: int
    """Scope"""
    text: str
    """Display text, e.g. ``"价格涨到 600"``"""
    state: list[int]
    """Trigger state flags"""


class AlertSymbolGroup:
    """Alert items for one security."""

    symbol: str
    """Security symbol"""
    code: str
    """Ticker code (without market)"""
    market: str
    """Market, e.g. ``"HK"``"""
    name: str
    """Security name"""
    price: str
    """Latest price"""
    chg: str
    """Day change amount"""
    p_chg: str
    """Day change percentage"""
    product: str
    """Product type"""
    indicators: list[AlertItem]
    """Alert items"""


class AlertList:
    """Alert list response."""

    lists: list[AlertSymbolGroup]
    """Alert groups per security"""


class AlertCondition:
    """Alert trigger condition."""

    class PriceRise(AlertCondition): ...
    """Price rises above threshold"""
    class PriceFall(AlertCondition): ...
    """Price falls below threshold"""
    class PercentRise(AlertCondition): ...
    """Percentage rises above threshold"""
    class PercentFall(AlertCondition): ...
    """Percentage falls below threshold"""


class AlertFrequency:
    """Alert trigger frequency."""

    class Daily(AlertFrequency): ...
    """Trigger once per day"""
    class EveryTime(AlertFrequency): ...
    """Trigger every time condition is met"""
    class Once(AlertFrequency): ...
    """Trigger only once"""


class AlertContext:
    """
    Price alert management context.

    Examples:
        ::

            from longbridge.openapi import Config, AlertContext, AlertCondition, AlertFrequency

            config = Config.from_env()
            ctx = AlertContext(config)

            ctx.add("700.HK", AlertCondition.PriceRise, "600", AlertFrequency.Once)
            alerts = ctx.list()
            for group in alerts.lists:
                print(group.symbol, len(group.indicators), "alerts")
    """

    def __init__(self, config: "Config") -> None:
        """Create an AlertContext."""
        ...

    def list(self) -> "AlertList":
        """List all price alerts."""
        ...

    def add(
        self,
        symbol: str,
        condition: "AlertCondition",
        trigger_value: str,
        frequency: "AlertFrequency",
    ) -> None:
        """
        Add a price alert.

        Args:
            symbol: Security symbol
            condition: Trigger condition
            trigger_value: Threshold value, e.g. ``"600"`` (price) or ``"5"`` (percentage)
            frequency: How often to trigger
        """
        ...

    def enable(self, alert_id: str) -> None:
        """Enable a price alert."""
        ...

    def disable(self, alert_id: str) -> None:
        """Disable a price alert."""
        ...

    def delete(self, alert_ids: list[str]) -> None:
        """Delete price alerts."""
        ...


# ── DCAContext ────────────────────────────────────────────────────

class DcaPlan:
    """One DCA (dollar-cost averaging) investment plan."""

    plan_id: str
    """Plan ID"""
    status: DCAStatus
    """Plan status"""
    symbol: str
    """Security symbol"""
    member_id: str
    """Member ID"""
    aaid: str
    """Account ID"""
    account_channel: str
    """Account channel"""
    display_account: str
    """Display account"""
    market: Market
    """Market"""
    per_invest_amount: str
    """Investment amount per period"""
    invest_frequency: DCAFrequency
    """Investment frequency"""
    invest_day_of_week: str
    """Day of week for weekly plans"""
    invest_day_of_month: str
    """Day of month for monthly plans"""
    allow_margin_finance: bool
    """Whether margin finance is allowed"""
    alter_hours: str
    """Reminder time"""
    created_at: str
    """Creation time"""
    updated_at: str
    """Last updated time"""
    next_trd_date: str
    """Next investment date"""
    stock_name: str
    """Security name"""
    cum_amount: str
    """Cumulative invested amount"""
    issue_number: int
    """Number of completed investment periods"""
    average_cost: str
    """Average cost"""
    cum_profit: str
    """Cumulative profit/loss"""


class DcaList:
    """DCA plan list response."""

    plans: list[DcaPlan]
    """DCA plans"""


class DcaStats:
    """DCA statistics response."""

    active_count: str
    """Number of active plans"""
    finished_count: str
    """Number of finished plans"""
    suspended_count: str
    """Number of suspended plans"""
    nearest_plans: list[DcaPlan]
    """Nearest upcoming plans"""
    rest_days: str
    """Days until next investment"""
    total_amount: str
    """Total invested amount"""
    total_profit: str
    """Total profit/loss"""


class DcaSupportInfo:
    """DCA support info for one security."""

    symbol: str
    """Security symbol"""
    support_regular_saving: bool
    """Whether DCA is supported for this security"""


class DcaSupportList:
    """DCA support check response."""

    infos: list[DcaSupportInfo]
    """Support info per security"""


class DcaHistoryRecord:
    """One DCA execution record."""

    created_at: str
    """Execution time"""
    order_id: str
    """Associated order ID"""
    status: str
    """Status"""
    action: str
    """Action type"""
    order_type: str
    """Order type"""
    executed_qty: str
    """Executed quantity"""
    executed_price: str
    """Executed price"""
    executed_amount: str
    """Executed amount"""
    rejected_reason: str
    """Rejection reason (if any)"""
    symbol: str
    """Security symbol"""


class DcaHistoryResponse:
    """DCA execution history response."""

    records: list[DcaHistoryRecord]
    """Execution history records"""
    has_more: bool
    """Whether more records exist"""


class DcaCalcDateResult:
    """Result for :meth:`DCAContext.calc_date`."""

    trade_date: str
    """Next projected trade date (unix timestamp string)"""


class DCAFrequency:
    """DCA investment frequency."""

    class Daily(DCAFrequency): ...
    """Daily investment"""
    class Weekly(DCAFrequency): ...
    """Weekly investment"""
    class Fortnightly(DCAFrequency): ...
    """Fortnightly (every two weeks) investment"""
    class Monthly(DCAFrequency): ...
    """Monthly investment"""


class DCAStatus:
    """DCA plan status."""

    class Active(DCAStatus): ...
    """Active plan"""
    class Suspended(DCAStatus): ...
    """Suspended plan"""
    class Finished(DCAStatus): ...
    """Permanently finished plan"""


class DCAContext:
    """
    Dollar-cost averaging (DCA) plan management context.

    Examples:
        ::

            from longbridge.openapi import Config, DCAContext, DCAFrequency

            config = Config.from_env()
            ctx = DCAContext(config)

            # Check support
            support = ctx.check_support(["AAPL.US", "700.HK"])
            for info in support.infos:
                print(info.symbol, info.support_regular_saving)

            # Get stats
            stats = ctx.stats()
            print("Active plans:", stats.active_count)
    """

    def __init__(self, config: "Config") -> None:
        """Create a DCAContext."""
        ...

    def list(
        self,
        status: "DCAStatus | None" = None,
        symbol: str | None = None,
    ) -> "DcaList":
        """
        List DCA plans.

        Args:
            status: Filter by plan status (``None`` = all)
            symbol: Filter by security symbol
        """
        ...

    def create(
        self,
        symbol: str,
        amount: str,
        frequency: "DCAFrequency",
        day_of_week: str | None = None,
        day_of_month: int | None = None,
        allow_margin: bool = False,
    ) -> "DcaList":
        """
        Create a new DCA plan.

        Args:
            symbol: Security symbol
            amount: Investment amount per period
            frequency: Investment frequency
            day_of_week: Day of week for weekly plans, e.g. ``"Mon"``
            day_of_month: Day of month for monthly plans (1–28)
            allow_margin: Whether to allow margin finance
        """
        ...

    def pause(self, plan_id: str) -> "DcaList":
        """Pause (suspend) a DCA plan."""
        ...

    def resume(self, plan_id: str) -> "DcaList":
        """Resume a suspended DCA plan."""
        ...

    def stop(self, plan_id: str) -> "DcaList":
        """Permanently stop a DCA plan."""
        ...

    def history(
        self,
        plan_id: str,
        page: int = 1,
        limit: int = 20,
    ) -> "DcaHistoryResponse":
        """
        Get execution history for a DCA plan.

        Args:
            plan_id: Plan ID
            page: Page number (1-based)
            limit: Results per page
        """
        ...

    def stats(self, symbol: str | None = None) -> "DcaStats":
        """
        Get DCA statistics.

        Args:
            symbol: Optional security filter
        """
        ...

    def check_support(self, symbols: list[str]) -> "DcaSupportList":
        """
        Check DCA support for a list of securities.

        Args:
            symbols: List of security symbols
        """
        ...

    def calc_date(
        self,
        symbol: str,
        frequency: "DCAFrequency",
        day_of_week: str | None = None,
        day_of_month: int | None = None,
    ) -> "DcaCalcDateResult":
        """
        Calculate the next projected trade date for a DCA plan.

        Args:
            symbol: Security symbol, e.g. ``"700.HK"``
            frequency: Investment frequency
            day_of_week: Day of week for weekly/fortnightly plans, e.g. ``"Mon"``
            day_of_month: Day of month for monthly plans (1–28)
        """
        ...

    def set_reminder(self, hours: str) -> None:
        """
        Update the advance reminder time for DCA plans.

        Args:
            hours: Reminder advance hours; must be ``"1"``, ``"6"``, or ``"12"``
        """
        ...


# ── SharelistContext ──────────────────────────────────────────────

class SharelistStock:
    """A stock in a community sharelist."""

    symbol: str
    """Security symbol"""
    name: str
    """Security name"""
    market: str
    """Market, e.g. ``"HK"``"""
    code: str
    """Ticker code"""
    intro: str
    """Brief description"""
    unread_change_log_category: str
    """Unread change log category"""
    change: str | None
    """Day change percentage"""
    last_done: str | None
    """Latest price"""
    trade_status: int | None
    """Trade status code"""
    latency: bool | None
    """Whether delayed quote"""


class SharelistScopes:
    """Sharelist subscription scopes."""

    subscription: bool
    """Whether the current user is subscribed"""
    is_self: bool
    """Whether the current user is the creator"""


class SharelistInfo:
    """Sharelist information."""

    id: int
    """Sharelist ID"""
    name: str
    """Name"""
    description: str
    """Description"""
    cover: str
    """Cover image URL"""
    subscribers_count: int
    """Number of subscribers"""
    this_year_chg: str
    """YTD change percentage"""
    stocks: list[SharelistStock]
    """Constituent stocks"""
    subscribed: bool
    """Whether the current user is subscribed"""
    chg: str
    """Day change percentage"""
    sharelist_type: int
    """Type: 0=regular, 3=official, 4=industry"""
    industry_code: str
    """Industry code (for industry sharelists)"""


class SharelistList:
    """Sharelist list response."""

    sharelists: list[SharelistInfo]
    """User's own and followed sharelists"""
    subscribed_sharelists: list[SharelistInfo]
    """Subscribed sharelists"""
    tail_mark: str
    """Pagination cursor for subscribed list"""


class SharelistDetail:
    """Sharelist detail response."""

    sharelist: SharelistInfo
    """Sharelist info"""
    scopes: SharelistScopes
    """Subscription scopes"""


class SharelistContext:
    """
    Community sharelist management context.

    Examples:
        ::

            from longbridge.openapi import Config, SharelistContext

            config = Config.from_env()
            ctx = SharelistContext(config)

            lists = ctx.list(20)
            for sl in lists.sharelists:
                print(sl.name, len(sl.stocks), "stocks")
    """

    def __init__(self, config: "Config") -> None:
        """Create a SharelistContext."""
        ...

    def list(self, count: int = 20) -> "SharelistList":
        """
        List user's own and subscribed sharelists.

        Args:
            count: Maximum number of results
        """
        ...

    def detail(self, id: int) -> "SharelistDetail":
        """
        Get sharelist detail with constituent stocks.

        Args:
            id: Sharelist ID
        """
        ...

    def popular(self, count: int = 20) -> "SharelistList":
        """
        Get popular / trending sharelists.

        Args:
            count: Maximum number of results
        """
        ...

    def create(self, name: str, description: str | None = None) -> "SharelistDetail":
        """
        Create a new community sharelist.

        Args:
            name: Sharelist name
            description: Optional description
        """
        ...

    def delete(self, id: int) -> None:
        """
        Delete a sharelist.

        Args:
            id: Sharelist ID
        """
        ...

    def add_securities(self, id: int, symbols: list[str]) -> None:
        """
        Add securities to a sharelist.

        Args:
            id: Sharelist ID
            symbols: Security symbols to add
        """
        ...

    def remove_securities(self, id: int, symbols: list[str]) -> None:
        """
        Remove securities from a sharelist.

        Args:
            id: Sharelist ID
            symbols: Security symbols to remove
        """
        ...

    def sort_securities(self, id: int, symbols: list[str]) -> None:
        """
        Reorder securities in a sharelist.

        Args:
            id: Sharelist ID
            symbols: Security symbols in desired order
        """
        ...


# ── QuoteContext extensions ───────────────────────────────────────

class ShortPositionsItem:
    """One short-position data point (unified for US and HK markets)."""

    timestamp: str
    """Trading date (RFC 3339)"""
    rate: str
    """Short ratio (both markets)"""
    close: str
    """Closing price (both markets)"""
    current_shares_short: str
    """[US] Number of short shares outstanding"""
    avg_daily_share_volume: str
    """[US] Average daily share volume"""
    days_to_cover: str
    """[US] Days to cover ratio"""
    amount: str
    """[HK] Short sale amount (HKD)"""
    balance: str
    """[HK] Short position balance"""
    cost: str
    """[HK] Cost / closing price"""


class ShortPositionsResponse:
    """Short interest / positions response (HK or US)."""

    data: List[ShortPositionsItem]
    """Short position data points"""


class ShortTradesItem:
    """One short-trade data point (unified for US and HK markets)."""

    timestamp: str
    """Trading date (RFC 3339)"""
    rate: str
    """Short ratio"""
    close: str
    """Closing price"""
    nus_amount: str
    """[US] NYSE short amount"""
    ny_amount: str
    """[US] NY short amount"""
    total_amount: str
    """[US] Total short amount"""
    amount: str
    """[HK] Short sale amount"""
    balance: str
    """[HK] Short position balance"""


class ShortTradesResponse:
    """Short trade records response (HK or US)."""

    data: List[ShortTradesItem]
    """Short trade data points"""


class OptionVolumeStats:
    """Real-time option call/put volume response."""

    c: str
    """Total call volume"""
    p: str
    """Total put volume"""


class OptionVolumeDailyStat:
    """One day's option volume statistics."""

    symbol: str
    """Underlying security symbol"""
    timestamp: str
    """Settlement date (unix timestamp string)"""
    total_volume: int
    """Total option volume (calls + puts)"""
    total_put_volume: int
    """Total put volume"""
    total_call_volume: int
    """Total call volume"""
    put_call_volume_ratio: str
    """Put/call volume ratio"""
    total_open_interest: int
    """Total open interest"""
    total_put_open_interest: int
    """Total put open interest"""
    total_call_open_interest: int
    """Total call open interest"""
    put_call_open_interest_ratio: str
    """Put/call open interest ratio"""


class OptionVolumeDaily:
    """Daily historical option volume response."""

    stats: list[OptionVolumeDailyStat]
    """Daily option volume statistics"""


# ── US-market API types ────────────────────────────────────────────

class USCryptoOverview:
    """US cryptocurrency market overview. Returned by QuoteContext.us_crypto_overview."""

    symbol: str
    """User-facing trading-pair symbol, e.g. ``"BTCUSD.BKKT"``"""
    name: str
    """Cryptocurrency full name"""
    ticker: str
    """Ticker / pair code, e.g. ``"BTCUSD"``"""
    base_asset: str
    """Base asset code, e.g. ``"BTC"``"""
    currency: str
    """Quote currency"""
    all_time_high: str
    """All-time high price"""
    all_time_high_date: str
    """All-time high date"""
    all_time_low: str
    """All-time low price"""
    all_time_low_date: str
    """All-time low date"""
    ipo_date: str
    """IPO / genesis date"""
    issue_price: str
    """Issue price"""
    shares: str
    """Total supply"""
    official_web_address: str
    """Official website URL"""
    logo: str
    """Logo image URL"""
    wiki_url: str
    """In-app wiki URL"""
    profile: str
    """Extended profile as JSON string"""


class USReportPeriod:
    """One reporting period window."""
    start_date: str
    end_date: str
    report_txt: str

class USFinancialISItem:
    """One income-statement entry."""
    revenue: str
    net_income: str
    net_margin: str
    report: "USReportPeriod"

class USFinancialBSItem:
    """One balance-sheet entry."""
    debt_assets_ratio: str
    total_assets: str
    total_liabilities: str
    report: "USReportPeriod"

class USFinancialCFItem:
    """One cash-flow entry."""
    operating: str
    investing: str
    financing: str
    report: "USReportPeriod"

class USFinancialOverview:
    """US financial overview (IS + BS + CF by period)."""
    ccy_symbol: str
    report_type: str
    is_list: List["USFinancialISItem"]
    bs_list: List["USFinancialBSItem"]
    cf_list: List["USFinancialCFItem"]

class USKeyMetricItem:
    """One period entry in key financial metrics."""
    ff_period: str
    ff_year: int
    fp_end: str
    report_txt: str
    rpt_date: str
    fields: List[Any]

class USKeyFinancialMetrics:
    """US key financial metrics (ROE, margins, debt ratio) by period."""
    currency: str
    report: str
    empty_fields: List[str]
    list: List["USKeyMetricItem"]

class USAIChatData:
    """AI chat context in analyst consensus."""
    agent_id: str
    handoff_agent_id: str
    symbol: str
    text: str
    chat_type: str
    workflow_type: str

class USConsensusEstimate:
    """Actual vs estimated value for one consensus metric."""
    actual: str
    estimate: str


class USConsensusItem:
    """One fiscal-year entry in USAnalystConsensus.list."""
    ebit: "USConsensusEstimate"
    eps: "USConsensusEstimate"
    fiscal_year: int
    report_txt: str
    revenue: "USConsensusEstimate"


class USAnalystConsensus:
    """US analyst consensus estimates and AI analysis."""
    ai_summary: str
    aichat_data: "USAIChatData"
    currency: str
    report: str
    list: List["USConsensusItem"]
    opt_reports: List[str]
    h5_data: Any


class USCashEntry:
    """One cash currency entry in USAssetOverview."""

    currency: str
    """Currency code, e.g. ``"USD"``"""
    frozen_buy_cash: str
    """Cash frozen for pending buy orders"""
    outstanding: str
    """Unsettled cash (positive = receivable, negative = payable)"""
    settled_cash: str
    """Settled cash balance"""
    total_amount: str
    """Total cash amount"""
    total_cash: str
    """Total available cash"""


class USCryptoEntry:
    """One cryptocurrency holding in USAssetOverview."""

    asset_type: str
    """Asset type, e.g. ``"CRYPTO"``"""
    average_cost: str
    """Average cost price"""
    symbol: str
    """Internal counter_id, e.g. ``"VA/BKKT/BTCUSD"``"""
    currency: str
    """Settlement currency"""
    industry_counter_id: str
    """Industry counter_id"""
    industry_name: str
    """Industry name"""


class USStockEntry:
    """One stock/equity position in USAssetOverview."""

    symbol: str
    """Ticker code, e.g. ``"AAPL"``"""
    full_symbol: str
    """Qualified symbol, e.g. ``"AAPL.US"``"""
    asset_type: str
    quantity: str
    currency: str
    average_cost: str
    market: str
    trade_status: str
    prev_close: str
    last_done: str
    market_price: str
    pretrade_close: str
    stock_invest_of_today: str
    today_pl: str
    pretrade_stock_invest_of_today: str
    pretrade_today_pl: str
    night_last_done: str
    night_prev_close: str
    position_side: str
    open_position_time: str
    name: str
    industry_counter_id: str
    industry_name: str


class USAssetOverview:
    """US account asset snapshot. Returned by TradeContext.us_asset_overview."""

    account_type: str
    """Account type code"""
    asset_timestamp: int
    """Snapshot timestamp (Unix seconds)"""
    cash_buy_power: str
    """Available cash buying power"""
    overnight_buy_power: str
    """Overnight buying power"""
    currency: str
    """Account currency"""
    cash_list: List[USCashEntry]
    """Cash balances by currency"""
    stock_list: List["USStockEntry"]
    """Stock/equity positions"""
    option_list: List[Any]
    """Option positions"""
    crypto_list: List[USCryptoEntry]
    """Cryptocurrency positions"""


class USRealizedPLMetric:
    """One time-period metric within a USRealizedPLEntry."""

    amount: str
    """Realized P&L amount"""
    period: int
    """Period code (server-defined)"""
    rate: str
    """Realized P&L rate"""


class USRealizedPLEntry:
    """One asset-category entry in USRealizedPL. category: 0=all, 1=stock, 2=option, 3=crypto."""

    category: int
    """Asset category code"""
    currency: str
    """Currency"""
    metrics: List[USRealizedPLMetric]
    """Time-period metrics"""


class USRealizedPL:
    """Realized P&L response for a US account. Returned by TradeContext.us_realized_pl."""

    realized_pl_list: List[USRealizedPLEntry]
    """Per-category realized P&L entries"""


class USOrderHistory:
    """One order state-transition entry within USOrderDetail."""
    exec_type: int
    status: str
    price: str
    qty: str
    time: str
    msg: str
    is_manually: bool
    opp_party_id: str
    trd_match_id: str
    operator: str
    op_entrust_way: str
    cxl_rej_response_to: int
    withdrawal_reason: str
    opp_name: str
    exec_id: str


class USButtonControl:
    """Action-button state for an order."""
    withdraw: int
    replace: int
    exceptionable: List[str]


class USChargeItem:
    """One fee category within USChargeDetail."""
    code: int
    name: str
    fees: List[str]


class USChargeDetail:
    """Fee breakdown for an order."""
    currency: str
    total_amount: str
    items: List["USChargeItem"]


class USAttachedOrder:
    """One bracket/conditional sub-order attached to a main order."""
    attached_type_display: int
    executed_qty: str
    quantity: str
    status: str
    trigger_price: str
    order_id: str
    gtd: str
    time_in_force: int
    tag: int
    activate_order_type: str
    activate_rth: int
    submit_price: str
    symbol: str
    withdrawn: bool


class USOrderDetail:
    """Full typed order object within USOrderDetailResponse."""
    id: str
    aaid: str
    account_channel: str
    action: int
    symbol: str
    underlying_symbol: str
    security_type: str
    name: str
    currency: str
    trade_currency: str
    order_type: str
    status: str
    price: str
    quantity: str
    executed_qty: str
    executed_price: str
    executed_amount: str
    operate_direction: str
    time_in_force: int
    gtd: str
    tag: int
    msg: str
    force_only_rth: int
    submitted_at: str
    done_at: str
    trigger_price: str
    trigger_at: str
    trigger_status: int
    trigger_exchange: str
    trigger_last_done: str
    trigger_count: int
    tailing_amount: str
    tailing_percent: str
    limit_offset: str
    limit_depth_level: int
    market_price: str
    submitted_amount: str
    estimated_fee: str
    free_status: int
    free_amount: str
    free_currency: str
    deductions_status: int
    deductions_amount: str
    deductions_currency: str
    platform_deductions_status: int
    platform_deductions_amount: str
    platform_deductions_currency: str
    display_account: str
    settlement_account: str
    settlement_channel: str
    customer_name: str
    real_name: str
    en_name: str
    joint_real_name: str
    joint_en_name: str
    org_id: str
    bcan: str
    op_entrust_way: int
    op_entrust_way_name: str
    remark: str
    notice: str
    short_sell_type: int
    ploy_type: str
    ploy_id: str
    ploy_status: str
    trend: int
    withdrawal_reason: str
    activate_order_type: str
    activate_rth: int
    submit_price: str
    contract_direction: str
    strike_price: str
    contract_size: str
    monitor_price: str
    button_control: "USButtonControl"
    charge_detail: Optional["USChargeDetail"]
    attached_orders: List["USAttachedOrder"]
    order_histories: List["USOrderHistory"]


class USOrderDetailResponse:
    """Response for TradeContext.us_order_detail."""
    order: Optional["USOrderDetail"]
    current_attached_order: Optional["USOrderDetail"]
    current_millisecond: str


class USRankTag:
    """A rank tag entry for a US company overview."""

    key: str
    location: int
    title: str
    text: str
    rank_type: int
    highlight_text: str


class USSharelistItem:
    """One entry in USCompanyOverview.share_list."""
    chg: str
    id: str
    name: str


class USCompanyOverview:
    """US company overview. Returned by FundamentalContext.us_company_overview."""

    intro: str
    market_cap: str
    ccy_symbol: str
    top_rank_tags: List[USRankTag]
    detail_url: str
    share_list: List["USSharelistItem"]


class USValuationMetric:
    """One valuation metric in USValuationOverview.metrics (e.g. key="pe")."""

    circle: str
    part: str
    metric: str
    desc: str
    industry_median: str


class USValuationOverview:
    """US valuation overview. Returned by FundamentalContext.us_valuation_overview."""

    metrics: Dict[str, "USValuationMetric"]
    """Map of metric key (e.g. ``"pe"``) to :class:`USValuationMetric`"""
    indicator: str
    range: int
    date: str
    ccy_symbol: str
    aichat_data: "USAIChatData"
    ai_summary: str


class USFinancialStatementField:
    """One financial line item within a USFinancialStatementPeriod."""

    display_order: int
    field: str
    id: str
    level: int
    name: str
    value: str
    value_type: str
    yoy: str


class USFinancialStatementPeriod:
    """One reporting period in USFinancialStatement."""

    ff_period: str
    ff_year: int
    fields: List["USFinancialStatementField"]
    fp_end: str
    report_txt: str
    rpt_date: str


class USFinancialStatement:
    """US financial statement (IS/BS/CF). Returned by FundamentalContext.us_financial_statement."""

    currency: str
    report: str
    list: List["USFinancialStatementPeriod"]
    empty_fields: List[str]
    """Currency"""


class USFiscalYearDividend:
    """Per-fiscal-year dividend row for a US ETF."""

    dividend: str
    dividend_yield: str
    fiscal_year: str
    currency: str
    fiscal_year_range: str


class USETFDividendInfo:
    """US ETF dividend history. Returned by FundamentalContext.us_etf_dividend_info."""

    dividend_ttm: str
    """TTM dividend per share"""
    dividend_yield_ttm: str
    """TTM dividend yield"""
    dividend_frequency: str
    """Distribution frequency"""
    currency: str
    """Currency"""
    fiscal_year_info: List["USFiscalYearDividend"]
    """Per-fiscal-year dividend records"""


class USDividendItem:
    """A single US dividend payment record."""

    dividend: str
    """Dividend per share"""
    dividend_type: str
    """Dividend type"""
    ex_date: str
    """Ex-dividend date"""
    payment_date: str
    """Payment date"""
    record_date: str
    """Record date"""


class USRecentDividend:
    """Trailing-12-month dividend summary."""

    dividend_ttm: str
    dividend_yield_ttm: str
    payouts: str
    currency: str


class USDividendHistoryItem:
    """One fiscal-year row in dividend_history or payout_ratios."""

    fiscal_year: str
    fiscal_year_range: str
    total_shareholder_yield: str
    dividend: str
    dividend_yield: str
    dividend_growth_rate: str
    dividend_payout_ratio: str
    dividend_to_cashflow_ratio: str
    net_buyback: str
    net_buyback_yield: str
    net_buyback_growth_rate: str
    net_buyback_payout_ratio: str
    net_buyback_to_cashflow_ratio: str
    currency: str


class USDividendPayoutRecord:
    """One actual dividend payment event."""

    dividend: str
    dividend_type: str
    currency: str
    ex_date: str
    payment_date: str
    record_date: str
    title: str
    start_time_unix: str


class USCompanyDividends:
    """US company historical dividends. Returned by FundamentalContext.us_company_dividends."""

    recent_dividends: "USRecentDividend"
    dividend_history: List["USDividendHistoryItem"]
    payout_ratios: List["USDividendHistoryItem"]
    dividend_payout_history: List["USDividendPayoutRecord"]


class USETFFile:
    """A single ETF document entry."""

    file_name: str
    file_path: str
    update_date: str
    code: str
    format: str


class USETFFilesResponse:
    """US ETF document list. Returned by FundamentalContext.us_etf_files."""

    files: List[USETFFile]
    """Document entries"""
