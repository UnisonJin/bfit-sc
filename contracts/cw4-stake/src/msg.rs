use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

pub use cw_controllers::ClaimsResponse;
use cw_utils::Duration;

#[cw_serde]
pub struct InstantiateMsg {
    /// denom of the token to stake
    pub denom: String,
    pub tokens_per_weight: Uint128,
    pub min_bond: Uint128,
    pub unbonding_period: Duration,

    // admin can only add/remove hooks, not change other parameters
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Bond will bond all staking tokens sent with the message and update membership weight
    Bond {},
    /// Unbond will start the unbonding process for the given number of tokens.
    /// The sender immediately loses weight from these tokens, and can claim them
    /// back to his wallet after `unbonding_period`
    Unbond { tokens: Uint128 },
    /// Claim is used to claim your native tokens that you previously "unbonded"
    /// after the contract-defined waiting period (eg. 1 week)
    Claim {},

    /// Change the admin
    UpdateAdmin { admin: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Claims shows the tokens in process of unbonding for this address
    #[returns(cw_controllers::ClaimsResponse)]
    Claims { address: String },

    // Show the number of tokens currently staked by this address.
    #[returns(StakedResponse)]
    Staked { address: String },

    #[returns(cw_controllers::AdminResponse)]
    Admin {},

    #[returns(cw4::TotalWeightResponse)]
    TotalWeight {},

    #[returns(cw4::MemberListResponse)]
    ListMembers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(cw4::MemberResponse)]
    Member {
        addr: String,
        at_height: Option<u64>,
    },
}

#[cw_serde]
pub struct StakedResponse {
    pub stake: Uint128,
    pub denom: String,
}
