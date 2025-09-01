//! The Atlas host and client SDK.
//!
//! This is the base library for all off-chain programs that interact with
//! Atlas or otherwise operate on Atlas data structures. On-chain programs
//! instead use the [`atlas-program`] crate, the modules of which are
//! re-exported by this crate, like the relationship between the Rust
//! `core` and `std` crates. As much of the functionality of this crate is
//! provided by `atlas-program`, see that crate's documentation for an
//! overview.
//!
//! [`atlas-program`]: https://docs.rs/atlas-program
//!
//! Many of the modules in this crate are primarily of use to the Atlas runtime
//! itself. Additional crates provide capabilities built on `atlas-sdk`, and
//! many programs will need to link to those crates as well, particularly for
//! clients communicating with Atlas nodes over RPC.
//!
//! Such crates include:
//!
//! - [`atlas-client`] - For interacting with a Atlas node via the [JSON-RPC API][json].
//! - [`atlas-cli-config`] - Loading and saving the Atlas CLI configuration file.
//! - [`atlas-clap-utils`] - Routines for setting up the CLI using [`clap`], as
//!   used by the Atlas CLI. Includes functions for loading all types of
//!   signers supported by the CLI.
//!
//! [`atlas-client`]: https://docs.rs/atlas-client
//! [`atlas-cli-config`]: https://docs.rs/atlas-cli-config
//! [`atlas-clap-utils`]: https://docs.rs/atlas-clap-utils
//! [json]: https://atlas.com/docs/rpc
//! [`clap`]: https://docs.rs/clap

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Allows macro expansion of `use ::atlas_sdk::*` to work within this crate
extern crate self as atlas_sdk;

#[deprecated(since = "2.2.0", note = "Use `atlas-message` crate instead")]
pub use atlas_message as message;
#[cfg(feature = "borsh")]
pub use atlas_program::borsh1;
#[cfg(not(target_os = "atlas"))]
pub use atlas_program::program_stubs;
pub use atlas_program::{
    account_info, big_mod_exp, blake3, bpf_loader, bpf_loader_deprecated, clock, config,
    custom_heap_default, custom_panic_default, debug_account_data, declare_deprecated_sysvar_id,
    declare_sysvar_id, ed25519_program, epoch_rewards, epoch_schedule, fee_calculator,
    impl_sysvar_get, incinerator, instruction, keccak, lamports, msg, native_token, program,
    program_error, program_option, program_pack, rent, secp256k1_program, serialize_utils,
    slot_hashes, slot_history, stable_layout, syscalls, sysvar, unchecked_div_by_const,
};
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `atlas-signer` crate instead")]
pub use atlas_signer::signers;
pub mod entrypoint;
pub mod entrypoint_deprecated;
pub mod example_mocks;
pub mod hash;
pub mod log;
pub mod native_loader;
pub mod pubkey;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `atlas-shred-version` crate instead")]
pub use atlas_shred_version as shred_version;
pub mod signature;
pub mod signer;
pub mod transaction;
pub mod transport;

#[deprecated(since = "2.1.0", note = "Use `atlas-account` crate instead")]
pub use atlas_account as account;
#[deprecated(
    since = "2.1.0",
    note = "Use `atlas_account::state_traits` crate instead"
)]
pub use atlas_account::state_traits as account_utils;
#[deprecated(since = "2.2.0", note = "Use `atlas-epoch-info` crate instead")]
pub use atlas_epoch_info as epoch_info;
#[deprecated(
    since = "2.2.0",
    note = "Use `atlas-epoch-rewards-hasher` crate instead"
)]
pub use atlas_epoch_rewards_hasher as epoch_rewards_hasher;
#[deprecated(since = "2.2.0", note = "Use `atlas-fee-structure` crate instead")]
pub use atlas_fee_structure as fee;
#[deprecated(since = "2.1.0", note = "Use `atlas-inflation` crate instead")]
pub use atlas_inflation as inflation;
#[deprecated(
    since = "2.2.0",
    note = "Use `atlas_message::inner_instruction` instead"
)]
pub use atlas_message::inner_instruction;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `atlas-offchain-message` crate instead")]
pub use atlas_offchain_message as offchain_message;
#[deprecated(since = "2.1.0", note = "Use `atlas-program-memory` crate instead")]
pub use atlas_program_memory as program_memory;
#[deprecated(since = "2.1.0", note = "Use `atlas_pubkey::pubkey` instead")]
/// Convenience macro to define a static public key.
///
/// Input: a single literal base58 string representation of a Pubkey
///
/// # Example
///
/// ```
/// use std::str::FromStr;
/// use atlas_program::{pubkey, pubkey::Pubkey};
///
/// static ID: Pubkey = pubkey!("My11111111111111111111111111111111111111111");
///
/// let my_id = Pubkey::from_str("My11111111111111111111111111111111111111111").unwrap();
/// assert_eq!(ID, my_id);
/// ```
pub use atlas_pubkey::pubkey;
#[deprecated(since = "2.1.0", note = "Use `atlas-sanitize` crate instead")]
pub use atlas_sanitize as sanitize;
/// Same as `declare_id` except report that this id has been deprecated.
pub use atlas_sdk_macro::declare_deprecated_id;
/// Convenience macro to declare a static public key and functions to interact with it.
///
/// Input: a single literal base58 string representation of a program's id
///
/// # Example
///
/// ```
/// # // wrapper is used so that the macro invocation occurs in the item position
/// # // rather than in the statement position which isn't allowed.
/// use std::str::FromStr;
/// use atlas_sdk::{declare_id, pubkey::Pubkey};
///
/// # mod item_wrapper {
/// #   use atlas_sdk::declare_id;
/// declare_id!("My11111111111111111111111111111111111111111");
/// # }
/// # use item_wrapper::id;
///
/// let my_id = Pubkey::from_str("My11111111111111111111111111111111111111111").unwrap();
/// assert_eq!(id(), my_id);
/// ```
pub use atlas_sdk_macro::declare_id;
/// Convenience macro to define multiple static public keys.
pub use atlas_sdk_macro::pubkeys;
#[deprecated(since = "2.2.0", note = "Use `atlas-serde` crate instead")]
pub use atlas_serde as deserialize_utils;
#[deprecated(since = "2.1.0", note = "Use `atlas-serde-varint` crate instead")]
pub use atlas_serde_varint as serde_varint;
#[deprecated(since = "2.1.0", note = "Use `atlas-short-vec` crate instead")]
pub use atlas_short_vec as short_vec;
#[deprecated(since = "2.2.0", note = "Use `atlas-time-utils` crate instead")]
pub use atlas_time_utils as timing;
#[cfg(feature = "full")]
#[deprecated(
    since = "2.2.0",
    note = "Use `atlas_transaction::simple_vote_transaction_checker` instead"
)]
pub use atlas_transaction::simple_vote_transaction_checker;

pub extern crate bs58;
