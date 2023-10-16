use std::{
    env,
    path::{Path, PathBuf},
};
fn main() {
    let mut cmake_conf = cmake::Config::new("htsengine");
    let include_dirs: Vec<PathBuf> = Vec::new();

    let debug = env::var("DEBUG").is_ok();
    if debug {
        cmake_conf.profile("Release");
    }

    let dst_dir = cmake_conf.build();
    let lib_dir = dst_dir.join("lib");
    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=hts_engine_API");
    generate_bindings(dst_dir.join("include"), include_dirs);
}

#[cfg(not(feature = "generate-bindings"))]
#[allow(unused_variables)]
fn generate_bindings(
    allow_dir: impl AsRef<Path>,
    include_dirs: impl IntoIterator<Item = impl AsRef<Path>>,
) {
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(
    allow_dir: impl AsRef<Path>,
    include_dirs: impl IntoIterator<Item = impl AsRef<Path>>,
) {
    let include_dir = allow_dir.as_ref();
    let clang_args = include_dirs
        .into_iter()
        .map(|dir| format!("-I{}", dir.as_ref().to_str().unwrap()))
        .chain([format!("-I{}", include_dir.to_str().unwrap())])
        .collect::<Vec<_>>();
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    let mut bind_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_recursively(true)
        .clang_args(clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .size_t_is_usize(true)
        .rustified_enum("*");
    let paths = std::fs::read_dir(include_dir).unwrap();
    for path in paths {
        let path = path.unwrap();
        let file_name = path.file_name().to_str().unwrap().to_string();
        bind_builder =
            bind_builder.allowlist_file(format!(".*{}", file_name.replace(".h", "\\.h")));
    }

    let bindings = bind_builder
        .generate()
        .expect("Unable to generate bindings");
    let generated_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("bindings.rs");
    println!("cargo:rerun-if-changed={:?}", generated_file);
    bindings
        .write_to_file(&generated_file)
        .expect("Couldn't write bindings!");
}
