use crate::quote::types::SecuritiesUpdateMode;

/// An request to create a watchlist group
#[napi_derive::napi(object)]
#[derive(Debug)]
pub struct CreateWatchlistGroup {
    /// Group name
    pub name: String,
    /// Securities
    pub securities: Option<Vec<String>>,
}

impl From<CreateWatchlistGroup> for longport::quote::RequestCreateWatchlistGroup {
    #[inline]
    fn from(CreateWatchlistGroup { name, securities }: CreateWatchlistGroup) -> Self {
        longport::quote::RequestCreateWatchlistGroup { name, securities }
    }
}

/// An request to delete a watchlist group
#[napi_derive::napi(object)]
#[derive(Debug)]
pub struct DeleteWatchlistGroup {
    /// Group id
    pub id: i64,
    /// Move securities in this group to the default group
    pub purge: bool,
}

/// An request to update a watchlist group
#[napi_derive::napi(object)]
#[derive(Debug)]
pub struct UpdateWatchlistGroup {
    /// Group id
    pub id: i64,
    /// Group name
    pub name: Option<String>,
    /// Securities
    pub securities: Option<Vec<String>>,
    /// Securities Update mode
    pub mode: SecuritiesUpdateMode,
}

impl From<UpdateWatchlistGroup> for longport::quote::RequestUpdateWatchlistGroup {
    #[inline]
    fn from(
        UpdateWatchlistGroup {
            id,
            name,
            securities,
            mode,
        }: UpdateWatchlistGroup,
    ) -> Self {
        longport::quote::RequestUpdateWatchlistGroup {
            id,
            name,
            securities,
            mode: mode.into(),
        }
    }
}
