use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    js::{
        common::{parse_bridge_pars, to_bridge_res},
        explorer_links::explorer_tx_id_link_env,
    },
    service::str_to_algos::base_units_to_display_units_str,
    teal::programs,
};
use anyhow::{Error, Result};
use core::{
    dependencies::{algod, indexer},
    flows::create_dao::storage::load_dao::load_dao,
    queries::funds_activity::{funds_activity, FundsActivityEntryType},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_funds_activity(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_funds_activity, pars: {:?}", pars);
    to_bridge_res(_bridge_load_funds_activity(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_funds_activity(
    pars: LoadFundsActivityParJs,
) -> Result<LoadFundsActivityResJs> {
    let algod = algod();
    let indexer = indexer();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let creator = pars.creator_address.parse().map_err(Error::msg)?;

    let dao_id = pars.dao_id.parse()?;

    let dao = load_dao(&algod, dao_id, &programs.escrows, &capi_deps).await?;

    let mut activity_entries = funds_activity(
        &algod,
        &indexer,
        &creator,
        dao_id,
        dao.customer_escrow.address(),
        dao.central_escrow.address(),
        &programs.escrows,
        &capi_deps,
    )
    .await?;
    // sort descendingly by date (most recent activity first)
    activity_entries.sort_by(|p1, p2| p2.date.cmp(&p1.date));

    // TODO limit results already with the queries?
    if let Some(max_results) = pars.max_results {
        let max_results = max_results.parse()?;
        activity_entries = activity_entries.into_iter().take(max_results).collect();
    }

    let mut view_data_entries = vec![];
    for entry in activity_entries {
        view_data_entries.push(FundsActivityViewData {
            amount: base_units_to_display_units_str(entry.amount, &funds_asset_specs()),
            is_income: match entry.type_ {
                FundsActivityEntryType::Income => "true",
                FundsActivityEntryType::Spending => "false",
            }
            .to_owned(),
            description: entry.description,
            date: entry.date.to_rfc2822(),
            tx_id: entry.tx_id.to_string(),
            tx_link: explorer_tx_id_link_env(&entry.tx_id),
        });
    }

    Ok(LoadFundsActivityResJs {
        entries: view_data_entries,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadFundsActivityParJs {
    pub dao_id: String,
    pub creator_address: String,
    pub max_results: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadFundsActivityResJs {
    pub entries: Vec<FundsActivityViewData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FundsActivityViewData {
    pub amount: String,
    pub is_income: String, // false: spending
    pub description: String,
    pub date: String,
    pub tx_id: String,
    pub tx_link: String,
}
