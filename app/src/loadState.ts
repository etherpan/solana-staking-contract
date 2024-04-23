import { web3 } from "@project-serum/anchor";
import { getAccount, getAssociatedTokenAddress } from "@solana/spl-token";
import { calculateStakeEntryPda } from "../program/pda";
import { AppState, IStakeEntryData, IStakePoolData, ITokenBalance } from "./state";

export async function loadState() {
    if (!AppState.walletConnected) return;
    await fetchStakePoolData();

    await fetchTokenAData();
    await fetchTokenBData();
    await fetchStakeEntryData();
} 

async function fetchTokenAData() {
    let data = await getTokenAccountNullable(AppState.tokenAAddress, AppState.adapter.publicKey);
    if (!data) return;
    AppState.tokenABalance = {
        tokenSymbol: "WMP",
        balance: Number(data.amount) / 1e9
    } as ITokenBalance;
}

async function fetchTokenBData() {
    let data = await getTokenAccountNullable(AppState.tokenBAddress, AppState.adapter.publicKey);
    if (!data) return;
    AppState.tokenBBalance = {
        tokenSymbol: "xWMP",
        balance: Number(data.amount) / 1e9
    } as ITokenBalance;
}

async function fetchStakePoolData() {
    let stakePoolData = await AppState.program.account.stakePool.fetchNullable(AppState.stakePoolAddress);
    AppState.stakePoolData = {
        xWmpEscrow: stakePoolData.escrowB,
        balance: stakePoolData.balance.toNumber() / 1e9,
        timestamp: new Date(stakePoolData.lastUpdateTimestamp.toNumber() * 1000)
    } as IStakePoolData;
}

async function fetchStakeEntryData() {
    let [stakeEntryAddress, _] = await calculateStakeEntryPda(AppState.adapter.publicKey, AppState.stakePoolAddress);
    let stakeEntryData = await AppState.program.account.stakeEntry.fetchNullable(stakeEntryAddress);
    if (stakeEntryData == null) return;

    AppState.stakeEntryAddress = stakeEntryAddress;
    AppState.stakeEntryData = {
        stakeBalance: stakeEntryData.balance.toNumber() / 1e9,
        rewards: stakeEntryData.rewards.toNumber() / 1e9,
        rewardsPerTokenPaid: stakeEntryData.rewardsPerTokenPaid.toNumber() / 1e9
    } as IStakeEntryData;
}

async function getTokenAccountNullable(mint: web3.PublicKey, owner: web3.PublicKey) {
    let associatedAddress = await getAssociatedTokenAddress(mint, owner);
    try {
        let data = await getAccount(AppState.connection, associatedAddress);
        return data;
    } catch {}
    return null;
}