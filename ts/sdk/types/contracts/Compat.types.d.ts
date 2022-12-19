/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
export type QueryMode = "stargate" | "binding";
export interface InstantiateMsg {
    gov: string;
    mode: QueryMode;
}
export type ExecuteMsg = {
    switch_query_mode: QueryMode;
};
export type QueryMsg = {
    estimate_swap_exact_amount_in: {
        amount: Coin;
        routes: SwapRoutes;
        sender: string;
    };
} | {
    estimate_swap_exact_amount_out: {
        amount: Coin;
        routes: SwapRoutes;
        sender: string;
    };
};
export type Uint128 = string;
export type SwapRoutes = SwapRoute[];
export interface Coin {
    amount: Uint128;
    denom: string;
    [k: string]: unknown;
}
export interface SwapRoute {
    pool_id: number;
    token_denom: string;
}
export type AmountResponse = Uint128;
//# sourceMappingURL=Compat.types.d.ts.map