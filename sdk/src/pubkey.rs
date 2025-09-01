#[cfg(feature = "full")]
pub use atlas_pubkey::new_rand;
#[cfg(target_os = "atlas")]
pub use atlas_pubkey::syscalls;
pub use atlas_pubkey::{
    bytes_are_curve_point, ParsePubkeyError, Pubkey, PubkeyError, MAX_SEEDS, MAX_SEED_LEN,
    PUBKEY_BYTES,
};
