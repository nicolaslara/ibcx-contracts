/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
export type Decimal = string;
export interface InstantiateMsg {
    compat: string;
    denom: string;
    fee_strategy: Fee;
    gov: string;
    initial_assets: [string, Decimal][];
    reserve_denom: string;
}
export interface Fee {
    burn?: Decimal | null;
    mint?: Decimal | null;
    stream?: Decimal | null;
}
export type ExecuteMsg = {
    mint: {
        amount: Uint128;
        receiver?: string | null;
        refund_to?: string | null;
    };
} | {
    burn: {
        redeem_to?: string | null;
    };
} | {
    gov: GovMsg;
} | {
    rebalance: RebalanceMsg;
};
export type Uint128 = string;
export type GovMsg = {
    pause: {
        expires_at: number;
    };
} | {
    release: {};
} | {
    update_gov: {
        new_gov: string;
    };
} | {
    update_compat: {
        new_compat: string;
    };
} | {
    update_reserve_denom: {
        new_denom: string;
    };
} | {
    update_trade_info: {
        cooldown: number;
        denom: string;
        max_trade_amount: Uint128;
        routes: SwapRoutes;
    };
};
export type SwapRoutes = SwapRoute[];
export type RebalanceMsg = {
    init: {
        deflation: [string, Decimal][];
        inflation: [string, Decimal][];
        manager: string;
    };
} | {
    trade: RebalanceTradeMsg;
} | {
    finalize: {};
};
export type RebalanceTradeMsg = {
    deflate: {
        amount: Uint128;
        denom: string;
        max_amount_in: Uint128;
    };
} | {
    inflate: {
        amount: Uint128;
        denom: string;
        min_amount_out: Uint128;
    };
};
export interface SwapRoute {
    pool_id: number;
    token_denom: string;
}
export type QueryMsg = {
    get_balance: {
        account: string;
    };
} | {
    get_config: {};
} | {
    get_pause_info: {};
} | {
    get_portfolio: {};
} | {
    simulate_mint: {
        amount: Uint128;
        funds: Coin[];
    };
} | {
    simulate_burn: {
        amount: Uint128;
    };
};
export interface Coin {
    amount: Uint128;
    denom: string;
    [k: string]: unknown;
}
export type Addr = string;
export interface GetConfigResponse {
    compat: Addr;
    denom: string;
    fee_strategy: Fee;
    gov: Addr;
    reserve_denom: string;
}
export interface GetPauseInfoResponse {
    expires_at?: number | null;
    paused: boolean;
}
export interface GetPortfolioResponse {
    assets: Coin[];
    total_supply: Uint128;
    units: [string, Decimal][];
}
export interface SimulateBurnResponse {
    burn_amount: Uint128;
    redeem_amount: Coin[];
}
export interface SimulateMintResponse {
    mint_amount: Uint128;
    refund_amount: Coin[];
}
//# sourceMappingURL=Core.types.d.ts.map