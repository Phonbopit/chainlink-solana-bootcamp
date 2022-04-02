# chainlink-solana-bootcamp

My projects on Solana Blockchain Developer Bootcamp - https://chain.link/bootcamp/solana-2022

**Day 1 - Exercises**

- [gm-program](/gm-program)
- [gm-program - Bonus Exercise](/gm-program2)
- [token-program](/token-program)
- [token-program - Bonus Exercise](/token-program-bonus)

**Day 2 - Exercises**

// TBC

## References

### Day 1

- [Day 1 - Presentation](https://drive.google.com/file/d/19j241YdwF1p2y6SP_S-HCdmsL7ds-kB4/view)
- [Solana Blockchain Developer Bootcamp Day 1 Exercises](https://docs.google.com/document/d/e/2PACX-1vSOgwdz9-vpBDwh3Epr3fdjzGyMWB1GHNT4H7YysNRyBFRJ0_qpcafgGcZUgNJLoyTH9IBVBaaInHsc/pub)

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

That's it! ❤️

### Day 2

// TBC
