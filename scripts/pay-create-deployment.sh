#!/bin/bash

set -e

export BACKEND_PRINCIPAL=$(dfx canister id backend)
export BACKEND_ACCOUNT_ID=$(dfx ledger account-id --of-principal $BACKEND_PRINCIPAL)

echo "Creating user..."
dfx canister call backend create_user

echo "Deploying with enough AKTs..."
result=$(dfx ledger transfer $BACKEND_ACCOUNT_ID --icp 5 --memo 0)
block_height=$(echo "$result" | grep -o '[0-9]*' | awk '{print $NF}')
# echo "Extracted Block Height: $block_height"
dfx canister call backend update_akt_balance '('$block_height')'

echo "Trying to double spend..."
result=$(dfx canister call backend update_akt_balance '('$block_height')')
echo "Failure: $result"

result=$(dfx canister call backend create_test_deployment)
deployment_id=$(echo "$result" | grep -o '"[^"]*"')
# echo "Extracted Deployment ID: $deployment_id"
echo "Waiting 60 seconds before updating deployment..."
sleep 60
dfx canister call backend update_test_deployment_sdl '('$deployment_id')'

echo "Depositing other 5 AKT on deployment escrow account..."
dfx canister call backend deposit_deployment '('$deployment_id', 5000000)'

echo "Waiting 20 seconds before closing deployment..."
sleep 20
dfx canister call backend close_deployment '('$deployment_id')'

# echo "Trying to deploy without enough AKTs..."
# result=$(dfx canister call backend create_test_deployment)
# echo "Failure: $result"