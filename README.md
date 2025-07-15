# Cosmwasm Secret Network

## Installation
https://docs.scrt.network/secret-network-documentation/development/readme-1/setting-up-your-environment
- Install Rust
- Add WASM build target
```
rustup target add wasm32-unknown-unknown
```
- Install docker
- Get Pulsar-3 LCD Network API endpoint from https://docs.scrt.network/secret-network-documentation/development/resources-api-contract-addresses/connecting-to-the-network/testnet-pulsar-3
- Get some tokens from Faucets
    https://pulsar-3-faucet.vercel.app/
- Explorers
    https://secretnodes.com/pulsar
    https://testnet.ping.pub/secret/

- Install BunJs

Fix schemars to "0.8.22" due to secret-toolkit repo. See https://github.com/scrtlabs/secret-toolkit/blob/master/Cargo.toml

## Test The Contract
```
  cargo test -- --nocapture
```
## Build The Contract
```
docker run --rm -v "$$(pwd)":/contract \
	--mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/contract/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	ghcr.io/scrtlabs/secret-contract-optimizer:1.0.13
```
The “secret-contract-optimizer” is hosted here: https://github.com/orgs/scrtlabs/packages/container/package/secret-contract-optimizer

## Frontend
Go to `secretjs` folder
Install JS packages:
```
pnpm install
```

Fill the .env file according to env.example, but leave those blank: ADDR0, CONTRACT_CODE_ID, CONTRACT_CODE_HASH, CONTRACT_ADDRESS

Run the JavaScript/TypeScript via BunJs:
```
  bun run index 0
  bun run index 2 key123 pw456
  bun run index 3 key123
```