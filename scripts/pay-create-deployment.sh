#!/bin/bash

export BACKEND_PRINCIPAL=$(dfx canister id backend)
export BACKEND_ACCOUNT_ID=$(dfx ledger account-id --of-principal $BACKEND_PRINCIPAL)
result=$(dfx ledger transfer $BACKEND_ACCOUNT_ID --icp 5 --memo 0)

echo "Result: $result"

block_height=$(echo "$result" | grep -o '[0-9]*' | awk '{print $NF}')

echo "Extracted Block Height: $block_height"

result=$(dfx canister call backend create_test_deployment '('$block_height')')

echo "Result: $result"

# Use grep to extract the line containing the ID, and cut to get the ID between ""
deployment_id=$(echo "$result" | grep -o '"[^"]*"' | cut -d '"' -f 2)

echo "Extracted ID: $deployment_id"

echo "Sleeping for 60 seconds..."
sleep 60
echo "Wake up after 60 seconds!"

dfx canister call backend close_deployment '("'$deployment_id'")'