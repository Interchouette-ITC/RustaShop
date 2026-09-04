//! Disk gate for the self-contained `install/` app (PrestaShop-style).

use std::path::{Path, PathBuf};

/// Directory name that Actix mounts as `/install`.
pub const INSTALL_DIR_NAME: &str = "install";

/// Canonical renamed-away name after a successful install.
pub const INSTALL_OFF_DIR_NAME: &str = "install.off";

/// Env override for the shop root that contains `install/`.
pub const ROOT_ENV: &str = "RUSTASHOP_ROOT";

/// Resolves the shop root (env or current directory).
#[must_use]
pub fn shop_root() -> PathBuf {
    std::env::var_os(ROOT_ENV).map_or_else(
        || std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        PathBuf::from,
    )
}

/// Path to the live install tree: `{root}/install`.
#[must_use]
pub fn install_dir(root: &Path) -> PathBuf {
    root.join(INSTALL_DIR_NAME)
}

/// Built SPA entry that must exist for `/install` to mount.
#[must_use]
pub fn install_dist_index(root: &Path) -> PathBuf {
    install_dir(root).join("dist").join("index.html")
}

/// Whether install artefacts are present and should be served.
#[must_use]
pub fn install_artefacts_present(root: &Path) -> bool {
    install_dist_index(root).is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn absent_without_dist() {
        let dir = tempfile_dir();
        assert!(!install_artefacts_present(&dir));
    }

    #[test]
    fn present_with_index() {
        let dir = tempfile_dir();
        let index = install_dist_index(&dir);
        fs::create_dir_all(index.parent().expect("parent")).expect("mkdir");
        fs::write(&index, "<!doctype html>").expect("write");
        assert!(install_artefacts_present(&dir));
    }

    fn tempfile_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rustashop-install-fs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("tmpdir");
        dir
    }
}
