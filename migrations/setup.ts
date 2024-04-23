import { AnchorProvider, web3, Wallet } from "@project-serum/anchor";

export async function setupProvider() {
    let connection = new web3.Connection(process.env.CLUSTER);
    let privateKeyBuffer: Uint8Array = require(process.env.WALLET);
    let signer = web3.Keypair.fromSecretKey(new Uint8Array(privateKeyBuffer));
    let wallet = new Wallet(signer);
    let provider = new AnchorProvider(connection, wallet, {});
    return {signer, provider};
}