use serde::Serialize;
use tsify::Tsify;

use crate::provider::create_dao_provider::ValidateDaoInputsError;

/// Note String used for many originally numeric fields: these fields are only to display to the user
/// using String allows to reuse them easily for different numbers, like u64 or Decimal and format them
#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(rename_all(serialize = "camelCase"))]
pub enum ValidationError {
    Empty,
    MinLength {
        min: String,
        actual: String,
    },
    MaxLength {
        max: String,
        actual: String,
    },
    Min {
        min: String,
    },
    Max {
        max: String,
    },
    Address,
    NotPositive, // (greater than 0)
    NotAnInteger,
    // note that this is not caused by the user, but programmatic error - js should always pass a valid date input
    NotTimestamp,
    NotADecimal,
    TooManyFractionalDigits {
        max: String,
        actual: String,
    },
    ShareCountLargerThanAvailable,
    MustBeAfterNow,
    MustBeLessThanMaxInvestAmount,
    MustBeGreaterThanMinInvestAmount,
    SharesForInvestorsGreaterThanSupply,
    BuyingLessSharesThanMinAmount {
        min: String,
    },
    BuyingMoreSharesThanMaxTotalAmount {
        max: String,
        currently_owned: String,
    },
    /// Related to validation but not directly attributable to the user (e.g. overflows when converting entered quantities to base units).
    /// Shouldn't happen normally - the conditions leading to these errors should be validated.
    Unexpected(String),
}

/// Temporary hack for backwards compatibility with previous validation (which returned only a string)
/// TODO all places that can trigger ValidationError should be adjusted in JS to handle the structured validation errors
impl From<ValidationError> for anyhow::Error {
    fn from(error: ValidationError) -> Self {
        anyhow::Error::msg(format!("{error:?}"))
    }
}

/// Temporary hack for backwards compatibility with previous validation (which returned only a string)
/// TODO all places that can trigger ValidationError should be adjusted in JS to handle the structured validation errors
impl From<ValidateDaoInputsError> for anyhow::Error {
    fn from(error: ValidateDaoInputsError) -> Self {
        anyhow::Error::msg(format!("{error:?}"))
    }
}
