#!/usr/bin/env bash

here=$(dirname "$0")
# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

set -e

rm -rf "$PUT_CONFIG_DIR"/latest-mainnet-beta-snapshot
mkdir -p "$PUT_CONFIG_DIR"/latest-mainnet-beta-snapshot
(
  cd "$PUT_CONFIG_DIR"/latest-mainnet-beta-snapshot || exit 1
  set -x
  wget http://api.mainnet-beta.put.com/genesis.tar.bz2
  wget --trust-server-names http://api.mainnet-beta.put.com/snapshot.tar.bz2
)

snapshot=$(ls "$PUT_CONFIG_DIR"/latest-mainnet-beta-snapshot/snapshot-[0-9]*-*.tar.zst)
if [[ -z $snapshot ]]; then
  echo Error: Unable to find latest snapshot
  exit 1
fi

if [[ ! $snapshot =~ snapshot-([0-9]*)-.*.tar.zst ]]; then
  echo Error: Unable to determine snapshot slot for "$snapshot"
  exit 1
fi

snapshot_slot="${BASH_REMATCH[1]}"

rm -rf "$PUT_CONFIG_DIR"/bootstrap-validator
mkdir -p "$PUT_CONFIG_DIR"/bootstrap-validator


# Create genesis ledger
if [[ -r $FAUCET_KEYPAIR ]]; then
  cp -f "$FAUCET_KEYPAIR" "$PUT_CONFIG_DIR"/faucet.json
else
  $put_keygen new --no-passphrase -fso "$PUT_CONFIG_DIR"/faucet.json
fi

if [[ -f $BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR ]]; then
  cp -f "$BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR" "$PUT_CONFIG_DIR"/bootstrap-validator/identity.json
else
  $put_keygen new --no-passphrase -so "$PUT_CONFIG_DIR"/bootstrap-validator/identity.json
fi

$put_keygen new --no-passphrase -so "$PUT_CONFIG_DIR"/bootstrap-validator/vote-account.json
$put_keygen new --no-passphrase -so "$PUT_CONFIG_DIR"/bootstrap-validator/stake-account.json

$put_ledger_tool create-snapshot \
  --ledger "$PUT_CONFIG_DIR"/latest-mainnet-beta-snapshot \
  --faucet-pubkey "$PUT_CONFIG_DIR"/faucet.json \
  --faucet-lamports 500000000000000000 \
  --bootstrap-validator "$PUT_CONFIG_DIR"/bootstrap-validator/identity.json \
                        "$PUT_CONFIG_DIR"/bootstrap-validator/vote-account.json \
                        "$PUT_CONFIG_DIR"/bootstrap-validator/stake-account.json \
  --hashes-per-tick sleep \
  "$snapshot_slot" "$PUT_CONFIG_DIR"/bootstrap-validator

$put_ledger_tool modify-genesis \
  --ledger "$PUT_CONFIG_DIR"/latest-mainnet-beta-snapshot \
  --hashes-per-tick sleep \
  "$PUT_CONFIG_DIR"/bootstrap-validator
