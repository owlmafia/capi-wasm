use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::model::dao_for_users::dao_to_dao_for_users;
use crate::model::dao_for_users_view_data::dao_for_users_to_view_data;
use crate::provider::view_dao_provider::{ViewDaoParJs, ViewDaoProvider, ViewDaoResJs};
use crate::service::available_funds::available_funds;
use crate::service::str_to_algos::base_units_to_display_units;
use algonaut::core::MicroAlgos;
use algonaut::transaction::url::LinkableTransactionBuilder;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base::decimal_util::DecimalExt;
use base::dependencies::algod;
use base::flows::create_dao::storage::load_dao::load_dao;

pub struct ViewDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ViewDaoProvider for ViewDaoProviderDef {
    async fn get(&self, pars: ViewDaoParJs) -> Result<ViewDaoResJs> {
        let algod = algod();
        let api = api();
        let funds_asset_specs = funds_asset_specs()?;
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        // TODO investor count: get all holders of asset (indexer?)

        let customer_payment_deeplink =
            LinkableTransactionBuilder::payment(*dao.customer_escrow.address(), MicroAlgos(0))
                .build()
                .as_url();

        let available_funds = available_funds(&algod, &dao, funds_asset_specs.id).await?;

        // TODO!! not-locked shares (use global function to get not-locked (name prob. "available" shares))
        let shares_available = algod
            .account_information(&dao.app_address())
            .await?
            .assets
            .iter()
            .find(|a| a.asset_id == dao.shares_asset_id)
            .ok_or({
                anyhow!("Invalid app state: Investor escrow doesn't have shares asset, Please contact support.")
            })?.amount;

        let investos_share_formatted = dao.specs.investors_part.value().format_percentage();

        let dao_view_data =
            dao_for_users_to_view_data(dao_to_dao_for_users(&dao, &dao_id)?, &funds_asset_specs);

        Ok(ViewDaoResJs {
            dao: dao_view_data,
            // shares_supply: shares_supply.to_string(),
            shares_available: shares_available.to_string(),
            investors_share: investos_share_formatted,
            available_funds: base_units_to_display_units(available_funds, &funds_asset_specs)
                .to_string(),
            customer_payment_deeplink: customer_payment_deeplink.to_string(),
        })
    }
}
