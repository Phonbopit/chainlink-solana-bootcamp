Token Program Exercise
---

Install `spl-token-cli`:

```bash
cargo install spl-token-cli
```

Create a new token:

```
spl-token create-token
```

Output will be following:

```
Creating token 5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo

Signature: 4j7a3Dp6EomjJQAh9G4idnRDkEuYbK5d82uuLYoNuDETNJsJgpP9SchMj3KcCq4erkr6i5dvj76B3gmi2sETZY4z
```

> Token id is `5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo` 

Check a supply (currently is 0)

```
spl-token supply <token_id>
```

Mint some token but first we need to create an account to hold a balance.

```
spl-token create-account <token_id>
```

Then

```
spl-token create-account 5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo

Creating account 7WGnXwjnWypjBk8Bhi9hxRagzFAbJsYjBRiKpj5K54mH
Signature: 4kqqxQb4q9x5ZzTUu8tsRfGzBCjE23Cn1fbxqCNcpyrsjrmPoa8zCv3bXbYMQU1Ri2qLCdXhK7upqReSpfvnWhbz
```

Check a balance of the new account:

```
spl-token balance 5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo
```

Mint 100token into the account:

```
spl-token mint 5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo 100

Minting 100 tokens
  Token: 5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo
  Recipient: 7WGnXwjnWypjBk8Bhi9hxRagzFAbJsYjBRiKpj5K54mH

Signature: 4iTXZpmkBE8vKJb3NUE2GRhNCuvzHeE3KL9wxtsUFqc5uZSozdaZPfw5nx4DBEcZixbsRBrgpHHZE4LCwbDU2p82
```

Check supply and balance again.

```bash
spl-token supply 5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo
100
```

```
spl-token balance 5BZK1hcZ7zyhjaA7DzSSYFf3LauQ9kK21s7a6G6py1xo
100
```

View all tokens that you own

```
spl-token accounts
```

Reference : https://spl.solana.com/token