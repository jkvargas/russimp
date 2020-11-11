use std::{env::var, path::PathBuf};
use std::fs;

fn main() {
    const BINDINGS_FILE : &str = "bindings.rs";

    let assimp_path = cmake::Config::new("assimp")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    let path_buf_to_bindings_file = PathBuf::from(var("OUT_DIR").unwrap()).join(BINDINGS_FILE);
    let path_to_bindings_file = path_buf_to_bindings_file.as_os_str().to_str().unwrap();

    bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-I", assimp_path.join("include").to_str().unwrap()])
        .whitelist_function("aiImportFile")
        .generate()
        .unwrap()
        .write_to_file(path_to_bindings_file)
        .unwrap();

    fs::copy(path_to_bindings_file, "../../../../src/bindings.rs").expect(format!("Error while copying {} file", BINDINGS_FILE).as_str());
}