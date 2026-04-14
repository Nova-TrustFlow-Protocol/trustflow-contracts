#  TrustFlow Contracts

Core smart contract infrastructure for the TrustFlow Protocol. Built on the Stellar network using the Soroban SDK.

## Overview
TrustFlow is a decentralized B2B escrow protocol. It replaces traditional, expensive letter-of-credit systems with programmable Soroban smart contracts, allowing businesses to secure cross-border payments with decentralized arbitration.

## Prerequisites
* [Rust](https://rustup.rs/) (>= 1.71)
* [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

## Setup & Build
Compile the smart contracts to WASM:
```bash
make build
```

## Testing
Run the comprehensive unit tests locally:
```bash
make test
```

## Contributing
Please see CONTRIBUTING.md for our enterprise contribution guidelines, branching naming conventions, and testing standards.