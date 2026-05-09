use longbridge_nodejs_macros::JsEnum;

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::Market")]
pub enum Market {
    /// Unknown
    Unknown,
    /// US market
    US,
    /// HK market
    HK,
    /// CN market
    CN,
    /// SG market
    SG,
    /// Crypto market
    Crypto,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[js(remote = "longbridge::Language")]
pub enum Language {
    /// zh-CN
    ZH_CN,
    /// zh-HK
    ZH_HK,
    /// en
    EN,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq)]
#[js(remote = "longbridge::PushCandlestickMode")]
pub enum PushCandlestickMode {
    /// Realtime mode
    Realtime,
    /// Confirmed mode
    Confirmed,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::portfolio::types::FlowDirection")]
pub enum FlowDirection {
    /// Unknown
    Unknown,
    /// Buy
    Buy,
    /// Sell
    Sell,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::portfolio::types::AssetType")]
pub enum AssetType {
    /// Unknown
    Unknown,
    /// Stock
    Stock,
    /// Fund
    Fund,
    /// Crypto
    Crypto,
}

#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::fundamental::types::InstitutionRecommend")]
pub enum InstitutionRecommend {
    /// Unknown
    Unknown,
    /// Strong buy
    StrongBuy,
    /// Buy
    Buy,
    /// Hold
    Hold,
    /// Sell
    Sell,
    /// Strong sell
    StrongSell,
    /// Underperform
    Underperform,
    /// No opinion
    NoOpinion,
}
