use std::os::raw::c_char;

use longbridge::alert::{AlertItem, AlertList, AlertSymbolGroup};

use crate::types::{CString, CVec, ToFFI};

/// A single alert indicator configuration for a symbol.
#[repr(C)]
pub struct CAlertItem {
    /// Unique alert identifier.
    pub id: *const c_char,
    /// Identifier of the indicator that triggers this alert.
    pub indicator_id: *const c_char,
    /// Whether this alert is currently enabled.
    pub enabled: bool,
    /// Alert notification frequency code.
    pub frequency: i32,
    /// Scope of the alert (e.g. per-symbol or global).
    pub scope: i32,
    /// Human-readable description text for the alert.
    pub text: *const c_char,
    /// Pointer to an array of state codes associated with this alert.
    pub state: *const i32,
    /// Number of elements in the `state` array.
    pub num_state: usize,
    /// JSON-serialized map of additional indicator parameter values.
    pub value_map: *const c_char,
}

pub(crate) struct CAlertItemOwned {
    id: CString,
    indicator_id: CString,
    enabled: bool,
    frequency: i32,
    scope: i32,
    text: CString,
    state: CVec<i32>,
    value_map: CString,
}

impl From<AlertItem> for CAlertItemOwned {
    fn from(v: AlertItem) -> Self {
        Self {
            id: v.id.into(),
            indicator_id: v.indicator_id.into(),
            enabled: v.enabled,
            frequency: v.frequency,
            scope: v.scope,
            text: v.text.into(),
            state: v.state.into(),
            value_map: serde_json::to_string(&v.value_map)
                .unwrap_or_default()
                .into(),
        }
    }
}

impl CAlertItem {
    /// Reconstruct a [`longbridge::alert::AlertItem`] from this C struct.
    ///
    /// # Safety
    /// All pointer fields must be valid null-terminated C strings and the
    /// `state` pointer must point to at least `num_state` valid `i32` values.
    pub unsafe fn to_alert_item(&self) -> longbridge::alert::AlertItem {
        use crate::types::cstr_to_rust;
        let state = std::slice::from_raw_parts(self.state, self.num_state).to_vec();
        let value_map_str = cstr_to_rust(self.value_map);
        let value_map = serde_json::from_str(&value_map_str).unwrap_or(serde_json::Value::Null);
        longbridge::alert::AlertItem {
            id: cstr_to_rust(self.id),
            indicator_id: cstr_to_rust(self.indicator_id),
            enabled: self.enabled,
            frequency: self.frequency,
            scope: self.scope,
            text: cstr_to_rust(self.text),
            state,
            value_map,
        }
    }
}

impl ToFFI for CAlertItemOwned {
    type FFIType = CAlertItem;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAlertItem {
            id: self.id.to_ffi_type(),
            indicator_id: self.indicator_id.to_ffi_type(),
            enabled: self.enabled,
            frequency: self.frequency,
            scope: self.scope,
            text: self.text.to_ffi_type(),
            state: self.state.to_ffi_type(),
            num_state: self.state.len(),
            value_map: self.value_map.to_ffi_type(),
        }
    }
}

/// A symbol together with all of its associated alert indicators.
#[repr(C)]
pub struct CAlertSymbolGroup {
    /// Full symbol string (e.g. "700.HK").
    pub symbol: *const c_char,
    /// Short ticker code without market suffix.
    pub code: *const c_char,
    /// Market the symbol belongs to (e.g. "HK", "US").
    pub market: *const c_char,
    /// Display name of the security.
    pub name: *const c_char,
    /// Latest price as a string.
    pub price: *const c_char,
    /// Absolute price change as a string.
    pub chg: *const c_char,
    /// Percentage price change as a string.
    pub p_chg: *const c_char,
    /// Product type string (e.g. "stock", "fund").
    pub product: *const c_char,
    /// Pointer to an array of alert indicator items for this symbol.
    pub indicators: *const CAlertItem,
    /// Number of elements in the `indicators` array.
    pub num_indicators: usize,
}

pub(crate) struct CAlertSymbolGroupOwned {
    symbol: CString,
    code: CString,
    market: CString,
    name: CString,
    price: CString,
    chg: CString,
    p_chg: CString,
    product: CString,
    indicators: CVec<CAlertItemOwned>,
}

impl From<AlertSymbolGroup> for CAlertSymbolGroupOwned {
    fn from(v: AlertSymbolGroup) -> Self {
        Self {
            symbol: v.symbol.into(),
            code: v.code.into(),
            market: v.market.into(),
            name: v.name.into(),
            price: v.price.map(|d| d.to_string()).unwrap_or_default().into(),
            chg: v.chg.map(|d| d.to_string()).unwrap_or_default().into(),
            p_chg: v.p_chg.map(|d| d.to_string()).unwrap_or_default().into(),
            product: v.product.into(),
            indicators: v.indicators.into(),
        }
    }
}

impl ToFFI for CAlertSymbolGroupOwned {
    type FFIType = CAlertSymbolGroup;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAlertSymbolGroup {
            symbol: self.symbol.to_ffi_type(),
            code: self.code.to_ffi_type(),
            market: self.market.to_ffi_type(),
            name: self.name.to_ffi_type(),
            price: self.price.to_ffi_type(),
            chg: self.chg.to_ffi_type(),
            p_chg: self.p_chg.to_ffi_type(),
            product: self.product.to_ffi_type(),
            indicators: self.indicators.to_ffi_type(),
            num_indicators: self.indicators.len(),
        }
    }
}

/// Top-level response containing alert symbol groups.
#[repr(C)]
pub struct CAlertList {
    /// Pointer to an array of symbol group items.
    pub lists: *const CAlertSymbolGroup,
    /// Number of elements in the `lists` array.
    pub num_lists: usize,
}

pub(crate) struct CAlertListOwned {
    lists: CVec<CAlertSymbolGroupOwned>,
}

impl From<AlertList> for CAlertListOwned {
    fn from(v: AlertList) -> Self {
        Self {
            lists: v.lists.into(),
        }
    }
}

impl ToFFI for CAlertListOwned {
    type FFIType = CAlertList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CAlertList {
            lists: self.lists.to_ffi_type(),
            num_lists: self.lists.len(),
        }
    }
}
