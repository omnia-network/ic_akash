#!/bin/bash

export BACKEND_PRINCIPAL=$(dfx canister id backend)
export BACKEND_ACCOUNT_ID=$(dfx ledger account-id --of-principal $BACKEND_PRINCIPAL)
result=$(dfx ledger transfer $BACKEND_ACCOUNT_ID --icp 5 --memo 0)

block_height=$(echo "$result" | grep -o '[0-9]*' | awk '{print $NF}')
echo "Extracted Block Height: $block_height"

result=$(dfx canister call backend create_test_deployment '('$block_height')')

deployment_id=$(echo "$result" | grep -o '"[^"]*"')
echo "Extracted Deployment ID: $deployment_id"

echo "Waiting 60 seconds before closing deployment..."
sleep 60

dfx canister call backend close_deployment '('$deployment_id')'
echo "Deployment closed"

echo "Trying to double spend the deployment..."
result=$(dfx canister call backend create_test_deployment '('$block_height')')
echo "Deployment creation failed: $result"