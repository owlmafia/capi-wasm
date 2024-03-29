use crate::error::FrError;
use crate::js::bridge::log_wrap_new;
use crate::js::common::{signed_js_tx_to_signed_tx1, SignedTxFromJs};
use crate::js::to_sign_js::ToSignJs;
use crate::provider::create_dao_provider::validate_min_raised_target_end_date;
use crate::provider::providers;
use anyhow::{Error, Result};
use base::dev_settings::{dev_settings, submit_dev_settings, DevSettings, DevSettingsSigned};
use base::flows::create_dao::storage::load_dao::load_dao;
use mbase::dependencies::algod;
use mbase::util::network_util::wait_for_pending_transaction;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

pub struct DevProviderDef {}

impl DevProviderDef {
    pub async fn txs(&self, pars: DevSettingsParJs) -> Result<DevSettingsResJs, FrError> {
        let algod = algod();

        let dao = load_dao(&algod, pars.dao_id.parse()?).await?;

        let sender_address = pars.sender_address.parse().map_err(Error::msg)?;

        let min_raise_target_end_date =
            validate_min_raised_target_end_date(&pars.min_raise_target_end_date)?;

        let to_sign = dev_settings(
            &algod,
            &sender_address,
            dao.app_id,
            &DevSettings {
                min_raise_target_end_date,
            },
        )
        .await?;

        let to_sign_txs = vec![to_sign.app_call_tx];

        Ok(DevSettingsResJs {
            to_sign: ToSignJs::new(to_sign_txs)?,
        })
    }

    pub async fn submit(
        &self,
        pars: SubmitDevSettingsParJs,
    ) -> Result<SubmitDevSettingsResJs, FrError> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(FrError::Internal(format!(
                "Unexpected add roadmap item txs length: {}",
                pars.txs.len()
            )));
        }
        let tx = &pars.txs[0];

        let tx_id = submit_dev_settings(
            &algod,
            &DevSettingsSigned {
                app_call_tx: signed_js_tx_to_signed_tx1(tx)?,
            },
        )
        .await?;

        log::debug!("Submit dev_settings res: {:?}", tx_id);

        let _ = wait_for_pending_transaction(&algod, &tx_id).await?;

        Ok(SubmitDevSettingsResJs {})
    }
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct DevSettingsParJs {
    pub dao_id: String,
    pub sender_address: String,
    pub min_raise_target_end_date: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct DevSettingsResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitDevSettingsParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitDevSettingsResJs {}

#[wasm_bindgen(js_name=setDevSettings)]
pub async fn set_dev_settings(pars: DevSettingsParJs) -> Result<DevSettingsResJs, FrError> {
    log_wrap_new("set_dev_settings", pars, async move |pars| {
        providers()?.dev_settings.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitSetDevSettings)]
pub async fn submit_set_dev_settings(
    pars: SubmitDevSettingsParJs,
) -> Result<SubmitDevSettingsResJs, FrError> {
    log_wrap_new("submit_set_dev_settings", pars, async move |pars| {
        providers()?.dev_settings.submit(pars).await
    })
    .await
}
