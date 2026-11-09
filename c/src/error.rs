use std::os::raw::c_char;

use longbridge::Error;
use longbridge_c_macros::CEnum;

use crate::types::{CString, ToFFI};

/// Error kind
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::SimpleErrorKind")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CErrorKind {
    /// HTTP error
    #[c(remote = "Http")]
    ErrorKindHttp,
    /// OpenAPI error
    #[c(remote = "OpenApi")]
    ErrorKindOpenApi,
    /// Other error
    #[c(remote = "Other")]
    ErrorKindOther,
    /// OAuth error
    #[c(remote = "OAuth")]
    ErrorKindOAuth,
}

pub struct CError {
    kind: CErrorKind,
    code: i64,
    message: CString,
}

impl From<Error> for CError {
    fn from(err: Error) -> Self {
        let err = err.into_simple_error();
        Self {
            kind: err.kind().into(),
            code: err.code().unwrap_or_default(),
            message: err.message().to_string().into(),
        }
    }
}

/// Free the error object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_error_free(error: *mut CError) {
    let _ = Box::from_raw(error);
}

pub(crate) unsafe fn set_error(error: *mut *mut CError, err: Option<Error>) {
    if !error.is_null() {
        match err {
            Some(err) => *error = Box::into_raw(Box::new(err.into())),
            None => *error = std::ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_error_message(error: *const CError) -> *const c_char {
    (*error).message.to_ffi_type()
}

/// Returns the error code.
///
/// For `ErrorKindHttp` it is the HTTP status, for
/// `ErrorKindOpenApi` the OpenAPI error code. For `ErrorKindOAuth`
/// it is a stable code identifying the failure:
///
/// | Code | Meaning |
/// |------|---------|
/// | 0 | other (e.g. NULL OAuth handle) |
/// | 1 | no token available |
/// | 2 | no refresh token |
/// | 3 | token refresh failed |
/// | 4 | token revocation failed |
/// | 5 | authorization failed |
/// | 6 | authorization timeout |
/// | 7 | authorization code exchange failed |
/// | 8 | CSRF token mismatch (no longer returned) |
/// | 9 | failed to bind the callback server |
/// | 10 | failed to get the local address |
/// | 11 | callback channel closed |
/// | 12 | cannot determine the home directory |
/// | 13 | failed to read the token file |
/// | 14 | failed to parse the token file |
/// | 15 | failed to write the token file |
/// | 16 | failed to delete the token file |
/// | 17 | failed to create the token directory |
/// | 18 | failed to serialize the token |
///
/// It is 0 when the error carries no code; check `lb_error_kind` first.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_error_code(error: *const CError) -> i64 {
    (*error).code
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_error_kind(error: *const CError) -> CErrorKind {
    (*error).kind
}
