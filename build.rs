// build.rs

use std::path::{Path, PathBuf};
use std::process::Command;
use std::env::var;

fn main() {
    let dir = "./libwally-core";
    let configure = format!("{}/configure", &dir);
    if !Path::new(&configure).exists() {
        let status = Command::new("./tools/autogen.sh").current_dir(dir).status().unwrap();
        assert!(status.success());
    }

    let dst = autotools::Config::new("libwally-core")
        .enable("elements", None)
        .enable_static()
        .disable_shared()
        .with("pic", None)
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=wallycore");
    println!("cargo:rustc-link-lib=static=secp256k1");

    // generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header("libwally-core/include/wally_address.h")
        .header("libwally-core/include/wally_bip32.h")
        .header("libwally-core/include/wally_bip38.h")
        .header("libwally-core/include/wally_bip39.h")
        .header("libwally-core/include/wally_core.h")
        .header("libwally-core/include/wally_crypto.h")
        .header("libwally-core/include/wally_elements.h")
        .header("libwally-core/include/wally_psbt.h")
        .header("libwally-core/include/wally_script.h")
        .header("libwally-core/include/wally_symmetric.h")
        .header("libwally-core/include/wally_transaction.h")
        .size_t_is_usize(true)
        .blacklist_item("WALLY_OK")  // value redefined because interpreted as u32 instead of i32
        .clang_arg("-DBUILD_ELEMENTS")
        .rustfmt_bindings(true)
        .generate()
        .expect("unable to generate bindings");

    // setup the path to write bindings into
    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
