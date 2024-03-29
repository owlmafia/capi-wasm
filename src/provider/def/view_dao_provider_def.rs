use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::error::FrError;
use crate::model::dao_js::ToDaoJs;
use crate::provider::view_dao_provider::{ViewDaoParJs, ViewDaoProvider, ViewDaoResJs};
use crate::service::available_funds::owned_funds;
use crate::service::number_formats::base_units_to_display_units_readable;
use algonaut::core::MicroAlgos;
use algonaut::transaction::url::LinkableTransactionBuilder;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base::flows::create_dao::storage::load_dao::load_dao;
use mbase::dependencies::algod;
use mbase::util::decimal_util::DecimalExt;

pub struct ViewDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ViewDaoProvider for ViewDaoProviderDef {
    async fn get(&self, pars: ViewDaoParJs) -> Result<ViewDaoResJs, FrError> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id).await?;

        // TODO investor count: get all holders of asset (indexer?)

        let customer_payment_deeplink =
            LinkableTransactionBuilder::payment(dao.app_address(), MicroAlgos(0))
                .build()
                .as_url();

        // TODO optimize: we're fetching the global state here again (it's also fetched to create the dao)
        // maybe add available funds field to dao? or retrieve the global state first and create dao and this with it?
        let owned_funds = owned_funds(&algod, &dao, funds_asset_specs.id, &capi_deps).await?;

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

        let investos_share_formatted = dao.investors_share.value().format_percentage();

        let dao_view_data = dao.to_js(&funds_asset_specs)?;

        Ok(ViewDaoResJs {
            dao: dao_view_data,
            shares_available: shares_available.to_string(),
            investors_share: investos_share_formatted,
            available_funds: base_units_to_display_units_readable(owned_funds, &funds_asset_specs)?,
            customer_payment_deeplink: customer_payment_deeplink.to_string(),
        })
    }
}
