#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

#[cfg(not(feature = "some"))]
compile_error!("current feature set includes no parsers");

syntastica_macros::parsers_ffi!();
