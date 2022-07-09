#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version; // cw2 is a spec which lets users have contract meta&data (name, version)

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetPollResponse, InstantiateMsg, QueryMsg};
use crate::state::{Config, Poll, CONFIG, POLLS};

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
fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String,
) -> Result<Response, ContractError> {
    // does the POLLS map already have the same key (question) of this value? if so we error out.
    if POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError {
            val: "Key already taken!".to_string(),
        });
    }

    // create poll in memory
    let poll = Poll {
        question: question.clone(),
        yes_votes: 0,
        no_votes: 0,
    };
    // save poll to chain
    POLLS.save(deps.storage, question, &poll)?;
    // tell user they were success
    Ok(Response::new().add_attribute("action", "create_poll"))
}

fn execute_vote(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    question: String,
    choice: String,
) -> Result<Response, ContractError> {
    // error out if there no poll with this given Question (key) in the storage on chain
    if !POLLS.has(deps.storage, question.clone()) {
        return Err(ContractError::CustomError {
            val: "Poll does not exist!".to_string(),
        });
    }

    let mut poll = POLLS.load(deps.storage, question.clone())?;

    if choice == "yes" {
        poll.yes_votes += 1;
    } else if choice == "no" {
        poll.no_votes += 1;
    } else {
        return Err(ContractError::CustomError {
            val: "Unrecognized Choice!".to_string(),
        });
    }

    // saves poll -> chain with updated poll value (either +1 yes or +1 no)
    POLLS.save(deps.storage, question.clone(), &poll)?;
    Ok(Response::new().add_attribute("action", "vote"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // get data from the contract
    match msg {
        QueryMsg::GetPoll { question } => query_get_poll(deps, env, question),
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(deps.storage)?),
    }
}

fn query_get_poll(deps: Deps, _env: Env, question: String) -> StdResult<Binary> {
    // encoded binary result
    let poll = POLLS.may_load(deps.storage, question)?;
    to_binary(&GetPollResponse { poll })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, GetPollResponse, InstantiateMsg, QueryMsg};
    use crate::state::{Config, Poll};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr};

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
        let response = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        // check the response
        assert_eq!(
            response.attributes,
            vec![("action".to_string(), "instantiate".to_string())]
        );

        let msg = QueryMsg::GetConfig {};
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let config: Config = from_binary(&resp).unwrap();
        assert_eq!(
            config,
            Config {
                admin_address: Addr::unchecked("addr1")
            }
        );
    }

    #[test]
    fn test_create_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(),
        };

        // Before you execute a contract you need to instantiate it
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            question: "Do you love Spark IBC?".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "create_poll")]);

        let msg = QueryMsg::GetPoll {
            question: "Do you love Spark IBC?".to_string(),
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let get_poll_response: GetPollResponse = from_binary(&resp).unwrap();
        assert_eq!(
            get_poll_response,
            GetPollResponse {
                poll: Some(Poll {
                    question: "Do you love Spark IBC?".to_string(),
                    yes_votes: 0,
                    no_votes: 0
                })
            }
        );

        let msg = ExecuteMsg::CreatePoll {
            question: "Do you love Spark IBC?".to_string(),
        };
        let _resp = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    #[test]
    fn test_poll_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(), // String, String::from("addr1")
        };

        // Before you execute a contract you need to instantiate it
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // We need a poll to vote on!
        let msg = ExecuteMsg::CreatePoll {
            question: "Do you love Spark IBC?".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Success case, we vote on a poll that exists, with a valid option
        let msg = ExecuteMsg::Vote {
            question: "Do you love Spark IBC?".to_string(),
            choice: "yes".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "vote"),]);

        let msg = QueryMsg::GetPoll {
            question: "Do you love Spark IBC?".to_string(),
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let get_poll_response: GetPollResponse = from_binary(&resp).unwrap();
        assert_eq!(
            get_poll_response,
            GetPollResponse {
                poll: Some(Poll {
                    question: "Do you love Spark IBC?".to_string(),
                    yes_votes: 1,
                    no_votes: 0
                })
            }
        );

        // Error case 1: we vote on a poll that does not exist
        let msg = ExecuteMsg::Vote {
            question: "Do you hate Spark IBC?".to_string(),
            choice: "no".to_string(),
        };
        let _resp = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Error case 2: we vote on a poll that exists, but with an invalid choice
        let msg = ExecuteMsg::Vote {
            question: "Do you love Spark IBC?".to_string(),
            choice: "maybe".to_string(),
        };
        let _resp = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
}
