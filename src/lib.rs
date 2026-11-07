//! SigmaOS-du — Disk usage model.
//!
//! Models file/directory size calculation, block size handling,
//! and human-readable formatting. Deterministic and std-only.

pub mod entry;
pub mod calculator;
pub mod formatter;

pub use entry::{DuEntry, EntryType};
pub use calculator::{DuCalculator, SizeResult};
pub use formatter::{SizeFormat, HumanFormatter};
