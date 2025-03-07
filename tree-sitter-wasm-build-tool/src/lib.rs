#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms)]
#![deny(missing_docs)]

use std::{env, error::Error, fs, path::PathBuf};

macro_rules! copy_file {
    ($sysroot:ident, $name:literal) => {
        fs::write(
            $sysroot.join($name),
            include_bytes!(concat!("../wasm-sysroot/", $name)),
        )?;
    };
}

/// Add the required C header files to the given instance of [`cc::Build`], to allow compilation of
/// tree-sitter parsers to the `wasm32-unknown-unknown` target.
pub fn add_wasm_headers(config: &mut cc::Build) -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let sysroot_dir = out_dir.join("sysroot");
    if env::var("TARGET")? == "wasm32-unknown-unknown" {
        fs::create_dir_all(&sysroot_dir)?;

        copy_file!(sysroot_dir, "stdint.h");
        copy_file!(sysroot_dir, "stdlib.h");
        copy_file!(sysroot_dir, "stdio.h");
        copy_file!(sysroot_dir, "stdbool.h");
        copy_file!(sysroot_dir, "wctype.h");

        config.include(&sysroot_dir);

        copy_file!(out_dir, "wctype.c");
        config.file(out_dir.join("wctype.c"));
    }

    Ok(())
}
