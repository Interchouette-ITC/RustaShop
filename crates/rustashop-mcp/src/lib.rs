//! Axum MCP and agent HTTP tools.
//!
//! Workspace member and crate name marker. No MCP routes or tool handlers yet.
//! Re-exports the application kernel status for shared diagnostics.

/// Crate name marker for workspace and diagnostics checks.
pub const MCP_CRATE: &str = "rustashop-mcp";

/// Kernel integration status from the `rustashop` application package.
#[must_use]
pub const fn kernel_status() -> &'static str {
    rustashop::kernel_status()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_marker_and_kernel_status() {
        assert_eq!(MCP_CRATE, "rustashop-mcp");
        assert_eq!(kernel_status(), rustashop::SERENADE_KERNEL_PENDING);
    }
}
