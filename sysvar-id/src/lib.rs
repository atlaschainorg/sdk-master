//! Access to special accounts with dynamically-updated data.
//!
//! Sysvars are special accounts that contain dynamically-updated data about the
//! network cluster, the blockchain history, and the executing transaction. Each
//! sysvar is defined in its own crate. The [`clock`], [`epoch_schedule`],
//! [`instructions`], and [`rent`] sysvars are most useful to on-chain programs.
//!
//! [`clock`]: https://docs.rs/atlas-clock/latest
//! [`epoch_schedule`]: https://docs.rs/atlas-epoch-schedule/latest
//! [`instructions`]: https://docs.rs/atlas-program/latest/atlas_program/sysvar/instructions
//! [`rent`]: https://docs.rs/atlas-rent/latest
//!
//! All sysvar accounts are owned by the account identified by [`atlas_sysvar::ID`].
//!
//! [`atlas_sysvar::ID`]: crate::ID
//!
//! For more details see the Atlas [documentation on sysvars][sysvardoc].
//!
//! [sysvardoc]: https://docs.atlaslabs.com/runtime/sysvars

/// Re-export types required for macros
pub use {
    atlas_pubkey::{declare_deprecated_id, declare_id, Pubkey},
    atlas_sdk_ids::sysvar::{check_id, id, ID},
};

/// A type that holds sysvar data and has an associated sysvar `Pubkey`.
pub trait SysvarId {
    /// The `Pubkey` of the sysvar.
    fn id() -> Pubkey;

    /// Returns `true` if the given pubkey is the program ID.
    fn check_id(pubkey: &Pubkey) -> bool;
}

/// Implements [`SysvarId`] for a module that already uses
/// `declare_id``
#[macro_export]
macro_rules! impl_sysvar_id(
    ($type:ty) => {
        impl $crate::SysvarId for $type {
            fn id() -> $crate::Pubkey {
                id()
            }

            fn check_id(pubkey: &$crate::Pubkey) -> bool {
                check_id(pubkey)
            }
        }
    }
);

/// Implements [`SysvarId`] for a module that already uses
/// `declare_deprecated_id``
#[macro_export]
macro_rules! impl_deprecated_sysvar_id(
    ($type:ty) => {
        impl $crate::SysvarId for $type {
            fn id() -> $crate::Pubkey {
                #[allow(deprecated)]
                id()
            }

            fn check_id(pubkey: &$crate::Pubkey) -> bool {
                #[allow(deprecated)]
                check_id(pubkey)
            }
        }
    }
);

/// Declares an ID that implements [`SysvarId`].
#[macro_export]
macro_rules! declare_sysvar_id(
    ($name:expr, $type:ty) => (
        $crate::declare_id!($name);
        $crate::impl_sysvar_id!($type);
    )
);

/// Same as [`declare_sysvar_id`] except that it reports that this ID has been deprecated.
#[macro_export]
macro_rules! declare_deprecated_sysvar_id(
    ($name:expr, $type:ty) => (
        $crate::declare_deprecated_id!($name);
        $crate::impl_deprecated_sysvar_id!($type);
    )
);
