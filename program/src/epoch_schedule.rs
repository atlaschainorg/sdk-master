#[deprecated(
    since = "2.1.0",
    note = "Use atlas-clock and atlas-epoch-schedule crates instead."
)]
pub use {
    atlas_clock::{Epoch, Slot, DEFAULT_SLOTS_PER_EPOCH},
    atlas_epoch_schedule::*,
};
