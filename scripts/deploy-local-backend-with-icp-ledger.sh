#!/bin/bash

dfx identity new minter
dfx identity use minter
export MINTER_ACCOUNT_ID=$(dfx ledger account-id)
echo "minter account id: $MINTER_ACCOUNT_ID"

dfx identity use default
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

dfx deploy backend --argument '(false)'
export BACKEND_PRINCIPAL=$(dfx canister id backend)
export BACKEND_ACCOUNT_ID=$(dfx ledger account-id --of-principal $BACKEND_PRINCIPAL)
echo "backend account id: $BACKEND_ACCOUNT_ID"

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

echo "Backend account balance: $(dfx ledger balance $BACKEND_ACCOUNT_ID)"
echo "Default account balance: $(dfx ledger balance $DEFAULT_ACCOUNT_ID)"
