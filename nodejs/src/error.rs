pub(crate) struct ErrorNewType(pub(crate) longport::Error);

impl std::convert::From<ErrorNewType> for napi::Error {
    #[inline]
    fn from(err: ErrorNewType) -> napi::Error {
        napi::Error::from_reason(err.0.to_string())
    }
}
