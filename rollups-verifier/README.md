# RISC Zero Proof Verifier for Cartesi Rollups

This project implements a Cartesi Rollups application that verifies RISC Zero zero-knowledge proofs. It allows on-chain verification of off-chain computations without revealing sensitive data.

## Overview

The verifier receives ZK proofs as inputs through the Cartesi Rollups framework and verifies them against a known image ID. This enables privacy-preserving age verification without revealing actual birthdates.

## Project Structure

```
rollups-verifier/
├── Cargo.toml           # Project dependencies
├── src/
│   └── main.rs          # Verifier implementation
├── .cargo/
│   └── config.toml      # RISC-V build configuration
└── Dockerfile           # Container definition for Cartesi Machine
```

## Building the Application

Build the Cartesi application:

```bash
cartesi build
```

This command builds a Cartesi machine with the RISC-V Rust application inside.

## Running the Application

Start the local Cartesi Rollups node:

```bash
cartesi run
```

This starts a local Anvil node on port 8545 and deploys the necessary contracts.

## Verifying Proofs

To verify a proof:

1. Generate a proof using the `generate_proof` project
2. Use the `rollups.sh` script to send the proof to the verifier:
   ```bash
   cd ../generate_proof
   ./rollups.sh
   ```

## Monitoring Verification

Check the application logs to see verification results:

```
validator-1  | Received payload length: 219958 bytes
validator-1  | Receipt length: 219926 bytes
validator-1  | Image ID length: 32 bytes
validator-1  | Verified journal data: true
validator-1  | Proof verified successfully!
```

## Customizing the Verifier

To use this verifier with your own RISC Zero proofs:

1. Replace the `AGE_VERIFY_ID` constant with your own image ID
2. Adjust the `verify_zkp` function if your proof format is different
3. Modify the journal decoding if your proof outputs different data

## Troubleshooting

If you encounter dependency issues with the RISC Zero toolchain, you may need to update Rust version in the Dockerfile to a specific version:

```Dockerfile
FROM ubuntu:22.04 AS builder

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.84.0
```
