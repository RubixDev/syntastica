#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "docs",
    cfg_attr(doc, doc = ::document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#))
)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![warn(rust_2018_idioms, unreachable_pub)]
// #![deny(missing_docs)]

mod error;
pub mod provider;
pub mod style;
pub mod theme;

pub use error::*;
