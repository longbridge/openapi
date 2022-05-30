const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      return readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'longbridge.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.android-arm64.node')
          } else {
            nativeBinding = require('longbridge-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'longbridge.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.android-arm-eabi.node')
          } else {
            nativeBinding = require('longbridge-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'longbridge.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.win32-x64-msvc.node')
          } else {
            nativeBinding = require('longbridge-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'longbridge.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('longbridge-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'longbridge.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('longbridge-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'longbridge.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.darwin-x64.node')
          } else {
            nativeBinding = require('longbridge-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'longbridge.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.darwin-arm64.node')
          } else {
            nativeBinding = require('longbridge-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'longbridge.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./longbridge.freebsd-x64.node')
      } else {
        nativeBinding = require('longbridge-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'longbridge.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./longbridge.linux-x64-musl.node')
            } else {
              nativeBinding = require('longbridge-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'longbridge.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./longbridge.linux-x64-gnu.node')
            } else {
              nativeBinding = require('longbridge-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'longbridge.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./longbridge.linux-arm64-musl.node')
            } else {
              nativeBinding = require('longbridge-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'longbridge.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./longbridge.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('longbridge-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'longbridge.linux-arm-gnueabihf.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./longbridge.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('longbridge-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { Config, Decimal, QuoteContext, PushQuoteEvent, PushDepthEvent, PushBrokersEvent, PushTradesEvent, DerivativeType, TradeStatus, TradeSession, SubType, TradeDirection, OptionType, OptionDirection, WarrantType, Period, AdjustType, SecurityStaticInfo, PrePostQuote, SecurityQuote, OptionQuote, WarrantQuote, Depth, SecurityDepth, Brokers, SecurityBrokers, ParticipantInfo, Trade, IntradayLine, Candlestick, StrikePriceInfo, IssuerInfo, TradingSessionInfo, MarketTradingSession, RealtimeQuote, PushQuote, PushDepth, PushBrokers, PushTrades, MarketTradingDays, NaiveDate, Time, sleep, TradeContext, GetCashFlowOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions, ReplaceOrderOptions, SubmitOrderOptions, TopicType, Execution, OrderStatus, OrderSide, OrderType, OrderTag, TimeInForceType, TriggerStatus, OutsideRTH, Order, PushOrderChanged, SubmitOrderResponse, CashInfo, AccountBalance, BalanceType, CashFlowDirection, CashFlow, FundPositionsResponse, FundPositionChannel, FundPosition, StockPositionsResponse, StockPositionChannel, StockPosition, Market } = nativeBinding

module.exports.Config = Config
module.exports.Decimal = Decimal
module.exports.QuoteContext = QuoteContext
module.exports.PushQuoteEvent = PushQuoteEvent
module.exports.PushDepthEvent = PushDepthEvent
module.exports.PushBrokersEvent = PushBrokersEvent
module.exports.PushTradesEvent = PushTradesEvent
module.exports.DerivativeType = DerivativeType
module.exports.TradeStatus = TradeStatus
module.exports.TradeSession = TradeSession
module.exports.SubType = SubType
module.exports.TradeDirection = TradeDirection
module.exports.OptionType = OptionType
module.exports.OptionDirection = OptionDirection
module.exports.WarrantType = WarrantType
module.exports.Period = Period
module.exports.AdjustType = AdjustType
module.exports.SecurityStaticInfo = SecurityStaticInfo
module.exports.PrePostQuote = PrePostQuote
module.exports.SecurityQuote = SecurityQuote
module.exports.OptionQuote = OptionQuote
module.exports.WarrantQuote = WarrantQuote
module.exports.Depth = Depth
module.exports.SecurityDepth = SecurityDepth
module.exports.Brokers = Brokers
module.exports.SecurityBrokers = SecurityBrokers
module.exports.ParticipantInfo = ParticipantInfo
module.exports.Trade = Trade
module.exports.IntradayLine = IntradayLine
module.exports.Candlestick = Candlestick
module.exports.StrikePriceInfo = StrikePriceInfo
module.exports.IssuerInfo = IssuerInfo
module.exports.TradingSessionInfo = TradingSessionInfo
module.exports.MarketTradingSession = MarketTradingSession
module.exports.RealtimeQuote = RealtimeQuote
module.exports.PushQuote = PushQuote
module.exports.PushDepth = PushDepth
module.exports.PushBrokers = PushBrokers
module.exports.PushTrades = PushTrades
module.exports.MarketTradingDays = MarketTradingDays
module.exports.NaiveDate = NaiveDate
module.exports.Time = Time
module.exports.sleep = sleep
module.exports.TradeContext = TradeContext
module.exports.GetCashFlowOptions = GetCashFlowOptions
module.exports.GetHistoryExecutionsOptions = GetHistoryExecutionsOptions
module.exports.GetHistoryOrdersOptions = GetHistoryOrdersOptions
module.exports.GetTodayExecutionsOptions = GetTodayExecutionsOptions
module.exports.GetTodayOrdersOptions = GetTodayOrdersOptions
module.exports.ReplaceOrderOptions = ReplaceOrderOptions
module.exports.SubmitOrderOptions = SubmitOrderOptions
module.exports.TopicType = TopicType
module.exports.Execution = Execution
module.exports.OrderStatus = OrderStatus
module.exports.OrderSide = OrderSide
module.exports.OrderType = OrderType
module.exports.OrderTag = OrderTag
module.exports.TimeInForceType = TimeInForceType
module.exports.TriggerStatus = TriggerStatus
module.exports.OutsideRTH = OutsideRTH
module.exports.Order = Order
module.exports.PushOrderChanged = PushOrderChanged
module.exports.SubmitOrderResponse = SubmitOrderResponse
module.exports.CashInfo = CashInfo
module.exports.AccountBalance = AccountBalance
module.exports.BalanceType = BalanceType
module.exports.CashFlowDirection = CashFlowDirection
module.exports.CashFlow = CashFlow
module.exports.FundPositionsResponse = FundPositionsResponse
module.exports.FundPositionChannel = FundPositionChannel
module.exports.FundPosition = FundPosition
module.exports.StockPositionsResponse = StockPositionsResponse
module.exports.StockPositionChannel = StockPositionChannel
module.exports.StockPosition = StockPosition
module.exports.QuoteContext = QuoteContext
module.exports.Market = Market
