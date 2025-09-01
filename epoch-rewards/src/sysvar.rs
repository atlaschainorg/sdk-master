pub use atlas_sdk_ids::sysvar::epoch_rewards::{check_id, id, ID};
use {crate::EpochRewards, atlas_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(EpochRewards);
