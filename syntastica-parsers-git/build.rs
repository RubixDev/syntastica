fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    println!("cargo::rerun-if-env-changed=SYNTASTICA_PARSERS_CLONE_DIR");
    println!("cargo::rerun-if-env-changed=SYNTASTICA_PARSERS_CACHE_DIR");
    if let Ok(dir) = env::var("SYNTASTICA_PARSERS_CLONE_DIR") {
        println!("cargo::rerun-if-changed={dir}")
    }
    if let Ok(dir) = env::var("SYNTASTICA_PARSERS_CACHE_DIR") {
        println!("cargo::rerun-if-changed={dir}")
    }

    if !(cfg!(feature = "docs") && env::var("DOCS_RS").is_ok()) {
        syntastica_macros::parsers_git!();
    }

    // for documenting features when using nightly
    #[cfg(feature = "docs")]
    {
        let channel = match rustc_version::version_meta().unwrap().channel {
            rustc_version::Channel::Dev => "CHANNEL_DEV",
            rustc_version::Channel::Nightly => "CHANNEL_NIGHTLY",
            rustc_version::Channel::Beta => "CHANNEL_BETA",
            rustc_version::Channel::Stable => "CHANNEL_STABLE",
        };
        println!("cargo::rustc-cfg={channel}");
    }
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}

include!("./build_helper.rs");
