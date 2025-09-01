//! Information about epoch duration.
//!
//! The _epoch schedule_ sysvar provides access to the [`EpochSchedule`] type,
//! which includes the number of slots per epoch, timing of leader schedule
//! selection, and information about epoch warm-up time.
//!
//! [`EpochSchedule`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! See also the Atlas [documentation on the epoch schedule sysvar][sdoc].
//!
//! [sdoc]: https://docs.atlaslabs.com/runtime/sysvars#epochschedule
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use atlas_account_info::AccountInfo;
//! # use atlas_epoch_schedule::EpochSchedule;
//! # use atlas_msg::msg;
//! # use atlas_program_error::{ProgramError, ProgramResult};
//! # use atlas_pubkey::Pubkey;
//! # use atlas_sdk_ids::sysvar::epoch_schedule;
//! # use atlas_sysvar::Sysvar;
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let epoch_schedule = EpochSchedule::get()?;
//!     msg!("epoch_schedule: {:#?}", epoch_schedule);
//!
//!     Ok(())
//! }
//! #
//! # use atlas_sysvar_id::SysvarId;
//! # let p = EpochSchedule::id();
//! # let l = &mut 1120560;
//! # let d = &mut vec![0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//! # let a = AccountInfo::new(&p, false, false, l, d, &p, false);
//! # let accounts = &[a.clone(), a];
//! # process_instruction(
//! #     &Pubkey::new_unique(),
//! #     accounts,
//! #     &[],
//! # )?;
//! # Ok::<(), ProgramError>(())
//! ```
//!
//! Accessing via on-chain program's account parameters:
//!
//! ```
//! # use atlas_account_info::{AccountInfo, next_account_info};
//! # use atlas_epoch_schedule::EpochSchedule;
//! # use atlas_msg::msg;
//! # use atlas_program_error::{ProgramError, ProgramResult};
//! # use atlas_pubkey::Pubkey;
//! # use atlas_sdk_ids::sysvar::epoch_schedule;
//! # use atlas_sysvar::{Sysvar, SysvarSerialize};
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!     let epoch_schedule_account_info = next_account_info(account_info_iter)?;
//!
//!     assert!(epoch_schedule::check_id(epoch_schedule_account_info.key));
//!
//!     let epoch_schedule = EpochSchedule::from_account_info(epoch_schedule_account_info)?;
//!     msg!("epoch_schedule: {:#?}", epoch_schedule);
//!
//!     Ok(())
//! }
//! #
//! # use atlas_sysvar_id::SysvarId;
//! # let p = EpochSchedule::id();
//! # let l = &mut 1120560;
//! # let d = &mut vec![0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//! # let a = AccountInfo::new(&p, false, false, l, d, &p, false);
//! # let accounts = &[a.clone(), a];
//! # process_instruction(
//! #     &Pubkey::new_unique(),
//! #     accounts,
//! #     &[],
//! # )?;
//! # Ok::<(), ProgramError>(())
//! ```
//!
//! Accessing via the RPC client:
//!
//! ```
//! # use atlas_epoch_schedule::EpochSchedule;
//! # use atlas_example_mocks::atlas_account;
//! # use atlas_example_mocks::atlas_rpc_client;
//! # use atlas_rpc_client::rpc_client::RpcClient;
//! # use atlas_account::Account;
//! # use atlas_sdk_ids::sysvar::epoch_schedule;
//! # use anyhow::Result;
//! #
//! fn print_sysvar_epoch_schedule(client: &RpcClient) -> Result<()> {
//! #   client.set_get_account_response(epoch_schedule::ID, Account {
//! #       lamports: 1120560,
//! #       data: vec![0, 32, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
//! #       owner: atlas_sdk_ids::system_program::ID,
//! #       executable: false,
//! # });
//! #
//!     let epoch_schedule = client.get_account(&epoch_schedule::ID)?;
//!     let data: EpochSchedule = bincode::deserialize(&epoch_schedule.data)?;
//!
//!     Ok(())
//! }
//! #
//! # let client = RpcClient::new(String::new());
//! # print_sysvar_epoch_schedule(&client)?;
//! #
//! # Ok::<(), anyhow::Error>(())
//! ```
#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
use crate::{impl_sysvar_get, Sysvar};
pub use {
    atlas_epoch_schedule::EpochSchedule,
    atlas_sdk_ids::sysvar::epoch_schedule::{check_id, id, ID},
};

impl Sysvar for EpochSchedule {
    impl_sysvar_get!(atlas_get_epoch_schedule_sysvar);
}

#[cfg(feature = "bincode")]
impl SysvarSerialize for EpochSchedule {}
