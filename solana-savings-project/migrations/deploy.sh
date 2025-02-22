#!/bin/bash

# Build the smart contract first
anchor build

# Deploy to devnet (you can change to mainnet or testnet)
anchor deploy --provider.cluster devnet
