sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

PATH="/home/odmin/.local/share/solana/install/active_release/bin:$PATH"

Please update your PATH environment variable to include the solana programs:
    export PATH="/home/odmin/.local/share/solana/install/active_release/bin:$PATH"

arrrow garment kite truth yellow pelican roast latin discover jewel describe merry polar equip pistol next forest dignity kick word steel seminar actual genuine



Wrote new keypair to /home/odmin/.config/solana/id.json
============================================================================
pubkey: 7ahuPfWXPaMYdojjvF22HpzG82bQ7d8vphZ695QwtnJu
============================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
fame hunt symbol legend knock beyond because hat make blue shoulder indicate
============================================================================


AgA3fgbZaBtpVhYQQNjKQXK52rq6aFqLYndrs1JHnMhH

<!--  -->
odmin@trustvs:~/Works/anchor-staking-first$ solana address -k target/deploy/wmp_staking-keypair.json
6ZhoNfqVk93YrLhYqtMjqy2PDq1da74eNrcPM7sWDYxX

<!--  -->
odmin@trustvs:~/Works/anchor-staking-first$ anchor deploy
Deploying workspace: https://api.devnet.solana.com
Upgrade authority: /home/odmin/.config/solana/id.json
Deploying program "wmp-staking"...
Program path: /home/odmin/Works/anchor-staking-first/target/deploy/wmp_staking.so...
Program Id: 6ZhoNfqVk93YrLhYqtMjqy2PDq1da74eNrcPM7sWDYxX

<!-- Run script to deploy-token.ts -->
Mint: 9trCDEP5bDrVnjYszG2bSFTJanuvr9VioJZuFkhfSUU9
Minted to ELKkXGe2c7mZN6fg7FwFLhb3escgmr4rU7a13H9WTpMZ

Mint: 7AiV1FiY2KEcQyFN1jxk9YdaVB1sZ2T9qGiGKHQtWkJE
Minted to AB9THyg6jXXZr3KsbGWHvnwtvshvZP4h9RENXwkpasFZ

Stake pool: GDTxSo2A9c6Z1n5oeXFmTDSq1yqCw3KUt9ZJkEetyDGE

<!-- New Deploy with browser wallet -->
odmin@trustvs:~/Works/anchor-staking-first$ anchor deploy
Deploying workspace: https://api.devnet.solana.com
Upgrade authority: /home/odmin/.config/solana/id.json
Deploying program "wmp-staking"...
Program path: /home/odmin/Works/anchor-staking-first/target/deploy/wmp_staking.so...
Program Id: zCqMotQEKDpxkbv96kFS4p9XcCN19T48vtumdFe1GFd

Mint:           6aff8xDrjQt1groo52AUcVgcTbQfxbywDD5kfEtdUJF4
Minted to       53w2LRiLPKf9oUtYcMKVAcX79wNzfoAqk6JNHxieuif8

Mint:           Gjodxkd8fr2cT1wETwnqDZwSZHYpV9fhnT3RNCsqQkeh
Minted to:      YTuBZAFWF1ncRFtXafpmg2UAbktyvvALoqPyk9NkNfE

Stake pool: 7VKdH26G3QuG5R26QWWVAPaDvXseXtnnoRgBrJSKKDQ8


<!-- Token deploy -->
odmin@trustvs:~/Works/anchor-staking-first$ yarn deploy:token-devnet
yarn run v1.22.21
warning package.json: No license field
$ tsc && CLUSTER=https://api.devnet.solana.com WALLET=/home/odmin/.config/solana/id.json node build/migrations/deploy-token.js
debug wallet:: NodeWallet {
  payer: Keypair {
    _keypair: { publicKey: [Uint8Array], secretKey: [Uint8Array] }
  }
}
Mint: FB2G58hVJrw3Zo6CJnqyb9DpxwoitP42kZcjuKWjRaFY
Minted to 3wMug7QoCV74xJrfQaJT7frGn8AXx5KsxPJw7M4GZCHi
ok!
Done in 49.40s.
    

debug id=====================> 13
ok!
Done in 12.98s.
odmin@trustvs:~/Works/anchor-staking-first$ yarn migrate:create-stake-pool-devnet
yarn run v1.22.21
warning package.json: No license field
$ tsc && CLUSTER=https://api.devnet.solana.com WALLET=/home/odmin/.config/solana/id.json node build/migrations/create-stake-pool.js
debug wallet:: NodeWallet {
  payer: Keypair {
    _keypair: { publicKey: [Uint8Array], secretKey: [Uint8Array] }
  }
}
debug id=====================> 13
escrowB:    3HVCAePsmm17EnD6qTFFvqzp9C14LdRFxX16ELksv8CJ
debug escrowB:: PublicKey {
  _bn: <BN: 21f08b4fab3ba8ce8cc37e774d7904b73b062e9918d53555e0a9724eef68cc13>
}
accounts: {
  creator: PublicKey {
    _bn: <BN: 6120c31053248155093ca0ad5abb27d4e9d466186ff1db96e7d1cd46b275804f>
  },
  mintA: PublicKey {
    _bn: <BN: 1e46af1403ae37a33d78b00633e016eb2a6b47bc525d1634cf8d27adf2ba173b>
  },
  mintB: PublicKey {
    _bn: <BN: 1e46af1403ae37a33d78b00633e016eb2a6b47bc525d1634cf8d27adf2ba173b>
  },
  globalData: PublicKey {
    _bn: <BN: 227643500626c18067c3079ba728cf0931dda5c97441b898d98a4d0506489ae7>
  },
  escrowA: PublicKey {
    _bn: <BN: 291d66fa96636188cf49be627ba64260a32a62b239c8c2e78f94b67fe25222be>
  },
  escrowB: PublicKey {
    _bn: <BN: 21f08b4fab3ba8ce8cc37e774d7904b73b062e9918d53555e0a9724eef68cc13>
  },
  stakePool: PublicKey {
    _bn: <BN: ae1dfc2a6597e64241212cc71042b63661cb72ee4878e7147dbe50801fb99f5d>
  },
  systemProgram: PublicKey { _bn: <BN: 0> },
  tokenProgram: PublicKey {
    _bn: <BN: 6ddf6e1d765a193d9cbe146ceeb79ac1cb485ed5f5b37913a8cf5857eff00a9>
  },
  associatedTokenProgram: PublicKey {
    _bn: <BN: 8c97258f4e2489f1bb3d1029148e0d830b5a1399daff1084048e7bd8dbe9f859>
  },
  rent: PublicKey {
    _bn: <BN: 6a7d517192c5c51218cc94c3d4af17f58daee089ba1fd44e3dbd98a00000000>
  }
}
Stake pool: CigTamvWqZvgwFYJjFxotD1apXG72ZtN4rbyg7tRZskx


