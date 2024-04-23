import { createAssociatedTokenAccount, createMint, mintTo } from "@solana/spl-token";
import { setupProvider } from "./setup";
import { tokenAmountFromStr } from "../app/program/utils";

async function main() {
    let {signer, provider} = await setupProvider();

    let mint = await createMint(provider.connection, signer, signer.publicKey, null, 9);
    console.log("Mint: " + mint.toBase58());

    let associatedAddress = await createAssociatedTokenAccount(provider.connection, signer, mint, signer.publicKey);

    await mintTo(provider.connection, signer, mint, associatedAddress, signer.publicKey, tokenAmountFromStr("9000000").toNumber());

    console.log(`Minted to ${associatedAddress.toBase58()}`);
}

main()
    .then(x => console.log("ok!"))
    .catch(err => console.error(err));