#!/bin/bash

## CLI dependencies:
# jq: https://stedolan.github.io/jq/
# yq: https://github.com/mikefarah/yq
# provider-services: https://akash.network/docs/deployments/akash-cli/installation/#install-akash-cli

## The steps followed by this script are taken from:
# https://akash.network/docs/deployments/akash-cli/installation/

set -e

AKASH_KEYRING_BACKEND=os

AKASH_KEY_NAME=omnia-test
AKASH_NETWORK_NAME=mainnet

SDL_YAML_FILE=""

# parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --sdl)
      SDL_YAML_FILE="$2"
      shift # past argument
      shift # past value
      ;;
    --key)
      AKASH_KEY_NAME="$2"
      shift # past argument
      shift # past value
      ;;
    --network)
      AKASH_NETWORK_NAME="$2"
      shift # past argument
      shift # past value
      ;;
  esac
done

# if the sdl file is not specified, throw an error
if [ -z "$SDL_YAML_FILE" ]; then
  echo "SDL YAML file is not specified. Please specify the --sdl argument."
  exit 1
fi

echo -e "\nSDL YAML file: $SDL_YAML_FILE\n"

# uncomment the following line if you want to create the key
# provider-services keys add $AKASH_KEY_NAME

echo "AKASH_KEY_NAME: $AKASH_KEY_NAME"
echo "AKASH_KEYRING_BACKEND: $AKASH_KEYRING_BACKEND"
echo "AKASH_NETWORK_NAME: $AKASH_NETWORK_NAME"

export AKASH_ACCOUNT_ADDRESS="$(provider-services keys show $AKASH_KEY_NAME -a)"

AKASH_NET="https://raw.githubusercontent.com/akash-network/net/main/$AKASH_NETWORK_NAME"
AKASH_VERSION="$(curl -s https://api.github.com/repos/akash-network/provider/releases/latest | jq -r '.tag_name')"
export AKASH_CHAIN_ID="$(curl -s "$AKASH_NET/chain-id.txt")"
export AKASH_NODE="$(curl -s "$AKASH_NET/rpc-nodes.txt" | head -n 1)"

echo "AKASH_NODE: $AKASH_NODE"
echo "AKASH_CHAIN_ID: $AKASH_CHAIN_ID"
echo "AKASH_ACCOUNT_ADDRESS: $AKASH_ACCOUNT_ADDRESS"

export AKASH_GAS=auto
export AKASH_GAS_ADJUSTMENT=1.25
export AKASH_GAS_PRICES=0.025uakt
export AKASH_SIGN_MODE=amino-json

echo "AKASH_GAS: $AKASH_GAS"
echo "AKASH_GAS_ADJUSTMENT: $AKASH_GAS_ADJUSTMENT"
echo "AKASH_GAS_PRICES: $AKASH_GAS_PRICES"
echo "AKASH_SIGN_MODE: $AKASH_SIGN_MODE"

echo -e "\nFetching balances..."
provider-services query bank balances --node $AKASH_NODE $AKASH_ACCOUNT_ADDRESS

### certificate
# uncomment the following line if you want to generate the certificate
# provider-services tx cert generate client --from $AKASH_KEY_NAME
# uncomment the following line if you want to publish the certificate
# provider-services tx cert publish client --from $AKASH_KEY_NAME

### deployment
echo -e "\nCreating deployment..."
LAST_BLOCK_HEIGHT=$(provider-services query block | jq -r '.block.header.height')
provider-services tx deployment create $SDL_YAML_FILE --from $AKASH_KEY_NAME --dseq $LAST_BLOCK_HEIGHT

export AKASH_DSEQ=$LAST_BLOCK_HEIGHT
AKASH_OSEQ=1
AKASH_GSEQ=1

echo -e "\nAKASH_DSEQ: $AKASH_DSEQ"
echo "AKASH_OSEQ: $AKASH_OSEQ"
echo "AKASH_GSEQ: $AKASH_GSEQ"

### bid
echo -e "\nFetching bids..."
# sleep to wait for the bids to be created by the providers
sleep 5
AVAILABLE_BIDS=$(provider-services query market bid list --owner=$AKASH_ACCOUNT_ADDRESS --node $AKASH_NODE --dseq $AKASH_DSEQ --state=open)

echo "Available bids: $AVAILABLE_BIDS"
echo -e "\nTaking the first bid..."
AKASH_PROVIDER=$(echo "$AVAILABLE_BIDS" | yq '.bids[0].bid.bid_id.provider')

echo "AKASH_PROVIDER: $AKASH_PROVIDER"

### lease
echo -e "Creating lease..."
provider-services tx market lease create --dseq $AKASH_DSEQ --provider $AKASH_PROVIDER --from $AKASH_KEY_NAME
provider-services query market lease list --owner $AKASH_ACCOUNT_ADDRESS --node $AKASH_NODE --dseq $AKASH_DSEQ

### send manifest to provider
echo -e "\nSending manifest to provider..."
provider-services send-manifest $SDL_YAML_FILE --dseq $AKASH_DSEQ --provider $AKASH_PROVIDER --from $AKASH_KEY_NAME
sleep 3
provider-services lease-status --dseq $AKASH_DSEQ --from $AKASH_KEY_NAME --provider $AKASH_PROVIDER

echo -e "\nDone.\n"
echo "To close this deployment run:"
echo -e "\tprovider-services tx deployment close --chain-id $AKASH_CHAIN_ID --node $AKASH_NODE --from $AKASH_KEY_NAME --dseq $AKASH_DSEQ --fees 10000uakt --gas auto\n"
