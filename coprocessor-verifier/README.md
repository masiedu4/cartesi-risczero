# Cartesi Coprocessor ZK Proof Verifier

A Cartesi Coprocessor application that verifies RISC Zero zero-knowledge proofs. This application receives ZK proofs as inputs and verifies them within the Cartesi Machine.

## Prerequisites

- [Cartesi Coprocessor CLI and Cartesi Machine](https://docs.cartesi.io/cartesi-co-processor-tutorial/installation/) installed
- [Docker Desktop](https://www.docker.com/products/docker-desktop/) installed and running
- [Cast](https://book.getfoundry.sh/cast/) (part of Foundry) for sending transactions
- Basic understanding of Rust and zero-knowledge proofs

## Project Structure

```
coprocessor-verifier/
├── .cargo/              # Cargo config
├── .cartesi/            # Cartesi configuration
├── contracts/           # Smart contracts for deployment
├── src/                 # Rust application code
├── Cargo.toml           # Rust dependencies
├── Cargo.lock           # Pinned dependencies
├── Dockerfile           # Docker image configuration
├── .dockerignore        # Docker ignore rules
└── .gitignore           # Git ignore rules
```

## Quick Start

1. **Start the local development environment**:

   ```bash
   cartesi-coprocessor start-devnet
   ```

2. **Build the Cartesi Coprocessor application**:

   ```bash
   cartesi-coprocessor build
   ```

3. **Publish the application to the local devnet**:

   ```bash
   cartesi-coprocessor publish --network devnet
   ```

4. **Deploy the smart contract**:

   ```bash
   # Get the machine hash and coprocessor address
   cartesi-coprocessor address-book

   # Deploy the contract with the coprocessor address and machine hash
   cd contracts
   cartesi-coprocessor deploy \
       --contract-name MyContract \
       --network devnet \
       --constructor-args <COPROCESSOR_ADDRESS> <MACHINE_HASH>
   ```

5. **Send a ZK proof for verification**:

   ```bash
   # Navigate to the generate_proof directory
   cd ../generate_proof

   # Update the contract address in coprocessor.sh
   # Then run:
   ./coprocessor.sh
   ```

6. **Monitor the verification results**:
   ```bash
   docker logs -f cartesi-coprocessor-operator
   ```

## Customizing for Your Own Proofs

To use this verifier with your own RISC Zero proofs:

1. Replace `AGE_VERIFY_ID` in `src/main.rs` with your program's image ID
2. Modify the journal decoding to match your proof's output format
3. Adjust the verification logic if needed

## Smart Contracts

The contracts directory contains Solidity smart contracts for deploying your Cartesi Coprocessor application. Built with Foundry for testing and deployment.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
