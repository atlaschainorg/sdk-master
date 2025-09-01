pub use atlas_sdk_ids::sysvar::epoch_schedule::{check_id, id, ID};
use {crate::EpochSchedule, atlas_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(EpochSchedule);
