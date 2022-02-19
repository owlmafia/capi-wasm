use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use anyhow::{anyhow, Result};
use core::dependencies::algod;
use core::flows::unlock::unlock::{submit_unlock, UnlockSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_unlock, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_unlock(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_unlock(pars: SubmitUnlockParJs) -> Result<SubmitUnlockResJs> {
    let algod = algod();

    if pars.txs.len() != 2 {
        return Err(anyhow!("Invalid unlock txs count: {}", pars.txs.len()));
    }
    let app_call_tx = &pars.txs[0];
    let pay_fee_tx = &pars.txs[1];

    let res = submit_unlock(
        &algod,
        UnlockSigned {
            central_app_optout_tx: signed_js_tx_to_signed_tx1(app_call_tx)?,
            shares_xfer_tx_signed: rmp_serde::from_slice(&pars.pt.shares_xfer_tx_msg_pack)?,
            pay_shares_xfer_fee_tx: signed_js_tx_to_signed_tx1(pay_fee_tx)?,
        },
    )
    .await?;

    log::debug!("Submit unlock res: {:?}", res);

    Ok(SubmitUnlockResJs {})
}

/// The assets creation signed transactions and the specs to create the project
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitUnlockParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitUnlockPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitUnlockPassthroughParJs {
    pub shares_xfer_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitUnlockResJs {}