use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128
};

use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, StrategyResponse};
use crate::state::{Hop, RouterInvokeMsg, Strategy, BLOCK_SIZE, STRATEGIES};
use secret_toolkit::snip20::send_from_msg_with_code_hash;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
 
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::PerformEncryptedSwap {strategy_id} => try_perform_encrypted_swap(deps, env, &info, strategy_id),
        ExecuteMsg::InitializeStrategy{
                    owner,
                    asset_to_sell,
                    asset_to_buy,
                    total_amount,
            } => try_initialize_strategy(deps, env, &info,  owner,
                asset_to_sell,
                asset_to_buy,
                total_amount),
    }}

    fn try_initialize_strategy(
        deps: DepsMut,
        _env: Env,
        _info: &MessageInfo,
        owner: String,
        asset_to_sell: String,
        asset_to_buy: String,
        total_amount: i32,
    ) -> StdResult<Response> {
        // Create a new strategy from the input parameters
        let new_strategy = Strategy {
            owner,
            asset_to_sell,
            asset_to_buy,
            total_amount,
        };
    
        let strategy_key = STRATEGIES.get_len(deps.storage)?;
    
        // Insert the new strategy into the Keymap
        STRATEGIES.insert(deps.storage, &strategy_key, &new_strategy)?;
    
        // Return success response
        Ok(Response::new()
            .add_attribute("method", "try_initialize_strategy")
            .add_attribute("strategy_key", strategy_key.to_string()))
    }
    
fn try_perform_encrypted_swap(
    deps: DepsMut,
   _env: Env,
   info: &MessageInfo,
   strategy_id: u32, 
  
) -> StdResult<Response> {
// Step 1: Query the strategy by the strategy_id
let strategy = STRATEGIES
.get(deps.storage, &strategy_id).unwrap(); 

let amount = strategy.total_amount;

// token address + codehash for sUSDC
let token_in_address = "secret17d24y82ccnar8hlxmlkfur35pykl520hmn4uy0";
let token_in_code_hash = "1691e4e24714e324a8d2345183027a918bba5c737bb2cbdbedda3cf8e7672faf";

// Swap path for USDC -> sSCRT
let swap_path = vec![
   Hop {
       addr: "secret1hqfl9fmwvljsyd5keydr7sg3ak3v0yfux7nsga".to_string(),
       code_hash: "2f4042b96baaa01fcd66456b33a6e3447880903010294a83ec7294ae110afa74".to_string(),
   }
];

// Prepare the message for the swap
let swap_msg = RouterInvokeMsg::SwapTokensForExact {
   expected_return: None,
   path: swap_path,
   recipient: Some(info.sender.to_string()),
};

let msg: Option<Binary> = Some(to_binary(&swap_msg)?);

let uint128amount = convert_to_uint128(amount);

let shade_swap_router_address = "secret137sjm7hgqdp4d0dldqnrxe2ktw02meaygnjd0e".to_string();
let shade_swap_router_code_hash = "93dac48bf508eeb4c619fcb8b1cb260f9957e31450740a2b7325440ddf92daa8".to_string();

let send_msg = send_from_msg_with_code_hash(
    info.sender.to_string(),
    shade_swap_router_address,
    Some(shade_swap_router_code_hash),
    uint128amount,
    msg,  
    None,
    None,
    BLOCK_SIZE,
    token_in_code_hash.to_string(),
    token_in_address.to_string(),
)?;

let messages = vec![send_msg];  // Add CosmosMsg to messages


Ok(Response::new()
   .add_messages(messages))
}

fn convert_to_uint128(amount: i32) -> Uint128 {
    Uint128::from(amount as u128)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg:: QueryStrategy {
            id
        } => to_binary(&query_strategy(deps, id)?),
    }
}

fn query_strategy(deps: Deps, id: u32) -> StdResult<StrategyResponse> {
    let strategy = STRATEGIES
        .get(deps.storage, &id);
       
        match strategy {
            Some(strategy) => Ok(StrategyResponse{strategy: strategy}),
            None => Err(StdError::generic_err("strategy doesn't exist")),
        }
}


