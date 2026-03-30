use std::sync::Arc;

use napi::Result;

use crate::{
    asset::{
        requests::{GetStatementDownloadUrlRequest, GetStatementListRequest, StatementType},
        types::{GetStatementDownloadUrlResponse, GetStatementListResponse},
    },
    config::Config,
    error::ErrorNewType,
};

/// Asset context
#[napi_derive::napi]
#[derive(Clone)]
pub struct AssetContext {
    ctx: longbridge::asset::AssetContext,
}

#[napi_derive::napi]
impl AssetContext {
    /// Create a new `AssetContext`
    #[napi]
    pub fn new(config: &Config) -> AssetContext {
        Self {
            ctx: longbridge::asset::AssetContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get statement data list
    #[napi]
    pub async fn statements(
        &self,
        req: Option<GetStatementListRequest>,
    ) -> Result<GetStatementListResponse> {
        let req = req.unwrap_or_default();
        let st = req.statement_type.unwrap_or(StatementType::Daily).into();
        let mut opts = longbridge::asset::GetStatementListOptions::new(st);
        if let Some(start_date) = req.start_date {
            opts = opts.page(start_date);
        }
        if let Some(limit) = req.limit {
            opts = opts.page_size(limit);
        }
        Ok(self
            .ctx
            .statements(opts)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get statement data download URL
    #[napi]
    pub async fn statement_download_url(
        &self,
        req: GetStatementDownloadUrlRequest,
    ) -> Result<GetStatementDownloadUrlResponse> {
        let opts = longbridge::asset::GetStatementOptions::new(req.file_key);
        Ok(self
            .ctx
            .statement_download_url(opts)
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
