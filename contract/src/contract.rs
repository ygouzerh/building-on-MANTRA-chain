#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;
use cw_storage_plus::Bound;
use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ListResponse, QueryMsg};
use crate::state::{Entry, Priority, Status, ENTRY_SEQ, LIST};

// version info for migration
const CONTRACT_NAME: &str = "to-do-list";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ENTRY_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::NewEntry {
            description,
            priority,
            owner,
        } => execute_create_new_entry(deps, info, description, priority, owner),
        ExecuteMsg::UpdateEntry {
            id,
            description,
            status,
            priority,
            owner,
        } => {
            // execute_update_entry(deps, info, id, description, status, priority, owner)
            Ok(Response::new())
        }
        ExecuteMsg::DeleteEntry {
            id,
            owner,
        }=> {
            // execute_delete_entry(deps, info, id, owner)
            Ok(Response::new())
        }
        // ExecuteMsg::UpdateEntry {
        //     id,
        //     description,
        //     status,
        //     priority,
        //     owner
        // } => execute_update_entry(deps, info, id, description, status, priority, owner),
        // ExecuteMsg::DeleteEntry { id, owner } => execute_delete_entry(deps, info, id, owner),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryUserList { user, start_after, limit } => {
            to_binary(&query_user_list(deps, user, start_after, limit)?)
        }
    }
}

pub fn execute_create_new_entry(
    deps: DepsMut,
    info: MessageInfo,
    description: String,
    priority: Option<Priority>,
    owner: String,
) -> Result<Response, ContractError> {
    let id = ENTRY_SEQ.update(deps.storage, |id| -> StdResult<_> {Ok(id.add(1))})?;
    let new_entry = Entry {
        id,
        description,
        status: Status::ToDo,
        priority: priority.unwrap_or(Priority::None),
        owner,
    };
    LIST.save(deps.storage, id, &new_entry)?;
    Ok(Response::new()
        .add_attribute("method", "execute_create_new_entry")
        .add_attribute("new_entry_id", id.to_string()))
}

// Limits for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn query_user_list(
    deps: Deps, 
    user: String, 
    start_after: Option<u64>, 
    limit: Option<u32>
) -> StdResult<ListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let entries: StdResult<Vec<_>> = LIST
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|item| {
            if let Ok((_, entry)) = item {
                entry.owner == user
            } else {
                false
            }
        })
        .take(limit)
        .collect();

    let result = ListResponse {
        entries: entries?.into_iter().map(|(_, entry)| entry).collect(),
    };
    Ok(result)
}
