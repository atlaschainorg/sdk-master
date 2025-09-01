//! Epoch rewards for current epoch
//!
//! The _epoch rewards_ sysvar provides access to the [`EpochRewards`] type,
//! which tracks whether the rewards period (including calculation and
//! distribution) is in progress, as well as the details needed to resume
//! distribution when starting from a snapshot during the rewards period. The
//! sysvar is repopulated at the start of the first block of each epoch.
//! Therefore, the sysvar contains data about the current epoch until a new
//! epoch begins. Fields in the sysvar include:
//!   - distribution starting block height
//!   - the number of partitions in the distribution
//!   - the parent-blockhash seed used to generate the partition hasher
//!   - the total rewards points calculated for the epoch
//!   - total rewards for epoch, in lamports
//!   - rewards for the epoch distributed so far, in lamports
//!   - whether the rewards period is active
//!
//! [`EpochRewards`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! See also the Atlas [documentation on the epoch rewards sysvar][sdoc].
//!
//! [sdoc]: https://docs.atlaslabs.com/runtime/sysvars#epochrewards
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use atlas_account_info::AccountInfo;
//! # use atlas_epoch_rewards::EpochRewards;
//! # use atlas_msg::msg;
//! # use atlas_program_error::{ProgramError, ProgramResult};
//! # use atlas_pubkey::Pubkey;
//! # use atlas_sysvar::Sysvar;
//! # use atlas_sdk_ids::sysvar::epoch_rewards;
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let epoch_rewards = EpochRewards::get()?;
//!     msg!("epoch_rewards: {:#?}", epoch_rewards);
//!
//!     Ok(())
//! }
//! #
//! # use atlas_sysvar_id::SysvarId;
//! # let p = EpochRewards::id();
//! # let l = &mut 1559040;
//! # let epoch_rewards = EpochRewards {
//! #     distribution_starting_block_height: 42,
//! #     total_rewards: 100,
//! #     distributed_rewards: 10,
//! #     active: true,
//! #     ..EpochRewards::default()
//! # };
//! # let mut d: Vec<u8> = bincode::serialize(&epoch_rewards).unwrap();
//! # let a = AccountInfo::new(&p, false, false, l, &mut d, &p, false);
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
//! # use atlas_epoch_rewards::EpochRewards;
//! # use atlas_msg::msg;
//! # use atlas_program_error::{ProgramError, ProgramResult};
//! # use atlas_pubkey::Pubkey;
//! # use atlas_sysvar::{Sysvar, SysvarSerialize};
//! # use atlas_sdk_ids::sysvar::epoch_rewards;
//! #
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!     let account_info_iter = &mut accounts.iter();
//!     let epoch_rewards_account_info = next_account_info(account_info_iter)?;
//!
//!     assert!(epoch_rewards::check_id(epoch_rewards_account_info.key));
//!
//!     let epoch_rewards = EpochRewards::from_account_info(epoch_rewards_account_info)?;
//!     msg!("epoch_rewards: {:#?}", epoch_rewards);
//!
//!     Ok(())
//! }
//! #
//! # use atlas_sysvar_id::SysvarId;
//! # let p = EpochRewards::id();
//! # let l = &mut 1559040;
//! # let epoch_rewards = EpochRewards {
//! #     distribution_starting_block_height: 42,
//! #     total_rewards: 100,
//! #     distributed_rewards: 10,
//! #     active: true,
//! #     ..EpochRewards::default()
//! # };
//! # let mut d: Vec<u8> = bincode::serialize(&epoch_rewards).unwrap();
//! # let a = AccountInfo::new(&p, false, false, l, &mut d, &p, false);
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
//! # use atlas_epoch_rewards::EpochRewards;
//! # use atlas_example_mocks::atlas_account;
//! # use atlas_example_mocks::atlas_rpc_client;
//! # use atlas_rpc_client::rpc_client::RpcClient;
//! # use atlas_account::Account;
//! # use atlas_sdk_ids::sysvar::epoch_rewards;
//! # use anyhow::Result;
//! #
//! fn print_sysvar_epoch_rewards(client: &RpcClient) -> Result<()> {
//! #   let epoch_rewards = EpochRewards {
//! #       distribution_starting_block_height: 42,
//! #       total_rewards: 100,
//! #       distributed_rewards: 10,
//! #       active: true,
//! #       ..EpochRewards::default()
//! #   };
//! #   let data: Vec<u8> = bincode::serialize(&epoch_rewards)?;
//! #   client.set_get_account_response(epoch_rewards::ID, Account {
//! #       lamports: 1120560,
//! #       data,
//! #       owner: atlas_sdk_ids::system_program::ID,
//! #       executable: false,
//! # });
//! #
//!     let epoch_rewards = client.get_account(&epoch_rewards::ID)?;
//!     let data: EpochRewards = bincode::deserialize(&epoch_rewards.data)?;
//!
//!     Ok(())
//! }
//! #
//! # let client = RpcClient::new(String::new());
//! # print_sysvar_epoch_rewards(&client)?;
//! #
//! # Ok::<(), anyhow::Error>(())
//! ```

#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
use crate::{impl_sysvar_get, Sysvar};
pub use {
    atlas_epoch_rewards::EpochRewards,
    atlas_sdk_ids::sysvar::epoch_rewards::{check_id, id, ID},
};

impl Sysvar for EpochRewards {
    impl_sysvar_get!(atlas_get_epoch_rewards_sysvar);
}

#[cfg(feature = "bincode")]
impl SysvarSerialize for EpochRewards {}
