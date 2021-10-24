# ipfs-uploader

![CI](https://github.com/LesnyRumcajs/ipfs-uploader/workflows/Rust/badge.svg)

Simple IPFS file upload with CID storage as a smart contract. Quite good exercise to get hands-on experience with Ethereum, IPFS and their ecosystem. I highly recommend it!

Special kudos to [chainsafe.io](https://chainsafe.io/) for the inspiration.

## Prerequisites

- `solc` - solidity compiler to compile the contract
- `docker` and `docker-compose` to bring up IPFS and Ganache for local testing
- `rust` and `cargo` to compile the project

## Installation

Run `cargo install --path .` inside the project.

## Usage

The project consists of a:
- `docker-compose` setup for ganache and IPFS
- `contract-deploy` binary
- `ipfs-uploader` binary

Log level is governed by `RUST_LOG` environment variable (set it to e.g. `debug` or `info` if you want more verbose output).
### Bring up IPFS and Ganache
`docker-compose up --detach`

*It is important to emphasize that the ganache setup has pre-determined accounts to facilitate integration tests.*

### Deploy the smart contract to ganache blockchain

Use `contract-deploy`
```
Helper utility to upload the CidStorage smart contract.

USAGE:
    contract-deploy [OPTIONS] --account-address <account-address> --contract-path <contract-path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --account-address <account-address>    Account address
        --contract-name <contract-name>        Contract name [default: CidStorage]
        --contract-path <contract-path>        Contract file path
        --ganache-address <ganache-address>    Ganache address [default: http://localhost:8545]
```

Example:

Using one of the pre-determined accounts:
```
❯ contract-deploy --account-address 0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1 --contract-path contracts/Cid
Storage.sol
```
You will get a sample output: `0xe78a0f7e598cc8b0bb87894b0f60dd2a88d6a8ab` which is the contract address. Use it with `ipfs-uploader` binary.

### Upload a file to IPFS and store its CID in a smart contract

Use `ipfs-uploader`. On success, it will output the CID in `stdout`.
```
CLI tool to add a file to IPFS and store the CID in a dedicated smart contract

USAGE:
    ipfs-uploader [OPTIONS] --account-address <account-address> --contract-address <contract-address> --contract-path <contract-path> --input-file <input-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --account-address <account-address>      Account address
        --contract-address <contract-address>    Contract address
        --contract-name <contract-name>          Contract name [default: CidStorage]
        --contract-path <contract-path>          Contract file path
        --ganache-address <ganache-address>      Ganache address [default: http://localhost:8545]
        --input-file <input-file>                Input file to be sent to IPFS
```

Example:
```
❯ ipfs-uploader --input-file Cargo.lock --contract-path contracts/CidStorage.sol --account-address 0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1
--contract-address 0xe78a0f7e598cc8b0bb87894b0f60dd2a88d6a8ab
`QmWydrveDuqN3iCZXMcnNFfhTYEzfRF95wBE9DKoZ5QRxk`
```

### Verify the CID content is correct

Using the CID from previous command:
```
docker exec ipfs_host ipfs cat QmWydrveDuqN3iCZXMcnNFfhTYEzfRF95wBE9DKoZ5QRxk 
```

## TODOs

Sample ideas on how to improve the project further can be found [here](TODO.md).