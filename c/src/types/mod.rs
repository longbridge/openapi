mod array;
mod cow;
mod datetime;
mod decimal;
mod language;
mod market;
mod option;
mod push_candlestick_mode;
mod string;

use std::{ffi::CStr, os::raw::c_char};

pub(crate) use array::CVec;
pub(crate) use cow::CCow;
pub(crate) use datetime::{CDate, CDateTime, CTime};
pub(crate) use decimal::CDecimal;
pub(crate) use language::CLanguage;
pub(crate) use market::CMarket;
pub(crate) use option::COption;
pub(crate) use push_candlestick_mode::CPushCandlestickMode;
pub(crate) use string::CString;

pub(crate) trait ToFFI {
    type FFIType;

    fn to_ffi_type(&self) -> Self::FFIType;
}

impl ToFFI for f64 {
    type FFIType = f64;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

impl ToFFI for i64 {
    type FFIType = i64;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

impl ToFFI for i32 {
    type FFIType = i32;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

impl ToFFI for bool {
    type FFIType = bool;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

impl<T> ToFFI for *const *const T {
    type FFIType = *const T;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        if self.is_null() {
            std::ptr::null()
        } else {
            unsafe { *(*self) }
        }
    }
}

pub(crate) unsafe fn cstr_to_rust(value: *const c_char) -> String {
    CStr::from_ptr(value as *const c_char)
        .to_str()
        .map(ToString::to_string)
        .expect("invalid cstr")
}

/// Like [`std::slice::from_raw_parts`], but tolerates a null `data` pointer
/// when `len` is `0`.
///
/// `from_raw_parts` requires a non-null, aligned pointer even for a zero-length
/// slice, but `std::vector::data()` in C++ is allowed to return `nullptr` for
/// an empty vector — which is exactly what the C++ binding passes for an
/// omitted list argument. Calling `from_raw_parts(null, 0)` is undefined
/// behaviour and aborts the process under the debug UB checks.
pub(crate) unsafe fn slice_from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T] {
    if len == 0 || data.is_null() {
        &[]
    } else {
        std::slice::from_raw_parts(data, len)
    }
}

pub(crate) unsafe fn cstr_array_to_rust(values: *const *const c_char, n: usize) -> Vec<String> {
    slice_from_raw_parts(values, n)
        .iter()
        .copied()
        .map(|value| cstr_to_rust(value))
        .collect::<Vec<_>>()
}
