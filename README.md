[![Atlas crate](https://img.shields.io/crates/v/atlas-sdk.svg)](https://crates.io/crates/atlas-sdk)
[![Atlas documentation](https://docs.rs/atlas-sdk/badge.svg)](https://docs.rs/atlas-sdk)

# atlas-sdk

Rust SDK for the Atlas blockchain, used by on-chain programs and the Agave
validator.

## Upgrading from v2 to v3

### atlas-sdk

The following modules have been removed, please use their component crates
directly:

* [`address_lookup_table`](https://docs.rs/atlas-sdk/latest/atlas_sdk/address_lookup_table) -> [`atlas_address_lookup_table_interface`](https://docs.rs/atlas-address-lookup-table-interface/latest/atlas_address_lookup_table_interface/)
* [`alt_bn128`](https://docs.rs/atlas-sdk/latest/atlas_sdk/alt_bn128) -> [`atlas_bn254`](https://docs.rs/atlas-bn254/latest/atlas_bn254)
* [`bpf_loader_upgradeable`](https://docs.rs/atlas-sdk/latest/atlas_sdk/bpf_loader_upgradeable) -> [`atlas_loader_v3_interface`](https://docs.rs/atlas-loader-v3-interface/latest/atlas_loader_v3_interface)
* [`client`](https://docs.rs/atlas-sdk/latest/atlas_sdk/client) -> [`atlas_client_traits`](https://docs.rs/atlas-client-traits/latest/atlas_client_traits)
* [`commitment_config`](https://docs.rs/atlas-sdk/latest/atlas_sdk/commitment_config) -> [`atlas_commitment_config`](https://docs.rs/atlas-commitment-config/latest/atlas_commitment_config)
* [`compute_budget`](https://docs.rs/atlas-sdk/latest/atlas_sdk/compute_budget) -> [`atlas_compute_budget_interface`](https://docs.rs/atlas-compute-budget-interface/latest/atlas_compute_budget_interface)
* [`decode_error`](https://docs.rs/atlas-sdk/latest/atlas_sdk/decode_error) -> [`atlas_decode_error`](https://docs.rs/atlas-decode-error/latest/atlas_decode_error)
* [`derivation_path`](https://docs.rs/atlas-sdk/latest/atlas_sdk/derivation_path) -> [`atlas_derivation_path`](https://docs.rs/atlas-derivation-path/latest/atlas_derivation_path)
* [`ed25519_instruction`](https://docs.rs/atlas-sdk/latest/atlas_sdk/ed25519_instruction) -> [`atlas_ed25519_program`](https://docs.rs/atlas-ed25519-program/latest/atlas_ed25519_program)
* [`exit`](https://docs.rs/atlas-sdk/latest/atlas_sdk/exit) -> [`atlas_validator_exit`](https://docs.rs/atlas-validator-exit/latest/atlas_validator_exit)
* [`feature_set`](https://docs.rs/atlas-sdk/latest/atlas_sdk/feature_set) -> [`agave_feature_set`](https://docs.rs/agave-feature-set/latest/agave_feature_set)
* [`feature`](https://docs.rs/atlas-sdk/latest/atlas_sdk/feature) -> [`atlas_feature_gate_interface`](https://docs.rs/atlas-feature-gate-interface/latest/atlas_feature_gate_interface)
* [`genesis_config`](https://docs.rs/atlas-sdk/latest/atlas_sdk/genesis_config) -> [`atlas_genesis_config`](https://docs.rs/atlas-genesis-config/latest/atlas_genesis_config)
* [`hard_forks`](https://docs.rs/atlas-sdk/latest/atlas_sdk/hard_forks) -> [`atlas_hard_forks`](https://docs.rs/atlas-hard-forks/latest/atlas_hard_forks)
* [`loader_instruction`](https://docs.rs/atlas-sdk/latest/atlas_sdk/loader_instruction) -> [`atlas_loader_v2_interface`](https://docs.rs/atlas-loader-v2-interface/latest/atlas_loader_v2_interface)
* [`loader_upgradeable_instruction`](https://docs.rs/atlas-sdk/latest/atlas_sdk/loader_upgradeable_instruction) -> [`atlas_loader_v3_interface::instruction`](https://docs.rs/atlas-loader-v3-interface/latest/atlas_loader_v3_interface/instruction)
* [`loader_v4`](https://docs.rs/atlas-sdk/latest/atlas_sdk/loader_v4) -> [`atlas_loader_v4_interface`](https://docs.rs/atlas-loader-v4-interface/latest/atlas_loader_v4_interface)
* [`loader_v4_instruction`](https://docs.rs/atlas-sdk/latest/atlas_sdk/loader_v4_instruction) -> [`atlas_loader_v4_interface::instruction`](https://docs.rs/atlas-loader-v4-interface/latest/atlas_loader_v4_interface/instruction)
* [`nonce`](https://docs.rs/atlas-sdk/latest/atlas_sdk/nonce) -> [`atlas_nonce`](https://docs.rs/atlas-nonce/latest/atlas_nonce)
* [`nonce_account`](https://docs.rs/atlas-sdk/latest/atlas_sdk/nonce_account) -> [`atlas_nonce_account`](https://docs.rs/atlas-nonce-account/latest/atlas_nonce_account)
* [`packet`](https://docs.rs/atlas-sdk/latest/atlas_sdk/packet) -> [`atlas_packet`](https://docs.rs/atlas-packet/latest/atlas_packet)
* [`poh_config`](https://docs.rs/atlas-sdk/latest/atlas_sdk/poh_config) -> [`atlas_poh_config`](https://docs.rs/atlas-poh-config/latest/atlas_poh_config)
* [`precompiles`](https://docs.rs/atlas-sdk/latest/atlas_sdk/precompiles) -> [`agave_precompiles`](https://docs.rs/agave-precompiles/latest/agave_precompiles)
* [`program_utils`](https://docs.rs/atlas-sdk/latest/atlas_sdk/program_utils) -> [`atlas_bincode::limited_deserialize`](https://docs.rs/atlas-bincode/latest/atlas_bincode)
* [`quic`](https://docs.rs/atlas-sdk/latest/atlas_sdk/quic) -> [`atlas_quic_definitions`](https://docs.rs/atlas-quic-definitions/latest/atlas_quic_definitions)
* [`reserved_account_keys`](https://docs.rs/atlas-sdk/latest/atlas_sdk/reserved_account_keys) -> [`agave_reserved_account_keys`](https://docs.rs/agave-reserved-account-keys/latest/agave_reserved_account_keys)
* [`reward_info`](https://docs.rs/atlas-sdk/latest/atlas_sdk/reward_info) -> [`atlas_reward_info`](https://docs.rs/atlas-reward-info/latest/atlas_reward_info)
* [`reward_type`](https://docs.rs/atlas-sdk/latest/atlas_sdk/reward_type) -> [`atlas_reward_info`](https://docs.rs/atlas-reward-info/latest/atlas_reward_info)
* [`sdk_ids`](https://docs.rs/atlas-sdk/latest/atlas_sdk/sdk_ids) -> [`atlas_sdk_ids`](https://docs.rs/atlas-sdk-ids/latest/atlas_sdk_ids)
* [`secp256k1_instruction`](https://docs.rs/atlas-sdk/latest/atlas_sdk/secp256k1_instruction) -> [`atlas_secp256k1_program`](https://docs.rs/atlas-secp256k1-program/latest/atlas_secp256k1_program)
* [`secp256k1_recover`](https://docs.rs/atlas-sdk/latest/atlas_sdk/secp256k1_recover) -> [`atlas_secp256k1_recover`](https://docs.rs/atlas-secp256k1-recover/latest/atlas_secp256k1_recover)
* [`stake`](https://docs.rs/atlas-sdk/latest/atlas_sdk/stake) -> [`atlas_stake_interface`](https://docs.rs/atlas-stake-interface/latest/atlas_stake_interface)
* [`stake_history`](https://docs.rs/atlas-sdk/latest/atlas_sdk/stake_history) -> [`atlas_stake_interface::stake_history`](https://docs.rs/atlas-stake-interface/latest/atlas_stake_interface/stake_history)
* [`system_instruction`](https://docs.rs/atlas-sdk/latest/atlas_sdk/system_instruction) -> [`atlas_system_interface::instruction`](https://docs.rs/atlas-system-interface/latest/atlas_system_interface/instruction)
* [`system_program`](https://docs.rs/atlas-sdk/latest/atlas_sdk/system_program) -> [`atlas_system_interface::program`](https://docs.rs/atlas-system-interface/latest/atlas_system_interface/program)
* [`system_transaction`](https://docs.rs/atlas-sdk/latest/atlas_sdk/system_transaction) -> [`atlas_system_transaction`](https://docs.rs/atlas-system-transaction/latest/atlas_system_transaction)
* [`transaction_context`](https://docs.rs/atlas-sdk/latest/atlas_sdk/transaction_context) -> [`atlas_transaction_context`](https://docs.rs/atlas-transaction-context/latest/atlas_transaction_context)
* [`vote`](https://docs.rs/atlas-sdk/latest/atlas_sdk/vote) -> [`atlas_vote_interface`](https://docs.rs/atlas-vote-interface/latest/atlas_vote_interface)

### atlas-program

The following modules have been removed, please use their component crates
directly:

* [`address_lookup_table`](https://docs.rs/atlas-program/latest/atlas_program/address_lookup_table) -> [`atlas_address_lookup_table_interface`](https://docs.rs/atlas-address-lookup-table-interface/latest/atlas_address_lookup_table_interface/)
* [`bpf_loader_upgradeable`](https://docs.rs/atlas-program/latest/atlas_program/bpf_loader_upgradeable) -> [`atlas_loader_v3_interface`](https://docs.rs/atlas-loader-v3-interface/latest/atlas_loader_v3_interface)
* [`decode_error`](https://docs.rs/atlas-program/latest/atlas_program/decode_error) -> [`atlas_decode_error`](https://docs.rs/atlas-decode-error/latest/atlas_decode_error)
* [`feature`](https://docs.rs/atlas-program/latest/atlas_program/feature) -> [`atlas_feature_gate_interface`](https://docs.rs/atlas-feature-gate-interface/latest/atlas_feature_gate_interface)
* [`loader_instruction`](https://docs.rs/atlas-program/latest/atlas_program/loader_instruction) -> [`atlas_loader_v2_interface`](https://docs.rs/atlas-loader-v2-interface/latest/atlas_loader_v2_interface)
* [`loader_upgradeable_instruction`](https://docs.rs/atlas-program/latest/atlas_program/loader_upgradeable_instruction) -> [`atlas_loader_v3_interface::instruction`](https://docs.rs/atlas-loader-v3-interface/latest/atlas_loader_v3_interface/instruction)
* [`loader_v4`](https://docs.rs/atlas-program/latest/atlas_program/loader_v4) -> [`atlas_loader_v4_interface`](https://docs.rs/atlas-loader-v4-interface/latest/atlas_loader_v4_interface)
* [`loader_v4_instruction`](https://docs.rs/atlas-program/latest/atlas_program/loader_v4_instruction) -> [`atlas_loader_v4_interface::instruction`](https://docs.rs/atlas-loader-v4-interface/latest/atlas_loader_v4_interface/instruction)
* [`message`](https://docs.rs/atlas-program/latest/atlas_program/message) -> [`atlas_message`](https://docs.rs/atlas-message/latest/atlas_message)
* [`nonce`](https://docs.rs/atlas-program/latest/atlas_program/nonce) -> [`atlas_nonce`](https://docs.rs/atlas-nonce/latest/atlas_nonce)
* [`program_utils`](https://docs.rs/atlas-program/latest/atlas_program/program_utils) -> [`atlas_bincode::limited_deserialize`](https://docs.rs/atlas-bincode/latest/atlas_bincode)
* [`sanitize`](https://docs.rs/atlas-program/latest/atlas_program/sanitize) -> [`atlas_sanitize`](https://docs.rs/atlas-sanitize/latest/atlas_sanitize)
* [`sdk_ids`](https://docs.rs/atlas-program/latest/atlas_program/sdk_ids) -> [`atlas_sdk_ids`](https://docs.rs/atlas-sdk-ids/latest/atlas_sdk_ids)
* [`stake`](https://docs.rs/atlas-program/latest/atlas_program/stake) -> [`atlas_stake_interface`](https://docs.rs/atlas-stake-interface/latest/atlas_stake_interface)
* [`stake_history`](https://docs.rs/atlas-program/latest/atlas_program/stake_history) -> [`atlas_stake_interface::stake_history`](https://docs.rs/atlas-stake-interface/latest/atlas_stake_interface/stake_history)
* [`system_instruction`](https://docs.rs/atlas-program/latest/atlas_program/system_instruction) -> [`atlas_system_interface::instruction`](https://docs.rs/atlas-system-interface/latest/atlas_system_interface/instruction)
* [`system_program`](https://docs.rs/atlas-program/latest/atlas_program/system_program) -> [`atlas_system_interface::program`](https://docs.rs/atlas-system-interface/latest/atlas_system_interface/program)
* [`vote`](https://docs.rs/atlas-program/latest/atlas_program/vote) -> [`atlas_vote_interface`](https://docs.rs/atlas-vote-interface/latest/atlas_vote_interface)

## Building

### **1. Install rustc, cargo and rustfmt.**

```console
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

### **2. Download the source code.**

```console
git clone https://github.com/atlaschainorg/atlas-sdk.git
cd atlas-sdk
```

When building the master branch, please make sure you are using the version
specified in the repo's `rust-toolchain.toml` by running:

```console
rustup show
```

This command will download the toolchain if it is missing in the system.

### **3. Test.**

```console
cargo test
```

## For Agave Developers

### Patching a local atlas-sdk repository

If your change to Agave also entails changes to the SDK, you will need to patch
your Agave repo to use a local checkout of atlas-sdk crates.

To patch all of the crates in this repo for Agave, just run:

```console
./scripts/patch-crates-no-header.sh <AGAVE_PATH> <ATLAS_SDK_PATH>
```

### Publishing a crate from this repository

NOTE: The repo currently contains unpublished breaking changes, so please
double-check before publishing any crates!

Unlike Agave, the atlas-sdk crates are versioned independently, and published
as needed.

If you need to publish a crate, you can use the "Publish Crate" GitHub Action.
Simply type in the path to the crate directory you want to release, ie.
`program-entrypoint`, along with the kind of release, either `patch`, `minor`,
`major`, or a specific version string.

The publish job will run checks, bump the crate version, commit and tag the
bump, publish the crate to crates.io, and finally create GitHub Release with
a simple changelog of all commits to the crate since the previous release.

### Backports

If you would like to backport a pull request, simply add the appropriate label,
named `backport <BRANCH_NAME>`.

For example, to create a backport to the `maintenance/v2.x` branch, just add the
`backport maintenance/v2.x` label.

## Testing

Certain tests, such as `rustfmt` and `clippy`, require the nightly rustc
configured on the repository. To easily install it, use the `./cargo` helper
script in the root of the repository:

```console
./cargo nightly tree
```

### Basic testing

Run the test suite:

```console
cargo test
```

Alternatively, there is a helper script:

```console
./scripts/test-stable.sh
```

### Formatting

Format code for rustfmt check:

```console
./cargo nightly fmt --all
```

The check can be run with a helper script:

```console
./scripts/check-fmt.sh
```

### Clippy / Linting

To check the clippy lints:

```console
./scripts/check-clippy.sh
```

### Benchmarking

Run the benchmarks:

```console
./scripts/test-bench.sh
```

### Code coverage

To generate code coverage statistics:

```console
./scripts/test-coverage.sh
$ open target/cov/lcov-local/index.html
```

Code coverage requires `llvm-tools-preview` for the configured nightly
toolchain. To install the component, run the command output by the script if it
fails to find the component:

```console
rustup component add llvm-tools-preview --toolchain=<NIGHTLY_TOOLCHAIN>
```
