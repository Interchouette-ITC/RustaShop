//! Application kernel package for rustashop.
//!
//! Holds the Serenade integration marker. Domain and HTTP crates boot without
//! this package until the kernel path is wired ([#49](https://github.com/Interchouette-ITC/rustashop/issues/49)).

/// Diagnostics marker while Serenade lifecycle is not wired into this package.
pub const SERENADE_KERNEL_PENDING: &str = "serenade-pending";

/// Returns the kernel integration status for health and diagnostics surfaces.
#[must_use]
pub const fn kernel_status() -> &'static str {
    SERENADE_KERNEL_PENDING
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_status_is_pending_marker() {
        assert_eq!(kernel_status(), SERENADE_KERNEL_PENDING);
    }
}
