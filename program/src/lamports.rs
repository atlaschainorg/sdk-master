//! Re-exports the [`LamportsError`] type for backwards compatibility.
#[deprecated(
    since = "2.1.0",
    note = "Use atlas_instruction_error::LamportsError instead"
)]
pub use atlas_instruction_error::LamportsError;
