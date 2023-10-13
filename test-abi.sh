#!/usr/bin/env bash
#
# Easily run the ABI tests for the entire repo or a subset
#

here=$(dirname "$0")
set -x
export PUT_ABI_DUMP_DIR=/app/root/put_project/abi
exec "${here}/cargo" nightly test --lib -- test_abi_ --nocapture
