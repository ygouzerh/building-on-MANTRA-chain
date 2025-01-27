use crate::state::{Entry, Priority, Status};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde] // Allows this struct to be serialized and deserialized
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    NewEntry {
        description: String,
        priority: Option<Priority>,
        owner: String,
    },
    UpdateEntry {
        id: u64,
        description: Option<String>,
        status: Option<Status>,
        priority: Option<Priority>,
        owner: String,
    },
    DeleteEntry {
        id: u64,
        owner: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ListResponse)]
    QueryUserList { 
        user: String, 
        start_after: Option<u64>, 
        limit: Option<u32> 
    },
}

#[cw_serde]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}