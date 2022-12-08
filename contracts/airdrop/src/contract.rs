use cosmwasm_std::{
    attr, coins, entry_point, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo,
    Order, Response, StdResult, Uint128,
};
use cw_storage_plus::Bound;
use ibc_interface::{
    airdrop::{
        AirdropId, AirdropIdOptional, CheckQualificationResponse, ExecuteMsg, GetAirdropResponse,
        GetClaimResponse, InstantiateMsg, LatestAirdropResponse, ListAirdropsResponse,
        ListClaimsResponse, MigrateMsg, QueryMsg,
    },
    get_and_check_limit,
    types::RangeOrder,
    DEFAULT_LIMIT, MAX_LIMIT,
};
use sha2::Digest;

use crate::{
    error::ContractError,
    state::{Airdrop, AIRDROPS, CLAIM_LOGS, LABELS, LATEST_AIRDROP_ID},
    CONTRACT_NAME, CONTRACT_VERSION,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    LATEST_AIRDROP_ID.save(deps.storage, &0)?;

    Ok(Default::default())
}

// verify merkle proof (from https://github.com/cosmwasm/cw-tokens/blob/master/contracts/cw20-merkle-airdrop/src/contract.rs)
fn verify_merkle_proof(
    root: &str,
    proof: Vec<String>,
    verifier: &Addr,
    amount: Uint128,
) -> Result<(), ContractError> {
    let user_input = format!("{}{}", verifier, amount);

    let hash = sha2::Sha256::digest(user_input.as_bytes())
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongLength {})?;

    let hash = proof.into_iter().try_fold(hash, |hash, p| {
        let mut proof_buf = [0; 32];
        hex::decode_to_slice(p, &mut proof_buf)?;
        let mut hashes = [hash, proof_buf];
        hashes.sort_unstable();
        sha2::Sha256::digest(&hashes.concat())
            .as_slice()
            .try_into()
            .map_err(|_| ContractError::WrongLength {})
    })?;

    let mut root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(&root, &mut root_buf)?;
    if root_buf != hash {
        return Err(ContractError::InvalidProof {});
    }

    Ok(())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Regsiter {
            merkle_root,
            denom,
            label,
        } => {
            let received = cw_utils::must_pay(&info, &denom)?;

            // check merkle root length
            let mut root_buf: [u8; 32] = [0; 32];
            hex::decode_to_slice(&merkle_root, &mut root_buf)?;

            let airdrop_id = LATEST_AIRDROP_ID.load(deps.storage)?;
            LATEST_AIRDROP_ID.save(deps.storage, &(airdrop_id + 1))?;

            if let Some(label) = label {
                if LABELS.has(deps.storage, label.clone()) {
                    return Err(ContractError::KeyAlreadyExists {
                        typ: "label".to_string(),
                        key: label,
                    });
                }

                LABELS.save(deps.storage, label, &airdrop_id)?;
            }

            AIRDROPS.save(
                deps.storage,
                airdrop_id,
                &Airdrop {
                    merkle_root: merkle_root.clone(),
                    denom,
                    total_amount: received,
                    total_claimed: Uint128::zero(),
                },
            )?;

            Ok(Response::new().add_attributes(vec![
                attr("action", "register"),
                attr("executor", info.sender),
                attr("merkle_root", merkle_root),
                attr("total_amount", received),
            ]))
        }
        Fund { id } => {
            let airdrop_id = match id {
                AirdropId::Id(id) => id,
                AirdropId::Label(label) => LABELS.load(deps.storage, label)?,
            };
            let mut airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;

            let received = cw_utils::must_pay(&info, &airdrop.denom)?;
            airdrop.total_amount = airdrop.total_amount.checked_add(received)?;

            AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;

            Ok(Response::new().add_attributes(vec![
                attr("action", "fund"),
                attr("executor", info.sender),
                attr("airdrop_id", airdrop_id.to_string()),
                attr("amount", received),
            ]))
        }
        Claim {
            id,
            amount,
            beneficiary,
            merkle_proof,
        } => {
            let airdrop_id = match id {
                AirdropId::Id(id) => id,
                AirdropId::Label(label) => LABELS.load(deps.storage, label)?,
            };

            let verifier = beneficiary
                .map(|v| deps.api.addr_validate(&v))
                .transpose()?
                .unwrap_or_else(|| info.sender.clone());

            if CLAIM_LOGS
                .may_load(deps.storage, (airdrop_id, verifier.clone()))?
                .is_some()
            {
                return Err(ContractError::AlreadyClaimed {
                    airdrop_id,
                    claimer: verifier,
                });
            }

            // verify merkle proof (from https://github.com/cosmwasm/cw-tokens/blob/master/contracts/cw20-merkle-airdrop/src/contract.rs)
            let mut airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;
            verify_merkle_proof(&airdrop.merkle_root, merkle_proof, &verifier, amount)?;

            CLAIM_LOGS.save(deps.storage, (airdrop_id, verifier.clone()), &amount)?;

            airdrop.total_claimed = airdrop.total_claimed.checked_add(amount)?;
            AIRDROPS.save(deps.storage, airdrop_id, &airdrop)?;

            Ok(Response::new()
                .add_message(BankMsg::Send {
                    to_address: verifier.to_string(),
                    amount: coins(amount.u128(), airdrop.denom),
                })
                .add_attributes(vec![
                    attr("action", "claim"),
                    attr("executor", info.sender),
                    attr("airdrop_id", airdrop_id.to_string()),
                    attr("beneficiary", verifier),
                    attr("amount", amount),
                ]))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use QueryMsg::*;

    match msg {
        GetAirdrop { id } => {
            let (airdrop_id, label) = match id {
                AirdropId::Id(id) => (id, None),
                AirdropId::Label(l) => (LABELS.load(deps.storage, l.clone())?, Some(l)),
            };
            let airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;

            Ok(to_binary(&GetAirdropResponse {
                id: airdrop_id,
                label,
                denom: airdrop.denom,
                total_amount: airdrop.total_amount,
                total_claimed: airdrop.total_claimed,
            })?)
        }
        ListAirdrops {
            start_after,
            limit,
            order,
        } => {
            let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
            let order = order.unwrap_or(RangeOrder::Asc).into();

            let resps = match start_after {
                AirdropIdOptional::Id(id) => {
                    let (min, max) = match order {
                        Order::Ascending => (id.map(Bound::exclusive), None),
                        Order::Descending => (None, id.map(Bound::exclusive)),
                    };

                    AIRDROPS
                        .range(deps.storage, min, max, order)
                        .take(limit)
                        .map(|item| {
                            let (k, v) = item?;

                            Ok(GetAirdropResponse {
                                id: k,
                                label: None,
                                denom: v.denom,
                                total_amount: v.total_amount,
                                total_claimed: v.total_claimed,
                            })
                        })
                        .collect::<StdResult<_>>()?
                }
                AirdropIdOptional::Label(l) => {
                    let start = l.map(|v| deps.api.addr_validate(&v)).transpose()?;
                    let (min, max) = match order {
                        Order::Ascending => (start.map(Bound::exclusive), None),
                        Order::Descending => (None, start.map(Bound::exclusive)),
                    };

                    LABELS
                        .range(deps.storage, min, max, order)
                        .take(limit)
                        .map(|item| {
                            let (k, v) = item?;

                            let airdrop = AIRDROPS.load(deps.storage, v)?;

                            Ok(GetAirdropResponse {
                                id: v,
                                label: Some(k),
                                denom: airdrop.denom,
                                total_amount: airdrop.total_amount,
                                total_claimed: airdrop.total_claimed,
                            })
                        })
                        .collect::<StdResult<_>>()?
                }
            };

            Ok(to_binary(&ListAirdropsResponse(resps))?)
        }
        LatestAirdropId {} => Ok(to_binary(&LatestAirdropResponse(
            LATEST_AIRDROP_ID.load(deps.storage)?,
        ))?),
        GetClaim { id, account } => {
            let airdrop_id = match id {
                AirdropId::Id(id) => id,
                AirdropId::Label(l) => LABELS.load(deps.storage, l)?,
            };
            let account = deps.api.addr_validate(&account)?;
            let amount = CLAIM_LOGS.load(deps.storage, (airdrop_id, account.clone()))?;

            Ok(to_binary(&GetClaimResponse { amount, account })?)
        }
        ListClaims {
            id,
            start_after,
            limit,
            order,
        } => {
            let airdrop_id = match id {
                AirdropId::Id(id) => id,
                AirdropId::Label(l) => LABELS.load(deps.storage, l)?,
            };

            let start = start_after
                .map(|v| deps.api.addr_validate(&v))
                .transpose()?;
            let limit = get_and_check_limit(limit, MAX_LIMIT, DEFAULT_LIMIT)? as usize;
            let order = order.unwrap_or(RangeOrder::Asc).into();
            let (min, max) = match order {
                Order::Ascending => (start.map(Bound::exclusive), None),
                Order::Descending => (None, start.map(Bound::exclusive)),
            };

            let resps = CLAIM_LOGS
                .prefix(airdrop_id)
                .range(deps.storage, min, max, order)
                .take(limit)
                .map(|item| {
                    let (k, v) = item?;

                    Ok(GetClaimResponse {
                        account: k,
                        amount: v,
                    })
                })
                .collect::<StdResult<_>>()?;

            Ok(to_binary(&ListClaimsResponse(resps))?)
        }
        CheckQualification {
            id,
            amount,
            beneficiary,
            merkle_proof,
        } => {
            let airdrop_id = match id {
                AirdropId::Id(id) => id,
                AirdropId::Label(l) => LABELS.load(deps.storage, l)?,
            };
            let airdrop = AIRDROPS.load(deps.storage, airdrop_id)?;
            let account = deps.api.addr_validate(&beneficiary)?;

            Ok(to_binary(&CheckQualificationResponse(
                verify_merkle_proof(&airdrop.merkle_root, merkle_proof, &account, amount).is_ok(),
            ))?)
        }
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Default::default())
}
