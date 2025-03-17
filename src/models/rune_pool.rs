use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "endTime")]
    pub end_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "startCount")]
    pub start_count: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "endCount")]
    pub end_count: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "startUnits")]
    pub start_units: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "endUnits")]
    pub end_units: u64,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interval {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "endTime")]
    pub end_time: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub count: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub units: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunePoolResponse {
    pub meta: Meta,
    pub intervals: Vec<Interval>,
}
