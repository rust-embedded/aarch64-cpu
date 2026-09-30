//! Provides linker script to top-level binary crate

use std::{env, fs, path::Path};

const SCRIPT: &str = "link.x";

fn main() {
    copy_linker_script_into_linker_search_path();
}

fn copy_linker_script_into_linker_search_path() {
    let out_dir = env::var("OUT_DIR").expect("Cargo did not set OUT_DIR");
    let script = include_str!("src/link.x");
    fs::write(Path::new(&out_dir).join(SCRIPT), script).expect("could not write into $OUT_DIR");
    println!("cargo::rustc-link-search={out_dir}");
    // We do not pass the linker script to the linker (-Tlink.x) -
    // this allows users to supply their own linker script if they wish.
}
