/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { AirdropId, Uint128, ClaimProofOptional, RangeOrder, AirdropIdOptional, ClaimProof, CheckQualificationResponse, GetAirdropResponse, GetClaimResponse, LatestAirdropResponse, ListAirdropsResponse, ListClaimsResponse } from "./Airdrop.types";
export interface AirdropReadOnlyInterface {
    contractAddress: string;
    getAirdrop: ({ id }: {
        id: AirdropId;
    }) => Promise<GetAirdropResponse>;
    listAirdrops: ({ limit, order, startAfter }: {
        limit?: number;
        order?: RangeOrder;
        startAfter: AirdropIdOptional;
    }) => Promise<ListAirdropsResponse>;
    latestAirdropId: () => Promise<LatestAirdropResponse>;
    getClaim: ({ claimProof, id }: {
        claimProof: ClaimProof;
        id: AirdropId;
    }) => Promise<GetClaimResponse>;
    listClaims: ({ id, limit, order, startAfter }: {
        id: AirdropId;
        limit?: number;
        order?: RangeOrder;
        startAfter?: string;
    }) => Promise<ListClaimsResponse>;
    checkQualification: ({ amount, claimProof, id, merkleProof }: {
        amount: Uint128;
        claimProof: ClaimProof;
        id: AirdropId;
        merkleProof: string[];
    }) => Promise<CheckQualificationResponse>;
}
export declare class AirdropQueryClient implements AirdropReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    getAirdrop: ({ id }: {
        id: AirdropId;
    }) => Promise<GetAirdropResponse>;
    listAirdrops: ({ limit, order, startAfter }: {
        limit?: number;
        order?: RangeOrder;
        startAfter: AirdropIdOptional;
    }) => Promise<ListAirdropsResponse>;
    latestAirdropId: () => Promise<LatestAirdropResponse>;
    getClaim: ({ claimProof, id }: {
        claimProof: ClaimProof;
        id: AirdropId;
    }) => Promise<GetClaimResponse>;
    listClaims: ({ id, limit, order, startAfter }: {
        id: AirdropId;
        limit?: number;
        order?: RangeOrder;
        startAfter?: string;
    }) => Promise<ListClaimsResponse>;
    checkQualification: ({ amount, claimProof, id, merkleProof }: {
        amount: Uint128;
        claimProof: ClaimProof;
        id: AirdropId;
        merkleProof: string[];
    }) => Promise<CheckQualificationResponse>;
}
export interface AirdropInterface extends AirdropReadOnlyInterface {
    contractAddress: string;
    sender: string;
    register: ({ bearer, denom, label, merkleRoot }: {
        bearer?: boolean;
        denom: string;
        label?: string;
        merkleRoot: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    fund: ({ id }: {
        id: AirdropId;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    claim: ({ amount, claimProof, id, merkleProof }: {
        amount: Uint128;
        claimProof: ClaimProofOptional;
        id: AirdropId;
        merkleProof: string[];
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    multiClaim: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    close: ({ id }: {
        id: AirdropId;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class AirdropClient extends AirdropQueryClient implements AirdropInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    register: ({ bearer, denom, label, merkleRoot }: {
        bearer?: boolean;
        denom: string;
        label?: string;
        merkleRoot: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    fund: ({ id }: {
        id: AirdropId;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    claim: ({ amount, claimProof, id, merkleProof }: {
        amount: Uint128;
        claimProof: ClaimProofOptional;
        id: AirdropId;
        merkleProof: string[];
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    multiClaim: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    close: ({ id }: {
        id: AirdropId;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Airdrop.client.d.ts.map