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
