use std::fmt::{self, Debug, Formatter};

use napi::{Error, Result};

#[napi_derive::napi]
#[derive(Copy, Clone)]
pub struct Decimal(pub(crate) rust_decimal::Decimal);

impl From<rust_decimal::Decimal> for Decimal {
    #[inline]
    fn from(value: rust_decimal::Decimal) -> Self {
        Self(value)
    }
}

impl Debug for Decimal {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[napi_derive::napi]
impl Decimal {
    #[napi(factory)]
    pub fn new(value: String) -> Result<Self> {
        Ok(Self(value.parse().map_err(|err| {
            Error::from_reason(format!("invalid decimal: {}", err))
        })?))
    }
}
