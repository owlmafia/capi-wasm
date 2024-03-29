use super::{mock_dao_for_users_view_data, mock_to_sign, req_delay};
use crate::dependencies::funds_asset_specs;
use crate::error::FrError;
use crate::provider::create_dao_provider::{
    CreateDaoParJs, CreateDaoProvider, CreateDaoRes, CreateDaoResJs, SubmitCreateDaoParJs,
    SubmitSetupDaoPassthroughParJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct CreateDaoProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CreateDaoProvider for CreateDaoProviderMock {
    async fn txs(&self, pars: CreateDaoParJs) -> Result<CreateDaoResJs, FrError> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        let creator_address = pars.pt.inputs.creator.parse().map_err(Error::msg)?;

        // this is just (local) data validation / conversion, so ok in mock (we want to test the validation UI too)
        let dao_specs = pars.pt.inputs.to_dao_specs(&funds_asset_specs)?;

        req_delay().await;

        Ok(CreateDaoResJs {
            to_sign: mock_to_sign(&algod, &creator_address).await?,
            // note that data returned here doesn't matter to UI as it's just passthrough
            pt: SubmitSetupDaoPassthroughParJs {
                specs: dao_specs,
                creator: creator_address.to_string(),
                shares_asset_id: 1234567890,
                app_id: 121212121,
                description_url: None,
                setup_date: "0".to_owned(),
            },
        })
    }

    async fn submit(&self, _: SubmitCreateDaoParJs) -> Result<CreateDaoRes, FrError> {
        req_delay().await;

        Ok(CreateDaoRes {
            dao: mock_dao_for_users_view_data()?,
        })
    }
}
