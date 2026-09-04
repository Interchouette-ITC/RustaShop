//! `rustashop` application kernel crate.
//!
//! Exposes a diagnostics marker until the serenade application lifecycle is
//! wired into this package. Domain and HTTP crates do not depend on that wiring.

/// Diagnostics marker for kernel integration status.
pub const SERENADE_KERNEL_PENDING: &str = "serenade-pending";

/// Returns the kernel integration status for health and diagnostics surfaces.
#[must_use]
pub const fn kernel_status() -> &'static str {
    SERENADE_KERNEL_PENDING
}
