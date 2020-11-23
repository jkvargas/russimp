use std::{env::var, path::PathBuf};

const BINDINGS_FILE : &str = "bindings.rs";
const WRAPPER_FILE : &str = "wrapper.h";

fn main() {
    let assimp_path = cmake::Config::new("assimp")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    let output_path = PathBuf::from(var("OUT_DIR").unwrap());
    let path_bindings_buf_src = output_path.join(format!("../../../../../russimp-sys/src/{}", BINDINGS_FILE));
    let path_bindings_file_src = path_bindings_buf_src.as_os_str().to_str().unwrap();
    let assimp_compiled_lib_path = output_path.join("lib");
    let assimp_compiled_include_path = output_path.join("include");

    println!("cargo:rustc-link-search={}", assimp_compiled_lib_path.display());
    println!("cargo:include={}", assimp_compiled_include_path.display());

    bindgen::Builder::default()
        .header(WRAPPER_FILE)
        .clang_args(&["-I", assimp_path.join("include").to_str().unwrap()])
        .whitelist_function("aiImportFile")
        .whitelist_type("aiPostProcessSteps")
        .whitelist_type("aiPrimitiveType")
        .whitelist_type("aiTextureType")
        .whitelist_function("aiReleaseImport")
        .whitelist_function("aiGetErrorString")
        .generate()
        .unwrap()
        .write_to_file(path_bindings_file_src)
        .unwrap();

    println!("cargo:rustc-flags=-l assimp");
}