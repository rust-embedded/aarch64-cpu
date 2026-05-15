//! Provides linker scripts to the examples

use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let package_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let script_name = "memory.x";
    let script_source = package_root.join("src").join(script_name);

    // put memory layout (linker script) in the linker search path
    fs::copy(&script_source, out_dir.join(script_name))?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed={}", script_source.display());

    // use aarch64-pmsa-rt linker script
    println!("cargo:rustc-link-arg=-Tlink.x");

    Ok(())
}
