use crate::dependencies::FundsAssetSpecs;
use crate::inputs_validation::ValidationError;
use crate::js::common::SignedTxFromJs;
use crate::js::js_types_workarounds::VersionedContractAccountJs;
use crate::model::dao_for_users_view_data::DaoForUsersViewData;
use crate::service::str_to_algos::validate_funds_amount_input;
use algonaut::core::Address;
use anyhow::Result;
use async_trait::async_trait;
use base::flows::create_dao::model::CreateSharesSpecs;
use base::flows::create_dao::setup_dao_specs::SetupDaoSpecs;
use base::flows::create_dao::share_amount::ShareAmount;
use base::flows::create_dao::shares_percentage::SharesPercentage;
use base::funds::FundsAmount;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryInto;
use std::fmt::Debug;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateDaoProvider {
    async fn txs(&self, pars: CreateDaoParJs) -> Result<CreateDaoResJs>;
    /// create daos specs + signed assets txs -> create dao result
    /// submits the signed assets, creates rest of dao with generated asset ids
    async fn submit(&self, pars: SubmitCreateDaoParJs) -> Result<DaoForUsersViewData>;
}

pub struct ValidatedDaoInputs {
    pub name: String,
    pub description: String,
    pub creator: Address,
    pub token_name: String,
    pub share_supply: ShareAmount,
    pub share_price: FundsAmount,
    pub investors_part: SharesPercentage,
    pub logo_url: String,
    pub social_media_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDaoFormInputsJs {
    pub creator: String, // not strictly a form input ("field"), but for purpose here it can be
    pub dao_name: String,
    pub dao_description: String,
    pub share_count: String,
    pub share_price: String,
    pub investors_part: String, // percentage (0..100), with decimals (max decimals number defined in validations)
    pub logo_url: String,
    pub social_media_url: String,
}

impl CreateDaoFormInputsJs {
    pub fn to_dao_specs(
        &self,
        funds_asset_specs: &FundsAssetSpecs,
    ) -> Result<SetupDaoSpecs, ValidateDaoInputsError> {
        let validated_inputs = validate_dao_inputs(self, funds_asset_specs)?;
        Ok(validated_inputs_to_dao_specs(validated_inputs))
    }
}

fn validated_inputs_to_dao_specs(inputs: ValidatedDaoInputs) -> SetupDaoSpecs {
    SetupDaoSpecs {
        name: inputs.name,
        description: inputs.description,
        shares: CreateSharesSpecs {
            token_name: inputs.token_name,
            supply: inputs.share_supply,
        },
        investors_part: inputs.investors_part,
        share_price: inputs.share_price,
        logo_url: inputs.logo_url,
        social_media_url: inputs.social_media_url,
    }
}

pub fn validate_dao_inputs(
    inputs: &CreateDaoFormInputsJs,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<ValidatedDaoInputs, ValidateDaoInputsError> {
    let dao_name_res = validate_dao_name(&inputs.dao_name);
    let dao_description_res = validate_dao_description(&inputs.dao_description);
    let creator_address_res = validate_address(&inputs.creator);
    let share_supply_res = validate_share_supply(&inputs.share_count);
    let share_price_res = validate_share_price(&inputs.share_price, funds_asset_specs);
    let logo_url_res = validate_logo_url(&inputs.logo_url);
    let social_media_url_res = validate_social_media_url(&inputs.social_media_url);
    let investors_part_res = validate_investors_share(&inputs.investors_part);

    let dao_name_err = dao_name_res.clone().err();
    let dao_description_err = dao_description_res.clone().err();
    let creator_address_err = creator_address_res.clone().err();
    let share_supply_err = share_supply_res.clone().err();
    let share_price_err = share_price_res.clone().err();
    let logo_url_err = logo_url_res.clone().err();
    let social_media_url_err = social_media_url_res.clone().err();
    let investors_part_err = investors_part_res.clone().err();

    if [
        dao_name_err,
        dao_description_err,
        creator_address_err,
        share_supply_err,
        share_price_err,
        logo_url_err,
        social_media_url_err,
        investors_part_err,
    ]
    .iter()
    .any(|e| e.is_some())
    {
        let errors = CreateAssetsInputErrors {
            name: dao_name_res.err(),
            description: dao_description_res.err(),
            creator: creator_address_res.err(),
            share_supply: share_supply_res.err(),
            share_price: share_price_res.err(),
            investors_share: investors_part_res.err(),
            logo_url: logo_url_res.err(),
            social_media_url: social_media_url_res.err(),
        };
        return Err(ValidateDaoInputsError::AllFieldsValidation(errors));
    }

    // Note error handling here: these errors *should* not happen, as there are caught above.
    // this is to protect from programmatic errors - being careful, because we want to avoid crashes in WASM at any cost.
    // ideally ensure it via the compiler - couldn't find how quickly other than nesting all the validations with match which is not a great alternative.
    let dao_name = dao_name_res.map_err(|e| to_single_field_val_error("dao_name", e))?;
    let dao_description =
        dao_description_res.map_err(|e| to_single_field_val_error("dao_description", e))?;
    let creator_address =
        creator_address_res.map_err(|e| to_single_field_val_error("creator_address", e))?;
    let investors_part =
        investors_part_res.map_err(|e| to_single_field_val_error("investors_share", e))?;
    let share_supply =
        share_supply_res.map_err(|e| to_single_field_val_error("share_supply", e))?;
    let share_price = share_price_res.map_err(|e| to_single_field_val_error("share_price", e))?;
    let logo_url = logo_url_res.map_err(|e| to_single_field_val_error("logo_url", e))?;
    let social_media_url =
        social_media_url_res.map_err(|e| to_single_field_val_error("social_media_url", e))?;

    // derived from other fields
    let asset_name = generate_asset_name(&dao_name).map_err(|_| {
        ValidateDaoInputsError::NonValidation(format!(
            "Error generating asset name, based on: {dao_name}"
        ))
    })?;

    Ok(ValidatedDaoInputs {
        name: dao_name,
        description: dao_description,
        creator: creator_address,
        token_name: asset_name,
        share_supply,
        share_price,
        investors_part,
        logo_url,
        social_media_url,
    })
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDaoParJs {
    pub pt: CreateDaoPassthroughParJs,
    // same order as the unsigned txs were sent to JS
    pub create_assets_signed_txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDaoPassthroughParJs {
    pub inputs: CreateDaoFormInputsJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateDaoResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitSetupDaoPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize)]
pub enum ValidateDaoInputsError {
    AllFieldsValidation(CreateAssetsInputErrors),
    SingleFieldValidation {
        field: String,
        error: ValidationError,
    },
    NonValidation(String),
}

/// Errors to be shown next to the respective input fields
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateAssetsInputErrors {
    pub name: Option<ValidationError>,
    pub description: Option<ValidationError>,
    pub creator: Option<ValidationError>,
    pub share_supply: Option<ValidationError>,
    pub share_price: Option<ValidationError>,
    pub investors_share: Option<ValidationError>,
    pub logo_url: Option<ValidationError>,
    pub social_media_url: Option<ValidationError>,
}

fn validate_dao_name(name: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(name, 2, 40)
}

fn validate_dao_description(descr: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(descr, 0, 200)
}

fn validate_address(str: &str) -> Result<Address, ValidationError> {
    str.parse().map_err(|_| ValidationError::Address)
}

fn validate_text_min_max_length(
    text: &str,
    min: usize,
    max: usize,
) -> Result<String, ValidationError> {
    let text = text.trim();

    let dao_name_len = text.len();
    if dao_name_len < min {
        return Err(ValidationError::MinLength {
            min: min.to_string(),
            actual: dao_name_len.to_string(),
        });
    }
    if dao_name_len > max {
        return Err(ValidationError::MaxLength {
            max: max.to_string(),
            actual: dao_name_len.to_string(),
        });
    }

    Ok(text.to_owned())
}

fn generate_asset_name(validated_dao_name: &str) -> Result<String> {
    let mut asset_name = validated_dao_name;
    let asset_name_max_length = 7;
    if validated_dao_name.len() > asset_name_max_length {
        asset_name = &asset_name[0..asset_name_max_length];
    }
    Ok(asset_name.to_owned())
}

fn validate_share_supply(input: &str) -> Result<ShareAmount, ValidationError> {
    let share_count: u64 = input.parse().map_err(|_| ValidationError::NotAnInteger)?;
    if share_count == 0 {
        return Err(ValidationError::Min {
            min: 1u8.to_string(),
            actual: share_count.to_string(),
        });
    }
    Ok(ShareAmount::new(share_count))
}

fn validate_share_price(
    input: &str,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<FundsAmount, ValidationError> {
    validate_funds_amount_input(input, funds_asset_specs)
}

fn validate_investors_share(input: &str) -> Result<SharesPercentage, ValidationError> {
    let value = input
        .parse::<Decimal>()
        .map_err(|_| ValidationError::NotADecimal)?;

    let min = 0u8.into();
    let max = 100u8.into();

    if value < min {
        Err(ValidationError::Min {
            min: min.to_string(),
            actual: value.to_string(),
        })
    } else if value > max {
        Err(ValidationError::Max {
            max: max.to_string(),
            actual: value.to_string(),
        })
    } else {
        // from here we use (0..1) percentage - 100 based is just for user friendliness
        (value / Decimal::from(100u8)).try_into().map_err(|_| {
            ValidationError::Unexpected(format!("Couldn't divide {value} by 100").to_owned())
        })
    }
}

fn validate_logo_url(input: &str) -> Result<String, ValidationError> {
    let count = input.len();
    let max_chars = 100;
    if count > max_chars {
        return Err(ValidationError::MaxLength {
            max: max_chars.to_string(),
            actual: count.to_string(),
        });
    }
    Ok(input.to_owned())
}

fn validate_social_media_url(input: &str) -> Result<String, ValidationError> {
    let count = input.len();
    let max_chars = 100;
    if count > max_chars {
        return Err(ValidationError::MaxLength {
            max: max_chars.to_string(),
            actual: count.to_string(),
        });
    }
    Ok(input.to_owned())
}

fn to_single_field_val_error(field_name: &str, e: ValidationError) -> ValidateDaoInputsError {
    ValidateDaoInputsError::SingleFieldValidation {
        field: field_name.to_owned(),
        error: e,
    }
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitCreateDaoParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitSetupDaoPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSetupDaoPassthroughParJs {
    pub specs: SetupDaoSpecs,
    // not sure how to passthrough, if we use Address, when deserializing, we get:
    // index.js:1 Error("invalid type: sequence, expected a 32 byte array", line: 1, column: 10711)
    // looking at the logs, the passed JsValue looks like an array ([1, 2...])
    pub creator: String,
    // can't use SignedTransactions because of deserialization issue mainly (only?) with Address
    // see note on `creator` above
    // Note: multiple transactions: the tx vector is serialized into a single u8 vector
    pub customer_escrow_optin_to_funds_asset_tx_msg_pack: Vec<u8>,
    pub shares_asset_id: u64,
    pub customer_escrow: VersionedContractAccountJs,
    pub app_id: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitCreateDaoResJs {
    // next step tx: save the dao
    pub to_sign: Value,
}
