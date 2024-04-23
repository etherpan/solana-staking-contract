import { Program, web3 } from "@project-serum/anchor";
import { createStakePool, setProgram, setStakePoolRewards } from "../app/program/instructions";
import { WMP_STAKING_PROGRAM_ID } from "../app/program/program-id";
import { tokenAmount } from "../app/program/utils";
import * as artifacts from "../target/types/wmp_staking";
import * as anchor from "@project-serum/anchor";
import { setupProvider } from "./setup";
import { getNextId } from "../app/program/state";
import { getCreateStakePoolAccounts } from "../app/program/accounts";


async function main() {
    let stakeToken = new web3.PublicKey("B2f8bWQrdoyN4VFY9A2tTR4NxoTKFnqQGTod7Tfbzxe1");
    let rewardToken = new web3.PublicKey("4epbBqGmx52wnYdhX9NrxVTuJ7LkFYxG3CAqgfPeizH9");
    
    let {signer, provider} = await setupProvider();
    let program = new Program<artifacts.WmpStaking>(artifacts.IDL, WMP_STAKING_PROGRAM_ID, provider);
    setProgram(program);

    let stakePool = await createStakePool(signer, stakeToken, rewardToken, new anchor.BN(4));
    // let stakePool = new web3.PublicKey("9jx32m78BJmfH7gVJnr99t9TcybHa29w4Da5KYYfW3CK")
    await setStakePoolRewards(stakePool, signer, tokenAmount(1));

    console.log("Stake pool: " + stakePool.toBase58());
}

main()
    .then(() => console.log("ok!"))
    .catch(err => console.error(err));