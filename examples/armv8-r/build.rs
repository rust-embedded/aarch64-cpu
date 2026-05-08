use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let script = "memory.ld";

    // put memory layout (linker script) in the linker search path
    fs::copy(script, out_dir.join(script))?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg=-T{script}");
    println!("cargo:rerun-if-changed={script}");

    // linker script provided by aarch64-rt
    println!("cargo:rustc-link-arg=-Timage.ld");
    // linker script provided by defmt
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    Ok(())
}
