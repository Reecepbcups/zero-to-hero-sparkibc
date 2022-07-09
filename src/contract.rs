#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version; // cw2 is a spec which lets users have contract meta&data (name, version)

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Poll, POLLS};

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

    // we need to write unit test now to confirm this code actually works, so we make at the bottom
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    // we remove the _'s so we can actually use them
    deps: DepsMut,
    env: Env, 
    info: MessageInfo, 
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll { question } => execute_create_poll(deps, env, info, question),
        ExecuteMsg::Vote { question, choice } => execute_vote(deps, env, info, question, choice),        
    }
}

// we can call anything, but smart to prefix with execute_
fn execute_create_poll(deps: DepsMut, _env: Env, _info: MessageInfo, question: String) -> Result<Response, ContractError> {
    // does the POLLS map already have the same key (question) of this value? if so we error out.
    if POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError { val: "Key already taken!".to_string() });
    }

    // create poll in memory
    let poll = Poll { question: question.clone(), yes_votes: 0, no_votes: 0 };
    // save poll to chain
    POLLS.save(deps.storage, question, &poll)?;
    // tell user they were success
    Ok(Response::new().add_attribute("action", "create_poll"))
}

fn execute_vote(deps: DepsMut, _env: Env, _info: MessageInfo, question: String, choice: String) -> Result<Response, ContractError> {
    // error out if there no poll with this given Question (key) in the storage on chain
    if !POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError { val: "Poll does not exist!".to_string() });
    }

    let mut poll = POLLS.load(deps.storage, question.clone())?;

    if choice == "yes" {
        poll.yes_votes += 1;
    } else if choice == "no" {
        poll.no_votes += 1;
    } else {
        return Err(ContractError::CustomError { val: "Unrecognized Choice!".to_string() });
    }

    // saves poll -> chain with updated poll value (either +1 yes or +1 no)
    POLLS.save(deps.storage, question.clone(), &poll)?;
    Ok(Response::new().add_attribute("action", "vote"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!() // get data from the contract
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::msg::{InstantiateMsg, ExecuteMsg};
    use crate::contract::execute;

    use super::instantiate;

    // mocking = fake testing values given by the chain

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        // we dont require funds to be sent in, so we set a blank array/list of funds.
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };

        // forces the result, kind of like ? but we need to do it directly here
        let response = instantiate(deps.as_mut(), env, info, msg).unwrap();
        
        // check the response
        assert_eq!(response.attributes, vec![("action".to_string(), "instantiate".to_string())]);
    }

    #[test]
    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };
        let _response = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            question: "What is your favorite color?".to_string(),
        };
        let response = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(response.attributes, vec![("action".to_string(), "create_poll".to_string())]);

        // // lets create a poll this time with the same question so it fails & we can check the error
        // let msg = ExecuteMsg::CreatePoll {
        //     question: "What is your favorite color?".to_string(),
        // };
        // let response = execute(deps.as_mut(), env, info, msg).unwrap_err();
        // assert_eq!(response, "Key already taken!".to_string());       
    }


    #[test]
    fn test_poll_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };
        let _response = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            question: "Do you love spark IBC?".to_string(),
        };
        let _response = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // success case, vote on poll with valid option
        let msg = ExecuteMsg::Vote {
            question: "Do you love spark IBC?".to_string(),
            choice: "yes".to_string(),
        };
        let response = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(response.attributes, vec![("action".to_string(), "vote".to_string())]);

        // ERROR case, vote on poll that does not exist
        let msg = ExecuteMsg::Vote {
            question: "This is not a valid question!".to_string(),
            choice: "no".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // ERROR case, vote on poll that exists with invalid option
        let msg = ExecuteMsg::Vote {
            question: "Do you love spark IBC?".to_string(),
            choice: "this is not a valid choice".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();


    }


}
