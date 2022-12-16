use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Order, StdResult, Uint128};
use ibc_alias::{CosmosMsg, QuerierWrapper};
use osmo_bindings::{OsmosisQuery, SwapAmount, SwapResponse};
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgSwapExactAmountIn, MsgSwapExactAmountOut, SwapAmountInRoute, SwapAmountOutRoute,
};

#[cw_serde]
pub struct SwapRoute {
    pub pool_id: u64,
    pub token_denom: String,
}

#[cw_serde]
pub struct SwapRoutes(pub Vec<SwapRoute>);

impl From<SwapRoutes> for Vec<SwapAmountInRoute> {
    fn from(val: SwapRoutes) -> Self {
        val.0
            .into_iter()
            .map(|v| SwapAmountInRoute {
                pool_id: v.pool_id,
                token_out_denom: v.token_denom,
            })
            .collect()
    }
}

impl From<SwapRoutes> for Vec<SwapAmountOutRoute> {
    fn from(val: SwapRoutes) -> Self {
        val.0
            .into_iter()
            .map(|v| SwapAmountOutRoute {
                pool_id: v.pool_id,
                token_in_denom: v.token_denom,
            })
            .collect()
    }
}

impl SwapRoutes {
    pub fn sim_swap_exact_in(
        &self,
        querier: &QuerierWrapper,
        sender: &Addr,
        token_in: &str,
        token_in_amount: Uint128,
    ) -> StdResult<Uint128> {
        let resp: SwapResponse = querier.query(
            &OsmosisQuery::estimate_swap(
                sender,
                self.0.first().unwrap().pool_id,
                token_in,
                &self.0.last().unwrap().token_denom,
                SwapAmount::In(token_in_amount),
            )
            .into(),
        )?;
        Ok(resp.amount.as_out())
    }

    pub fn sim_swap_exact_out(
        &self,
        querier: &QuerierWrapper,
        sender: &Addr,
        token_out: &str,
        token_out_amount: Uint128,
    ) -> StdResult<Uint128> {
        let resp: SwapResponse = querier.query(
            &OsmosisQuery::estimate_swap(
                sender,
                self.0.first().unwrap().pool_id,
                &self.0.last().unwrap().token_denom,
                token_out,
                SwapAmount::Out(token_out_amount),
            )
            .into(),
        )?;
        Ok(resp.amount.as_in())
    }

    pub fn msg_swap_exact_in(
        &self,
        sender: &Addr,
        token_in: &str,
        token_in_amount: Uint128,
        token_out_min: Uint128,
    ) -> CosmosMsg {
        MsgSwapExactAmountIn {
            sender: sender.to_string(),
            routes: self.clone().into(),
            token_in: Some(coin(token_in_amount.u128(), token_in).into()),
            token_out_min_amount: token_out_min.to_string(),
        }
        .into()
    }

    pub fn msg_swap_exact_out(
        &self,
        sender: &Addr,
        token_out: &str,
        token_out_amount: Uint128,
        token_in_max: Uint128,
    ) -> CosmosMsg {
        MsgSwapExactAmountOut {
            sender: sender.to_string(),
            routes: self.clone().into(),
            token_in_max_amount: token_in_max.to_string(),
            token_out: Some(coin(token_out_amount.u128(), token_out).into()),
        }
        .into()
    }
}

#[cw_serde]
pub enum RangeOrder {
    Asc,
    Desc,
}

impl From<Order> for RangeOrder {
    fn from(order: Order) -> Self {
        match order {
            Order::Ascending => Self::Asc,
            Order::Descending => Self::Desc,
        }
    }
}

impl From<RangeOrder> for Order {
    fn from(order: RangeOrder) -> Self {
        match order {
            RangeOrder::Asc => Order::Ascending,
            RangeOrder::Desc => Order::Descending,
        }
    }
}
