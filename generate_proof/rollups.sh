#!/bin/bash

# Hardcoded addresses that we know work in Anvil
export INPUT_BOX_ADDRESS="0x59b22D57D4f067708AB0c00552767405926dc768"
export DAPP_ADDRESS="0xab7528bb862fB57E8A2BCd567a2e929a0Be56a5e"
export MNEMONIC="test test test test test test test test test test test junk"

# Extract proof input
INPUT=$(cat proof_input.json | jq -r '.input')

# Validate hex input
if [ $((${#INPUT} % 2)) -eq 1 ] || ! [[ $INPUT =~ ^[0-9a-fA-F]+$ ]]; then
    echo "Error: Invalid hex input"
    exit 1
fi

# Send the transaction
cast send $INPUT_BOX_ADDRESS "addInput(address,bytes)" $DAPP_ADDRESS "0x$INPUT" --mnemonic "$MNEMONIC"