# Age Verification Zero-Knowledge Proof Generator

This project demonstrates how to create and verify zero-knowledge proofs for age verification using RISC Zero zkVM. It allows users to prove they are above a certain age (21 years) without revealing their actual birthdate.

## Prerequisites

- [Install the RISC Zero toolchain](https://dev.risczero.com/api/zkvm/install):
  ```bash
  curl -L https://risczero.com/install | bash
  rzup install
  ```

## Project Structure

```
generate_proof/
├── Cargo.toml           # Workspace configuration
├── host/                # Host program that runs the zkVM
├── methods/             # Guest program that runs inside zkVM
├── rollups.sh           # Script to send proofs to Cartesi Rollups
└── coprocessor.sh       # Script to send proofs to Cartesi Coprocessor
```

## Usage

### 1. Generate a Proof

Build and run to generate the zero-knowledge proof:

```bash
# For production use
RISC0_DEV_MODE=0 cargo run --release

# For faster iteration during development
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

### 2. Check the Generated Proof

```bash
cat proof_input.json
```

### 3. Using the Proof with Cartesi

You can send the generated proof to either a Cartesi Rollups verifier or a Cartesi Coprocessor:

#### Option A: Send to Cartesi Rollups

```bash
# Make sure the script is executable
chmod +x rollups.sh

# Make sure the Cartesi Rollups node is running
./rollups.sh
```

#### Option B: Send to Cartesi Coprocessor

```bash
# Make sure the script is executable
chmod +x coprocessor.sh

# Make sure the Cartesi Coprocessor is running
./coprocessor.sh
```

## Remote Proving with Bonsai

For production deployments, you can use the Bonsai proving service:

1. Configure Bonsai credentials:

   ```bash
   export BONSAI_API_KEY=your_api_key_here
   export BONSAI_API_URL=https://api.bonsai.xyz
   ```

2. Generate the proof:
   ```bash
   RISC0_DEV_MODE=0 cargo run --release
   ```

## Troubleshooting

If you encounter dependency issues with the RISC Zero toolchain, you may need to pin specific versions of dependencies. For example:

```bash
cd methods/guest
cargo update -p bytemuck_derive --precise 1.5.0
```

This project includes the Cargo.lock file to ensure consistent builds across different environments.
