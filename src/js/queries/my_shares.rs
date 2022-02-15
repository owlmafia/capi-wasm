use crate::{
    js::common::{parse_bridge_pars, to_bridge_res},
    teal::programs,
};
use anyhow::{Error, Result};
use core::{
    dependencies::{algod, indexer},
    flows::create_project::storage::load_project::load_project,
    state::{
        account_state::asset_holdings, app_state::ApplicationLocalStateError,
        central_app_state::central_investor_state,
    },
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_my_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_my_shares, pars: {:?}", pars);
    to_bridge_res(_bridge_my_shares(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_my_shares(pars: MySharesParJs) -> Result<MySharesResJs> {
    let algod = algod();
    let indexer = indexer();

    let project_id = pars.project_id.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs().escrows)
        .await?
        .project;

    log::debug!("Project: {project:?}");

    let my_address = &pars.my_address.parse().map_err(Error::msg)?;

    let staked_shares =
        match central_investor_state(&algod, my_address, project.central_app_id).await {
            Ok(state) => state.shares,
            Err(ApplicationLocalStateError::NotOptedIn) => 0, // not invested -> 0 shares
            Err(e) => return Err(Error::msg(e)),
        };

    let free_shares = match asset_holdings(&algod, my_address, project.shares_asset_id).await {
        Ok(shares) => shares,
        Err(e) => return Err(Error::msg(e)),
    };

    Ok(MySharesResJs {
        total: (staked_shares + free_shares).to_string(),
        free: free_shares.to_string(),
        staked: staked_shares.to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct MySharesParJs {
    pub project_id: String,
    pub my_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MySharesResJs {
    pub total: String,
    pub free: String,
    pub staked: String,
}