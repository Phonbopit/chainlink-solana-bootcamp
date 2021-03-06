# chainlink-solana-bootcamp

My projects on Solana Blockchain Developer Bootcamp - https://chain.link/bootcamp/solana-2022

**Day 1 - Exercises**

- [gm-program](/gm-program)
- [gm-program - Bonus Exercise](/gm-program2)
- [token-program](/token-program)
- [token-program - Bonus Exercise](/token-program-bonus)

**Day 2 - Exercises**

- [gm-anchor](/gm-anchor)
- [gm-anchor - Bonus Exercise](/gm-anchor2)
- [solana-social](/solana-social)
- [solana-social - Bonus Exercises](/solana-social2/)
- [solana-chainlink](/solana-chainlink/)
- [solana-chainlink - Bonus Exercises](/solana-chainlink2/)

## References

### Day 1

- [Day 1 - Presentation](https://drive.google.com/file/d/19j241YdwF1p2y6SP_S-HCdmsL7ds-kB4/view)
- [Solana Blockchain Developer Bootcamp Day 1 Exercises](https://docs.google.com/document/d/e/2PACX-1vSOgwdz9-vpBDwh3Epr3fdjzGyMWB1GHNT4H7YysNRyBFRJ0_qpcafgGcZUgNJLoyTH9IBVBaaInHsc/pub)

### Day 2

- [Day 2 - Solana Bootcamp 2022_Day 2 Weekend](https://drive.google.com/file/d/1q21-c6i_ATB4Qgtz8WIfS4lOvvC-YRvs/view)
- [Day 2 - Solana Developer Bootcamp: Day 2 Exercises](https://docs.google.com/document/d/e/2PACX-1vTm2gQPzKGtoZtTeXJGw6ux69gKDrAtiC8qD6GqWTQwfLaokAv9nnTgnGaniHOOLTZoKosRy0FgvGVy/pub)

## Usage

#### Configure Solana CLI

1. Set CLI config url to localhost cluster.

```
solana config set --url localhost
```

2. Generate a new keypair (`--force` if exists.)

```
solana-keygen new

# or force
solana-keygen --force
```

#### Start local cluster

1. Start a local solana cluter:

```
solana-test-validator
```

2. Start listening to transaction logs with a new terminal:

```
solana logs
```

#### Build & Deploy a program

1. Build a program

```
cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program

# Or with scripts
yarn build:program
```

2. Deploy a program:

```
solana program deploy dist/program/<PROGRAM_NAME>

# program are gm_program, gm_program2, token_program

# or use scripts
yarn deploy:program
```

#### Run a Client to test

```
yarn start
```


### Create project with Anchor

```
anchor init gm-anchor
```

build a program

```
anchor build
```

Get program ID using following command:

```
solana address -k ./target/deploy/gm_anchor-keypair.json
```

Update `declare_id` and build again:

```
anchor build
```

then deploy a program:

```
anchor deploy
```

deploy on a testnet

```
anchor deploy --provider.cluster devnet
```

Run a client script

```
yarn start <option>
```

- See scripts in `package.json` each project for more detail.

That's it! ??????
