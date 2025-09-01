pub use atlas_sdk_ids::sysvar::rent::{check_id, id, ID};
use {crate::Rent, atlas_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Rent);
