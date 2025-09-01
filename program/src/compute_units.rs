/// Return the remaining compute units the program may consume
#[inline]
pub fn atlas_remaining_compute_units() -> u64 {
    #[cfg(target_os = "atlas")]
    unsafe {
        crate::syscalls::atlas_remaining_compute_units()
    }

    #[cfg(not(target_os = "atlas"))]
    {
        crate::program_stubs::atlas_remaining_compute_units()
    }
}
