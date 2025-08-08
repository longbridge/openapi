use std::{
    ffi::c_void,
    future::Future,
    sync::{Arc, LazyLock},
};

use longport::Result;
use tokio::runtime::Runtime;

use crate::error::CError;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime")
});

pub type CAsyncCallback = extern "C" fn(*const CAsyncResult);

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CAsyncResult {
    pub ctx: *const c_void,
    pub error: *const CError,
    pub data: *mut c_void,
    pub length: usize,
    pub userdata: *mut c_void,
}

pub(crate) trait ToAsyncResult {
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult;
}

impl ToAsyncResult for CAsyncResult {
    #[inline]
    fn to_async_result(&self, _ctx: *const c_void) -> CAsyncResult {
        *self
    }
}

impl ToAsyncResult for i64 {
    #[inline]
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        CAsyncResult {
            ctx,
            error: std::ptr::null_mut(),
            data: *self as *mut _,
            length: 0,
            userdata: std::ptr::null_mut(),
        }
    }
}

impl<T> ToAsyncResult for *const T {
    #[inline]
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        CAsyncResult {
            ctx,
            error: std::ptr::null_mut(),
            data: *self as *mut c_void,
            length: 1,
            userdata: std::ptr::null_mut(),
        }
    }
}

impl<T> ToAsyncResult for *mut T {
    #[inline]
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        CAsyncResult {
            ctx,
            error: std::ptr::null(),
            data: *self as *mut c_void,
            length: 1,
            userdata: std::ptr::null_mut(),
        }
    }
}

impl<T> ToAsyncResult for (*const T, usize) {
    #[inline]
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        CAsyncResult {
            ctx,
            error: std::ptr::null_mut(),
            data: self.0 as *mut c_void,
            length: self.1,
            userdata: std::ptr::null_mut(),
        }
    }
}

impl<T> ToAsyncResult for (*mut T, usize) {
    #[inline]
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        CAsyncResult {
            ctx,
            error: std::ptr::null_mut(),
            data: self.0 as *mut c_void,
            length: self.1,
            userdata: std::ptr::null_mut(),
        }
    }
}

impl ToAsyncResult for () {
    #[inline]
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        CAsyncResult {
            ctx,
            error: std::ptr::null_mut(),
            data: std::ptr::null_mut(),
            length: 0,
            userdata: std::ptr::null_mut(),
        }
    }
}

pub(crate) fn execute_async<P, F, T>(
    callback: CAsyncCallback,
    ctx: *const P,
    userdata: *mut c_void,
    fut: F,
) where
    F: Future<Output = Result<T>> + Send + 'static,
    T: ToAsyncResult,
    P: Send,
{
    unsafe {
        let _guard = RUNTIME.enter();
        let ctx_pointer = ctx as usize;
        let userdata_pointer = userdata as usize;

        if !ctx.is_null() {
            Arc::increment_strong_count(ctx);
        }
        tokio::spawn(async move {
            match fut.await {
                Ok(res) => {
                    let mut res = res.to_async_result(ctx_pointer as *const c_void);
                    res.userdata = userdata_pointer as *mut c_void;
                    callback(&res)
                }
                Err(err) => {
                    let err = err.into();
                    let res = CAsyncResult {
                        ctx: ctx_pointer as *mut c_void,
                        error: &err,
                        data: std::ptr::null_mut(),
                        length: 0,
                        userdata: userdata_pointer as *mut c_void,
                    };
                    callback(&res);
                }
            }

            if ctx_pointer != 0 {
                Arc::from_raw(ctx_pointer as *const P);
            }
        });
    }
}
