use crate::js::common::{signed_js_tx_to_signed_tx1, to_my_algo_tx1};
use crate::provider::def::create_dao_provider_def::maybe_upload_image;
use crate::provider::update_data_provider::{
    SubmitUpdateDataParJs, SubmitUpdateDataResJs, UpdatableDataParJs, UpdatableDataResJs,
    UpdateDataParJs, UpdateDataPassthroughJs, UpdateDataProvider, UpdateDataResJs,
};
use algonaut::core::Address;
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::api::version::Version;
use base::api::version::VersionedAddress;
use base::dependencies::image_api;
use base::flows::create_dao::setup_dao_specs::CompressedImage;
use base::flows::update_data::update_data::{
    submit_update_data, update_data, UpdatableDaoData, UpdateDaoDataSigned,
};
use base::state::dao_app_state::dao_global_state;
use mbase::dependencies::algod;
use mbase::models::dao_id::DaoId;
use mbase::models::funds::FundsAmount;
use mbase::models::image_hash::ImageHash;

pub struct UpdateDataProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateDataProvider for UpdateDataProviderDef {
    async fn get(&self, pars: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
        let algod = algod();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;

        let app_state = dao_global_state(&algod, dao_id.0).await?;

        Ok(UpdatableDataResJs {
            owner: app_state.owner.to_string(),
            customer_escrow: app_state.customer_escrow.address.to_string(),
            customer_escrow_version: app_state.customer_escrow.version.0.to_string(),
            project_name: app_state.project_name,
            project_desc: app_state.project_desc,
            share_price: app_state.share_price.to_string(),
            image_hash: app_state.image_hash.map(|h| h.as_str()),
            social_media_url: app_state.social_media_url,
        })
    }

    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs> {
        let algod = algod();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;
        let owner = pars.owner.parse().map_err(Error::msg)?;

        // TODO escrow versioning
        // we're currently saving only the addresses, so don't have the programs to lsig
        // so we've to store the version too (although it could be skipped by just trying all available versions against the address, which seems very inefficient)
        // and use this version to retrieve the program
        // the teal has to be updated to store the version, either in the same field as the address or a separate field with all the escrow's versions

        let image = pars.image.map(CompressedImage::new);
        let image_hash = image.as_ref().map(|i| i.hash());

        let to_sign = update_data(
            &algod,
            &owner,
            dao_id.0,
            &UpdatableDaoData {
                customer_escrow: VersionedAddress::new(
                    parse_addr(pars.customer_escrow)?,
                    parse_int(pars.customer_escrow_version)?,
                ),
                project_name: pars.project_name,
                project_desc: pars.project_desc,
                share_price: FundsAmount::new(pars.share_price.parse().map_err(Error::msg)?),
                image_hash: image_hash.clone(),
                social_media_url: pars.social_media_url,
                owner,
            },
        )
        .await?;

        Ok(UpdateDataResJs {
            to_sign: to_my_algo_tx1(&to_sign.update).map_err(Error::msg)?,
            pt: UpdateDataPassthroughJs {
                dao_id: dao_id.to_string(),
                image: image.map(|i| i.bytes()),
                image_hash: image_hash.map(|h| h.bytes()),
            },
        })
    }

    async fn submit(&self, pars: SubmitUpdateDataParJs) -> Result<SubmitUpdateDataResJs> {
        let algod = algod();
        let image_api = image_api();

        let dao_id = pars.pt.dao_id.parse::<DaoId>().map_err(Error::msg)?;
        let image = pars.pt.image.map(CompressedImage::new);
        let image_hash = match pars.pt.image_hash {
            Some(bytes) => Some(ImageHash::from_bytes(bytes)?),
            None => None,
        };

        let tx_id = submit_update_data(
            &algod,
            UpdateDaoDataSigned {
                update: signed_js_tx_to_signed_tx1(&pars.tx)?,
            },
        )
        .await?;

        // Note that it's required to upload the image after the DAO update, because the image api checks the hash in the app's global state.
        let (maybe_image_url, maybe_image_upload_error) =
            maybe_upload_image(&algod, &image_api, tx_id, dao_id.0, image, image_hash).await?;

        Ok(SubmitUpdateDataResJs {
            image_url: maybe_image_url,
            image_upload_error: maybe_image_upload_error,
        })
    }
}

fn parse_int(str: String) -> Result<Version> {
    Ok(Version(str.parse().map_err(Error::msg)?))
}

fn parse_addr(s: String) -> Result<Address> {
    s.parse().map_err(Error::msg)
}