use crate::teal::{customer_escrow, dao_app_approval, dao_app_clear, investing_escrow};
use anyhow::{anyhow, Result};
use core::{
    api::{
        api::Api,
        contract::Contract,
        version::{Version, VersionedTealSourceTemplate, Versions},
    },
    teal::TealSourceTemplate,
};

pub struct TealStringsApi {}

impl Api for TealStringsApi {
    fn last_versions(&self) -> Versions {
        Versions {
            app_approval: Version(1),
            app_clear: Version(1),
            customer_escrow: Version(1),
            investing_escrow: Version(1),
        }
    }

    fn template(
        &self,
        contract: Contract,
        version: Version,
    ) -> Result<VersionedTealSourceTemplate> {
        match contract {
            Contract::DaoCustomer => dao_customer_teal(version),
            Contract::DaoInvesting => dao_investing_teal(version),
            Contract::DaoAppApproval => dao_app_approval_teal(version),
            Contract::DaoAppClear => dao_app_clear_teal(version),
            Contract::CapiAppApproval | Contract::CapiAppClear | Contract::CapiCentral => Err(
                anyhow!("Contract not supported/neeeded in WASM: {contract:?}"),
            ),
        }
    }
}

fn dao_customer_teal(version: Version) -> Result<VersionedTealSourceTemplate> {
    match version.0 {
        1 => load_versioned_teal_template(version, customer_escrow::SRC),
        _ => not_found_err("dao customer", version),
    }
}

fn dao_investing_teal(version: Version) -> Result<VersionedTealSourceTemplate> {
    match version.0 {
        1 => load_versioned_teal_template(version, investing_escrow::SRC),
        _ => not_found_err("dao investing", version),
    }
}

fn dao_app_approval_teal(version: Version) -> Result<VersionedTealSourceTemplate> {
    match version.0 {
        1 => load_versioned_teal_template(version, dao_app_approval::SRC),
        _ => not_found_err("dao app", version),
    }
}

fn dao_app_clear_teal(version: Version) -> Result<VersionedTealSourceTemplate> {
    match version.0 {
        1 => load_versioned_teal_template(version, dao_app_clear::SRC),
        _ => not_found_err("dao app", version),
    }
}

fn load_versioned_teal_template(
    version: Version,
    str: &str,
) -> Result<VersionedTealSourceTemplate> {
    Ok(VersionedTealSourceTemplate {
        version,
        template: TealSourceTemplate(str.as_bytes().to_vec()),
    })
}

fn not_found_err<T>(id: &str, version: Version) -> Result<T> {
    Err(anyhow!("Not found version: {version:?} for: {id}"))
}
