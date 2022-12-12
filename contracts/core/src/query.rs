use cosmwasm_std::{to_binary, Binary, Coin, Deps, Env, Uint128};
use ibc_interface::core::{
    GetConfigResponse, GetPauseInfoResponse, GetPortfolioResponse, SimulateBurnResponse,
    SimulateMintResponse,
};

use crate::{
    error::ContractError,
    state::{assert_assets, get_assets, get_redeem_amounts, GOV, PAUSED, TOKEN},
};

pub fn config(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let gov = GOV.load(deps.storage)?;
    let token = TOKEN.load(deps.storage)?;

    Ok(to_binary(&GetConfigResponse {
        gov,
        denom: token.denom,
        reserve_denom: token.reserve_denom,
    })?)
}

pub fn pause_info(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let pause_info = PAUSED.load(deps.storage)?;

    Ok(to_binary(&GetPauseInfoResponse {
        paused: pause_info.paused,
        expires_at: pause_info.expires_at,
    })?)
}

pub fn portfolio(deps: Deps, _env: Env) -> Result<Binary, ContractError> {
    let token = TOKEN.load(deps.storage)?;

    Ok(to_binary(&GetPortfolioResponse {
        total_supply: token.total_supply,
        assets: get_redeem_amounts(deps.storage, token.total_supply)?,
        units: get_assets(deps.storage)?,
    })?)
}

pub fn simulate_mint(
    deps: Deps,
    _env: Env,
    amount: Uint128,
    funds: Vec<Coin>,
) -> Result<Binary, ContractError> {
    let refund_amount = assert_assets(deps.storage, funds, amount)?;

    Ok(to_binary(&SimulateMintResponse {
        mint_amount: amount,
        refund_amount,
    })?)
}

pub fn simulate_burn(deps: Deps, _env: Env, amount: Uint128) -> Result<Binary, ContractError> {
    let redeem_amount = get_redeem_amounts(deps.storage, amount)?;

    Ok(to_binary(&SimulateBurnResponse {
        burn_amount: amount,
        redeem_amount,
    })?)
}
