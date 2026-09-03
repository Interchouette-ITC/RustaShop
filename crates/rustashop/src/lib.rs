//! `RustaShop` application kernel.
//!
//! Serenade path dependency lands in issue #49. Until then this crate holds the
//! composition root placeholder.

/// Marker until the Serenade kernel is wired as a path or git dependency.
pub const SERENADE_KERNEL_PENDING: &str = "serenade-pending";

/// Returns the kernel integration status for health and diagnostics surfaces.
#[must_use]
pub const fn kernel_status() -> &'static str {
    SERENADE_KERNEL_PENDING
}
