#!/usr/bin/env bash
#
# Starts an instance of put-faucet
#
here=$(dirname "$0")

# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

[[ -f "$PUT_CONFIG_DIR"/faucet.json ]] || {
  echo "$PUT_CONFIG_DIR/faucet.json not found, create it by running:"
  echo
  echo "  ${here}/setup.sh"
  exit 1
}

set -x
# shellcheck disable=SC2086 # Don't want to double quote $put_faucet
exec $put_faucet --keypair "$PUT_CONFIG_DIR"/faucet.json "$@"
