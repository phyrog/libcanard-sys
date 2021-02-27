extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    cc::Build::new()
        .file("vendor/libcanard/canard.c")
        .file("vendor/libcanard/canard_dsdl.c")
        .compile("canard");

    println!("cargo:rustc-link-lib=static=canard");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .ctypes_prefix("cty")
        .use_core()
        .header("wrapper.h")
        .blacklist_item("CANARD_MTU_CAN_CLASSIC")
        .blacklist_item("CANARD_MTU_CAN_FD")
        .blacklist_item("CANARD_NODE_ID_MAX")
        .blacklist_item("CANARD_NODE_ID_UNSET")
        .blacklist_item("CANARD_PRIORITY_MAX")
        .blacklist_item("CANARD_TRANSFER_ID_MAX")
        .rustified_enum("CanardPriority")
        .rustified_enum("CanardTransferKind")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
