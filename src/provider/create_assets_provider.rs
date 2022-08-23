use super::create_dao_provider::{
    CreateDaoFormInputsJs, CreateDaoPassthroughParJs, ValidateDaoInputsError,
    ValidationDaoInputsOrAnyhowError,
};
use crate::js::{
    common::to_js_value,
    inputs_validation_js::{to_validation_error_js, ValidationErrorJs},
    to_sign_js::ToSignJs,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateAssetsProvider {
    async fn txs(
        &self,
        pars: CreateDaoAssetsParJs,
    ) -> Result<CreateDaoAssetsResJs, ValidationDaoInputsOrAnyhowError>;
}

/// Errors to be shown next to the respective input fields
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateAssetsInputErrorsJs {
    // just some string to identify the struct in js
    pub type_identifier: String,
    pub name: Option<ValidationErrorJs>,
    pub description: Option<ValidationErrorJs>,
    pub creator: Option<ValidationErrorJs>,
    pub share_supply: Option<ValidationErrorJs>,
    pub share_price: Option<ValidationErrorJs>,
    pub investors_share: Option<ValidationErrorJs>,
    pub social_media_url: Option<ValidationErrorJs>,
    pub min_raise_target: Option<ValidationErrorJs>,
    pub min_raise_target_end_date: Option<ValidationErrorJs>,
    pub min_invest_shares: Option<ValidationErrorJs>,
    pub max_invest_shares: Option<ValidationErrorJs>,
    pub shares_for_investors: Option<ValidationErrorJs>,

    // note that these are not text inputs, but still, inputs
    pub image_url: Option<ValidationErrorJs>,
    pub prospectus_url: Option<ValidationErrorJs>,
    pub prospectus_bytes: Option<ValidationErrorJs>,
}

impl From<ValidateDaoInputsError> for JsValue {
    fn from(error: ValidateDaoInputsError) -> JsValue {
        match error {
            ValidateDaoInputsError::AllFieldsValidation(e) => {
                let errors_js = CreateAssetsInputErrorsJs {
                    type_identifier: "input_errors".to_owned(),
                    name: e.name.map(to_validation_error_js),
                    description: e.description.map(to_validation_error_js),
                    creator: e.creator.map(to_validation_error_js),
                    share_supply: e.share_supply.map(to_validation_error_js),
                    share_price: e.share_price.map(to_validation_error_js),
                    investors_share: e.investors_share.map(to_validation_error_js),
                    social_media_url: e.social_media_url.map(to_validation_error_js),
                    min_raise_target: e.min_raise_target.map(to_validation_error_js),
                    min_raise_target_end_date: e
                        .min_raise_target_end_date
                        .map(to_validation_error_js),
                    image_url: e.image_url.map(to_validation_error_js),
                    prospectus_url: e.prospectus_url.map(to_validation_error_js),
                    prospectus_bytes: e.prospectus_bytes.map(to_validation_error_js),
                    min_invest_shares: e.min_invest_amount.map(to_validation_error_js),
                    max_invest_shares: e.max_invest_amount.map(to_validation_error_js),
                    shares_for_investors: e.shares_for_investors.map(to_validation_error_js),
                };
                match JsValue::from_serde(&errors_js) {
                    Ok(js) => js,
                    Err(e) => to_js_value(e),
                }
            }

            _ => to_js_value(match error {
                ValidateDaoInputsError::AllFieldsValidation(e) => format!("{e:?}"),
                ValidateDaoInputsError::SingleFieldValidation { field, error } => {
                    format!("{field:?} => {error:?}")
                }
                ValidateDaoInputsError::NonValidation(msg) => msg,
            }),
        }
    }
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDaoAssetsParJs {
    pub inputs: CreateDaoFormInputsJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateDaoAssetsResJs {
    pub to_sign: ToSignJs,
    pub pt: CreateDaoPassthroughParJs, // passthrough
}
