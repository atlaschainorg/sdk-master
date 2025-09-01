//! Hashing with the [SHA-256] hash function, and a general [`Hash`] type.
//!
//! [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
//! [`Hash`]: struct@Hash

#[cfg(not(target_os = "atlas"))]
pub use atlas_sha256_hasher::Hasher;
pub use {
    atlas_hash::{Hash, ParseHashError, HASH_BYTES},
    atlas_sha256_hasher::{hash, hashv},
};
