extern crate bindgen;

use std::env;
use std::path::PathBuf;

const C_LIB_PATHS: [&str; 2] = ["CoinUtils/CoinUtils/src", "Cbc/Cbc/src"];

fn main() {
    println!("cargo:rerun-if-env-changed=COIN_LIB_DIR");
    let coin_lib_dir = env::var("COIN_LIB_DIR")
        .expect("Missing the COIN_LIB_DIR to specify the COIN-OR library path");

    println!("cargo:rustc-link-search={}/build/lib", coin_lib_dir);
    println!("cargo:rustc-link-lib=CbcSolver");

    let builder = bindgen::Builder::default().header("wrapper.h").clang_args(
        C_LIB_PATHS
            .iter()
            .map(|lib| format!("-I/{}/{}", coin_lib_dir, lib)),
    );

    let bindings = builder
        .generate()
        .map_err(|e| {
            println!("{:?}", e);
            panic!()
        })
        .unwrap();

    let mut out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    out_path.push("src/ffi.rs");
    bindings.write_to_file(&out_path).expect(&format!(
        "Unable to write bindings to {}",
        out_path.display()
    ));
}
