import { Program, web3 } from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import { WmpStaking } from "../../target/types/wmp_staking";
import { getClaimRewardsAccounts, getCreateStakeEntryAccounts, getCreateStakePoolAccounts, getInitializeAccounts, getRecoverRewardsAccounts, getSetStakePoolRewardsAccounts, getStakeAccounts, getUnstakeAccounts, getWithdrawLpAccounts } from "./accounts";
import { getNextId } from "./state";
import { fundAccountsWithTokens } from "../../tests/accounts-pool";
import { tokenAmount } from "./utils";

export let program: Program<WmpStaking>;

export function setProgram(stakingProgram: Program<WmpStaking>) {
  program = stakingProgram;
}

export async function initialize(admin: web3.Signer) {
  let accounts = await getInitializeAccounts(admin.publicKey);
  const tx = await program.methods
    .initialize()
    .accounts(accounts)
    .signers([admin])
    .rpc({skipPreflight: true});

    await program.provider.connection.confirmTransaction(tx);
}

export async function createStakePool(creator: web3.Signer, mintA: web3.PublicKey, mintB: web3.PublicKey, fee: anchor.BN) {
    let id = await getNextId();
    let accounts = await getCreateStakePoolAccounts(creator.publicKey, mintA, mintB, id);
    const tx = await program.methods
      .createStakePool(fee)
      .accounts(accounts)
      .signers([creator])
      .rpc();

    await program.provider.connection.confirmTransaction(tx);
    return accounts.stakePool;
}

export async function setStakePoolRewards(stakePool: web3.PublicKey, admin: web3.Signer, rewardsPerSecond: anchor.BN) {
  let accounts = await getSetStakePoolRewardsAccounts(admin.publicKey, stakePool);
  const tx = await program.methods
    .setStakePoolRewards(rewardsPerSecond)
    .accounts(accounts)
    .signers([admin])
    .rpc();

  await program.provider.connection.confirmTransaction(tx);
  return stakePool;
}

export async function createStakePoolWithRewards(creator: web3.Signer, mintA: web3.PublicKey, mintB: web3.PublicKey, rewardsPerSecond: anchor.BN, mintBAuthority: web3.Signer = null) {
  let stakePool = await createStakePool(creator, mintA, mintB, new anchor.BN(2));
  if (mintBAuthority) {
    await fundAccountsWithTokens(
      program.provider.connection,
      [stakePool],
      mintB,
      mintBAuthority,
      tokenAmount(1000).toNumber()
    );
  }
  return await setStakePoolRewards(stakePool, creator, rewardsPerSecond);
}

export async function createStakeEntry(staker: web3.Signer, stakePool: web3.PublicKey) {
  let accounts = await getCreateStakeEntryAccounts(staker.publicKey, stakePool);
  const tx = await program.methods
    .createStakeEntry()
    .accounts(accounts)
    .signers([staker])
    .rpc();

  await program.provider.connection.confirmTransaction(tx);
  return accounts.stakeEntry;
}

export async function createStakeEntryIx(staker: web3.PublicKey, stakePool: web3.PublicKey) {
  let accounts = await getCreateStakeEntryAccounts(staker, stakePool);
  const ix = await program.methods
    .createStakeEntry()
    .accounts(accounts)
    .signers([])
    .instruction();

  return ix;
}

export async function stake(
  staker: web3.Signer, 
  mintA: web3.PublicKey, 
  amount: anchor.BN, 
  stakePool: web3.PublicKey, 
  stakeEntry: web3.PublicKey = null) {

  if (stakeEntry == null) {
    stakeEntry = await createStakeEntry(staker, stakePool);
  }
  
  let accounts = await getStakeAccounts(staker.publicKey, stakePool, mintA);

  const tx = await program.methods
    .stake(amount)
    .accounts(accounts)
    .signers([staker])
    .rpc();

  await program.provider.connection.confirmTransaction(tx);

  return accounts.stakeEntry;
}

export async function createStakeIx(
  staker: web3.PublicKey, 
  mintA: web3.PublicKey, 
  amount: anchor.BN, 
  stakePool: web3.PublicKey) {

  let accounts = await getStakeAccounts(staker, stakePool, mintA);

  const ix = await program.methods
    .stake(amount)
    .accounts(accounts)
    .signers([])
    .instruction();

  return ix;
}

export async function createUnstakeIx(
  staker: web3.PublicKey, 
  mintA: web3.PublicKey, 
  amount: anchor.BN, 
  stakePool: web3.PublicKey) {

  let accounts = await getUnstakeAccounts(staker, stakePool, mintA);

  const ix = await program.methods
    .unstake(amount)
    .accounts(accounts)
    .signers([])
    .instruction();

  return ix;
}

export async function createClaimRewardsIx(
  staker: web3.PublicKey, 
  mintB: web3.PublicKey,
  stakePool: web3.PublicKey) {

  let accounts = await getClaimRewardsAccounts(staker, stakePool, mintB);

  const ix = await program.methods
    .claimRewards()
    .accounts(accounts)
    .signers([])
    .instruction();

  return ix;
}
