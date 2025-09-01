//! Implementations of syscalls used when `atlas-program` is built for non-SBF targets.

#![cfg(not(target_os = "atlas"))]

use {
    base64::{prelude::BASE64_STANDARD, Engine},
    atlas_account_info::AccountInfo,
    atlas_instruction::{error::UNSUPPORTED_SYSVAR, Instruction},
    atlas_program_error::ProgramResult,
    atlas_program_memory::stubs,
    atlas_pubkey::Pubkey,
    std::sync::{Arc, RwLock},
};

lazy_static::lazy_static! {
    static ref SYSCALL_STUBS: Arc<RwLock<Box<dyn SyscallStubs>>> = Arc::new(RwLock::new(Box::new(DefaultSyscallStubs {})));
}

// The default syscall stubs may not do much, but `set_syscalls()` can be used
// to swap in alternatives
pub fn set_syscall_stubs(syscall_stubs: Box<dyn SyscallStubs>) -> Box<dyn SyscallStubs> {
    std::mem::replace(&mut SYSCALL_STUBS.write().unwrap(), syscall_stubs)
}

pub trait SyscallStubs: Sync + Send {
    fn atlas_log(&self, message: &str) {
        println!("{message}");
    }
    fn atlas_log_compute_units(&self) {
        atlas_log("SyscallStubs: atlas_log_compute_units() not available");
    }
    fn atlas_remaining_compute_units(&self) -> u64 {
        atlas_log("SyscallStubs: atlas_remaining_compute_units() defaulting to 0");
        0
    }
    fn atlas_invoke_signed(
        &self,
        _instruction: &Instruction,
        _account_infos: &[AccountInfo],
        _signers_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        atlas_log("SyscallStubs: atlas_invoke_signed() not available");
        Ok(())
    }
    fn atlas_get_sysvar(
        &self,
        _sysvar_id_addr: *const u8,
        _var_addr: *mut u8,
        _offset: u64,
        _length: u64,
    ) -> u64 {
        UNSUPPORTED_SYSVAR
    }
    fn atlas_get_clock_sysvar(&self, _var_addr: *mut u8) -> u64 {
        UNSUPPORTED_SYSVAR
    }
    fn atlas_get_epoch_schedule_sysvar(&self, _var_addr: *mut u8) -> u64 {
        UNSUPPORTED_SYSVAR
    }
    fn atlas_get_fees_sysvar(&self, _var_addr: *mut u8) -> u64 {
        UNSUPPORTED_SYSVAR
    }
    fn atlas_get_rent_sysvar(&self, _var_addr: *mut u8) -> u64 {
        UNSUPPORTED_SYSVAR
    }
    fn atlas_get_epoch_rewards_sysvar(&self, _var_addr: *mut u8) -> u64 {
        UNSUPPORTED_SYSVAR
    }
    fn atlas_get_last_restart_slot(&self, _var_addr: *mut u8) -> u64 {
        UNSUPPORTED_SYSVAR
    }
    fn atlas_get_epoch_stake(&self, _vote_address: *const u8) -> u64 {
        0
    }
    /// # Safety
    unsafe fn atlas_memcpy(&self, dst: *mut u8, src: *const u8, n: usize) {
        stubs::atlas_memcpy(dst, src, n)
    }
    /// # Safety
    unsafe fn atlas_memmove(&self, dst: *mut u8, src: *const u8, n: usize) {
        stubs::atlas_memmove(dst, src, n)
    }
    /// # Safety
    unsafe fn atlas_memcmp(&self, s1: *const u8, s2: *const u8, n: usize, result: *mut i32) {
        stubs::atlas_memcmp(s1, s2, n, result)
    }
    /// # Safety
    unsafe fn atlas_memset(&self, s: *mut u8, c: u8, n: usize) {
        stubs::atlas_memset(s, c, n)
    }
    fn atlas_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
        None
    }
    fn atlas_set_return_data(&self, _data: &[u8]) {}
    fn atlas_log_data(&self, fields: &[&[u8]]) {
        println!(
            "data: {}",
            fields
                .iter()
                .map(|v| BASE64_STANDARD.encode(v))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
    fn atlas_get_processed_sibling_instruction(&self, _index: usize) -> Option<Instruction> {
        None
    }
    fn atlas_get_stack_height(&self) -> u64 {
        0
    }
}

struct DefaultSyscallStubs {}
impl SyscallStubs for DefaultSyscallStubs {}

pub fn atlas_log(message: &str) {
    SYSCALL_STUBS.read().unwrap().atlas_log(message);
}

pub fn atlas_log_64(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) {
    atlas_log(&format!(
        "{arg1:#x}, {arg2:#x}, {arg3:#x}, {arg4:#x}, {arg5:#x}"
    ));
}

pub fn atlas_log_compute_units() {
    SYSCALL_STUBS.read().unwrap().atlas_log_compute_units();
}

pub fn atlas_remaining_compute_units() -> u64 {
    SYSCALL_STUBS.read().unwrap().atlas_remaining_compute_units()
}

pub fn atlas_invoke_signed(
    instruction: &Instruction,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    SYSCALL_STUBS
        .read()
        .unwrap()
        .atlas_invoke_signed(instruction, account_infos, signers_seeds)
}

#[allow(dead_code)]
pub(crate) fn atlas_get_sysvar(
    sysvar_id_addr: *const u8,
    var_addr: *mut u8,
    offset: u64,
    length: u64,
) -> u64 {
    SYSCALL_STUBS
        .read()
        .unwrap()
        .atlas_get_sysvar(sysvar_id_addr, var_addr, offset, length)
}

pub(crate) fn atlas_get_clock_sysvar(var_addr: *mut u8) -> u64 {
    SYSCALL_STUBS.read().unwrap().atlas_get_clock_sysvar(var_addr)
}

pub(crate) fn atlas_get_epoch_schedule_sysvar(var_addr: *mut u8) -> u64 {
    SYSCALL_STUBS
        .read()
        .unwrap()
        .atlas_get_epoch_schedule_sysvar(var_addr)
}

pub(crate) fn atlas_get_fees_sysvar(var_addr: *mut u8) -> u64 {
    SYSCALL_STUBS.read().unwrap().atlas_get_fees_sysvar(var_addr)
}

pub(crate) fn atlas_get_rent_sysvar(var_addr: *mut u8) -> u64 {
    SYSCALL_STUBS.read().unwrap().atlas_get_rent_sysvar(var_addr)
}

pub(crate) fn atlas_get_last_restart_slot(var_addr: *mut u8) -> u64 {
    SYSCALL_STUBS
        .read()
        .unwrap()
        .atlas_get_last_restart_slot(var_addr)
}

pub fn atlas_get_epoch_stake(vote_address: *const u8) -> u64 {
    SYSCALL_STUBS
        .read()
        .unwrap()
        .atlas_get_epoch_stake(vote_address)
}

pub fn atlas_get_return_data() -> Option<(Pubkey, Vec<u8>)> {
    SYSCALL_STUBS.read().unwrap().atlas_get_return_data()
}

pub fn atlas_set_return_data(data: &[u8]) {
    SYSCALL_STUBS.read().unwrap().atlas_set_return_data(data)
}

pub fn atlas_log_data(data: &[&[u8]]) {
    SYSCALL_STUBS.read().unwrap().atlas_log_data(data)
}

pub fn atlas_get_processed_sibling_instruction(index: usize) -> Option<Instruction> {
    SYSCALL_STUBS
        .read()
        .unwrap()
        .atlas_get_processed_sibling_instruction(index)
}

pub fn atlas_get_stack_height() -> u64 {
    SYSCALL_STUBS.read().unwrap().atlas_get_stack_height()
}

pub(crate) fn atlas_get_epoch_rewards_sysvar(var_addr: *mut u8) -> u64 {
    SYSCALL_STUBS
        .read()
        .unwrap()
        .atlas_get_epoch_rewards_sysvar(var_addr)
}
