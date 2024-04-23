
import { Program } from "@project-serum/anchor";
import { initialize, setProgram } from "../app/program/instructions";
import { setupProvider } from "./setup";
import * as artifacts from "../target/types/wmp_staking";
import { WMP_STAKING_PROGRAM_ID } from "../app/program/program-id";

async function main() {
    let {signer, provider} = await setupProvider();
    let program = new Program<artifacts.WmpStaking>(artifacts.IDL, WMP_STAKING_PROGRAM_ID, provider);
    setProgram(program);
    await initialize(signer);
}

main()
    .then(() => console.log("ok!"))
    .catch(err => console.error(err));