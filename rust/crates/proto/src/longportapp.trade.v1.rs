/// Sub is Sub command content, command is 16
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sub {
    #[prost(string, repeated, tag="1")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// SubResponse is response of Sub Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubResponse {
    /// 订阅成功
    #[prost(string, repeated, tag="1")]
    pub success: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 订阅失败
    #[prost(message, repeated, tag="2")]
    pub fail: ::prost::alloc::vec::Vec<sub_response::Fail>,
    /// 当前订阅
    #[prost(string, repeated, tag="3")]
    pub current: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SubResponse`.
pub mod sub_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fail {
        #[prost(string, tag="1")]
        pub topic: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub reason: ::prost::alloc::string::String,
    }
}
/// Unsub is Unsub command content, command is 17
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unsub {
    #[prost(string, repeated, tag="1")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// UnsubResponse is response of Unsub request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubResponse {
    /// 当前订阅
    #[prost(string, repeated, tag="3")]
    pub current: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Notification is push message, command is 18
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(enumeration="ContentType", tag="2")]
    pub content_type: i32,
    #[prost(enumeration="DispatchType", tag="3")]
    pub dispatch_type: i32,
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// trade gateway command, see: <https://open.longbridgeapp.com/docs/trade/trade-push>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Command {
    CmdUnknown = 0,
    CmdSub = 16,
    CmdUnsub = 17,
    CmdNotify = 18,
}
/// Dispatch type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DispatchType {
    DispatchUndefined = 0,
    DispatchDirect = 1,
    DispatchBroadcast = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    ContentUndefined = 0,
    ContentJson = 1,
    ContentProto = 2,
}
