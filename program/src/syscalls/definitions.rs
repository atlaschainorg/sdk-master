#[deprecated(since = "2.1.0", note = "Use `atlas_cpi::syscalls` instead")]
pub use atlas_cpi::syscalls::{
    atlas_get_return_data, atlas_invoke_signed_c, atlas_invoke_signed_rust, atlas_set_return_data,
};
#[deprecated(
    since = "2.2.0",
    note = "Use `atlas_define_syscall::definitions` instead"
)]
pub use atlas_define_syscall::definitions::{
    sol_alt_bn128_compression, sol_alt_bn128_group_op, sol_big_mod_exp, sol_blake3,
    sol_curve_group_op, sol_curve_multiscalar_mul, sol_curve_pairing_map, sol_curve_validate_point,
    atlas_get_clock_sysvar, atlas_get_epoch_rewards_sysvar, atlas_get_epoch_schedule_sysvar,
    atlas_get_epoch_stake, atlas_get_fees_sysvar, atlas_get_last_restart_slot, atlas_get_rent_sysvar,
    atlas_get_sysvar, sol_keccak256, atlas_remaining_compute_units,
};
#[cfg(target_feature = "static-syscalls")]
pub use atlas_define_syscall::sys_hash;
#[deprecated(since = "2.1.0", note = "Use `atlas_instruction::syscalls` instead")]
pub use atlas_instruction::syscalls::{
    atlas_get_processed_sibling_instruction, atlas_get_stack_height,
};
#[deprecated(since = "2.1.0", note = "Use `atlas_msg::syscalls` instead.")]
pub use atlas_msg::syscalls::{atlas_log_, atlas_log_64_, atlas_log_compute_units_, atlas_log_data};
#[deprecated(
    since = "2.1.0",
    note = "Use `atlas_program_memory::syscalls` instead"
)]
pub use atlas_program_memory::syscalls::{atlas_memcmp_, atlas_memcpy_, atlas_memmove_, atlas_memset_};
#[deprecated(since = "2.1.0", note = "Use `atlas_pubkey::syscalls` instead")]
pub use atlas_pubkey::syscalls::{
    sol_create_program_address, atlas_log_pubkey, sol_try_find_program_address,
};
#[deprecated(
    since = "2.1.0",
    note = "Use `atlas_secp256k1_recover::sol_secp256k1_recover` instead"
)]
pub use atlas_secp256k1_recover::sol_secp256k1_recover;
#[deprecated(since = "2.1.0", note = "Use atlas_sha256_hasher::sol_sha256 instead")]
pub use atlas_sha256_hasher::sol_sha256;
