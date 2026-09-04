use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_root = rustashop_template_shop_default::root();
    let template_id = rustashop_template_shop_default::id();
    std::fs::create_dir_all(manifest.join("generated")).expect("create generated");
    let css_out = manifest.join("generated/components.css");
    let out_dir = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR")).join("rangular");
    std::fs::create_dir_all(&out_dir).expect("create rangular OUT_DIR");

    println!("cargo:rerun-if-changed={}", template_root.display());

    let mut css = String::from("/* Generated from templates/shop/default - do not edit. */\n");
    append_scss(&mut css, "tokens", &template_root.join("tokens.scss"));
    append_scss(&mut css, "chrome", &template_root.join("chrome.scss"));

    css.push_str("\n@layer components {\n");
    let panels = [("product_card", "product_card_view")];
    for (dir, fn_name) in panels {
        compile_template_panel(template_id, &out_dir, &mut css, dir, fn_name);
    }
    css.push_str("}\n");

    std::fs::write(&css_out, css).unwrap_or_else(|err| {
        panic!("write {}: {err}", css_out.display());
    });
}

fn compile_template_panel(
    template_id: &str,
    out_dir: &Path,
    css: &mut String,
    dir: &str,
    fn_name: &str,
) {
    append_scss(
        css,
        dir,
        &rustashop_template_shop_default::component_file(dir, "scss"),
    );

    let html_path = rustashop_template_shop_default::component_file(dir, "html");
    let html = std::fs::read_to_string(&html_path).unwrap_or_else(|err| {
        panic!("read {}: {err}", html_path.display());
    });
    let source = format!("templates/shop/{template_id}/{dir}/{dir}.html");
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
