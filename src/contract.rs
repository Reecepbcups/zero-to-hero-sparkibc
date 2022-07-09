#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version; // cw2 is a spec which lets users have contract metadata (name, version)

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

const CONTRACT_NAME: &str = "crates.io:zero-to-hero-discord";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); // Config.toml -> [package] -> version

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env, // stores environment variables in the contract, such as the contract address, block height, block time, chain-id, transaction
    _info: MessageInfo, // metadata of the address _info.sender = our address. We could use this for our admin address. _info.funds = funds from the chain
    msg: InstantiateMsg, // Set up the contract
) -> Result<Response, ContractError> {
    // deps.storage - DepsMut is a mutuable/Changable section which we can alter.
    // ?; -> if there is an error, it will stop the contract from running (unwraps the error, think of it like a null check, of null panic() basically)
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    msg.admin_address.parse()?;
    
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env, 
    _info: MessageInfo, 
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!() // get data from the contract
}

#[cfg(test)]
mod tests {}
