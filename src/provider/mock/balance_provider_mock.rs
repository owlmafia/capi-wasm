use crate::{
    error::FrError,
    model::QuantityChangeJs,
    provider::balance_provider::{
        BalanceChangeParJs, BalanceChangeResJs, BalanceParJs, BalanceProvider, BalanceResJs,
    },
};
use anyhow::Result;
use async_trait::async_trait;

use super::req_delay;

pub struct BalanceProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BalanceProvider for BalanceProviderMock {
    async fn get(&self, _: BalanceParJs) -> Result<BalanceResJs, FrError> {
        req_delay().await;

        Ok(BalanceResJs {
            balance_algos: "123.45".to_owned(),
            balance_funds_asset: "111.22".to_owned(),
        })
    }

    async fn get_balance_change(
        &self,
        _: BalanceChangeParJs,
    ) -> Result<BalanceChangeResJs, FrError> {
        Ok(BalanceChangeResJs {
            change: QuantityChangeJs::Up.to_owned(),
        })
    }
}
