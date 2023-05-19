use rustc_version::Channel;

fn main() {
    let channel = match rustc_version::version_meta().unwrap().channel {
        Channel::Dev => "CHANNEL_DEV",
        Channel::Nightly => "CHANNEL_NIGHTLY",
        Channel::Beta => "CHANNEL_BETA",
        Channel::Stable => "CHANNEL_STABLE",
    };
    println!("cargo:rustc-cfg={channel}");
    println!("cargo:rerun-if-changed=build.rs");
}
