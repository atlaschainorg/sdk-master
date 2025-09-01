//! Information about the network’s clock, ticks, slots, etc.
//!
//! The _clock sysvar_ provides access to the [`Clock`] type, which includes the
//! current slot, the current epoch, and the approximate real-world time of the
//! slot.
//!
//! [`Clock`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! See also the Atlas [documentation on the clock sysvar][sdoc].
//!
//! [sdoc]: https://docs.atlaslabs.com/runtime/sysvars#clock
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use atlas_account_info::AccountInfo;
//! # use atlas_clock::Clock;
//! # use atlas_msg::msg;
//! # use atlas_program_error::{ProgramError, ProgramResult};
//! # use atlas_pubkey::Pubkey;
//! # use atlas_sysvar::Sysvar;
//! #
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let clock = Clock::get()?;
//!     msg!("clock: {:#?}", clock);
//!
//!     Ok(())
//! }
//! #
//! # use atlas_sysvar_id::SysvarId;
//! # let p = Clock::id();
//! # let l = &mut 1169280;
//! # let d = &mut vec![240, 153, 233, 7, 0, 0, 0, 0, 11, 115, 118, 98, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 0, 0, 52, 1, 0, 0, 0, 0, 0, 0, 121, 50, 119, 98, 0, 0, 0, 0];
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
//! # use atlas_clock::Clock;
//! # use atlas_msg::msg;
//! # use atlas_program_error::{ProgramError, ProgramResult};
//! # use atlas_pubkey::Pubkey;
//! # use atlas_sysvar::{Sysvar, SysvarSerialize};
//! # use atlas_sdk_ids::sysvar::clock;
//! #
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!     let clock_account_info = next_account_info(account_info_iter)?;
//!
//!     assert!(clock::check_id(clock_account_info.key));
//!
//!     let clock = Clock::from_account_info(clock_account_info)?;
//!     msg!("clock: {:#?}", clock);
//!
//!     Ok(())
//! }
//! #
//! # use atlas_sysvar_id::SysvarId;
//! # let p = Clock::id();
//! # let l = &mut 1169280;
//! # let d = &mut vec![240, 153, 233, 7, 0, 0, 0, 0, 11, 115, 118, 98, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 0, 0, 52, 1, 0, 0, 0, 0, 0, 0, 121, 50, 119, 98, 0, 0, 0, 0];
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
//! # use atlas_clock::Clock;
//! # use atlas_example_mocks::atlas_account;
//! # use atlas_example_mocks::atlas_rpc_client;
//! # use atlas_rpc_client::rpc_client::RpcClient;
//! # use atlas_account::Account;
//! # use atlas_sdk_ids::sysvar::clock;
//! # use anyhow::Result;
//! #
//! fn print_sysvar_clock(client: &RpcClient) -> Result<()> {
//! #   client.set_get_account_response(clock::ID, Account {
//! #       lamports: 1169280,
//! #       data: vec![240, 153, 233, 7, 0, 0, 0, 0, 11, 115, 118, 98, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 0, 0, 52, 1, 0, 0, 0, 0, 0, 0, 121, 50, 119, 98, 0, 0, 0, 0],
//! #       owner: atlas_sdk_ids::system_program::ID,
//! #       executable: false,
//! #   });
//! #
//!     let clock = client.get_account(&clock::ID)?;
//!     let data: Clock = bincode::deserialize(&clock.data)?;
//!
//!     Ok(())
//! }
//! #
//! # let client = RpcClient::new(String::new());
//! # print_sysvar_clock(&client)?;
//! #
//! # Ok::<(), anyhow::Error>(())
//! ```

#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
use crate::{impl_sysvar_get, Sysvar};
pub use {
    atlas_clock::Clock,
    atlas_sdk_ids::sysvar::clock::{check_id, id, ID},
};

impl Sysvar for Clock {
    impl_sysvar_get!(atlas_get_clock_sysvar);
}

#[cfg(feature = "bincode")]
impl SysvarSerialize for Clock {}
