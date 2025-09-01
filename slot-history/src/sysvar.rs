pub use atlas_sdk_ids::sysvar::slot_history::{check_id, id, ID};
use {crate::SlotHistory, atlas_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(SlotHistory);
