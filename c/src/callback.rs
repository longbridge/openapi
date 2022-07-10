use std::ffi::c_void;

pub type CFreeUserDataFunc = Option<extern "C" fn(*mut c_void)>;

pub(crate) struct Callback<F> {
    pub(crate) f: F,
    pub(crate) userdata: *mut c_void,
    pub(crate) free_userdata: CFreeUserDataFunc,
}

impl<F> Drop for Callback<F> {
    fn drop(&mut self) {
        if let Some(free_userdata) = self.free_userdata {
            free_userdata(self.userdata);
        }
    }
}
