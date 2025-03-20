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
└── methods/             # Guest program that runs inside zkVM
```

## Usage

1. Build and run to generate the proof:

   ```bash
   RISC0_DEV_MODE=0 cargo run --release
   ```

2. Check the generated proof:
   ```bash
   cat proof_input.json
   ```

The proof is saved in `proof_input.json`, which can be used to verify the age requirement without revealing the actual birthdate.

## Development Mode

For faster iteration during development:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run
```

## Using Bonsai for Remote Proving

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

## Important Note

If you encounter dependency issues with the RISC Zero toolchain, you may need to pin specific versions of dependencies. For example:

```bash
cd methods/guest
cargo update -p bytemuck_derive --precise 1.5.0
```

This project includes the Cargo.lock file to ensure consistent builds across different environments.
