# Cartesi + RISC Zero Integration

A comprehensive toolkit for integrating RISC Zero zero-knowledge proofs with Cartesi's off-chain computation infrastructure. This project demonstrates how to generate zero-knowledge proofs using RISC Zero and verify them within Cartesi's Linux environment through two integration patterns: **Cartesi Rollups** and **Cartesi Coprocessor**.

## Overview

This project combines two powerful technologies:

- **RISC Zero**: A zero-knowledge virtual machine (zkVM) that enables developers to prove the correctness of computations while keeping input data private.
- **Cartesi**: A platform providing scalable off-chain computation in a deterministic Linux environment, bridging blockchain smart contracts with powerful off-chain processing.

## Integration Patterns

The toolkit demonstrates two key integration patterns:

1. **Cartesi Rollups Integration**: Verifies RISC Zero proofs within Cartesi's Linux runtime, enabling privacy-preserving computations in rollups while handling complex verification logic off-chain.

2. **Cartesi Coprocessor Integration**: Leverages Cartesi Machine for computation-heavy tasks while generating RISC Zero proofs for privacy-sensitive operations, creating hybrid solutions with optimal resource allocation.

## Architecture

The integration consists of three main components:



1. **RISC Zero Host Program**: Orchestrates the proof generation process, manages the proof generation pipeline, prepares private inputs, and serializes proof data for Cartesi verification.

2. **RISC Zero Guest Program**: Defines the computation to be proven, optimizes for efficient proving with clear I/O boundaries, and implements selective disclosure and STARK-based proof generation.

3. **Cartesi Verifier**: Runs inside the Cartesi Machine and validates RISC Zero proofs, managing verification state and reporting results to the rollup or coprocessor.

## Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Docker Desktop](https://docs.docker.com/get-docker/)
- [Foundry](https://book.getfoundry.sh/)
- [Cartesi CLI](https://docs.cartesi.io/)
- [RISC Zero zkVM](https://dev.risczero.com/api/zkvm/install)

## Installation

1. **Install Rust & Cargo**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Docker Desktop** following the [Docker Desktop installation guide](https://docs.docker.com/get-docker/).

3. **Install Foundry**:
   ```bash
   curl -L https://foundry.paradigm.xyz | bash
   foundryup
   ```

4. **Install Cartesi CLI** (using Homebrew or NPM):
   ```bash
   # Using Homebrew
   brew install cartesi/tap/cartesi
   
   # Or using NPM
   npm install -g @cartesi/cli
   ```

5. **Install RISC Zero zkVM**:
   ```bash
   curl -L https://risczero.com/install | bash
   rzup install
   ```

## Project Structure

```
cartesi-risczero/
├── generate_proof/              # RISC Zero proof generation example
│   ├── host/                    # Host program that runs the zkVM
│   └── methods/                 # Guest program that runs inside zkVM
├── rollups-verifier/            # Cartesi Rollups verification example
│   └── src/                     # Rust application for verifying proofs
├── coprocessor-verifier/        # Cartesi Coprocessor verification example
│   ├── contracts/               # Smart contracts for deployment
│   └── src/                     # Rust application for verifying proofs
└── README.md                    # This file
```

## Usage

### 1. Generate a ZK Proof

```bash
# Create a new RISC Zero project
cargo risczero new generate_proof --guest-name age_verify
cd generate_proof

# Build and run to generate the proof
RISC0_DEV_MODE=0 cargo run --release
```

The proof is saved in `proof_input.json` and will be used in the next step to verify the proof in the Cartesi Machine.

### 2. Verify a ZK Proof in Cartesi Rollups

```bash
# Create a new Cartesi Rollups project
cartesi create rollups-verifier --template rust
cd rollups-verifier

# Build and run the application
cartesi build
cartesi run

# Send the proof for verification
cd ../generate_proof
./rollups.sh
```

### 3. Verify a ZK Proof with Cartesi Coprocessor

```bash
# Create a new Cartesi Coprocessor project
cartesi-coprocessor create --dapp-name coprocessor-verifier --template rust
cd coprocessor-verifier

# Start the local development environment
cartesi-coprocessor start-devnet

# Build and publish the application
cartesi-coprocessor build
cartesi-coprocessor publish --network devnet

# Deploy the smart contract
cd contracts
cartesi-coprocessor deploy \
    --contract-name MyContract \
    --network devnet \
    --constructor-args <COPROCESSOR_ADDRESS> <MACHINE_HASH>

# Send the proof for verification
cd ../generate_proof
./coprocessor.sh
```

## Receipt Types and Proving Options

RISC Zero supports three types of receipts:

1. **Composite Receipt** (default): Contains multiple STARK proofs for program segments (>100kb). Best for development and testing.

2. **Succinct Receipt**: Compressed using STARK recursion with a single unified proof for the entire computation (>100kb). Ideal for production systems with moderate size constraints.

3. **Groth16 Receipt**: Uses STARK-to-SNARK conversion for maximum compression with trusted setup (less than 1kb). Optimal for on-chain verification and storage-constrained systems.

For on-chain verification and production deployments, Groth16 receipts are recommended due to their minimal size:

```rust
let receipt = prover
    .prove_with_opts(env, PASSWORD_ELF, &ProverOpts::groth16())
    .unwrap()
    .receipt;
```

> **Important**: Groth16 receipt generation requires x86 architecture. If you're on Apple Silicon or another architecture, you'll need to use a remote x86 server, use the Bonsai proving service, or use composite or succinct receipts instead.

## Using Bonsai for Remote Proving

For production deployments, we recommend using the Bonsai proving service with Groth16 receipts:

```bash
export BONSAI_API_KEY=your_api_key_here
export BONSAI_API_URL=https://api.bonsai.xyz

RISC0_DEV_MODE=0 cargo run --release
```


## Important Links

- [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- [Docker Desktop Installation](https://docs.docker.com/get-docker/)
- [Foundry Documentation](https://book.getfoundry.sh/)
- [RISC Zero Installation Guide](https://dev.risczero.com/api/zkvm/install)
- [Cartesi Documentation](https://docs.cartesi.io/)
- [RISC Zero Documentation](https://dev.risczero.com/api)
- [Cartesi Coprocessor](https://docs.mugen.builders/cartesi-co-processor-tutorial/introduction)

## License

This project is licensed under the MIT License.
