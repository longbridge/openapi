use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
};

use napi::{Either, Error, Result};
use rust_decimal::{
    MathematicalOps,
    prelude::{FromPrimitive, ToPrimitive},
};
use serde_json::Value;

use crate::utils::ToJSON;

#[napi_derive::napi]
#[derive(Copy, Clone, Default)]
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

macro_rules! decimal_constants {
    ($(($id:ident, $name:literal)),*) => {
        #[napi_derive::napi]
        impl Decimal {
            $(
            #[napi(factory, js_name = $name)]
            #[allow(non_snake_case)]
            #[inline]
            pub fn $id() -> Self {
                Decimal(rust_decimal::Decimal::$id)
            }
            )*
        }
    };
}

decimal_constants!(
    (E, "E"),
    (E_INVERSE, "E_INVERSE"),
    (HALF_PI, "HALF_PI"),
    (MAX, "MAX"),
    (MIN, "MIN"),
    (NEGATIVE_ONE, "NEGATIVE_ONE"),
    (ONE, "ONE"),
    (ONE_HUNDRED, "ONE_HUNDRED"),
    (ONE_THOUSAND, "ONE_THOUSAND"),
    (PI, "PI"),
    (QUARTER_PI, "QUARTER_PI"),
    (TEN, "TEN"),
    (TWO, "TWO"),
    (TWO_PI, "TWO_PI"),
    (ZERO, "ZERO")
);

#[napi_derive::napi]
impl Decimal {
    #[napi(constructor)]
    pub fn new(value: Either<String, f64>) -> Result<Self> {
        match value {
            Either::A(value) => {
                Ok(Self(value.parse().map_err(|err| {
                    Error::from_reason(format!("invalid decimal: {err}"))
                })?))
            }
            Either::B(value) => Ok(Self(rust_decimal::Decimal::from_f64(value).ok_or_else(
                || Error::from_reason(format!("can not create decimal from number: {value}")),
            )?)),
        }
    }

    #[napi(factory)]
    pub fn new_with_scale(num: i64, scale: u32) -> Self {
        Decimal(rust_decimal::Decimal::new(num, scale))
    }

    #[napi]
    #[allow(clippy::wrong_self_convention, clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[napi]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_number(&self) -> f64 {
        self.0.to_f64().unwrap_or_default()
    }

    /// Computes the absolute value.
    #[napi]
    #[inline]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Returns the smallest integer greater than or equal to a number.
    #[napi]
    #[inline]
    pub fn ceil(&self) -> Self {
        Self(self.0.ceil())
    }

    /// Returns the largest integer less than or equal to a number.
    #[napi]
    #[inline]
    pub fn floor(&self) -> Self {
        Self(self.0.floor())
    }

    /// Returns a new Decimal representing the fractional portion of the number.
    #[napi]
    #[inline]
    pub fn fract(&self) -> Self {
        Self(self.0.fract())
    }

    /// Returns `true` if the decimal is negative.
    #[napi]
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    /// Returns `true` if the decimal is positive.
    #[napi]
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    /// Returns `true` if this Decimal number is equivalent to zero.
    #[napi]
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns the maximum of the two numbers.
    #[napi]
    #[inline]
    pub fn max(&self, other: &Decimal) -> Self {
        Self(self.0.max(other.0))
    }

    /// Returns the minimum of the two numbers.
    #[napi]
    #[inline]
    pub fn min(&self, other: &Decimal) -> Self {
        Self(self.0.min(other.0))
    }

    /// Strips any trailing zero’s from a Decimal and converts `-0` to `0`.
    #[napi]
    #[inline]
    pub fn normalize(&self) -> Self {
        Self(self.0.normalize())
    }

    /// Returns a new Decimal number with no fractional portion (i.e. an
    /// integer). Rounding currently follows “Bankers Rounding” rules. e.g.
    /// `6.5` -> `6`, `7.5` -> `8`
    #[napi]
    #[inline]
    pub fn round(&self) -> Self {
        Self(self.0.round())
    }

    /// Returns a new Decimal number with the specified number of decimal
    /// points for fractional portion. Rounding currently follows “Bankers
    /// Rounding” rules. e.g. 6.5 -> 6, 7.5 -> 8
    #[napi]
    #[inline]
    pub fn round_dp(&self, dp: u32) -> Self {
        Self(self.0.round_dp(dp))
    }

    /// Returns a new Decimal integral with no fractional portion. This is a
    /// true truncation whereby no rounding is performed.
    #[napi]
    #[inline]
    pub fn trunc(&self) -> Self {
        Self(self.0.trunc())
    }

    /// Performs the `+` operation.
    #[napi]
    #[inline]
    pub fn add(&self, other: &Decimal) -> Self {
        Self(self.0 + other.0)
    }

    /// Performs the `-` operation.
    #[napi]
    #[inline]
    pub fn sub(&self, other: &Decimal) -> Self {
        Self(self.0 - other.0)
    }

    /// Performs the `*` operation.
    #[napi]
    #[inline]
    pub fn mul(&self, other: &Decimal) -> Self {
        Self(self.0 * other.0)
    }

    /// Performs the `/` operation.
    #[napi]
    #[inline]
    pub fn div(&self, other: &Decimal) -> Self {
        Self(self.0 / other.0)
    }

    /// Performs the `%` operation.
    #[napi]
    #[inline]
    pub fn rem(&self, other: &Decimal) -> Self {
        Self(self.0 % other.0)
    }

    /// Performs the unary `-` operation.
    #[napi]
    #[inline]
    pub fn neg(&self) -> Self {
        Self(-self.0)
    }

    /// Returns `true` if the value of this Decimal is greater than the value of
    /// `x`, otherwise returns `false`.
    #[napi]
    #[inline]
    pub fn greater_than(&self, other: &Decimal) -> bool {
        self.0 > other.0
    }

    /// Returns `true` if the value of this Decimal is greater than or equal to
    /// the value of `x`, otherwise returns `false`.
    #[napi]
    #[inline]
    pub fn greater_than_or_equal_to(&self, other: &Decimal) -> bool {
        self.0 >= other.0
    }

    /// Returns `true` if the value of this Decimal equals the value of `x`,
    /// otherwise returns `false`.
    #[napi]
    #[inline]
    pub fn equals(&self, other: &Decimal) -> bool {
        self.0 == other.0
    }

    /// Returns `true` if the value of this Decimal is less than the value of
    /// `x`, otherwise returns `false`.
    #[napi]
    #[inline]
    pub fn less_than(&self, other: &Decimal) -> bool {
        self.0 <= other.0
    }

    /// Returns `true` if the value of this Decimal is less than or equal to the
    /// value of `x`, otherwise returns `false`.
    #[napi]
    #[inline]
    pub fn less_than_or_equal_to(&self, other: &Decimal) -> bool {
        self.0 <= other.0
    }

    /// Compares the values of two Decimals.
    ///
    /// Returns `-1` if the value of this Decimal is less than the value of
    /// `x`.
    ///
    /// Returns `1` if the value of this Decimal is greater than the value of
    /// `x`.
    ///
    /// Returns `0` if the value of this Decimal equals the value of `x`.
    #[napi]
    #[inline]
    pub fn compared_to(&self, other: &Decimal) -> i32 {
        match self.0.cmp(&other.0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    /// Computes the sine of a number (in radians)
    #[napi]
    #[inline]
    pub fn sin(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_sin()
                .ok_or_else(|| Error::from_reason("overflowed"))?,
        ))
    }

    /// Computes the cosine of a number (in radians)
    #[napi]
    #[inline]
    pub fn cos(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_cos()
                .ok_or_else(|| Error::from_reason("overflowed"))?,
        ))
    }

    /// Computes the tangent of a number (in radians). Panics upon overflow or
    /// upon approaching a limit.
    #[napi]
    #[inline]
    pub fn tan(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_tan()
                .ok_or_else(|| Error::from_reason("overflowed"))?,
        ))
    }

    /// The square root of a Decimal. Uses a standard Babylonian method.
    #[napi]
    #[inline]
    pub fn sqrt(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .sqrt()
                .ok_or_else(|| Error::from_reason("overflow"))?,
        ))
    }

    /// Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not
    /// whole then the approximation e<sup>y*ln(x)</sup> is used.
    #[napi]
    #[inline]
    pub fn pow(&self, exp: &Decimal) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_powd(exp.0)
                .ok_or_else(|| Error::from_reason("overflow"))?,
        ))
    }

    /// Calculates the natural logarithm for a Decimal calculated using Taylor’s
    /// series.
    #[napi]
    #[inline]
    pub fn ln(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_ln()
                .ok_or_else(|| Error::from_reason("overflow"))?,
        ))
    }

    /// Calculates the base 10 logarithm of a specified Decimal number.
    #[napi]
    #[inline]
    pub fn log10(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_log10()
                .ok_or_else(|| Error::from_reason("overflow"))?,
        ))
    }

    /// The estimated exponential function, ex. Stops calculating when it is
    /// within tolerance of roughly `0.0000002`.
    #[napi]
    #[inline]
    pub fn exp(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_exp()
                .ok_or_else(|| Error::from_reason("overflow"))?,
        ))
    }

    /// The estimated exponential function, e<sup>x</sup> using the `tolerance`
    /// provided as a hint as to when to stop calculating. A larger
    /// tolerance will cause the number to stop calculating sooner at the
    /// potential cost of a slightly less accurate result.
    #[napi]
    #[inline]
    pub fn exp_with_tolerance(&self, tolerance: &Decimal) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_exp_with_tolerance(tolerance.0)
                .ok_or_else(|| Error::from_reason("overflow"))?,
        ))
    }

    /// Abramowitz Approximation of Error Function from [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
    #[napi]
    #[inline]
    pub fn erf(&self) -> Self {
        Self(self.0.erf())
    }

    /// The Cumulative distribution function for a Normal distribution
    #[napi]
    #[inline]
    pub fn norm_cdf(&self) -> Self {
        Self(self.0.norm_cdf())
    }

    /// The Probability density function for a Normal distribution.
    #[napi]
    #[inline]
    pub fn norm_pdf(&self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_norm_pdf()
                .ok_or_else(|| Error::from_reason("overflow"))?,
        ))
    }

    #[napi(js_name = "toJSON")]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_json(&self) -> Value {
        <Self as ToJSON>::to_json(self)
    }
}

impl ToJSON for Decimal {
    fn to_json(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
