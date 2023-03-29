#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Reply, CosmosMsg, WasmMsg};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{STATE, State};

use cw0::parse_reply_instantiate_data;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:project-name";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    STATE.save(
        deps.storage,
        &State {
            child_codeid: msg.child_codeid,
            children: vec![],
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewContract {  } => new_contract(deps, env),
    }
}

fn new_contract(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let instantiate_child_msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        code_id: state.child_codeid,
        funds: vec![],
        msg: to_binary(&child::msg::InstantiateMsg {}).unwrap(),
        label: "child contract".to_string(),
        admin: Some(env.contract.address.to_string()),
    });
    Ok(Response::new()
        .add_message(instantiate_child_msg)
        .add_attribute("action", "new_contract"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Children {} => to_binary(&query_children(deps)?),
    }
}

fn query_children(deps: Deps) -> StdResult<Vec<String>> {
    match STATE.may_load(deps.storage)? {
        Some(state) => Ok(state.children),
        None => Ok(vec![]),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    // parse the reply data so we can get the contract address
    let res = parse_reply_instantiate_data(msg.clone())
    .map_err(|e| ContractError::ParseReplyError(e.to_string()))?;

    let child_contract = deps.api.addr_validate(&res.contract_address)?;
 
    // add the contract address to the list of children in state
    let mut state = STATE.load(deps.storage)?;
    state.children.push(child_contract.to_string());
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    const OWNER: &str = "owner";

    fn init_contract(deps: DepsMut, env: Env) {();
        let _res = instantiate(
            deps,
            env,
            mock_info(OWNER, &[]),
            InstantiateMsg {
                child_codeid: 1,
            },
        )
        .unwrap();
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let current_env = mock_env();
        init_contract(deps.as_mut(), current_env);
        let state: State = STATE.load(deps.as_ref().storage).unwrap();
        assert_eq!(state.child_codeid, 1);
    }
}
