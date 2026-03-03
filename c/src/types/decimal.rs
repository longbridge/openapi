use std::{
    cell::UnsafeCell,
    cmp::Ordering,
    ffi::{CStr, CString},
    os::raw::c_char,
};

use rust_decimal::{Decimal, MathematicalOps, prelude::ToPrimitive};

use crate::types::ToFFI;

#[derive(Debug)]
pub struct CDecimal {
    pub(crate) value: Decimal,
    value_str: UnsafeCell<CString>,
}

impl From<Decimal> for CDecimal {
    #[inline]
    fn from(value: Decimal) -> Self {
        Self {
            value,
            value_str: Default::default(),
        }
    }
}

impl ToFFI for CDecimal {
    type FFIType = *const CDecimal;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        self
    }
}

/// Create a decimal value with a 64 bit `m` representation and corresponding
/// `e` scale.
#[unsafe(no_mangle)]
pub extern "C" fn lb_decimal_new(num: i64, scale: u32) -> *mut CDecimal {
    Box::into_raw(Box::new(Decimal::new(num, scale).into()))
}

/// Clone the decimal value
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_clone(value: *const CDecimal) -> *mut CDecimal {
    Box::into_raw(Box::new((*value).value.into()))
}

/// Create a decimal value from string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_from_str(value: *const c_char) -> *mut CDecimal {
    match CStr::from_ptr(value)
        .to_str()
        .ok()
        .and_then(|value| value.parse::<Decimal>().ok())
    {
        Some(value) => Box::into_raw(Box::new(value.into())),
        None => std::ptr::null_mut(),
    }
}

/// Create a decimal value from double
#[unsafe(no_mangle)]
pub extern "C" fn lb_decimal_from_double(value: f64) -> *mut CDecimal {
    match Decimal::from_f64_retain(value) {
        Some(value) => Box::into_raw(Box::new(value.into())),
        None => std::ptr::null_mut(),
    }
}

/// Free the decimal value
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_free(value: *mut CDecimal) {
    let _ = Box::from_raw(value);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_to_double(value: *const CDecimal) -> f64 {
    (*value).value.to_f64().unwrap_or_default()
}

/// Computes the absolute value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_abs(value: *mut CDecimal) {
    let new_value = (*value).value.abs();
    (*value).value = new_value;
}

/// Returns the smallest integer greater than or equal to a number.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_ceil(value: *mut CDecimal) {
    let new_value = (*value).value.ceil();
    (*value).value = new_value;
}

/// Returns the largest integer less than or equal to a number.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_floor(value: *mut CDecimal) {
    let new_value = (*value).value.floor();
    (*value).value = new_value;
}

/// Returns a new Decimal representing the fractional portion of the number.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_fract(value: *mut CDecimal) {
    let new_value = (*value).value.fract();
    (*value).value = new_value;
}

/// Returns `true` if the decimal is negative.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_is_negative(value: *const CDecimal) -> bool {
    (*value).value.is_sign_negative()
}

/// Returns `true` if the decimal is positive.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_is_positive(value: *const CDecimal) -> bool {
    (*value).value.is_sign_positive()
}

/// Returns `true` if this Decimal number is equivalent to zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_is_zero(value: *const CDecimal) -> bool {
    (*value).value.is_zero()
}

/// Returns the maximum of the two numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_max(a: *const CDecimal, b: *const CDecimal) -> *const CDecimal {
    if (*a).value > (*b).value { a } else { b }
}

/// Returns the minimum of the two numbers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_min(a: *const CDecimal, b: *const CDecimal) -> *const CDecimal {
    if (*a).value < (*b).value { a } else { b }
}

/// Strips any trailing zero’s from a Decimal and converts `-0` to `0`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_normalize(value: *mut CDecimal) {
    let new_value = (*value).value.normalize();
    (*value).value = new_value;
}

/// Returns a new Decimal number with no fractional portion (i.e. an
/// integer). Rounding currently follows “Bankers Rounding” rules. e.g.
/// `6.5` -> `6`, `7.5` -> `8`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_round(value: *mut CDecimal) {
    let new_value = (*value).value.round();
    (*value).value = new_value;
}

/// Returns a new Decimal number with the specified number of decimal points for
/// fractional portion. Rounding currently follows “Bankers Rounding” rules.
/// e.g. 6.5 -> 6, 7.5 -> 8
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_round_dp(value: *mut CDecimal, dp: u32) {
    let new_value = (*value).value.round_dp(dp);
    (*value).value = new_value;
}

/// Returns a new Decimal integral with no fractional portion. This is a
/// true truncation whereby no rounding is performed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_trunc(value: *mut CDecimal) {
    let new_value = (*value).value.trunc();
    (*value).value = new_value;
}

/// Performs the `+` operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_add(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).value + (*b).value;
    (*a).value = new_value;
}

/// Performs the `-` operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_sub(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).value - (*b).value;
    (*a).value = new_value;
}

/// Performs the `*` operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_mul(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).value * (*b).value;
    (*a).value = new_value;
}

/// Performs the `/` operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_div(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).value / (*b).value;
    (*a).value = new_value;
}

/// Performs the `%` operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_rem(a: *mut CDecimal, b: *const CDecimal) {
    let new_value = (*a).value % (*b).value;
    (*a).value = new_value;
}

/// Performs the unary `-` operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_neg(value: *mut CDecimal) {
    let new_value = -(*value).value;
    (*value).value = new_value;
}

/// Returns `true` if the value of this Decimal is greater than the value of
/// `x`, otherwise returns `false`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_gt(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).value > (*b).value
}

/// Returns `true` if the value of this Decimal is greater than or equal to
/// the value of `x`, otherwise returns `false`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_gte(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).value >= (*b).value
}

/// Returns `true` if the value of this Decimal equals the value of `x`,
/// otherwise returns `false`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_eq(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).value == (*b).value
}

/// Returns `true` if the value of this Decimal is less than the value of
/// `x`, otherwise returns `false`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_lt(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).value < (*b).value
}

/// Returns `true` if the value of this Decimal is less than or equal to the
/// value of `x`, otherwise returns `false`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_lte(a: *const CDecimal, b: *const CDecimal) -> bool {
    (*a).value <= (*b).value
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_cmp(a: *const CDecimal, b: *const CDecimal) -> i32 {
    match (*a).value.cmp(&(*b).value) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Computes the sine of a number (in radians)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_sin(value: *mut CDecimal) {
    let new_value = (*value).value.checked_sin().expect("overflowed");
    (*value).value = new_value;
}

/// Computes the cosine of a number (in radians)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_cos(value: *mut CDecimal) {
    let new_value = (*value).value.checked_cos().expect("overflowed");
    (*value).value = new_value;
}

/// Computes the tangent of a number (in radians). Panics upon overflow or
/// upon approaching a limit.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_tan(value: *mut CDecimal) {
    let new_value = (*value).value.checked_tan().expect("overflowed");
    (*value).value = new_value;
}

/// The square root of a Decimal. Uses a standard Babylonian method.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_sqrt(value: *mut CDecimal) {
    let new_value = (*value).value.sqrt().expect("overflowed");
    (*value).value = new_value;
}

/// Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not
/// whole then the approximation e<sup>y*ln(x)</sup> is used.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_pow(value: *mut CDecimal, exp: *const CDecimal) {
    let new_value = (*value)
        .value
        .checked_powd((*exp).value)
        .expect("overflowed");
    (*value).value = new_value;
}

/// Calculates the natural logarithm for a Decimal calculated using Taylor’s
/// series.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_ln(value: *mut CDecimal) {
    let new_value = (*value).value.checked_ln().expect("overflowed");
    (*value).value = new_value;
}

/// Calculates the base 10 logarithm of a specified Decimal number.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_log10(value: *mut CDecimal) {
    let new_value = (*value).value.checked_log10().expect("overflowed");
    (*value).value = new_value;
}

/// The estimated exponential function, ex. Stops calculating when it is
/// within tolerance of roughly `0.0000002`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_exp(value: *mut CDecimal) {
    let new_value = (*value).value.checked_exp().expect("overflowed");
    (*value).value = new_value;
}

/// The estimated exponential function, e<sup>x</sup> using the `tolerance`
/// provided as a hint as to when to stop calculating. A larger
/// tolerance will cause the number to stop calculating sooner at the
/// potential cost of a slightly less accurate result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_exp_with_tolerance(
    value: *mut CDecimal,
    tolerance: *const CDecimal,
) {
    let new_value = (*value)
        .value
        .checked_exp_with_tolerance((*tolerance).value)
        .expect("overflowed");
    (*value).value = new_value;
}

/// Abramowitz Approximation of Error Function from [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_erf(value: *mut CDecimal) {
    let new_value = (*value).value.erf();
    (*value).value = new_value;
}

/// The Cumulative distribution function for a Normal distribution
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_normal_cdf(value: *mut CDecimal) {
    let new_value = (*value).value.norm_cdf();
    (*value).value = new_value;
}

/// The Probability density function for a Normal distribution.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_norm_pdf(value: *mut CDecimal) {
    let new_value = (*value).value.norm_pdf();
    (*value).value = new_value;
}

/// The Probability density function for a Normal distribution.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_decimal_to_string(value: *const CDecimal) -> *const c_char {
    *(*value).value_str.get() = CString::new((*value).value.to_string()).unwrap();
    (*(*value).value_str.get()).as_ptr()
}
