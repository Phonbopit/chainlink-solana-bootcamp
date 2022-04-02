GM Program
---

## Usage

Install dependencies

```
yarn
```

Start a local network

```
solana-test-validator
```

Create new tab for logs

```
solana logs
```

Then, compile a program

```
cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program
```

Deploy a program

```
solana program deploy dist/program/gm_program.so
```

Call from a clent

```
yarn start
```