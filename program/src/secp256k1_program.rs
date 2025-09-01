//! The [secp256k1 native program][np].
//!
//! [np]: https://docs.atlaslabs.com/runtime/programs#secp256k1-program
//!
//! Constructors for secp256k1 program instructions, and documentation on the
//! program's usage can be found in [`atlas_sdk::secp256k1_instruction`].
//!
//! [`atlas_sdk::secp256k1_instruction`]: https://docs.rs/atlas-sdk/latest/atlas_sdk/secp256k1_instruction/index.html
pub use atlas_sdk_ids::secp256k1_program::{check_id, id, ID};
