use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let script = "memory.x";

    // put memory layout (linker script) in the linker search path
    fs::copy(script, out_dir.join(script))?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed={script}");

    // linker script provided by aarch64-pmsa-rt
    println!("cargo:rustc-link-arg=-Tlink.x");
    // linker script provided by defmt
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    Ok(())
}
