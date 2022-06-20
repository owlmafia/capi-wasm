use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait IncomeVsSpendingProvider {
    async fn get(&self, pars: IncomeVsSpendingParJs) -> Result<IncomeVsSpendingResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncomeVsSpendingParJs {
    pub dao_id: String,
    pub interval: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncomeVsSpendingResJs {
    pub points: Vec<ChartDataPointJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartDataPointJs {
    pub date: String,
    pub income: String,
    pub spending: String,
}

#[derive(Debug, Clone)]
pub struct IntervalData {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub interval: Duration,
}

pub fn to_interval_data(interval_str: &str) -> Result<IntervalData> {
    let end = Utc::now();
    match interval_str {
        "days7" => Ok(IntervalData {
            start: end - Duration::days(7),
            end,
            interval: Duration::days(1),
        }),
        "months3" => Ok(IntervalData {
            start: end - Duration::weeks(12),
            end,
            interval: Duration::weeks(1),
        }),
        "year" => Ok(IntervalData {
            start: end - Duration::weeks(48),
            end,
            interval: Duration::weeks(4),
        }),
        _ => Err(anyhow!("Not supported interval str: {:?}", interval_str)),
    }
}
