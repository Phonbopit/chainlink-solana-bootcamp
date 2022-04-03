GM Anchor
---

Create new keypair

```
solana-keygen new -o id.json
```

Airdrop SOL to `id.json` account.

```
solana airdrop 10 ./id.json
```

Start local cluster

```
solana-test-validator

# listening to logs
solana logs
```

Start a client with name

```
yarn start -- --name <YOUR_NAME>

# Example
yarn start -- --name "Chai Phonbopit"
```


## Anchor workflow

```
anchor build

# Update declare_id
declare_id!("program_id");

# build a modified code.
anchor build

# deploy to local cluster.
anchor deploy
```