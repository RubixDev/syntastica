#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

#[cfg(not(feature = "some"))]
compile_error!("current feature set includes no parsers");

#[cfg(feature = "some")]
syntastica_macros::parsers_ffi!();
