use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use cosmwasm_std::Uint128;

pub const BLOCK_SIZE: usize = 256;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Hop {
    pub addr: String,
    pub code_hash: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq ,JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RouterInvokeMsg {
    SwapTokensForExact {
        path: Vec<Hop>,
        expected_return: Option<Uint128>,
        recipient: Option<String>,
    },
}
