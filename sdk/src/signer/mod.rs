#![cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `atlas-presigner` crate instead")]
pub use atlas_presigner as presigner;
#[deprecated(since = "2.2.0", note = "Use `atlas-seed-derivable` crate instead")]
pub use atlas_seed_derivable::SeedDerivable;
#[deprecated(since = "2.2.0", note = "Use `atlas-signer` crate instead")]
pub use atlas_signer::{
    null_signer, signers, unique_signers, EncodableKey, EncodableKeypair, Signer, SignerError,
};
pub mod keypair;
