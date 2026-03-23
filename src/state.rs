use reqwest::Client as ReqwestClient;
use sqlx::postgres::PgPool;
use std::sync::Arc;

use crate::core::{config::Config, db};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Arc<Config>,
    pub http: ReqwestClient,
}

impl AppState {
    pub async fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        let pool = db::connect(&config.database_url).await?;

        let http = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        Ok(Self { pool, config, http })
    }
}
