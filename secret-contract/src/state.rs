use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use cosmwasm_std::Uint128;
use secret_toolkit::storage::Keymap;

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

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Strategy {
pub owner: String, 
pub asset_to_sell: String,
pub asset_to_buy: String, 
pub total_amount: i32
}

pub static STRATEGIES: Keymap<u32, Strategy> = Keymap::new(b"user strategies");

