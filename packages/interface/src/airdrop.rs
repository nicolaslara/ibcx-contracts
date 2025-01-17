use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::types::RangeOrder;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum AirdropId {
    Id(u64),
    Label(String),
}

impl AirdropId {
    pub fn id(id: u64) -> Self {
        AirdropId::Id(id)
    }

    pub fn label(label: impl Into<String>) -> Self {
        AirdropId::Label(label.into())
    }
}

#[cw_serde]
pub enum AirdropIdOptional {
    Id(Option<u64>),
    Label(Option<String>),
}

#[cw_serde]
pub enum ClaimProof {
    Account(String),
    ClaimProof(String),
}

#[cw_serde]
pub enum ClaimProofOptional {
    Account(Option<String>),
    ClaimProof(String),
}

impl ClaimProofOptional {
    pub fn account(address: impl Into<String>) -> Self {
        ClaimProofOptional::Account(Some(address.into()))
    }

    pub fn claim_proof(proof: impl Into<String>) -> Self {
        ClaimProofOptional::ClaimProof(proof.into())
    }
}

#[cw_serde]
pub struct ClaimPayload {
    pub id: AirdropId,
    pub amount: Uint128,
    pub claim_proof: ClaimProofOptional,
    pub merkle_proof: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register {
        merkle_root: String,
        denom: String,
        label: Option<String>,
        bearer: Option<bool>,
    },

    Fund {
        id: AirdropId,
    },

    Claim(ClaimPayload),
    MultiClaim(Vec<ClaimPayload>),

    Close {
        id: AirdropId,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAirdropResponse)]
    GetAirdrop { id: AirdropId },

    #[returns(ListAirdropsResponse)]
    ListAirdrops {
        start_after: AirdropIdOptional,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(LatestAirdropResponse)]
    LatestAirdropId {},

    #[returns(GetClaimResponse)]
    GetClaim {
        id: AirdropId,
        claim_proof: ClaimProof,
    },

    #[returns(ListClaimsResponse)]
    ListClaims {
        id: AirdropId,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(CheckQualificationResponse)]
    CheckQualification {
        id: AirdropId,
        amount: Uint128,
        claim_proof: ClaimProof,
        merkle_proof: Vec<String>,
    },
}

#[cw_serde]
pub struct GetAirdropResponse {
    pub id: u64,
    pub creator: String,
    pub label: Option<String>,
    pub denom: String,
    pub total_amount: Uint128,
    pub total_claimed: Uint128,
    pub merkle_root: String,
    pub bearer: bool,
    pub closed: bool,
}

#[cw_serde]
pub struct ListAirdropsResponse(pub Vec<GetAirdropResponse>);

#[cw_serde]
pub struct LatestAirdropResponse(pub u64);

#[cw_serde]
pub struct GetClaimResponse {
    pub amount: Uint128,
    pub claim_proof: ClaimProof,
}

#[cw_serde]
pub struct ListClaimsResponse(pub Vec<GetClaimResponse>);

#[cw_serde]
pub struct CheckQualificationResponse(pub bool);

#[cw_serde]
pub struct MigrateMsg {
    pub force: Option<bool>,
}
