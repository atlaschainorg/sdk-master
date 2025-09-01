#!/usr/bin/env bash

set -eo pipefail
here="$(dirname "$0")"
src_root="$(readlink -f "${here}/..")"
cd "${src_root}"

build_sbf_excludes=(
  --exclude atlas-client-traits
  --exclude atlas-ed25519-program
  --exclude atlas-example-mocks
  --exclude atlas-file-download
  --exclude atlas-genesis-config
  --exclude atlas-keypair
  --exclude atlas-logger
  --exclude atlas-offchain-message
  --exclude atlas-presigner
  --exclude atlas-quic-definitions
  --exclude atlas-sdk-wasm-js
  --exclude atlas-sdk-wasm-js-tests
  --exclude atlas-secp256k1-program
  --exclude atlas-secp256r1-program
  --exclude atlas-system-transaction
  --exclude atlas-system-wasm-js
  --exclude atlas-transaction
  --exclude atlas-sdk
)

./cargo nightly hack --workspace "${build_sbf_excludes[@]}" build-sbf

# This can be added back in once the SDK upgrades to v2.3 of Agave tools
#./cargo nightly build-sbf --manifest-path sdk/Cargo.toml --no-default-features
