pub use atlas_sdk_ids::sysvar::slot_hashes::{check_id, id, ID};
use {crate::SlotHashes, atlas_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(SlotHashes);
