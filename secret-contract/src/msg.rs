use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Strategy;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    
    InitializeStrategy{
        owner: String, 
        asset_to_sell: String,
        asset_to_buy: String, 
        total_amount: i32},
        PerformEncryptedSwap {strategy_id: u32},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryStrategy {
        id: u32
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct StrategyResponse {
    pub strategy: Strategy,
}

