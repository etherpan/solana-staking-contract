import { Program, Provider, web3 } from "@project-serum/anchor";
import { WmpStaking } from "../../target/types/wmp_staking";
import { CustomWalletAdapter } from "./walletAdapter";

export interface ITokenBalance {
    tokenSymbol: string,
    balance: number
}

export interface IStakeEntryData {
    stakeBalance: number,
    rewards: number,
    rewardsPerTokenPaid: number,
    timestamp: Date
}

export interface IStakePoolData {
    xWmpEscrow: web3.PublicKey,
    balance: number,
    timestamp: Date
}

export interface IAppState {
    provider: Provider,
    adapter: CustomWalletAdapter,
    connection: web3.Connection,
    program: Program<WmpStaking>,
    walletConnected: boolean,
    tokenABalance: ITokenBalance,
    tokenBBalance: ITokenBalance,
    stakeEntryData: IStakeEntryData,
    tokenAAddress: web3.PublicKey,
    tokenBAddress: web3.PublicKey,
    stakePoolAddress: web3.PublicKey,
    stakeEntryAddress: web3.PublicKey,
    stakePoolData: IStakePoolData
}

export let AppState = {
    tokenAAddress: new web3.PublicKey("B2f8bWQrdoyN4VFY9A2tTR4NxoTKFnqQGTod7Tfbzxe1"),
    tokenBAddress: new web3.PublicKey("4epbBqGmx52wnYdhX9NrxVTuJ7LkFYxG3CAqgfPeizH9"),
    stakePoolAddress: new web3.PublicKey("783XhGoGN65gsspTYp4NbtHyJsNAgnvqi4yxz3mFHhco"),
    connection: new web3.Connection("https://api.devnet.solana.com")
} as IAppState;