#!/bin/bash

set -e

dfx deps deploy

# dfx identity new minter
dfx identity use minter
export MINTER_ACCOUNT_ID=$(dfx ledger account-id)
echo -e "\nMinter account id: $MINTER_ACCOUNT_ID\n"

dfx identity use default
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

dfx deploy backend --argument '(false)'
export BACKEND_PRINCIPAL=$(dfx canister id backend)
export BACKEND_ACCOUNT_ID=$(dfx ledger account-id --of-principal $BACKEND_PRINCIPAL)
echo -e "\nBackend account id: $BACKEND_ACCOUNT_ID\n"

dfx deploy --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai icp_ledger_canister --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$BACKEND_ACCOUNT_ID\";
          record {
            e8s = 100_000_000_000 : nat64;
          };
        };
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 10_000_000_000 : nat64;
          };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record {
        e8s = 10_000 : nat64;
      };
      token_symbol = opt \"LICP\";
      token_name = opt \"Local ICP\";
    }
  })
"

dfx deploy --specified-id uf6dk-hyaaa-aaaaq-qaaaq-cai xrc --with-cycles 10000000000

echo -e "\nBackend account id: $BACKEND_ACCOUNT_ID"
echo "Backend account balance: $(dfx ledger balance $BACKEND_ACCOUNT_ID)"
echo "DFX default identity account balance: $(dfx ledger balance $DEFAULT_ACCOUNT_ID)"

BACKEND_AKASH_ADDRESS=$(dfx canister call backend address | grep -o '"[^"]*"')
echo -e "\nBackend Akash address: $BACKEND_AKASH_ADDRESS"
echo "Backend Akash balance: $(dfx canister call backend balance | grep -o 'Ok = [0-9]*' | grep -o '[0-9]*')"

echo -e "\nIf the backend Akash balance is 0, please top up the backend canister with some AKT."
echo "Use the Akash Faucet at https://faucet.sandbox-01.aksh.pw/ to top up the backend Akash address: $BACKEND_AKASH_ADDRESS"

echo -e "\nTo check the backend Akash balance, run:\n\tdfx canister call backend balance"
