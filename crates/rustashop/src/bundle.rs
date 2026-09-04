//! Product bundle registered on the Serenade application kernel.

use serenade_bundle::{BundleError, Extension, FRAMEWORK_BUNDLE};
use serenade_config::Config;
use serenade_di::ContainerBuilder;
use serenade_kernel::BundleInterface;

/// Canonical name for [`RustashopBundle`].
pub const RUSTASHOP_BUNDLE: &str = "rustashop";

/// Commerce application bundle (depends on [`FRAMEWORK_BUNDLE`]).
#[derive(Clone, Copy, Debug, Default)]
pub struct RustashopBundle;

impl BundleInterface for RustashopBundle {
    fn name(&self) -> &'static str {
        RUSTASHOP_BUNDLE
    }

    fn dependencies(&self) -> &'static [&'static str] {
        &[FRAMEWORK_BUNDLE]
    }
}

/// DI extension for the `rustashop` package key.
#[derive(Clone, Copy, Debug, Default)]
pub struct RustashopExtension;

impl Extension for RustashopExtension {
    fn alias(&self) -> &'static str {
        RUSTASHOP_BUNDLE
    }

    fn load(&self, config: &Config, builder: &mut ContainerBuilder) -> Result<(), BundleError> {
        config.apply_to(builder.parameters_mut());
        Ok(())
    }
}
