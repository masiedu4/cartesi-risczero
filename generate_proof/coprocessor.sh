#!/bin/bash

# Replace with your deployed contract address
CONTRACT_ADDRESS="0x1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f"

# Send the proof using cast
cast send $CONTRACT_ADDRESS "runExecution(bytes)" $(cat proof_input.json | jq -r '.input') \
    --rpc-url http://localhost:8545 \
    --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80