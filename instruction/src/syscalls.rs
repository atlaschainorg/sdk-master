#[cfg(feature = "syscalls")]
use crate::Instruction;
#[cfg(target_os = "atlas")]
pub use {
    crate::{AccountMeta, ProcessedSiblingInstruction},
    atlas_define_syscall::{define_syscall, definitions::atlas_get_stack_height},
    atlas_pubkey::Pubkey,
};

#[cfg(target_os = "atlas")]
define_syscall!(fn atlas_get_processed_sibling_instruction(index: u64, meta: *mut ProcessedSiblingInstruction, program_id: *mut Pubkey, data: *mut u8, accounts: *mut AccountMeta) -> u64);

/// Returns a sibling instruction from the processed sibling instruction list.
///
/// The processed sibling instruction list is a reverse-ordered list of
/// successfully processed sibling instructions. For example, given the call flow:
///
/// A
/// B -> C -> D
/// B -> E
/// B -> F
///
/// Then B's processed sibling instruction list is: `[A]`
/// Then F's processed sibling instruction list is: `[E, C]`
#[cfg(feature = "syscalls")]
pub fn get_processed_sibling_instruction(index: usize) -> Option<Instruction> {
    #[cfg(target_os = "atlas")]
    {
        let mut meta = ProcessedSiblingInstruction::default();
        let mut program_id = atlas_pubkey::Pubkey::default();

        if 1 == unsafe {
            atlas_get_processed_sibling_instruction(
                index as u64,
                &mut meta,
                &mut program_id,
                &mut u8::default(),
                &mut AccountMeta::default(),
            )
        } {
            let mut data = std::vec::Vec::new();
            let mut accounts = std::vec::Vec::new();
            data.resize_with(meta.data_len as usize, u8::default);
            accounts.resize_with(meta.accounts_len as usize, AccountMeta::default);

            let _ = unsafe {
                atlas_get_processed_sibling_instruction(
                    index as u64,
                    &mut meta,
                    &mut program_id,
                    data.as_mut_ptr(),
                    accounts.as_mut_ptr(),
                )
            };

            Some(Instruction::new_with_bytes(program_id, &data, accounts))
        } else {
            None
        }
    }

    #[cfg(not(target_os = "atlas"))]
    {
        core::hint::black_box(index);
        // Same value used in `atlas_sysvar::program_stubs`.
        None
    }
}

/// Get the current stack height.
///
/// Transaction-level instructions are height
/// [`crate::TRANSACTION_LEVEL_STACK_HEIGHT`]`, fist invoked inner instruction
/// is height `TRANSACTION_LEVEL_STACK_HEIGHT + 1`, and so forth.
#[cfg(feature = "syscalls")]
pub fn get_stack_height() -> usize {
    #[cfg(target_os = "atlas")]
    unsafe {
        atlas_get_stack_height() as usize
    }

    #[cfg(not(target_os = "atlas"))]
    // Same value used in `atlas_sysvar::program_stubs`.
    0
}
