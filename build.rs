extern crate cc;

use std::env;

fn main() {
  println!("cargo:rerun-if-changed=c-ffi/ffi.c");
  println!("cargo:rerun-if-changed=c-ffi/ffi.h");

  let mut build = cc::Build::new();

  build
    .file("c-ffi/ffi.c")
    .include("c-ffi")
    .cargo_metadata(true)
    .out_dir(env::var("OUT_DIR").unwrap())
    .compile("ffi");
}
