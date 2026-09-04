use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let theme = std::env::var("RUSTASHOP_THEME").unwrap_or_else(|_| "default".to_owned());
    let theme_root = manifest.join("../../templates").join(&theme);
    let css_out = manifest.join("style/components.generated.css");
    let out_dir = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("rangular");
    std::fs::create_dir_all(&out_dir).expect("create rangular OUT_DIR");

    println!("cargo:rerun-if-changed=../../templates/{theme}");
    println!("cargo:rerun-if-env-changed=RUSTASHOP_THEME");

    // (theme component dir name, generated view fn name)
    let panels = [("product_card", "product_card_view")];

    let mut css =
        String::from("/* Generated from theme SCSS - do not edit. */\n@layer components {\n");
    for (dir, fn_name) in panels {
        compile_theme_panel(&theme_root, &theme, &out_dir, &mut css, dir, fn_name);
    }
    css.push_str("}\n");

    std::fs::write(&css_out, css).unwrap_or_else(|err| {
        panic!("write {}: {err}", css_out.display());
    });
}

fn compile_theme_panel(
    theme_root: &Path,
    theme: &str,
    out_dir: &Path,
    css: &mut String,
    dir: &str,
    fn_name: &str,
) {
    let panel_dir = theme_root.join(dir);
    append_scss(css, dir, &panel_dir.join(format!("{dir}.scss")));

    let html_path = panel_dir.join(format!("{dir}.html"));
    let html = std::fs::read_to_string(&html_path).unwrap_or_else(|err| {
        panic!("read {}: {err}", html_path.display());
    });
    let source = format!("templates/{theme}/{dir}/{dir}.html");
    let aot = rangular_aot::compile_named(&html, &source, fn_name);
    assert!(aot.ok(), "{dir}.html: {:?}", aot.issues);
    let rs_path = out_dir.join(format!("{fn_name}.rs"));
    std::fs::write(&rs_path, &aot.code).unwrap_or_else(|err| {
        panic!("write {}: {err}", rs_path.display());
    });
}

fn append_scss(css: &mut String, label: &str, scss_path: &Path) {
    let scss = std::fs::read_to_string(scss_path).unwrap_or_else(|err| {
        panic!("read {}: {err}", scss_path.display());
    });
    let result = rangular_css::compile_scss(&scss);
    assert!(result.ok(), "{label}.scss: {:?}", result.issues);
    let _ = write!(css, "\n/* --- {label} --- */\n");
    css.push_str(&result.css);
    css.push('\n');
}
