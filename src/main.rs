// TODO: remove this in production.
#![allow(dead_code, unused_variables, unused_assignments)]

mod core;
mod error;
mod modules;
mod server;
mod state;
mod utils;
use std::sync::Arc;

use crate::core::config;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    // load dotenv.
    dotenvy::dotenv().ok();

    // ----------------- run setups ------------------------
    utils::tracing::setup_tracing();
    utils::crypto::setup_crypto().ok();
    // -----------------------------------------------------

    // ----------------- load config & state ---------------
    let config = Arc::new(config::Config::from_env());
    let state = AppState::new(config).await.unwrap();
    // -----------------------------------------------------

    // ----------------- setup server & start ----------------------
    let server = server::create_server(state);

    server::run_server(server).await;
    // -----------------------------------------------------
}
