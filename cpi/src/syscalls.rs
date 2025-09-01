/// Syscall definitions used by `atlas_cpi`.
pub use atlas_define_syscall::definitions::{
    atlas_invoke_signed_c, atlas_invoke_signed_rust, atlas_set_return_data,
};
use {atlas_define_syscall::define_syscall, atlas_pubkey::Pubkey};

define_syscall!(fn atlas_get_return_data(data: *mut u8, length: u64, program_id: *mut Pubkey) -> u64);
