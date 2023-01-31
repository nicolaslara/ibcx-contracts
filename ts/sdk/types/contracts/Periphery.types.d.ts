/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
export interface InstantiateMsg {
}
export type ExecuteMsg = {
    mint_exact_amount_out: {
        core_addr: string;
        input_asset: string;
        output_amount: Uint128;
        swap_info: [RouteKey, SwapRoutes][];
    };
} | {
    burn_exact_amount_in: {
        core_addr: string;
        min_output_amount: Uint128;
        output_asset: string;
        swap_info: [RouteKey, SwapRoutes][];
    };
};
export type Uint128 = string;
export type RouteKey = [string, string];
export type SwapRoutes = SwapRoute[];
export interface SwapRoute {
    pool_id: number;
    token_denom: string;
}
export interface MigrateMsg {
    force?: boolean | null;
}
//# sourceMappingURL=Periphery.types.d.ts.map