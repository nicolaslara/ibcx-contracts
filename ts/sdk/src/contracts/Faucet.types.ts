/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {}
export type ExecuteMsg = {
  create: {
    config: TokenCreationConfig;
    denom: string;
  };
} | {
  mint: {
    amount: Uint128;
    denom: string;
  };
} | {
  burn: {
    denom: string;
  };
} | {
  grant: {
    action: Action;
    denom: string;
    grantee: string;
  };
} | {
  revoke: {
    action: Action;
    denom: string;
    revokee: string;
  };
} | {
  release: {
    action: Action;
    denom: string;
  };
} | {
  block: {
    action: Action;
    denom: string;
  };
};
export type TokenCreationConfig = {
  managed: {
    admin: string;
  };
} | {
  unmanaged: {};
};
export type Uint128 = string;
export type Action = "mint" | "burn";
export type QueryMsg = {
  list_aliases: {
    limit?: number | null;
    order?: RangeOrder | null;
    start_after?: string | null;
  };
} | {
  get_token: {
    denom: string;
  };
} | {
  list_tokens: {
    limit?: number | null;
    order?: RangeOrder | null;
    start_after?: number | null;
  };
} | {
  get_last_token_id: {};
} | {
  get_role: {
    account: string;
    denom: string;
  };
} | {
  list_roles: {
    denom: string;
    limit?: number | null;
    order?: RangeOrder | null;
    start_after?: [string, string] | null;
  };
};
export type RangeOrder = "asc" | "desc";
export interface MigrateMsg {
  force?: boolean | null;
}
export type GetLastTokenIdResponse = number;
export interface GetRoleResponse {
  account: string;
  denom: string;
  roles: [Action, boolean][];
}
export interface GetTokenResponse {
  config: TokenCreationConfig;
  denom_r: string;
  denom_v: string;
  id: number;
}
export type ListAliasesResponse = [string, number][];
export type ListRolesResponse = [string, string, boolean][];
export type ListTokensResponse = GetTokenResponse[];