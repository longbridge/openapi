use std::os::raw::c_char;

use longbridge::asset::StatementItem;

use crate::types::{CString, ToFFI};

/// Statement item
#[repr(C)]
pub struct CStatementItem {
    /// Statement date (integer, e.g. 20250301)
    pub dt: i32,
    /// File key
    pub file_key: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CStatementItemOwned {
    dt: i32,
    file_key: CString,
}

impl From<StatementItem> for CStatementItemOwned {
    fn from(item: StatementItem) -> Self {
        Self {
            dt: item.dt,
            file_key: item.file_key.into(),
        }
    }
}

impl ToFFI for CStatementItemOwned {
    type FFIType = CStatementItem;
    fn to_ffi_type(&self) -> CStatementItem {
        CStatementItem {
            dt: self.dt,
            file_key: self.file_key.to_ffi_type(),
        }
    }
}
