use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("bindings.rs");

    let mut builder = bindgen::builder();
    builder = builder.header("src/stb_rect_pack.c");

    builder
        .allowlist_function("stb.*")
        .allowlist_type("stb.*")
        .allowlist_var("stb.*")
        .derive_default(true)
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(bindings_path)
        .expect("Failed to write bindings file");

    let mut builder = cc::Build::new();
    builder
        .files(&["src/stb_rect_pack.c"])
        .warnings(false)
        .compile("libstb");
}
