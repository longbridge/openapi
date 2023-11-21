use std::os::raw::c_char;

use longport::Error;

use crate::types::{CString, ToFFI};

pub struct CError {
    code: i64,
    message: CString,
}

impl From<Error> for CError {
    fn from(err: Error) -> Self {
        let err = err.into_simple_error();
        Self {
            code: err.code().unwrap_or_default(),
            message: err.message().to_string().into(),
        }
    }
}

/// Free the error object
#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn lb_error_message(error: *const CError) -> *const c_char {
    (*error).message.to_ffi_type()
}

#[no_mangle]
pub unsafe extern "C" fn lb_error_code(error: *const CError) -> i64 {
    (*error).code
}
