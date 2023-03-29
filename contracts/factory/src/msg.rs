use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub child_codeid: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    NewContract {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ChildrenResponse)]
    Children {},
}

#[cw_serde]
pub struct ChildrenResponse {
    pub children: Vec<String>,
}
