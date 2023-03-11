extern crate cbindgen;
use std::{env, path::Path};

pub fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings!")
        .write_to_file(Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.h"));
}
