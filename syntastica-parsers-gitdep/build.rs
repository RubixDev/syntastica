#[cfg(not(feature = "docs"))]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(feature = "docs")]
fn main() {
    // for documenting features when using nightly
    let channel = match rustc_version::version_meta().unwrap().channel {
        rustc_version::Channel::Dev => "CHANNEL_DEV",
        rustc_version::Channel::Nightly => "CHANNEL_NIGHTLY",
        rustc_version::Channel::Beta => "CHANNEL_BETA",
        rustc_version::Channel::Stable => "CHANNEL_STABLE",
    };
    println!("cargo:rustc-cfg={channel}");
    println!("cargo:rerun-if-changed=build.rs");
}
