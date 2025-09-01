//! Information about the last restart slot (hard fork).
//!
//! The _last restart sysvar_ provides access to the last restart slot kept in the
//! bank fork for the slot on the fork that executes the current transaction.
//! In case there was no fork it returns _0_.
//!
//! [`LastRestartSlot`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! See also the Atlas [SIMD proposal][simd].
//!
//! [simd]: https://github.com/atlas-foundation/atlas-improvement-documents/blob/main/proposals/0047-syscall-and-sysvar-for-last-restart-slot.md
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use atlas_account_info::AccountInfo;
//! # use atlas_msg::msg;
//! # use atlas_sysvar::Sysvar;
//! # use atlas_program_error::ProgramResult;
//! # use atlas_pubkey::Pubkey;
//! # use atlas_last_restart_slot::LastRestartSlot;
//!
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let last_restart_slot = LastRestartSlot::get();
//!     msg!("last restart slot: {:?}", last_restart_slot);
//!
//!     Ok(())
//! }
//! ```
//!
#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
use crate::{impl_sysvar_get, Sysvar};
pub use {
    atlas_last_restart_slot::LastRestartSlot,
    atlas_sdk_ids::sysvar::last_restart_slot::{check_id, id, ID},
};

impl Sysvar for LastRestartSlot {
    impl_sysvar_get!(atlas_get_last_restart_slot);
}

#[cfg(feature = "bincode")]
impl SysvarSerialize for LastRestartSlot {}
