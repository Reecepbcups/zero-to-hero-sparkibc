#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version; // cw2 is a spec which lets users have contract meta&data (name, version)

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

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
    
    // saving a variable -> the deps.api address validate function
    // we pass by value which is the admin_address, so &msg.admin_address gets the admin address we defined in the message.
    // This errors our if the users address is not valid. If we input "foo" = fail. If you put a juno address = success
    // If its successfull, it returns the type Addr (address) as we wanted
    let validated_address_address = deps.api.addr_validate(&msg.admin_address)?;

    let config = Config {
        // we created a config with the admin address we wanted to be in the contract when we created it
        admin_address: validated_address_address,    
    };

    // we need to save config -> state on chain.
    // So now we save this config to the storage on chain
    CONFIG.save(deps.storage, &config)?;

    // returns a result response Result<Response>
    // This has been successful, create a blank response.
    Ok(Response::new().add_attribute("action", "instantiate"))
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
