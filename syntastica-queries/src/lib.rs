#![doc = include_str!("../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub const ADA_HIGHLIGHTS: &str = include_str!("../generated_queries/ada/highlights.scm");
pub const ADA_INJECTIONS: &str = include_str!("../generated_queries/ada/injections.scm");
pub const ADA_LOCALS: &str = include_str!("../generated_queries/ada/locals.scm");
pub const ADA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ada/highlights_crates_io.scm");
pub const ADA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ada/injections_crates_io.scm");
pub const ADA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ada/locals_crates_io.scm");

pub const AGDA_HIGHLIGHTS: &str = include_str!("../generated_queries/agda/highlights.scm");
pub const AGDA_INJECTIONS: &str = include_str!("../generated_queries/agda/injections.scm");
pub const AGDA_LOCALS: &str = include_str!("../generated_queries/agda/locals.scm");
pub const AGDA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/agda/highlights_crates_io.scm");
pub const AGDA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/agda/injections_crates_io.scm");
pub const AGDA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/agda/locals_crates_io.scm");

pub const APEX_HIGHLIGHTS: &str = include_str!("../generated_queries/apex/highlights.scm");
pub const APEX_INJECTIONS: &str = include_str!("../generated_queries/apex/injections.scm");
pub const APEX_LOCALS: &str = include_str!("../generated_queries/apex/locals.scm");
pub const APEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/apex/highlights_crates_io.scm");
pub const APEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/apex/injections_crates_io.scm");
pub const APEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/apex/locals_crates_io.scm");

pub const ARDUINO_HIGHLIGHTS: &str = include_str!("../generated_queries/arduino/highlights.scm");
pub const ARDUINO_INJECTIONS: &str = include_str!("../generated_queries/arduino/injections.scm");
pub const ARDUINO_LOCALS: &str = include_str!("../generated_queries/arduino/locals.scm");
pub const ARDUINO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/arduino/highlights_crates_io.scm");
pub const ARDUINO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/arduino/injections_crates_io.scm");
pub const ARDUINO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/arduino/locals_crates_io.scm");

pub const ASM_HIGHLIGHTS: &str = include_str!("../generated_queries/asm/highlights.scm");
pub const ASM_INJECTIONS: &str = include_str!("../generated_queries/asm/injections.scm");
pub const ASM_LOCALS: &str = include_str!("../generated_queries/asm/locals.scm");
pub const ASM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/asm/highlights_crates_io.scm");
pub const ASM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/asm/injections_crates_io.scm");
pub const ASM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/asm/locals_crates_io.scm");

pub const ASM68K_HIGHLIGHTS: &str = include_str!("../generated_queries/asm68k/highlights.scm");
pub const ASM68K_INJECTIONS: &str = include_str!("../generated_queries/asm68k/injections.scm");
pub const ASM68K_LOCALS: &str = include_str!("../generated_queries/asm68k/locals.scm");
pub const ASM68K_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/asm68k/highlights_crates_io.scm");
pub const ASM68K_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/asm68k/injections_crates_io.scm");
pub const ASM68K_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/asm68k/locals_crates_io.scm");

pub const ASTRO_HIGHLIGHTS: &str = include_str!("../generated_queries/astro/highlights.scm");
pub const ASTRO_INJECTIONS: &str = include_str!("../generated_queries/astro/injections.scm");
pub const ASTRO_LOCALS: &str = include_str!("../generated_queries/astro/locals.scm");
pub const ASTRO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/astro/highlights_crates_io.scm");
pub const ASTRO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/astro/injections_crates_io.scm");
pub const ASTRO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/astro/locals_crates_io.scm");

pub const AUTHZED_HIGHLIGHTS: &str = include_str!("../generated_queries/authzed/highlights.scm");
pub const AUTHZED_INJECTIONS: &str = include_str!("../generated_queries/authzed/injections.scm");
pub const AUTHZED_LOCALS: &str = include_str!("../generated_queries/authzed/locals.scm");
pub const AUTHZED_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/authzed/highlights_crates_io.scm");
pub const AUTHZED_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/authzed/injections_crates_io.scm");
pub const AUTHZED_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/authzed/locals_crates_io.scm");

pub const AWK_HIGHLIGHTS: &str = include_str!("../generated_queries/awk/highlights.scm");
pub const AWK_INJECTIONS: &str = include_str!("../generated_queries/awk/injections.scm");
pub const AWK_LOCALS: &str = include_str!("../generated_queries/awk/locals.scm");
pub const AWK_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/awk/highlights_crates_io.scm");
pub const AWK_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/awk/injections_crates_io.scm");
pub const AWK_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/awk/locals_crates_io.scm");

pub const BASH_HIGHLIGHTS: &str = include_str!("../generated_queries/bash/highlights.scm");
pub const BASH_INJECTIONS: &str = include_str!("../generated_queries/bash/injections.scm");
pub const BASH_LOCALS: &str = include_str!("../generated_queries/bash/locals.scm");
pub const BASH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bash/highlights_crates_io.scm");
pub const BASH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bash/injections_crates_io.scm");
pub const BASH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bash/locals_crates_io.scm");

pub const BASS_HIGHLIGHTS: &str = include_str!("../generated_queries/bass/highlights.scm");
pub const BASS_INJECTIONS: &str = include_str!("../generated_queries/bass/injections.scm");
pub const BASS_LOCALS: &str = include_str!("../generated_queries/bass/locals.scm");
pub const BASS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bass/highlights_crates_io.scm");
pub const BASS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bass/injections_crates_io.scm");
pub const BASS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bass/locals_crates_io.scm");

pub const BEANCOUNT_HIGHLIGHTS: &str = include_str!("../generated_queries/beancount/highlights.scm");
pub const BEANCOUNT_INJECTIONS: &str = include_str!("../generated_queries/beancount/injections.scm");
pub const BEANCOUNT_LOCALS: &str = include_str!("../generated_queries/beancount/locals.scm");
pub const BEANCOUNT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/beancount/highlights_crates_io.scm");
pub const BEANCOUNT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/beancount/injections_crates_io.scm");
pub const BEANCOUNT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/beancount/locals_crates_io.scm");

pub const BIB_HIGHLIGHTS: &str = include_str!("../generated_queries/bib/highlights.scm");
pub const BIB_INJECTIONS: &str = include_str!("../generated_queries/bib/injections.scm");
pub const BIB_LOCALS: &str = include_str!("../generated_queries/bib/locals.scm");
pub const BIB_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bib/highlights_crates_io.scm");
pub const BIB_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bib/injections_crates_io.scm");
pub const BIB_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bib/locals_crates_io.scm");

pub const BIBTEX_HIGHLIGHTS: &str = include_str!("../generated_queries/bibtex/highlights.scm");
pub const BIBTEX_INJECTIONS: &str = include_str!("../generated_queries/bibtex/injections.scm");
pub const BIBTEX_LOCALS: &str = include_str!("../generated_queries/bibtex/locals.scm");
pub const BIBTEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bibtex/highlights_crates_io.scm");
pub const BIBTEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bibtex/injections_crates_io.scm");
pub const BIBTEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bibtex/locals_crates_io.scm");

pub const BICEP_HIGHLIGHTS: &str = include_str!("../generated_queries/bicep/highlights.scm");
pub const BICEP_INJECTIONS: &str = include_str!("../generated_queries/bicep/injections.scm");
pub const BICEP_LOCALS: &str = include_str!("../generated_queries/bicep/locals.scm");
pub const BICEP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bicep/highlights_crates_io.scm");
pub const BICEP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bicep/injections_crates_io.scm");
pub const BICEP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bicep/locals_crates_io.scm");

pub const BITBAKE_HIGHLIGHTS: &str = include_str!("../generated_queries/bitbake/highlights.scm");
pub const BITBAKE_INJECTIONS: &str = include_str!("../generated_queries/bitbake/injections.scm");
pub const BITBAKE_LOCALS: &str = include_str!("../generated_queries/bitbake/locals.scm");
pub const BITBAKE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bitbake/highlights_crates_io.scm");
pub const BITBAKE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bitbake/injections_crates_io.scm");
pub const BITBAKE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bitbake/locals_crates_io.scm");

pub const BLADE_HIGHLIGHTS: &str = include_str!("../generated_queries/blade/highlights.scm");
pub const BLADE_INJECTIONS: &str = include_str!("../generated_queries/blade/injections.scm");
pub const BLADE_LOCALS: &str = include_str!("../generated_queries/blade/locals.scm");
pub const BLADE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/blade/highlights_crates_io.scm");
pub const BLADE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/blade/injections_crates_io.scm");
pub const BLADE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/blade/locals_crates_io.scm");

pub const BLUEPRINT_HIGHLIGHTS: &str = include_str!("../generated_queries/blueprint/highlights.scm");
pub const BLUEPRINT_INJECTIONS: &str = include_str!("../generated_queries/blueprint/injections.scm");
pub const BLUEPRINT_LOCALS: &str = include_str!("../generated_queries/blueprint/locals.scm");
pub const BLUEPRINT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/blueprint/highlights_crates_io.scm");
pub const BLUEPRINT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/blueprint/injections_crates_io.scm");
pub const BLUEPRINT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/blueprint/locals_crates_io.scm");

pub const BP_HIGHLIGHTS: &str = include_str!("../generated_queries/bp/highlights.scm");
pub const BP_INJECTIONS: &str = include_str!("../generated_queries/bp/injections.scm");
pub const BP_LOCALS: &str = include_str!("../generated_queries/bp/locals.scm");
pub const BP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bp/highlights_crates_io.scm");
pub const BP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bp/injections_crates_io.scm");
pub const BP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bp/locals_crates_io.scm");

pub const BPFTRACE_HIGHLIGHTS: &str = include_str!("../generated_queries/bpftrace/highlights.scm");
pub const BPFTRACE_INJECTIONS: &str = include_str!("../generated_queries/bpftrace/injections.scm");
pub const BPFTRACE_LOCALS: &str = include_str!("../generated_queries/bpftrace/locals.scm");
pub const BPFTRACE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bpftrace/highlights_crates_io.scm");
pub const BPFTRACE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bpftrace/injections_crates_io.scm");
pub const BPFTRACE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bpftrace/locals_crates_io.scm");

pub const BRIGHTSCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/brightscript/highlights.scm");
pub const BRIGHTSCRIPT_INJECTIONS: &str = include_str!("../generated_queries/brightscript/injections.scm");
pub const BRIGHTSCRIPT_LOCALS: &str = include_str!("../generated_queries/brightscript/locals.scm");
pub const BRIGHTSCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/brightscript/highlights_crates_io.scm");
pub const BRIGHTSCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/brightscript/injections_crates_io.scm");
pub const BRIGHTSCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/brightscript/locals_crates_io.scm");

pub const BZL_HIGHLIGHTS: &str = include_str!("../generated_queries/bzl/highlights.scm");
pub const BZL_INJECTIONS: &str = include_str!("../generated_queries/bzl/injections.scm");
pub const BZL_LOCALS: &str = include_str!("../generated_queries/bzl/locals.scm");
pub const BZL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/bzl/highlights_crates_io.scm");
pub const BZL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/bzl/injections_crates_io.scm");
pub const BZL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/bzl/locals_crates_io.scm");

pub const C_HIGHLIGHTS: &str = include_str!("../generated_queries/c/highlights.scm");
pub const C_INJECTIONS: &str = include_str!("../generated_queries/c/injections.scm");
pub const C_LOCALS: &str = include_str!("../generated_queries/c/locals.scm");
pub const C_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/c/highlights_crates_io.scm");
pub const C_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/c/injections_crates_io.scm");
pub const C_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/c/locals_crates_io.scm");

pub const C3_HIGHLIGHTS: &str = include_str!("../generated_queries/c3/highlights.scm");
pub const C3_INJECTIONS: &str = include_str!("../generated_queries/c3/injections.scm");
pub const C3_LOCALS: &str = include_str!("../generated_queries/c3/locals.scm");
pub const C3_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/c3/highlights_crates_io.scm");
pub const C3_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/c3/injections_crates_io.scm");
pub const C3_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/c3/locals_crates_io.scm");

pub const C_SHARP_HIGHLIGHTS: &str = include_str!("../generated_queries/c_sharp/highlights.scm");
pub const C_SHARP_INJECTIONS: &str = include_str!("../generated_queries/c_sharp/injections.scm");
pub const C_SHARP_LOCALS: &str = include_str!("../generated_queries/c_sharp/locals.scm");
pub const C_SHARP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/c_sharp/highlights_crates_io.scm");
pub const C_SHARP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/c_sharp/injections_crates_io.scm");
pub const C_SHARP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/c_sharp/locals_crates_io.scm");

pub const CADDY_HIGHLIGHTS: &str = include_str!("../generated_queries/caddy/highlights.scm");
pub const CADDY_INJECTIONS: &str = include_str!("../generated_queries/caddy/injections.scm");
pub const CADDY_LOCALS: &str = include_str!("../generated_queries/caddy/locals.scm");
pub const CADDY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/caddy/highlights_crates_io.scm");
pub const CADDY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/caddy/injections_crates_io.scm");
pub const CADDY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/caddy/locals_crates_io.scm");

pub const CAIRO_HIGHLIGHTS: &str = include_str!("../generated_queries/cairo/highlights.scm");
pub const CAIRO_INJECTIONS: &str = include_str!("../generated_queries/cairo/injections.scm");
pub const CAIRO_LOCALS: &str = include_str!("../generated_queries/cairo/locals.scm");
pub const CAIRO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cairo/highlights_crates_io.scm");
pub const CAIRO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cairo/injections_crates_io.scm");
pub const CAIRO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cairo/locals_crates_io.scm");

pub const CAPNP_HIGHLIGHTS: &str = include_str!("../generated_queries/capnp/highlights.scm");
pub const CAPNP_INJECTIONS: &str = include_str!("../generated_queries/capnp/injections.scm");
pub const CAPNP_LOCALS: &str = include_str!("../generated_queries/capnp/locals.scm");
pub const CAPNP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/capnp/highlights_crates_io.scm");
pub const CAPNP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/capnp/injections_crates_io.scm");
pub const CAPNP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/capnp/locals_crates_io.scm");

pub const CHATITO_HIGHLIGHTS: &str = include_str!("../generated_queries/chatito/highlights.scm");
pub const CHATITO_INJECTIONS: &str = include_str!("../generated_queries/chatito/injections.scm");
pub const CHATITO_LOCALS: &str = include_str!("../generated_queries/chatito/locals.scm");
pub const CHATITO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/chatito/highlights_crates_io.scm");
pub const CHATITO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/chatito/injections_crates_io.scm");
pub const CHATITO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/chatito/locals_crates_io.scm");

pub const CIRCOM_HIGHLIGHTS: &str = include_str!("../generated_queries/circom/highlights.scm");
pub const CIRCOM_INJECTIONS: &str = include_str!("../generated_queries/circom/injections.scm");
pub const CIRCOM_LOCALS: &str = include_str!("../generated_queries/circom/locals.scm");
pub const CIRCOM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/circom/highlights_crates_io.scm");
pub const CIRCOM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/circom/injections_crates_io.scm");
pub const CIRCOM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/circom/locals_crates_io.scm");

pub const CLARITY_HIGHLIGHTS: &str = include_str!("../generated_queries/clarity/highlights.scm");
pub const CLARITY_INJECTIONS: &str = include_str!("../generated_queries/clarity/injections.scm");
pub const CLARITY_LOCALS: &str = include_str!("../generated_queries/clarity/locals.scm");
pub const CLARITY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/clarity/highlights_crates_io.scm");
pub const CLARITY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/clarity/injections_crates_io.scm");
pub const CLARITY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/clarity/locals_crates_io.scm");

pub const CLOJURE_HIGHLIGHTS: &str = include_str!("../generated_queries/clojure/highlights.scm");
pub const CLOJURE_INJECTIONS: &str = include_str!("../generated_queries/clojure/injections.scm");
pub const CLOJURE_LOCALS: &str = include_str!("../generated_queries/clojure/locals.scm");
pub const CLOJURE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/clojure/highlights_crates_io.scm");
pub const CLOJURE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/clojure/injections_crates_io.scm");
pub const CLOJURE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/clojure/locals_crates_io.scm");

pub const CMAKE_HIGHLIGHTS: &str = include_str!("../generated_queries/cmake/highlights.scm");
pub const CMAKE_INJECTIONS: &str = include_str!("../generated_queries/cmake/injections.scm");
pub const CMAKE_LOCALS: &str = include_str!("../generated_queries/cmake/locals.scm");
pub const CMAKE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cmake/highlights_crates_io.scm");
pub const CMAKE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cmake/injections_crates_io.scm");
pub const CMAKE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cmake/locals_crates_io.scm");

pub const COMMENT_HIGHLIGHTS: &str = include_str!("../generated_queries/comment/highlights.scm");
pub const COMMENT_INJECTIONS: &str = include_str!("../generated_queries/comment/injections.scm");
pub const COMMENT_LOCALS: &str = include_str!("../generated_queries/comment/locals.scm");
pub const COMMENT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/comment/highlights_crates_io.scm");
pub const COMMENT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/comment/injections_crates_io.scm");
pub const COMMENT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/comment/locals_crates_io.scm");

pub const COOK_HIGHLIGHTS: &str = include_str!("../generated_queries/cook/highlights.scm");
pub const COOK_INJECTIONS: &str = include_str!("../generated_queries/cook/injections.scm");
pub const COOK_LOCALS: &str = include_str!("../generated_queries/cook/locals.scm");
pub const COOK_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cook/highlights_crates_io.scm");
pub const COOK_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cook/injections_crates_io.scm");
pub const COOK_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cook/locals_crates_io.scm");

pub const CORN_HIGHLIGHTS: &str = include_str!("../generated_queries/corn/highlights.scm");
pub const CORN_INJECTIONS: &str = include_str!("../generated_queries/corn/injections.scm");
pub const CORN_LOCALS: &str = include_str!("../generated_queries/corn/locals.scm");
pub const CORN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/corn/highlights_crates_io.scm");
pub const CORN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/corn/injections_crates_io.scm");
pub const CORN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/corn/locals_crates_io.scm");

pub const CPON_HIGHLIGHTS: &str = include_str!("../generated_queries/cpon/highlights.scm");
pub const CPON_INJECTIONS: &str = include_str!("../generated_queries/cpon/injections.scm");
pub const CPON_LOCALS: &str = include_str!("../generated_queries/cpon/locals.scm");
pub const CPON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cpon/highlights_crates_io.scm");
pub const CPON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cpon/injections_crates_io.scm");
pub const CPON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cpon/locals_crates_io.scm");

pub const CPP_HIGHLIGHTS: &str = include_str!("../generated_queries/cpp/highlights.scm");
pub const CPP_INJECTIONS: &str = include_str!("../generated_queries/cpp/injections.scm");
pub const CPP_LOCALS: &str = include_str!("../generated_queries/cpp/locals.scm");
pub const CPP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cpp/highlights_crates_io.scm");
pub const CPP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cpp/injections_crates_io.scm");
pub const CPP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cpp/locals_crates_io.scm");

pub const CSHARP_HIGHLIGHTS: &str = include_str!("../generated_queries/csharp/highlights.scm");
pub const CSHARP_INJECTIONS: &str = include_str!("../generated_queries/csharp/injections.scm");
pub const CSHARP_LOCALS: &str = include_str!("../generated_queries/csharp/locals.scm");
pub const CSHARP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/csharp/highlights_crates_io.scm");
pub const CSHARP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/csharp/injections_crates_io.scm");
pub const CSHARP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/csharp/locals_crates_io.scm");

pub const CSS_HIGHLIGHTS: &str = include_str!("../generated_queries/css/highlights.scm");
pub const CSS_INJECTIONS: &str = include_str!("../generated_queries/css/injections.scm");
pub const CSS_LOCALS: &str = include_str!("../generated_queries/css/locals.scm");
pub const CSS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/css/highlights_crates_io.scm");
pub const CSS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/css/injections_crates_io.scm");
pub const CSS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/css/locals_crates_io.scm");

pub const CSV_HIGHLIGHTS: &str = include_str!("../generated_queries/csv/highlights.scm");
pub const CSV_INJECTIONS: &str = include_str!("../generated_queries/csv/injections.scm");
pub const CSV_LOCALS: &str = include_str!("../generated_queries/csv/locals.scm");
pub const CSV_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/csv/highlights_crates_io.scm");
pub const CSV_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/csv/injections_crates_io.scm");
pub const CSV_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/csv/locals_crates_io.scm");

pub const CUDA_HIGHLIGHTS: &str = include_str!("../generated_queries/cuda/highlights.scm");
pub const CUDA_INJECTIONS: &str = include_str!("../generated_queries/cuda/injections.scm");
pub const CUDA_LOCALS: &str = include_str!("../generated_queries/cuda/locals.scm");
pub const CUDA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cuda/highlights_crates_io.scm");
pub const CUDA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cuda/injections_crates_io.scm");
pub const CUDA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cuda/locals_crates_io.scm");

pub const CUE_HIGHLIGHTS: &str = include_str!("../generated_queries/cue/highlights.scm");
pub const CUE_INJECTIONS: &str = include_str!("../generated_queries/cue/injections.scm");
pub const CUE_LOCALS: &str = include_str!("../generated_queries/cue/locals.scm");
pub const CUE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cue/highlights_crates_io.scm");
pub const CUE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cue/injections_crates_io.scm");
pub const CUE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cue/locals_crates_io.scm");

pub const CYLC_HIGHLIGHTS: &str = include_str!("../generated_queries/cylc/highlights.scm");
pub const CYLC_INJECTIONS: &str = include_str!("../generated_queries/cylc/injections.scm");
pub const CYLC_LOCALS: &str = include_str!("../generated_queries/cylc/locals.scm");
pub const CYLC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/cylc/highlights_crates_io.scm");
pub const CYLC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/cylc/injections_crates_io.scm");
pub const CYLC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/cylc/locals_crates_io.scm");

pub const D_HIGHLIGHTS: &str = include_str!("../generated_queries/d/highlights.scm");
pub const D_INJECTIONS: &str = include_str!("../generated_queries/d/injections.scm");
pub const D_LOCALS: &str = include_str!("../generated_queries/d/locals.scm");
pub const D_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/d/highlights_crates_io.scm");
pub const D_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/d/injections_crates_io.scm");
pub const D_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/d/locals_crates_io.scm");

pub const DART_HIGHLIGHTS: &str = include_str!("../generated_queries/dart/highlights.scm");
pub const DART_INJECTIONS: &str = include_str!("../generated_queries/dart/injections.scm");
pub const DART_LOCALS: &str = include_str!("../generated_queries/dart/locals.scm");
pub const DART_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/dart/highlights_crates_io.scm");
pub const DART_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/dart/injections_crates_io.scm");
pub const DART_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/dart/locals_crates_io.scm");

pub const DESKTOP_HIGHLIGHTS: &str = include_str!("../generated_queries/desktop/highlights.scm");
pub const DESKTOP_INJECTIONS: &str = include_str!("../generated_queries/desktop/injections.scm");
pub const DESKTOP_LOCALS: &str = include_str!("../generated_queries/desktop/locals.scm");
pub const DESKTOP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/desktop/highlights_crates_io.scm");
pub const DESKTOP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/desktop/injections_crates_io.scm");
pub const DESKTOP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/desktop/locals_crates_io.scm");

pub const DHALL_HIGHLIGHTS: &str = include_str!("../generated_queries/dhall/highlights.scm");
pub const DHALL_INJECTIONS: &str = include_str!("../generated_queries/dhall/injections.scm");
pub const DHALL_LOCALS: &str = include_str!("../generated_queries/dhall/locals.scm");
pub const DHALL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/dhall/highlights_crates_io.scm");
pub const DHALL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/dhall/injections_crates_io.scm");
pub const DHALL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/dhall/locals_crates_io.scm");

pub const DIFF_HIGHLIGHTS: &str = include_str!("../generated_queries/diff/highlights.scm");
pub const DIFF_INJECTIONS: &str = include_str!("../generated_queries/diff/injections.scm");
pub const DIFF_LOCALS: &str = include_str!("../generated_queries/diff/locals.scm");
pub const DIFF_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/diff/highlights_crates_io.scm");
pub const DIFF_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/diff/injections_crates_io.scm");
pub const DIFF_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/diff/locals_crates_io.scm");

pub const DISASSEMBLY_HIGHLIGHTS: &str = include_str!("../generated_queries/disassembly/highlights.scm");
pub const DISASSEMBLY_INJECTIONS: &str = include_str!("../generated_queries/disassembly/injections.scm");
pub const DISASSEMBLY_LOCALS: &str = include_str!("../generated_queries/disassembly/locals.scm");
pub const DISASSEMBLY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/disassembly/highlights_crates_io.scm");
pub const DISASSEMBLY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/disassembly/injections_crates_io.scm");
pub const DISASSEMBLY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/disassembly/locals_crates_io.scm");

pub const DJOT_HIGHLIGHTS: &str = include_str!("../generated_queries/djot/highlights.scm");
pub const DJOT_INJECTIONS: &str = include_str!("../generated_queries/djot/injections.scm");
pub const DJOT_LOCALS: &str = include_str!("../generated_queries/djot/locals.scm");
pub const DJOT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/djot/highlights_crates_io.scm");
pub const DJOT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/djot/injections_crates_io.scm");
pub const DJOT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/djot/locals_crates_io.scm");

pub const DOCKERFILE_HIGHLIGHTS: &str = include_str!("../generated_queries/dockerfile/highlights.scm");
pub const DOCKERFILE_INJECTIONS: &str = include_str!("../generated_queries/dockerfile/injections.scm");
pub const DOCKERFILE_LOCALS: &str = include_str!("../generated_queries/dockerfile/locals.scm");
pub const DOCKERFILE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/dockerfile/highlights_crates_io.scm");
pub const DOCKERFILE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/dockerfile/injections_crates_io.scm");
pub const DOCKERFILE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/dockerfile/locals_crates_io.scm");

pub const DOT_HIGHLIGHTS: &str = include_str!("../generated_queries/dot/highlights.scm");
pub const DOT_INJECTIONS: &str = include_str!("../generated_queries/dot/injections.scm");
pub const DOT_LOCALS: &str = include_str!("../generated_queries/dot/locals.scm");
pub const DOT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/dot/highlights_crates_io.scm");
pub const DOT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/dot/injections_crates_io.scm");
pub const DOT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/dot/locals_crates_io.scm");

pub const DOXYGEN_HIGHLIGHTS: &str = include_str!("../generated_queries/doxygen/highlights.scm");
pub const DOXYGEN_INJECTIONS: &str = include_str!("../generated_queries/doxygen/injections.scm");
pub const DOXYGEN_LOCALS: &str = include_str!("../generated_queries/doxygen/locals.scm");
pub const DOXYGEN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/doxygen/highlights_crates_io.scm");
pub const DOXYGEN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/doxygen/injections_crates_io.scm");
pub const DOXYGEN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/doxygen/locals_crates_io.scm");

pub const DTD_HIGHLIGHTS: &str = include_str!("../generated_queries/dtd/highlights.scm");
pub const DTD_INJECTIONS: &str = include_str!("../generated_queries/dtd/injections.scm");
pub const DTD_LOCALS: &str = include_str!("../generated_queries/dtd/locals.scm");
pub const DTD_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/dtd/highlights_crates_io.scm");
pub const DTD_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/dtd/injections_crates_io.scm");
pub const DTD_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/dtd/locals_crates_io.scm");

pub const DTS_HIGHLIGHTS: &str = include_str!("../generated_queries/dts/highlights.scm");
pub const DTS_INJECTIONS: &str = include_str!("../generated_queries/dts/injections.scm");
pub const DTS_LOCALS: &str = include_str!("../generated_queries/dts/locals.scm");
pub const DTS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/dts/highlights_crates_io.scm");
pub const DTS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/dts/injections_crates_io.scm");
pub const DTS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/dts/locals_crates_io.scm");

pub const EARTHFILE_HIGHLIGHTS: &str = include_str!("../generated_queries/earthfile/highlights.scm");
pub const EARTHFILE_INJECTIONS: &str = include_str!("../generated_queries/earthfile/injections.scm");
pub const EARTHFILE_LOCALS: &str = include_str!("../generated_queries/earthfile/locals.scm");
pub const EARTHFILE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/earthfile/highlights_crates_io.scm");
pub const EARTHFILE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/earthfile/injections_crates_io.scm");
pub const EARTHFILE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/earthfile/locals_crates_io.scm");

pub const EBNF_HIGHLIGHTS: &str = include_str!("../generated_queries/ebnf/highlights.scm");
pub const EBNF_INJECTIONS: &str = include_str!("../generated_queries/ebnf/injections.scm");
pub const EBNF_LOCALS: &str = include_str!("../generated_queries/ebnf/locals.scm");
pub const EBNF_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ebnf/highlights_crates_io.scm");
pub const EBNF_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ebnf/injections_crates_io.scm");
pub const EBNF_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ebnf/locals_crates_io.scm");

pub const EDITORCONFIG_HIGHLIGHTS: &str = include_str!("../generated_queries/editorconfig/highlights.scm");
pub const EDITORCONFIG_INJECTIONS: &str = include_str!("../generated_queries/editorconfig/injections.scm");
pub const EDITORCONFIG_LOCALS: &str = include_str!("../generated_queries/editorconfig/locals.scm");
pub const EDITORCONFIG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/editorconfig/highlights_crates_io.scm");
pub const EDITORCONFIG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/editorconfig/injections_crates_io.scm");
pub const EDITORCONFIG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/editorconfig/locals_crates_io.scm");

pub const EDS_HIGHLIGHTS: &str = include_str!("../generated_queries/eds/highlights.scm");
pub const EDS_INJECTIONS: &str = include_str!("../generated_queries/eds/injections.scm");
pub const EDS_LOCALS: &str = include_str!("../generated_queries/eds/locals.scm");
pub const EDS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/eds/highlights_crates_io.scm");
pub const EDS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/eds/injections_crates_io.scm");
pub const EDS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/eds/locals_crates_io.scm");

pub const EELIXIR_HIGHLIGHTS: &str = include_str!("../generated_queries/eelixir/highlights.scm");
pub const EELIXIR_INJECTIONS: &str = include_str!("../generated_queries/eelixir/injections.scm");
pub const EELIXIR_LOCALS: &str = include_str!("../generated_queries/eelixir/locals.scm");
pub const EELIXIR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/eelixir/highlights_crates_io.scm");
pub const EELIXIR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/eelixir/injections_crates_io.scm");
pub const EELIXIR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/eelixir/locals_crates_io.scm");

pub const EJS_HIGHLIGHTS: &str = include_str!("../generated_queries/ejs/highlights.scm");
pub const EJS_INJECTIONS: &str = include_str!("../generated_queries/ejs/injections.scm");
pub const EJS_LOCALS: &str = include_str!("../generated_queries/ejs/locals.scm");
pub const EJS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ejs/highlights_crates_io.scm");
pub const EJS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ejs/injections_crates_io.scm");
pub const EJS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ejs/locals_crates_io.scm");

pub const ELISP_HIGHLIGHTS: &str = include_str!("../generated_queries/elisp/highlights.scm");
pub const ELISP_INJECTIONS: &str = include_str!("../generated_queries/elisp/injections.scm");
pub const ELISP_LOCALS: &str = include_str!("../generated_queries/elisp/locals.scm");
pub const ELISP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/elisp/highlights_crates_io.scm");
pub const ELISP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/elisp/injections_crates_io.scm");
pub const ELISP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/elisp/locals_crates_io.scm");

pub const ELIXIR_HIGHLIGHTS: &str = include_str!("../generated_queries/elixir/highlights.scm");
pub const ELIXIR_INJECTIONS: &str = include_str!("../generated_queries/elixir/injections.scm");
pub const ELIXIR_LOCALS: &str = include_str!("../generated_queries/elixir/locals.scm");
pub const ELIXIR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/elixir/highlights_crates_io.scm");
pub const ELIXIR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/elixir/injections_crates_io.scm");
pub const ELIXIR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/elixir/locals_crates_io.scm");

pub const ELM_HIGHLIGHTS: &str = include_str!("../generated_queries/elm/highlights.scm");
pub const ELM_INJECTIONS: &str = include_str!("../generated_queries/elm/injections.scm");
pub const ELM_LOCALS: &str = include_str!("../generated_queries/elm/locals.scm");
pub const ELM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/elm/highlights_crates_io.scm");
pub const ELM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/elm/injections_crates_io.scm");
pub const ELM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/elm/locals_crates_io.scm");

pub const ELSA_HIGHLIGHTS: &str = include_str!("../generated_queries/elsa/highlights.scm");
pub const ELSA_INJECTIONS: &str = include_str!("../generated_queries/elsa/injections.scm");
pub const ELSA_LOCALS: &str = include_str!("../generated_queries/elsa/locals.scm");
pub const ELSA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/elsa/highlights_crates_io.scm");
pub const ELSA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/elsa/injections_crates_io.scm");
pub const ELSA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/elsa/locals_crates_io.scm");

pub const ELVISH_HIGHLIGHTS: &str = include_str!("../generated_queries/elvish/highlights.scm");
pub const ELVISH_INJECTIONS: &str = include_str!("../generated_queries/elvish/injections.scm");
pub const ELVISH_LOCALS: &str = include_str!("../generated_queries/elvish/locals.scm");
pub const ELVISH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/elvish/highlights_crates_io.scm");
pub const ELVISH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/elvish/injections_crates_io.scm");
pub const ELVISH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/elvish/locals_crates_io.scm");

pub const ENFORCE_HIGHLIGHTS: &str = include_str!("../generated_queries/enforce/highlights.scm");
pub const ENFORCE_INJECTIONS: &str = include_str!("../generated_queries/enforce/injections.scm");
pub const ENFORCE_LOCALS: &str = include_str!("../generated_queries/enforce/locals.scm");
pub const ENFORCE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/enforce/highlights_crates_io.scm");
pub const ENFORCE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/enforce/injections_crates_io.scm");
pub const ENFORCE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/enforce/locals_crates_io.scm");

pub const ERB_HIGHLIGHTS: &str = include_str!("../generated_queries/erb/highlights.scm");
pub const ERB_INJECTIONS: &str = include_str!("../generated_queries/erb/injections.scm");
pub const ERB_LOCALS: &str = include_str!("../generated_queries/erb/locals.scm");
pub const ERB_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/erb/highlights_crates_io.scm");
pub const ERB_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/erb/injections_crates_io.scm");
pub const ERB_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/erb/locals_crates_io.scm");

pub const ERLANG_HIGHLIGHTS: &str = include_str!("../generated_queries/erlang/highlights.scm");
pub const ERLANG_INJECTIONS: &str = include_str!("../generated_queries/erlang/injections.scm");
pub const ERLANG_LOCALS: &str = include_str!("../generated_queries/erlang/locals.scm");
pub const ERLANG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/erlang/highlights_crates_io.scm");
pub const ERLANG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/erlang/injections_crates_io.scm");
pub const ERLANG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/erlang/locals_crates_io.scm");

pub const ERUBY_HIGHLIGHTS: &str = include_str!("../generated_queries/eruby/highlights.scm");
pub const ERUBY_INJECTIONS: &str = include_str!("../generated_queries/eruby/injections.scm");
pub const ERUBY_LOCALS: &str = include_str!("../generated_queries/eruby/locals.scm");
pub const ERUBY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/eruby/highlights_crates_io.scm");
pub const ERUBY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/eruby/injections_crates_io.scm");
pub const ERUBY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/eruby/locals_crates_io.scm");

pub const FAUST_HIGHLIGHTS: &str = include_str!("../generated_queries/faust/highlights.scm");
pub const FAUST_INJECTIONS: &str = include_str!("../generated_queries/faust/injections.scm");
pub const FAUST_LOCALS: &str = include_str!("../generated_queries/faust/locals.scm");
pub const FAUST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/faust/highlights_crates_io.scm");
pub const FAUST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/faust/injections_crates_io.scm");
pub const FAUST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/faust/locals_crates_io.scm");

pub const FENNEL_HIGHLIGHTS: &str = include_str!("../generated_queries/fennel/highlights.scm");
pub const FENNEL_INJECTIONS: &str = include_str!("../generated_queries/fennel/injections.scm");
pub const FENNEL_LOCALS: &str = include_str!("../generated_queries/fennel/locals.scm");
pub const FENNEL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fennel/highlights_crates_io.scm");
pub const FENNEL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fennel/injections_crates_io.scm");
pub const FENNEL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fennel/locals_crates_io.scm");

pub const FIDL_HIGHLIGHTS: &str = include_str!("../generated_queries/fidl/highlights.scm");
pub const FIDL_INJECTIONS: &str = include_str!("../generated_queries/fidl/injections.scm");
pub const FIDL_LOCALS: &str = include_str!("../generated_queries/fidl/locals.scm");
pub const FIDL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fidl/highlights_crates_io.scm");
pub const FIDL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fidl/injections_crates_io.scm");
pub const FIDL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fidl/locals_crates_io.scm");

pub const FIRRTL_HIGHLIGHTS: &str = include_str!("../generated_queries/firrtl/highlights.scm");
pub const FIRRTL_INJECTIONS: &str = include_str!("../generated_queries/firrtl/injections.scm");
pub const FIRRTL_LOCALS: &str = include_str!("../generated_queries/firrtl/locals.scm");
pub const FIRRTL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/firrtl/highlights_crates_io.scm");
pub const FIRRTL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/firrtl/injections_crates_io.scm");
pub const FIRRTL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/firrtl/locals_crates_io.scm");

pub const FISH_HIGHLIGHTS: &str = include_str!("../generated_queries/fish/highlights.scm");
pub const FISH_INJECTIONS: &str = include_str!("../generated_queries/fish/injections.scm");
pub const FISH_LOCALS: &str = include_str!("../generated_queries/fish/locals.scm");
pub const FISH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fish/highlights_crates_io.scm");
pub const FISH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fish/injections_crates_io.scm");
pub const FISH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fish/locals_crates_io.scm");

pub const FOAM_HIGHLIGHTS: &str = include_str!("../generated_queries/foam/highlights.scm");
pub const FOAM_INJECTIONS: &str = include_str!("../generated_queries/foam/injections.scm");
pub const FOAM_LOCALS: &str = include_str!("../generated_queries/foam/locals.scm");
pub const FOAM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/foam/highlights_crates_io.scm");
pub const FOAM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/foam/injections_crates_io.scm");
pub const FOAM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/foam/locals_crates_io.scm");

pub const FORTH_HIGHLIGHTS: &str = include_str!("../generated_queries/forth/highlights.scm");
pub const FORTH_INJECTIONS: &str = include_str!("../generated_queries/forth/injections.scm");
pub const FORTH_LOCALS: &str = include_str!("../generated_queries/forth/locals.scm");
pub const FORTH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/forth/highlights_crates_io.scm");
pub const FORTH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/forth/injections_crates_io.scm");
pub const FORTH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/forth/locals_crates_io.scm");

pub const FORTRAN_HIGHLIGHTS: &str = include_str!("../generated_queries/fortran/highlights.scm");
pub const FORTRAN_INJECTIONS: &str = include_str!("../generated_queries/fortran/injections.scm");
pub const FORTRAN_LOCALS: &str = include_str!("../generated_queries/fortran/locals.scm");
pub const FORTRAN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fortran/highlights_crates_io.scm");
pub const FORTRAN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fortran/injections_crates_io.scm");
pub const FORTRAN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fortran/locals_crates_io.scm");

pub const FSD_HIGHLIGHTS: &str = include_str!("../generated_queries/fsd/highlights.scm");
pub const FSD_INJECTIONS: &str = include_str!("../generated_queries/fsd/injections.scm");
pub const FSD_LOCALS: &str = include_str!("../generated_queries/fsd/locals.scm");
pub const FSD_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fsd/highlights_crates_io.scm");
pub const FSD_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fsd/injections_crates_io.scm");
pub const FSD_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fsd/locals_crates_io.scm");

pub const FSH_HIGHLIGHTS: &str = include_str!("../generated_queries/fsh/highlights.scm");
pub const FSH_INJECTIONS: &str = include_str!("../generated_queries/fsh/injections.scm");
pub const FSH_LOCALS: &str = include_str!("../generated_queries/fsh/locals.scm");
pub const FSH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fsh/highlights_crates_io.scm");
pub const FSH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fsh/injections_crates_io.scm");
pub const FSH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fsh/locals_crates_io.scm");

pub const FSHARP_HIGHLIGHTS: &str = include_str!("../generated_queries/fsharp/highlights.scm");
pub const FSHARP_INJECTIONS: &str = include_str!("../generated_queries/fsharp/injections.scm");
pub const FSHARP_LOCALS: &str = include_str!("../generated_queries/fsharp/locals.scm");
pub const FSHARP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fsharp/highlights_crates_io.scm");
pub const FSHARP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fsharp/injections_crates_io.scm");
pub const FSHARP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fsharp/locals_crates_io.scm");

pub const FUNC_HIGHLIGHTS: &str = include_str!("../generated_queries/func/highlights.scm");
pub const FUNC_INJECTIONS: &str = include_str!("../generated_queries/func/injections.scm");
pub const FUNC_LOCALS: &str = include_str!("../generated_queries/func/locals.scm");
pub const FUNC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/func/highlights_crates_io.scm");
pub const FUNC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/func/injections_crates_io.scm");
pub const FUNC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/func/locals_crates_io.scm");

pub const FUSION_HIGHLIGHTS: &str = include_str!("../generated_queries/fusion/highlights.scm");
pub const FUSION_INJECTIONS: &str = include_str!("../generated_queries/fusion/injections.scm");
pub const FUSION_LOCALS: &str = include_str!("../generated_queries/fusion/locals.scm");
pub const FUSION_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/fusion/highlights_crates_io.scm");
pub const FUSION_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/fusion/injections_crates_io.scm");
pub const FUSION_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/fusion/locals_crates_io.scm");

pub const GAP_HIGHLIGHTS: &str = include_str!("../generated_queries/gap/highlights.scm");
pub const GAP_INJECTIONS: &str = include_str!("../generated_queries/gap/injections.scm");
pub const GAP_LOCALS: &str = include_str!("../generated_queries/gap/locals.scm");
pub const GAP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gap/highlights_crates_io.scm");
pub const GAP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gap/injections_crates_io.scm");
pub const GAP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gap/locals_crates_io.scm");

pub const GAPTST_HIGHLIGHTS: &str = include_str!("../generated_queries/gaptst/highlights.scm");
pub const GAPTST_INJECTIONS: &str = include_str!("../generated_queries/gaptst/injections.scm");
pub const GAPTST_LOCALS: &str = include_str!("../generated_queries/gaptst/locals.scm");
pub const GAPTST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gaptst/highlights_crates_io.scm");
pub const GAPTST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gaptst/injections_crates_io.scm");
pub const GAPTST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gaptst/locals_crates_io.scm");

pub const GDRESOURCE_HIGHLIGHTS: &str = include_str!("../generated_queries/gdresource/highlights.scm");
pub const GDRESOURCE_INJECTIONS: &str = include_str!("../generated_queries/gdresource/injections.scm");
pub const GDRESOURCE_LOCALS: &str = include_str!("../generated_queries/gdresource/locals.scm");
pub const GDRESOURCE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gdresource/highlights_crates_io.scm");
pub const GDRESOURCE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gdresource/injections_crates_io.scm");
pub const GDRESOURCE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gdresource/locals_crates_io.scm");

pub const GDSCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/gdscript/highlights.scm");
pub const GDSCRIPT_INJECTIONS: &str = include_str!("../generated_queries/gdscript/injections.scm");
pub const GDSCRIPT_LOCALS: &str = include_str!("../generated_queries/gdscript/locals.scm");
pub const GDSCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gdscript/highlights_crates_io.scm");
pub const GDSCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gdscript/injections_crates_io.scm");
pub const GDSCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gdscript/locals_crates_io.scm");

pub const GDSHADERINC_HIGHLIGHTS: &str = include_str!("../generated_queries/gdshaderinc/highlights.scm");
pub const GDSHADERINC_INJECTIONS: &str = include_str!("../generated_queries/gdshaderinc/injections.scm");
pub const GDSHADERINC_LOCALS: &str = include_str!("../generated_queries/gdshaderinc/locals.scm");
pub const GDSHADERINC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gdshaderinc/highlights_crates_io.scm");
pub const GDSHADERINC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gdshaderinc/injections_crates_io.scm");
pub const GDSHADERINC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gdshaderinc/locals_crates_io.scm");

pub const GITATTRIBUTES_HIGHLIGHTS: &str = include_str!("../generated_queries/gitattributes/highlights.scm");
pub const GITATTRIBUTES_INJECTIONS: &str = include_str!("../generated_queries/gitattributes/injections.scm");
pub const GITATTRIBUTES_LOCALS: &str = include_str!("../generated_queries/gitattributes/locals.scm");
pub const GITATTRIBUTES_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gitattributes/highlights_crates_io.scm");
pub const GITATTRIBUTES_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gitattributes/injections_crates_io.scm");
pub const GITATTRIBUTES_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gitattributes/locals_crates_io.scm");

pub const GITCOMMIT_HIGHLIGHTS: &str = include_str!("../generated_queries/gitcommit/highlights.scm");
pub const GITCOMMIT_INJECTIONS: &str = include_str!("../generated_queries/gitcommit/injections.scm");
pub const GITCOMMIT_LOCALS: &str = include_str!("../generated_queries/gitcommit/locals.scm");
pub const GITCOMMIT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gitcommit/highlights_crates_io.scm");
pub const GITCOMMIT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gitcommit/injections_crates_io.scm");
pub const GITCOMMIT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gitcommit/locals_crates_io.scm");

pub const GITCONFIG_HIGHLIGHTS: &str = include_str!("../generated_queries/gitconfig/highlights.scm");
pub const GITCONFIG_INJECTIONS: &str = include_str!("../generated_queries/gitconfig/injections.scm");
pub const GITCONFIG_LOCALS: &str = include_str!("../generated_queries/gitconfig/locals.scm");
pub const GITCONFIG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gitconfig/highlights_crates_io.scm");
pub const GITCONFIG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gitconfig/injections_crates_io.scm");
pub const GITCONFIG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gitconfig/locals_crates_io.scm");

pub const GITDIFF_HIGHLIGHTS: &str = include_str!("../generated_queries/gitdiff/highlights.scm");
pub const GITDIFF_INJECTIONS: &str = include_str!("../generated_queries/gitdiff/injections.scm");
pub const GITDIFF_LOCALS: &str = include_str!("../generated_queries/gitdiff/locals.scm");
pub const GITDIFF_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gitdiff/highlights_crates_io.scm");
pub const GITDIFF_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gitdiff/injections_crates_io.scm");
pub const GITDIFF_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gitdiff/locals_crates_io.scm");

pub const GITIGNORE_HIGHLIGHTS: &str = include_str!("../generated_queries/gitignore/highlights.scm");
pub const GITIGNORE_INJECTIONS: &str = include_str!("../generated_queries/gitignore/injections.scm");
pub const GITIGNORE_LOCALS: &str = include_str!("../generated_queries/gitignore/locals.scm");
pub const GITIGNORE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gitignore/highlights_crates_io.scm");
pub const GITIGNORE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gitignore/injections_crates_io.scm");
pub const GITIGNORE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gitignore/locals_crates_io.scm");

pub const GITREBASE_HIGHLIGHTS: &str = include_str!("../generated_queries/gitrebase/highlights.scm");
pub const GITREBASE_INJECTIONS: &str = include_str!("../generated_queries/gitrebase/injections.scm");
pub const GITREBASE_LOCALS: &str = include_str!("../generated_queries/gitrebase/locals.scm");
pub const GITREBASE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gitrebase/highlights_crates_io.scm");
pub const GITREBASE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gitrebase/injections_crates_io.scm");
pub const GITREBASE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gitrebase/locals_crates_io.scm");

pub const GLEAM_HIGHLIGHTS: &str = include_str!("../generated_queries/gleam/highlights.scm");
pub const GLEAM_INJECTIONS: &str = include_str!("../generated_queries/gleam/injections.scm");
pub const GLEAM_LOCALS: &str = include_str!("../generated_queries/gleam/locals.scm");
pub const GLEAM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gleam/highlights_crates_io.scm");
pub const GLEAM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gleam/injections_crates_io.scm");
pub const GLEAM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gleam/locals_crates_io.scm");

pub const GLIMMER_JAVASCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/glimmer_javascript/highlights.scm");
pub const GLIMMER_JAVASCRIPT_INJECTIONS: &str = include_str!("../generated_queries/glimmer_javascript/injections.scm");
pub const GLIMMER_JAVASCRIPT_LOCALS: &str = include_str!("../generated_queries/glimmer_javascript/locals.scm");
pub const GLIMMER_JAVASCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/glimmer_javascript/highlights_crates_io.scm");
pub const GLIMMER_JAVASCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/glimmer_javascript/injections_crates_io.scm");
pub const GLIMMER_JAVASCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/glimmer_javascript/locals_crates_io.scm");

pub const GLIMMER_TYPESCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/glimmer_typescript/highlights.scm");
pub const GLIMMER_TYPESCRIPT_INJECTIONS: &str = include_str!("../generated_queries/glimmer_typescript/injections.scm");
pub const GLIMMER_TYPESCRIPT_LOCALS: &str = include_str!("../generated_queries/glimmer_typescript/locals.scm");
pub const GLIMMER_TYPESCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/glimmer_typescript/highlights_crates_io.scm");
pub const GLIMMER_TYPESCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/glimmer_typescript/injections_crates_io.scm");
pub const GLIMMER_TYPESCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/glimmer_typescript/locals_crates_io.scm");

pub const GLSL_HIGHLIGHTS: &str = include_str!("../generated_queries/glsl/highlights.scm");
pub const GLSL_INJECTIONS: &str = include_str!("../generated_queries/glsl/injections.scm");
pub const GLSL_LOCALS: &str = include_str!("../generated_queries/glsl/locals.scm");
pub const GLSL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/glsl/highlights_crates_io.scm");
pub const GLSL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/glsl/injections_crates_io.scm");
pub const GLSL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/glsl/locals_crates_io.scm");

pub const GN_HIGHLIGHTS: &str = include_str!("../generated_queries/gn/highlights.scm");
pub const GN_INJECTIONS: &str = include_str!("../generated_queries/gn/injections.scm");
pub const GN_LOCALS: &str = include_str!("../generated_queries/gn/locals.scm");
pub const GN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gn/highlights_crates_io.scm");
pub const GN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gn/injections_crates_io.scm");
pub const GN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gn/locals_crates_io.scm");

pub const GNUPLOT_HIGHLIGHTS: &str = include_str!("../generated_queries/gnuplot/highlights.scm");
pub const GNUPLOT_INJECTIONS: &str = include_str!("../generated_queries/gnuplot/injections.scm");
pub const GNUPLOT_LOCALS: &str = include_str!("../generated_queries/gnuplot/locals.scm");
pub const GNUPLOT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gnuplot/highlights_crates_io.scm");
pub const GNUPLOT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gnuplot/injections_crates_io.scm");
pub const GNUPLOT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gnuplot/locals_crates_io.scm");

pub const GO_HIGHLIGHTS: &str = include_str!("../generated_queries/go/highlights.scm");
pub const GO_INJECTIONS: &str = include_str!("../generated_queries/go/injections.scm");
pub const GO_LOCALS: &str = include_str!("../generated_queries/go/locals.scm");
pub const GO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/go/highlights_crates_io.scm");
pub const GO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/go/injections_crates_io.scm");
pub const GO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/go/locals_crates_io.scm");

pub const GOCTL_HIGHLIGHTS: &str = include_str!("../generated_queries/goctl/highlights.scm");
pub const GOCTL_INJECTIONS: &str = include_str!("../generated_queries/goctl/injections.scm");
pub const GOCTL_LOCALS: &str = include_str!("../generated_queries/goctl/locals.scm");
pub const GOCTL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/goctl/highlights_crates_io.scm");
pub const GOCTL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/goctl/injections_crates_io.scm");
pub const GOCTL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/goctl/locals_crates_io.scm");

pub const GOMOD_HIGHLIGHTS: &str = include_str!("../generated_queries/gomod/highlights.scm");
pub const GOMOD_INJECTIONS: &str = include_str!("../generated_queries/gomod/injections.scm");
pub const GOMOD_LOCALS: &str = include_str!("../generated_queries/gomod/locals.scm");
pub const GOMOD_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gomod/highlights_crates_io.scm");
pub const GOMOD_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gomod/injections_crates_io.scm");
pub const GOMOD_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gomod/locals_crates_io.scm");

pub const GOSUM_HIGHLIGHTS: &str = include_str!("../generated_queries/gosum/highlights.scm");
pub const GOSUM_INJECTIONS: &str = include_str!("../generated_queries/gosum/injections.scm");
pub const GOSUM_LOCALS: &str = include_str!("../generated_queries/gosum/locals.scm");
pub const GOSUM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gosum/highlights_crates_io.scm");
pub const GOSUM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gosum/injections_crates_io.scm");
pub const GOSUM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gosum/locals_crates_io.scm");

pub const GOTMPL_HIGHLIGHTS: &str = include_str!("../generated_queries/gotmpl/highlights.scm");
pub const GOTMPL_INJECTIONS: &str = include_str!("../generated_queries/gotmpl/injections.scm");
pub const GOTMPL_LOCALS: &str = include_str!("../generated_queries/gotmpl/locals.scm");
pub const GOTMPL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gotmpl/highlights_crates_io.scm");
pub const GOTMPL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gotmpl/injections_crates_io.scm");
pub const GOTMPL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gotmpl/locals_crates_io.scm");

pub const GOWORK_HIGHLIGHTS: &str = include_str!("../generated_queries/gowork/highlights.scm");
pub const GOWORK_INJECTIONS: &str = include_str!("../generated_queries/gowork/injections.scm");
pub const GOWORK_LOCALS: &str = include_str!("../generated_queries/gowork/locals.scm");
pub const GOWORK_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gowork/highlights_crates_io.scm");
pub const GOWORK_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gowork/injections_crates_io.scm");
pub const GOWORK_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gowork/locals_crates_io.scm");

pub const GPG_HIGHLIGHTS: &str = include_str!("../generated_queries/gpg/highlights.scm");
pub const GPG_INJECTIONS: &str = include_str!("../generated_queries/gpg/injections.scm");
pub const GPG_LOCALS: &str = include_str!("../generated_queries/gpg/locals.scm");
pub const GPG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gpg/highlights_crates_io.scm");
pub const GPG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gpg/injections_crates_io.scm");
pub const GPG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gpg/locals_crates_io.scm");

pub const GRAPHQL_HIGHLIGHTS: &str = include_str!("../generated_queries/graphql/highlights.scm");
pub const GRAPHQL_INJECTIONS: &str = include_str!("../generated_queries/graphql/injections.scm");
pub const GRAPHQL_LOCALS: &str = include_str!("../generated_queries/graphql/locals.scm");
pub const GRAPHQL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/graphql/highlights_crates_io.scm");
pub const GRAPHQL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/graphql/injections_crates_io.scm");
pub const GRAPHQL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/graphql/locals_crates_io.scm");

pub const GREN_HIGHLIGHTS: &str = include_str!("../generated_queries/gren/highlights.scm");
pub const GREN_INJECTIONS: &str = include_str!("../generated_queries/gren/injections.scm");
pub const GREN_LOCALS: &str = include_str!("../generated_queries/gren/locals.scm");
pub const GREN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gren/highlights_crates_io.scm");
pub const GREN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gren/injections_crates_io.scm");
pub const GREN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gren/locals_crates_io.scm");

pub const GROOVY_HIGHLIGHTS: &str = include_str!("../generated_queries/groovy/highlights.scm");
pub const GROOVY_INJECTIONS: &str = include_str!("../generated_queries/groovy/injections.scm");
pub const GROOVY_LOCALS: &str = include_str!("../generated_queries/groovy/locals.scm");
pub const GROOVY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/groovy/highlights_crates_io.scm");
pub const GROOVY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/groovy/injections_crates_io.scm");
pub const GROOVY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/groovy/locals_crates_io.scm");

pub const GROQ_HIGHLIGHTS: &str = include_str!("../generated_queries/groq/highlights.scm");
pub const GROQ_INJECTIONS: &str = include_str!("../generated_queries/groq/injections.scm");
pub const GROQ_LOCALS: &str = include_str!("../generated_queries/groq/locals.scm");
pub const GROQ_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/groq/highlights_crates_io.scm");
pub const GROQ_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/groq/injections_crates_io.scm");
pub const GROQ_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/groq/locals_crates_io.scm");

pub const GSTLAUNCH_HIGHLIGHTS: &str = include_str!("../generated_queries/gstlaunch/highlights.scm");
pub const GSTLAUNCH_INJECTIONS: &str = include_str!("../generated_queries/gstlaunch/injections.scm");
pub const GSTLAUNCH_LOCALS: &str = include_str!("../generated_queries/gstlaunch/locals.scm");
pub const GSTLAUNCH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/gstlaunch/highlights_crates_io.scm");
pub const GSTLAUNCH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/gstlaunch/injections_crates_io.scm");
pub const GSTLAUNCH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/gstlaunch/locals_crates_io.scm");

pub const HACK_HIGHLIGHTS: &str = include_str!("../generated_queries/hack/highlights.scm");
pub const HACK_INJECTIONS: &str = include_str!("../generated_queries/hack/injections.scm");
pub const HACK_LOCALS: &str = include_str!("../generated_queries/hack/locals.scm");
pub const HACK_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hack/highlights_crates_io.scm");
pub const HACK_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hack/injections_crates_io.scm");
pub const HACK_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hack/locals_crates_io.scm");

pub const HANDLEBARS_HIGHLIGHTS: &str = include_str!("../generated_queries/handlebars/highlights.scm");
pub const HANDLEBARS_INJECTIONS: &str = include_str!("../generated_queries/handlebars/injections.scm");
pub const HANDLEBARS_LOCALS: &str = include_str!("../generated_queries/handlebars/locals.scm");
pub const HANDLEBARS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/handlebars/highlights_crates_io.scm");
pub const HANDLEBARS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/handlebars/injections_crates_io.scm");
pub const HANDLEBARS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/handlebars/locals_crates_io.scm");

pub const HARE_HIGHLIGHTS: &str = include_str!("../generated_queries/hare/highlights.scm");
pub const HARE_INJECTIONS: &str = include_str!("../generated_queries/hare/injections.scm");
pub const HARE_LOCALS: &str = include_str!("../generated_queries/hare/locals.scm");
pub const HARE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hare/highlights_crates_io.scm");
pub const HARE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hare/injections_crates_io.scm");
pub const HARE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hare/locals_crates_io.scm");

pub const HASKELL_HIGHLIGHTS: &str = include_str!("../generated_queries/haskell/highlights.scm");
pub const HASKELL_INJECTIONS: &str = include_str!("../generated_queries/haskell/injections.scm");
pub const HASKELL_LOCALS: &str = include_str!("../generated_queries/haskell/locals.scm");
pub const HASKELL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/haskell/highlights_crates_io.scm");
pub const HASKELL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/haskell/injections_crates_io.scm");
pub const HASKELL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/haskell/locals_crates_io.scm");

pub const HASKELLPERSISTENT_HIGHLIGHTS: &str = include_str!("../generated_queries/haskellpersistent/highlights.scm");
pub const HASKELLPERSISTENT_INJECTIONS: &str = include_str!("../generated_queries/haskellpersistent/injections.scm");
pub const HASKELLPERSISTENT_LOCALS: &str = include_str!("../generated_queries/haskellpersistent/locals.scm");
pub const HASKELLPERSISTENT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/haskellpersistent/highlights_crates_io.scm");
pub const HASKELLPERSISTENT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/haskellpersistent/injections_crates_io.scm");
pub const HASKELLPERSISTENT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/haskellpersistent/locals_crates_io.scm");

pub const HAXE_HIGHLIGHTS: &str = include_str!("../generated_queries/haxe/highlights.scm");
pub const HAXE_INJECTIONS: &str = include_str!("../generated_queries/haxe/injections.scm");
pub const HAXE_LOCALS: &str = include_str!("../generated_queries/haxe/locals.scm");
pub const HAXE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/haxe/highlights_crates_io.scm");
pub const HAXE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/haxe/injections_crates_io.scm");
pub const HAXE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/haxe/locals_crates_io.scm");

pub const HCL_HIGHLIGHTS: &str = include_str!("../generated_queries/hcl/highlights.scm");
pub const HCL_INJECTIONS: &str = include_str!("../generated_queries/hcl/injections.scm");
pub const HCL_LOCALS: &str = include_str!("../generated_queries/hcl/locals.scm");
pub const HCL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hcl/highlights_crates_io.scm");
pub const HCL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hcl/injections_crates_io.scm");
pub const HCL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hcl/locals_crates_io.scm");

pub const HEEX_HIGHLIGHTS: &str = include_str!("../generated_queries/heex/highlights.scm");
pub const HEEX_INJECTIONS: &str = include_str!("../generated_queries/heex/injections.scm");
pub const HEEX_LOCALS: &str = include_str!("../generated_queries/heex/locals.scm");
pub const HEEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/heex/highlights_crates_io.scm");
pub const HEEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/heex/injections_crates_io.scm");
pub const HEEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/heex/locals_crates_io.scm");

pub const HELM_HIGHLIGHTS: &str = include_str!("../generated_queries/helm/highlights.scm");
pub const HELM_INJECTIONS: &str = include_str!("../generated_queries/helm/injections.scm");
pub const HELM_LOCALS: &str = include_str!("../generated_queries/helm/locals.scm");
pub const HELM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/helm/highlights_crates_io.scm");
pub const HELM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/helm/injections_crates_io.scm");
pub const HELM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/helm/locals_crates_io.scm");

pub const HELP_HIGHLIGHTS: &str = include_str!("../generated_queries/help/highlights.scm");
pub const HELP_INJECTIONS: &str = include_str!("../generated_queries/help/injections.scm");
pub const HELP_LOCALS: &str = include_str!("../generated_queries/help/locals.scm");
pub const HELP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/help/highlights_crates_io.scm");
pub const HELP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/help/injections_crates_io.scm");
pub const HELP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/help/locals_crates_io.scm");

pub const HEXDUMP_HIGHLIGHTS: &str = include_str!("../generated_queries/hexdump/highlights.scm");
pub const HEXDUMP_INJECTIONS: &str = include_str!("../generated_queries/hexdump/injections.scm");
pub const HEXDUMP_LOCALS: &str = include_str!("../generated_queries/hexdump/locals.scm");
pub const HEXDUMP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hexdump/highlights_crates_io.scm");
pub const HEXDUMP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hexdump/injections_crates_io.scm");
pub const HEXDUMP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hexdump/locals_crates_io.scm");

pub const HJSON_HIGHLIGHTS: &str = include_str!("../generated_queries/hjson/highlights.scm");
pub const HJSON_INJECTIONS: &str = include_str!("../generated_queries/hjson/injections.scm");
pub const HJSON_LOCALS: &str = include_str!("../generated_queries/hjson/locals.scm");
pub const HJSON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hjson/highlights_crates_io.scm");
pub const HJSON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hjson/injections_crates_io.scm");
pub const HJSON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hjson/locals_crates_io.scm");

pub const HLSL_HIGHLIGHTS: &str = include_str!("../generated_queries/hlsl/highlights.scm");
pub const HLSL_INJECTIONS: &str = include_str!("../generated_queries/hlsl/injections.scm");
pub const HLSL_LOCALS: &str = include_str!("../generated_queries/hlsl/locals.scm");
pub const HLSL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hlsl/highlights_crates_io.scm");
pub const HLSL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hlsl/injections_crates_io.scm");
pub const HLSL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hlsl/locals_crates_io.scm");

pub const HLSPLAYLIST_HIGHLIGHTS: &str = include_str!("../generated_queries/hlsplaylist/highlights.scm");
pub const HLSPLAYLIST_INJECTIONS: &str = include_str!("../generated_queries/hlsplaylist/injections.scm");
pub const HLSPLAYLIST_LOCALS: &str = include_str!("../generated_queries/hlsplaylist/locals.scm");
pub const HLSPLAYLIST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hlsplaylist/highlights_crates_io.scm");
pub const HLSPLAYLIST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hlsplaylist/injections_crates_io.scm");
pub const HLSPLAYLIST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hlsplaylist/locals_crates_io.scm");

pub const HOCON_HIGHLIGHTS: &str = include_str!("../generated_queries/hocon/highlights.scm");
pub const HOCON_INJECTIONS: &str = include_str!("../generated_queries/hocon/injections.scm");
pub const HOCON_LOCALS: &str = include_str!("../generated_queries/hocon/locals.scm");
pub const HOCON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hocon/highlights_crates_io.scm");
pub const HOCON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hocon/injections_crates_io.scm");
pub const HOCON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hocon/locals_crates_io.scm");

pub const HOON_HIGHLIGHTS: &str = include_str!("../generated_queries/hoon/highlights.scm");
pub const HOON_INJECTIONS: &str = include_str!("../generated_queries/hoon/injections.scm");
pub const HOON_LOCALS: &str = include_str!("../generated_queries/hoon/locals.scm");
pub const HOON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hoon/highlights_crates_io.scm");
pub const HOON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hoon/injections_crates_io.scm");
pub const HOON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hoon/locals_crates_io.scm");

pub const HTML_HIGHLIGHTS: &str = include_str!("../generated_queries/html/highlights.scm");
pub const HTML_INJECTIONS: &str = include_str!("../generated_queries/html/injections.scm");
pub const HTML_LOCALS: &str = include_str!("../generated_queries/html/locals.scm");
pub const HTML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/html/highlights_crates_io.scm");
pub const HTML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/html/injections_crates_io.scm");
pub const HTML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/html/locals_crates_io.scm");

pub const HTMLANGULAR_HIGHLIGHTS: &str = include_str!("../generated_queries/htmlangular/highlights.scm");
pub const HTMLANGULAR_INJECTIONS: &str = include_str!("../generated_queries/htmlangular/injections.scm");
pub const HTMLANGULAR_LOCALS: &str = include_str!("../generated_queries/htmlangular/locals.scm");
pub const HTMLANGULAR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/htmlangular/highlights_crates_io.scm");
pub const HTMLANGULAR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/htmlangular/injections_crates_io.scm");
pub const HTMLANGULAR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/htmlangular/locals_crates_io.scm");

pub const HTMLDJANGO_HIGHLIGHTS: &str = include_str!("../generated_queries/htmldjango/highlights.scm");
pub const HTMLDJANGO_INJECTIONS: &str = include_str!("../generated_queries/htmldjango/injections.scm");
pub const HTMLDJANGO_LOCALS: &str = include_str!("../generated_queries/htmldjango/locals.scm");
pub const HTMLDJANGO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/htmldjango/highlights_crates_io.scm");
pub const HTMLDJANGO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/htmldjango/injections_crates_io.scm");
pub const HTMLDJANGO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/htmldjango/locals_crates_io.scm");

pub const HTTP_HIGHLIGHTS: &str = include_str!("../generated_queries/http/highlights.scm");
pub const HTTP_INJECTIONS: &str = include_str!("../generated_queries/http/injections.scm");
pub const HTTP_LOCALS: &str = include_str!("../generated_queries/http/locals.scm");
pub const HTTP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/http/highlights_crates_io.scm");
pub const HTTP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/http/injections_crates_io.scm");
pub const HTTP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/http/locals_crates_io.scm");

pub const HURL_HIGHLIGHTS: &str = include_str!("../generated_queries/hurl/highlights.scm");
pub const HURL_INJECTIONS: &str = include_str!("../generated_queries/hurl/injections.scm");
pub const HURL_LOCALS: &str = include_str!("../generated_queries/hurl/locals.scm");
pub const HURL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hurl/highlights_crates_io.scm");
pub const HURL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hurl/injections_crates_io.scm");
pub const HURL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hurl/locals_crates_io.scm");

pub const HYPRLANG_HIGHLIGHTS: &str = include_str!("../generated_queries/hyprlang/highlights.scm");
pub const HYPRLANG_INJECTIONS: &str = include_str!("../generated_queries/hyprlang/injections.scm");
pub const HYPRLANG_LOCALS: &str = include_str!("../generated_queries/hyprlang/locals.scm");
pub const HYPRLANG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/hyprlang/highlights_crates_io.scm");
pub const HYPRLANG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/hyprlang/injections_crates_io.scm");
pub const HYPRLANG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/hyprlang/locals_crates_io.scm");

pub const IDL_HIGHLIGHTS: &str = include_str!("../generated_queries/idl/highlights.scm");
pub const IDL_INJECTIONS: &str = include_str!("../generated_queries/idl/injections.scm");
pub const IDL_LOCALS: &str = include_str!("../generated_queries/idl/locals.scm");
pub const IDL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/idl/highlights_crates_io.scm");
pub const IDL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/idl/injections_crates_io.scm");
pub const IDL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/idl/locals_crates_io.scm");

pub const IDRIS2_HIGHLIGHTS: &str = include_str!("../generated_queries/idris2/highlights.scm");
pub const IDRIS2_INJECTIONS: &str = include_str!("../generated_queries/idris2/injections.scm");
pub const IDRIS2_LOCALS: &str = include_str!("../generated_queries/idris2/locals.scm");
pub const IDRIS2_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/idris2/highlights_crates_io.scm");
pub const IDRIS2_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/idris2/injections_crates_io.scm");
pub const IDRIS2_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/idris2/locals_crates_io.scm");

pub const INI_HIGHLIGHTS: &str = include_str!("../generated_queries/ini/highlights.scm");
pub const INI_INJECTIONS: &str = include_str!("../generated_queries/ini/injections.scm");
pub const INI_LOCALS: &str = include_str!("../generated_queries/ini/locals.scm");
pub const INI_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ini/highlights_crates_io.scm");
pub const INI_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ini/injections_crates_io.scm");
pub const INI_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ini/locals_crates_io.scm");

pub const INKO_HIGHLIGHTS: &str = include_str!("../generated_queries/inko/highlights.scm");
pub const INKO_INJECTIONS: &str = include_str!("../generated_queries/inko/injections.scm");
pub const INKO_LOCALS: &str = include_str!("../generated_queries/inko/locals.scm");
pub const INKO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/inko/highlights_crates_io.scm");
pub const INKO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/inko/injections_crates_io.scm");
pub const INKO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/inko/locals_crates_io.scm");

pub const ISPC_HIGHLIGHTS: &str = include_str!("../generated_queries/ispc/highlights.scm");
pub const ISPC_INJECTIONS: &str = include_str!("../generated_queries/ispc/injections.scm");
pub const ISPC_LOCALS: &str = include_str!("../generated_queries/ispc/locals.scm");
pub const ISPC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ispc/highlights_crates_io.scm");
pub const ISPC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ispc/injections_crates_io.scm");
pub const ISPC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ispc/locals_crates_io.scm");

pub const JANET_HIGHLIGHTS: &str = include_str!("../generated_queries/janet/highlights.scm");
pub const JANET_INJECTIONS: &str = include_str!("../generated_queries/janet/injections.scm");
pub const JANET_LOCALS: &str = include_str!("../generated_queries/janet/locals.scm");
pub const JANET_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/janet/highlights_crates_io.scm");
pub const JANET_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/janet/injections_crates_io.scm");
pub const JANET_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/janet/locals_crates_io.scm");

pub const JAVA_HIGHLIGHTS: &str = include_str!("../generated_queries/java/highlights.scm");
pub const JAVA_INJECTIONS: &str = include_str!("../generated_queries/java/injections.scm");
pub const JAVA_LOCALS: &str = include_str!("../generated_queries/java/locals.scm");
pub const JAVA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/java/highlights_crates_io.scm");
pub const JAVA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/java/injections_crates_io.scm");
pub const JAVA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/java/locals_crates_io.scm");

pub const JAVADOC_HIGHLIGHTS: &str = include_str!("../generated_queries/javadoc/highlights.scm");
pub const JAVADOC_INJECTIONS: &str = include_str!("../generated_queries/javadoc/injections.scm");
pub const JAVADOC_LOCALS: &str = include_str!("../generated_queries/javadoc/locals.scm");
pub const JAVADOC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/javadoc/highlights_crates_io.scm");
pub const JAVADOC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/javadoc/injections_crates_io.scm");
pub const JAVADOC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/javadoc/locals_crates_io.scm");

pub const JAVASCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/javascript/highlights.scm");
pub const JAVASCRIPT_INJECTIONS: &str = include_str!("../generated_queries/javascript/injections.scm");
pub const JAVASCRIPT_LOCALS: &str = include_str!("../generated_queries/javascript/locals.scm");
pub const JAVASCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/javascript/highlights_crates_io.scm");
pub const JAVASCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/javascript/injections_crates_io.scm");
pub const JAVASCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/javascript/locals_crates_io.scm");

pub const JINJA_HIGHLIGHTS: &str = include_str!("../generated_queries/jinja/highlights.scm");
pub const JINJA_INJECTIONS: &str = include_str!("../generated_queries/jinja/injections.scm");
pub const JINJA_LOCALS: &str = include_str!("../generated_queries/jinja/locals.scm");
pub const JINJA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jinja/highlights_crates_io.scm");
pub const JINJA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jinja/injections_crates_io.scm");
pub const JINJA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jinja/locals_crates_io.scm");

pub const JINJA_INLINE_HIGHLIGHTS: &str = include_str!("../generated_queries/jinja_inline/highlights.scm");
pub const JINJA_INLINE_INJECTIONS: &str = include_str!("../generated_queries/jinja_inline/injections.scm");
pub const JINJA_INLINE_LOCALS: &str = include_str!("../generated_queries/jinja_inline/locals.scm");
pub const JINJA_INLINE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jinja_inline/highlights_crates_io.scm");
pub const JINJA_INLINE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jinja_inline/injections_crates_io.scm");
pub const JINJA_INLINE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jinja_inline/locals_crates_io.scm");

pub const JPROPERTIES_HIGHLIGHTS: &str = include_str!("../generated_queries/jproperties/highlights.scm");
pub const JPROPERTIES_INJECTIONS: &str = include_str!("../generated_queries/jproperties/injections.scm");
pub const JPROPERTIES_LOCALS: &str = include_str!("../generated_queries/jproperties/locals.scm");
pub const JPROPERTIES_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jproperties/highlights_crates_io.scm");
pub const JPROPERTIES_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jproperties/injections_crates_io.scm");
pub const JPROPERTIES_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jproperties/locals_crates_io.scm");

pub const JQ_HIGHLIGHTS: &str = include_str!("../generated_queries/jq/highlights.scm");
pub const JQ_INJECTIONS: &str = include_str!("../generated_queries/jq/injections.scm");
pub const JQ_LOCALS: &str = include_str!("../generated_queries/jq/locals.scm");
pub const JQ_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jq/highlights_crates_io.scm");
pub const JQ_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jq/injections_crates_io.scm");
pub const JQ_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jq/locals_crates_io.scm");

pub const JSDOC_HIGHLIGHTS: &str = include_str!("../generated_queries/jsdoc/highlights.scm");
pub const JSDOC_INJECTIONS: &str = include_str!("../generated_queries/jsdoc/injections.scm");
pub const JSDOC_LOCALS: &str = include_str!("../generated_queries/jsdoc/locals.scm");
pub const JSDOC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jsdoc/highlights_crates_io.scm");
pub const JSDOC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jsdoc/injections_crates_io.scm");
pub const JSDOC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jsdoc/locals_crates_io.scm");

pub const JSON_HIGHLIGHTS: &str = include_str!("../generated_queries/json/highlights.scm");
pub const JSON_INJECTIONS: &str = include_str!("../generated_queries/json/injections.scm");
pub const JSON_LOCALS: &str = include_str!("../generated_queries/json/locals.scm");
pub const JSON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/json/highlights_crates_io.scm");
pub const JSON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/json/injections_crates_io.scm");
pub const JSON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/json/locals_crates_io.scm");

pub const JSON5_HIGHLIGHTS: &str = include_str!("../generated_queries/json5/highlights.scm");
pub const JSON5_INJECTIONS: &str = include_str!("../generated_queries/json5/injections.scm");
pub const JSON5_LOCALS: &str = include_str!("../generated_queries/json5/locals.scm");
pub const JSON5_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/json5/highlights_crates_io.scm");
pub const JSON5_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/json5/injections_crates_io.scm");
pub const JSON5_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/json5/locals_crates_io.scm");

pub const JSONC_HIGHLIGHTS: &str = include_str!("../generated_queries/jsonc/highlights.scm");
pub const JSONC_INJECTIONS: &str = include_str!("../generated_queries/jsonc/injections.scm");
pub const JSONC_LOCALS: &str = include_str!("../generated_queries/jsonc/locals.scm");
pub const JSONC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jsonc/highlights_crates_io.scm");
pub const JSONC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jsonc/injections_crates_io.scm");
pub const JSONC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jsonc/locals_crates_io.scm");

pub const JSONNET_HIGHLIGHTS: &str = include_str!("../generated_queries/jsonnet/highlights.scm");
pub const JSONNET_INJECTIONS: &str = include_str!("../generated_queries/jsonnet/injections.scm");
pub const JSONNET_LOCALS: &str = include_str!("../generated_queries/jsonnet/locals.scm");
pub const JSONNET_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/jsonnet/highlights_crates_io.scm");
pub const JSONNET_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/jsonnet/injections_crates_io.scm");
pub const JSONNET_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/jsonnet/locals_crates_io.scm");

pub const JULIA_HIGHLIGHTS: &str = include_str!("../generated_queries/julia/highlights.scm");
pub const JULIA_INJECTIONS: &str = include_str!("../generated_queries/julia/injections.scm");
pub const JULIA_LOCALS: &str = include_str!("../generated_queries/julia/locals.scm");
pub const JULIA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/julia/highlights_crates_io.scm");
pub const JULIA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/julia/injections_crates_io.scm");
pub const JULIA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/julia/locals_crates_io.scm");

pub const JUST_HIGHLIGHTS: &str = include_str!("../generated_queries/just/highlights.scm");
pub const JUST_INJECTIONS: &str = include_str!("../generated_queries/just/injections.scm");
pub const JUST_LOCALS: &str = include_str!("../generated_queries/just/locals.scm");
pub const JUST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/just/highlights_crates_io.scm");
pub const JUST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/just/injections_crates_io.scm");
pub const JUST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/just/locals_crates_io.scm");

pub const KCL_HIGHLIGHTS: &str = include_str!("../generated_queries/kcl/highlights.scm");
pub const KCL_INJECTIONS: &str = include_str!("../generated_queries/kcl/injections.scm");
pub const KCL_LOCALS: &str = include_str!("../generated_queries/kcl/locals.scm");
pub const KCL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/kcl/highlights_crates_io.scm");
pub const KCL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/kcl/injections_crates_io.scm");
pub const KCL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/kcl/locals_crates_io.scm");

pub const KCONFIG_HIGHLIGHTS: &str = include_str!("../generated_queries/kconfig/highlights.scm");
pub const KCONFIG_INJECTIONS: &str = include_str!("../generated_queries/kconfig/injections.scm");
pub const KCONFIG_LOCALS: &str = include_str!("../generated_queries/kconfig/locals.scm");
pub const KCONFIG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/kconfig/highlights_crates_io.scm");
pub const KCONFIG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/kconfig/injections_crates_io.scm");
pub const KCONFIG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/kconfig/locals_crates_io.scm");

pub const KDL_HIGHLIGHTS: &str = include_str!("../generated_queries/kdl/highlights.scm");
pub const KDL_INJECTIONS: &str = include_str!("../generated_queries/kdl/injections.scm");
pub const KDL_LOCALS: &str = include_str!("../generated_queries/kdl/locals.scm");
pub const KDL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/kdl/highlights_crates_io.scm");
pub const KDL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/kdl/injections_crates_io.scm");
pub const KDL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/kdl/locals_crates_io.scm");

pub const KITTY_HIGHLIGHTS: &str = include_str!("../generated_queries/kitty/highlights.scm");
pub const KITTY_INJECTIONS: &str = include_str!("../generated_queries/kitty/injections.scm");
pub const KITTY_LOCALS: &str = include_str!("../generated_queries/kitty/locals.scm");
pub const KITTY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/kitty/highlights_crates_io.scm");
pub const KITTY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/kitty/injections_crates_io.scm");
pub const KITTY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/kitty/locals_crates_io.scm");

pub const KOS_HIGHLIGHTS: &str = include_str!("../generated_queries/kos/highlights.scm");
pub const KOS_INJECTIONS: &str = include_str!("../generated_queries/kos/injections.scm");
pub const KOS_LOCALS: &str = include_str!("../generated_queries/kos/locals.scm");
pub const KOS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/kos/highlights_crates_io.scm");
pub const KOS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/kos/injections_crates_io.scm");
pub const KOS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/kos/locals_crates_io.scm");

pub const KOTLIN_HIGHLIGHTS: &str = include_str!("../generated_queries/kotlin/highlights.scm");
pub const KOTLIN_INJECTIONS: &str = include_str!("../generated_queries/kotlin/injections.scm");
pub const KOTLIN_LOCALS: &str = include_str!("../generated_queries/kotlin/locals.scm");
pub const KOTLIN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/kotlin/highlights_crates_io.scm");
pub const KOTLIN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/kotlin/injections_crates_io.scm");
pub const KOTLIN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/kotlin/locals_crates_io.scm");

pub const KOTO_HIGHLIGHTS: &str = include_str!("../generated_queries/koto/highlights.scm");
pub const KOTO_INJECTIONS: &str = include_str!("../generated_queries/koto/injections.scm");
pub const KOTO_LOCALS: &str = include_str!("../generated_queries/koto/locals.scm");
pub const KOTO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/koto/highlights_crates_io.scm");
pub const KOTO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/koto/injections_crates_io.scm");
pub const KOTO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/koto/locals_crates_io.scm");

pub const KUSTO_HIGHLIGHTS: &str = include_str!("../generated_queries/kusto/highlights.scm");
pub const KUSTO_INJECTIONS: &str = include_str!("../generated_queries/kusto/injections.scm");
pub const KUSTO_LOCALS: &str = include_str!("../generated_queries/kusto/locals.scm");
pub const KUSTO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/kusto/highlights_crates_io.scm");
pub const KUSTO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/kusto/injections_crates_io.scm");
pub const KUSTO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/kusto/locals_crates_io.scm");

pub const LALRPOP_HIGHLIGHTS: &str = include_str!("../generated_queries/lalrpop/highlights.scm");
pub const LALRPOP_INJECTIONS: &str = include_str!("../generated_queries/lalrpop/injections.scm");
pub const LALRPOP_LOCALS: &str = include_str!("../generated_queries/lalrpop/locals.scm");
pub const LALRPOP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/lalrpop/highlights_crates_io.scm");
pub const LALRPOP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/lalrpop/injections_crates_io.scm");
pub const LALRPOP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/lalrpop/locals_crates_io.scm");

pub const LATEX_HIGHLIGHTS: &str = include_str!("../generated_queries/latex/highlights.scm");
pub const LATEX_INJECTIONS: &str = include_str!("../generated_queries/latex/injections.scm");
pub const LATEX_LOCALS: &str = include_str!("../generated_queries/latex/locals.scm");
pub const LATEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/latex/highlights_crates_io.scm");
pub const LATEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/latex/injections_crates_io.scm");
pub const LATEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/latex/locals_crates_io.scm");

pub const LD_HIGHLIGHTS: &str = include_str!("../generated_queries/ld/highlights.scm");
pub const LD_INJECTIONS: &str = include_str!("../generated_queries/ld/injections.scm");
pub const LD_LOCALS: &str = include_str!("../generated_queries/ld/locals.scm");
pub const LD_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ld/highlights_crates_io.scm");
pub const LD_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ld/injections_crates_io.scm");
pub const LD_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ld/locals_crates_io.scm");

pub const LEDGER_HIGHLIGHTS: &str = include_str!("../generated_queries/ledger/highlights.scm");
pub const LEDGER_INJECTIONS: &str = include_str!("../generated_queries/ledger/injections.scm");
pub const LEDGER_LOCALS: &str = include_str!("../generated_queries/ledger/locals.scm");
pub const LEDGER_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ledger/highlights_crates_io.scm");
pub const LEDGER_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ledger/injections_crates_io.scm");
pub const LEDGER_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ledger/locals_crates_io.scm");

pub const LEO_HIGHLIGHTS: &str = include_str!("../generated_queries/leo/highlights.scm");
pub const LEO_INJECTIONS: &str = include_str!("../generated_queries/leo/injections.scm");
pub const LEO_LOCALS: &str = include_str!("../generated_queries/leo/locals.scm");
pub const LEO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/leo/highlights_crates_io.scm");
pub const LEO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/leo/injections_crates_io.scm");
pub const LEO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/leo/locals_crates_io.scm");

pub const LIQUID_HIGHLIGHTS: &str = include_str!("../generated_queries/liquid/highlights.scm");
pub const LIQUID_INJECTIONS: &str = include_str!("../generated_queries/liquid/injections.scm");
pub const LIQUID_LOCALS: &str = include_str!("../generated_queries/liquid/locals.scm");
pub const LIQUID_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/liquid/highlights_crates_io.scm");
pub const LIQUID_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/liquid/injections_crates_io.scm");
pub const LIQUID_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/liquid/locals_crates_io.scm");

pub const LIQUIDSOAP_HIGHLIGHTS: &str = include_str!("../generated_queries/liquidsoap/highlights.scm");
pub const LIQUIDSOAP_INJECTIONS: &str = include_str!("../generated_queries/liquidsoap/injections.scm");
pub const LIQUIDSOAP_LOCALS: &str = include_str!("../generated_queries/liquidsoap/locals.scm");
pub const LIQUIDSOAP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/liquidsoap/highlights_crates_io.scm");
pub const LIQUIDSOAP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/liquidsoap/injections_crates_io.scm");
pub const LIQUIDSOAP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/liquidsoap/locals_crates_io.scm");

pub const LISP_HIGHLIGHTS: &str = include_str!("../generated_queries/lisp/highlights.scm");
pub const LISP_INJECTIONS: &str = include_str!("../generated_queries/lisp/injections.scm");
pub const LISP_LOCALS: &str = include_str!("../generated_queries/lisp/locals.scm");
pub const LISP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/lisp/highlights_crates_io.scm");
pub const LISP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/lisp/injections_crates_io.scm");
pub const LISP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/lisp/locals_crates_io.scm");

pub const LLVM_HIGHLIGHTS: &str = include_str!("../generated_queries/llvm/highlights.scm");
pub const LLVM_INJECTIONS: &str = include_str!("../generated_queries/llvm/injections.scm");
pub const LLVM_LOCALS: &str = include_str!("../generated_queries/llvm/locals.scm");
pub const LLVM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/llvm/highlights_crates_io.scm");
pub const LLVM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/llvm/injections_crates_io.scm");
pub const LLVM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/llvm/locals_crates_io.scm");

pub const LUA_HIGHLIGHTS: &str = include_str!("../generated_queries/lua/highlights.scm");
pub const LUA_INJECTIONS: &str = include_str!("../generated_queries/lua/injections.scm");
pub const LUA_LOCALS: &str = include_str!("../generated_queries/lua/locals.scm");
pub const LUA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/lua/highlights_crates_io.scm");
pub const LUA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/lua/injections_crates_io.scm");
pub const LUA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/lua/locals_crates_io.scm");

pub const LUADOC_HIGHLIGHTS: &str = include_str!("../generated_queries/luadoc/highlights.scm");
pub const LUADOC_INJECTIONS: &str = include_str!("../generated_queries/luadoc/injections.scm");
pub const LUADOC_LOCALS: &str = include_str!("../generated_queries/luadoc/locals.scm");
pub const LUADOC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/luadoc/highlights_crates_io.scm");
pub const LUADOC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/luadoc/injections_crates_io.scm");
pub const LUADOC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/luadoc/locals_crates_io.scm");

pub const LUAP_HIGHLIGHTS: &str = include_str!("../generated_queries/luap/highlights.scm");
pub const LUAP_INJECTIONS: &str = include_str!("../generated_queries/luap/injections.scm");
pub const LUAP_LOCALS: &str = include_str!("../generated_queries/luap/locals.scm");
pub const LUAP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/luap/highlights_crates_io.scm");
pub const LUAP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/luap/injections_crates_io.scm");
pub const LUAP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/luap/locals_crates_io.scm");

pub const LUAU_HIGHLIGHTS: &str = include_str!("../generated_queries/luau/highlights.scm");
pub const LUAU_INJECTIONS: &str = include_str!("../generated_queries/luau/injections.scm");
pub const LUAU_LOCALS: &str = include_str!("../generated_queries/luau/locals.scm");
pub const LUAU_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/luau/highlights_crates_io.scm");
pub const LUAU_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/luau/injections_crates_io.scm");
pub const LUAU_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/luau/locals_crates_io.scm");

pub const MAGIK_HIGHLIGHTS: &str = include_str!("../generated_queries/magik/highlights.scm");
pub const MAGIK_INJECTIONS: &str = include_str!("../generated_queries/magik/injections.scm");
pub const MAGIK_LOCALS: &str = include_str!("../generated_queries/magik/locals.scm");
pub const MAGIK_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/magik/highlights_crates_io.scm");
pub const MAGIK_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/magik/injections_crates_io.scm");
pub const MAGIK_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/magik/locals_crates_io.scm");

pub const MAKE_HIGHLIGHTS: &str = include_str!("../generated_queries/make/highlights.scm");
pub const MAKE_INJECTIONS: &str = include_str!("../generated_queries/make/injections.scm");
pub const MAKE_LOCALS: &str = include_str!("../generated_queries/make/locals.scm");
pub const MAKE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/make/highlights_crates_io.scm");
pub const MAKE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/make/injections_crates_io.scm");
pub const MAKE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/make/locals_crates_io.scm");

pub const MARKDOWN_HIGHLIGHTS: &str = include_str!("../generated_queries/markdown/highlights.scm");
pub const MARKDOWN_INJECTIONS: &str = include_str!("../generated_queries/markdown/injections.scm");
pub const MARKDOWN_LOCALS: &str = include_str!("../generated_queries/markdown/locals.scm");
pub const MARKDOWN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/markdown/highlights_crates_io.scm");
pub const MARKDOWN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/markdown/injections_crates_io.scm");
pub const MARKDOWN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/markdown/locals_crates_io.scm");

pub const MARKDOWN_INLINE_HIGHLIGHTS: &str = include_str!("../generated_queries/markdown_inline/highlights.scm");
pub const MARKDOWN_INLINE_INJECTIONS: &str = include_str!("../generated_queries/markdown_inline/injections.scm");
pub const MARKDOWN_INLINE_LOCALS: &str = include_str!("../generated_queries/markdown_inline/locals.scm");
pub const MARKDOWN_INLINE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/markdown_inline/highlights_crates_io.scm");
pub const MARKDOWN_INLINE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/markdown_inline/injections_crates_io.scm");
pub const MARKDOWN_INLINE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/markdown_inline/locals_crates_io.scm");

pub const MATLAB_HIGHLIGHTS: &str = include_str!("../generated_queries/matlab/highlights.scm");
pub const MATLAB_INJECTIONS: &str = include_str!("../generated_queries/matlab/injections.scm");
pub const MATLAB_LOCALS: &str = include_str!("../generated_queries/matlab/locals.scm");
pub const MATLAB_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/matlab/highlights_crates_io.scm");
pub const MATLAB_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/matlab/injections_crates_io.scm");
pub const MATLAB_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/matlab/locals_crates_io.scm");

pub const MENHIR_HIGHLIGHTS: &str = include_str!("../generated_queries/menhir/highlights.scm");
pub const MENHIR_INJECTIONS: &str = include_str!("../generated_queries/menhir/injections.scm");
pub const MENHIR_LOCALS: &str = include_str!("../generated_queries/menhir/locals.scm");
pub const MENHIR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/menhir/highlights_crates_io.scm");
pub const MENHIR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/menhir/injections_crates_io.scm");
pub const MENHIR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/menhir/locals_crates_io.scm");

pub const MERMAID_HIGHLIGHTS: &str = include_str!("../generated_queries/mermaid/highlights.scm");
pub const MERMAID_INJECTIONS: &str = include_str!("../generated_queries/mermaid/injections.scm");
pub const MERMAID_LOCALS: &str = include_str!("../generated_queries/mermaid/locals.scm");
pub const MERMAID_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/mermaid/highlights_crates_io.scm");
pub const MERMAID_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/mermaid/injections_crates_io.scm");
pub const MERMAID_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/mermaid/locals_crates_io.scm");

pub const MESON_HIGHLIGHTS: &str = include_str!("../generated_queries/meson/highlights.scm");
pub const MESON_INJECTIONS: &str = include_str!("../generated_queries/meson/injections.scm");
pub const MESON_LOCALS: &str = include_str!("../generated_queries/meson/locals.scm");
pub const MESON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/meson/highlights_crates_io.scm");
pub const MESON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/meson/injections_crates_io.scm");
pub const MESON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/meson/locals_crates_io.scm");

pub const MLIR_HIGHLIGHTS: &str = include_str!("../generated_queries/mlir/highlights.scm");
pub const MLIR_INJECTIONS: &str = include_str!("../generated_queries/mlir/injections.scm");
pub const MLIR_LOCALS: &str = include_str!("../generated_queries/mlir/locals.scm");
pub const MLIR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/mlir/highlights_crates_io.scm");
pub const MLIR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/mlir/injections_crates_io.scm");
pub const MLIR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/mlir/locals_crates_io.scm");

pub const MUTTRC_HIGHLIGHTS: &str = include_str!("../generated_queries/muttrc/highlights.scm");
pub const MUTTRC_INJECTIONS: &str = include_str!("../generated_queries/muttrc/injections.scm");
pub const MUTTRC_LOCALS: &str = include_str!("../generated_queries/muttrc/locals.scm");
pub const MUTTRC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/muttrc/highlights_crates_io.scm");
pub const MUTTRC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/muttrc/injections_crates_io.scm");
pub const MUTTRC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/muttrc/locals_crates_io.scm");

pub const NASM_HIGHLIGHTS: &str = include_str!("../generated_queries/nasm/highlights.scm");
pub const NASM_INJECTIONS: &str = include_str!("../generated_queries/nasm/injections.scm");
pub const NASM_LOCALS: &str = include_str!("../generated_queries/nasm/locals.scm");
pub const NASM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nasm/highlights_crates_io.scm");
pub const NASM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nasm/injections_crates_io.scm");
pub const NASM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nasm/locals_crates_io.scm");

pub const NGINX_HIGHLIGHTS: &str = include_str!("../generated_queries/nginx/highlights.scm");
pub const NGINX_INJECTIONS: &str = include_str!("../generated_queries/nginx/injections.scm");
pub const NGINX_LOCALS: &str = include_str!("../generated_queries/nginx/locals.scm");
pub const NGINX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nginx/highlights_crates_io.scm");
pub const NGINX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nginx/injections_crates_io.scm");
pub const NGINX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nginx/locals_crates_io.scm");

pub const NICKEL_HIGHLIGHTS: &str = include_str!("../generated_queries/nickel/highlights.scm");
pub const NICKEL_INJECTIONS: &str = include_str!("../generated_queries/nickel/injections.scm");
pub const NICKEL_LOCALS: &str = include_str!("../generated_queries/nickel/locals.scm");
pub const NICKEL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nickel/highlights_crates_io.scm");
pub const NICKEL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nickel/injections_crates_io.scm");
pub const NICKEL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nickel/locals_crates_io.scm");

pub const NIM_HIGHLIGHTS: &str = include_str!("../generated_queries/nim/highlights.scm");
pub const NIM_INJECTIONS: &str = include_str!("../generated_queries/nim/injections.scm");
pub const NIM_LOCALS: &str = include_str!("../generated_queries/nim/locals.scm");
pub const NIM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nim/highlights_crates_io.scm");
pub const NIM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nim/injections_crates_io.scm");
pub const NIM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nim/locals_crates_io.scm");

pub const NIM_FORMAT_STRING_HIGHLIGHTS: &str = include_str!("../generated_queries/nim_format_string/highlights.scm");
pub const NIM_FORMAT_STRING_INJECTIONS: &str = include_str!("../generated_queries/nim_format_string/injections.scm");
pub const NIM_FORMAT_STRING_LOCALS: &str = include_str!("../generated_queries/nim_format_string/locals.scm");
pub const NIM_FORMAT_STRING_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nim_format_string/highlights_crates_io.scm");
pub const NIM_FORMAT_STRING_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nim_format_string/injections_crates_io.scm");
pub const NIM_FORMAT_STRING_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nim_format_string/locals_crates_io.scm");

pub const NINJA_HIGHLIGHTS: &str = include_str!("../generated_queries/ninja/highlights.scm");
pub const NINJA_INJECTIONS: &str = include_str!("../generated_queries/ninja/injections.scm");
pub const NINJA_LOCALS: &str = include_str!("../generated_queries/ninja/locals.scm");
pub const NINJA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ninja/highlights_crates_io.scm");
pub const NINJA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ninja/injections_crates_io.scm");
pub const NINJA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ninja/locals_crates_io.scm");

pub const NIX_HIGHLIGHTS: &str = include_str!("../generated_queries/nix/highlights.scm");
pub const NIX_INJECTIONS: &str = include_str!("../generated_queries/nix/injections.scm");
pub const NIX_LOCALS: &str = include_str!("../generated_queries/nix/locals.scm");
pub const NIX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nix/highlights_crates_io.scm");
pub const NIX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nix/injections_crates_io.scm");
pub const NIX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nix/locals_crates_io.scm");

pub const NQC_HIGHLIGHTS: &str = include_str!("../generated_queries/nqc/highlights.scm");
pub const NQC_INJECTIONS: &str = include_str!("../generated_queries/nqc/injections.scm");
pub const NQC_LOCALS: &str = include_str!("../generated_queries/nqc/locals.scm");
pub const NQC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nqc/highlights_crates_io.scm");
pub const NQC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nqc/injections_crates_io.scm");
pub const NQC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nqc/locals_crates_io.scm");

pub const NU_HIGHLIGHTS: &str = include_str!("../generated_queries/nu/highlights.scm");
pub const NU_INJECTIONS: &str = include_str!("../generated_queries/nu/injections.scm");
pub const NU_LOCALS: &str = include_str!("../generated_queries/nu/locals.scm");
pub const NU_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/nu/highlights_crates_io.scm");
pub const NU_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/nu/injections_crates_io.scm");
pub const NU_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/nu/locals_crates_io.scm");

pub const OBJC_HIGHLIGHTS: &str = include_str!("../generated_queries/objc/highlights.scm");
pub const OBJC_INJECTIONS: &str = include_str!("../generated_queries/objc/injections.scm");
pub const OBJC_LOCALS: &str = include_str!("../generated_queries/objc/locals.scm");
pub const OBJC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/objc/highlights_crates_io.scm");
pub const OBJC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/objc/injections_crates_io.scm");
pub const OBJC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/objc/locals_crates_io.scm");

pub const OBJDUMP_HIGHLIGHTS: &str = include_str!("../generated_queries/objdump/highlights.scm");
pub const OBJDUMP_INJECTIONS: &str = include_str!("../generated_queries/objdump/injections.scm");
pub const OBJDUMP_LOCALS: &str = include_str!("../generated_queries/objdump/locals.scm");
pub const OBJDUMP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/objdump/highlights_crates_io.scm");
pub const OBJDUMP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/objdump/injections_crates_io.scm");
pub const OBJDUMP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/objdump/locals_crates_io.scm");

pub const OCAML_HIGHLIGHTS: &str = include_str!("../generated_queries/ocaml/highlights.scm");
pub const OCAML_INJECTIONS: &str = include_str!("../generated_queries/ocaml/injections.scm");
pub const OCAML_LOCALS: &str = include_str!("../generated_queries/ocaml/locals.scm");
pub const OCAML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ocaml/highlights_crates_io.scm");
pub const OCAML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ocaml/injections_crates_io.scm");
pub const OCAML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ocaml/locals_crates_io.scm");

pub const OCAML_INTERFACE_HIGHLIGHTS: &str = include_str!("../generated_queries/ocaml_interface/highlights.scm");
pub const OCAML_INTERFACE_INJECTIONS: &str = include_str!("../generated_queries/ocaml_interface/injections.scm");
pub const OCAML_INTERFACE_LOCALS: &str = include_str!("../generated_queries/ocaml_interface/locals.scm");
pub const OCAML_INTERFACE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ocaml_interface/highlights_crates_io.scm");
pub const OCAML_INTERFACE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ocaml_interface/injections_crates_io.scm");
pub const OCAML_INTERFACE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ocaml_interface/locals_crates_io.scm");

pub const OCAMLINTERFACE_HIGHLIGHTS: &str = include_str!("../generated_queries/ocamlinterface/highlights.scm");
pub const OCAMLINTERFACE_INJECTIONS: &str = include_str!("../generated_queries/ocamlinterface/injections.scm");
pub const OCAMLINTERFACE_LOCALS: &str = include_str!("../generated_queries/ocamlinterface/locals.scm");
pub const OCAMLINTERFACE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ocamlinterface/highlights_crates_io.scm");
pub const OCAMLINTERFACE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ocamlinterface/injections_crates_io.scm");
pub const OCAMLINTERFACE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ocamlinterface/locals_crates_io.scm");

pub const OCAMLLEX_HIGHLIGHTS: &str = include_str!("../generated_queries/ocamllex/highlights.scm");
pub const OCAMLLEX_INJECTIONS: &str = include_str!("../generated_queries/ocamllex/injections.scm");
pub const OCAMLLEX_LOCALS: &str = include_str!("../generated_queries/ocamllex/locals.scm");
pub const OCAMLLEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ocamllex/highlights_crates_io.scm");
pub const OCAMLLEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ocamllex/injections_crates_io.scm");
pub const OCAMLLEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ocamllex/locals_crates_io.scm");

pub const ODIN_HIGHLIGHTS: &str = include_str!("../generated_queries/odin/highlights.scm");
pub const ODIN_INJECTIONS: &str = include_str!("../generated_queries/odin/injections.scm");
pub const ODIN_LOCALS: &str = include_str!("../generated_queries/odin/locals.scm");
pub const ODIN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/odin/highlights_crates_io.scm");
pub const ODIN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/odin/injections_crates_io.scm");
pub const ODIN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/odin/locals_crates_io.scm");

pub const PASCAL_HIGHLIGHTS: &str = include_str!("../generated_queries/pascal/highlights.scm");
pub const PASCAL_INJECTIONS: &str = include_str!("../generated_queries/pascal/injections.scm");
pub const PASCAL_LOCALS: &str = include_str!("../generated_queries/pascal/locals.scm");
pub const PASCAL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pascal/highlights_crates_io.scm");
pub const PASCAL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pascal/injections_crates_io.scm");
pub const PASCAL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pascal/locals_crates_io.scm");

pub const PASSWD_HIGHLIGHTS: &str = include_str!("../generated_queries/passwd/highlights.scm");
pub const PASSWD_INJECTIONS: &str = include_str!("../generated_queries/passwd/injections.scm");
pub const PASSWD_LOCALS: &str = include_str!("../generated_queries/passwd/locals.scm");
pub const PASSWD_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/passwd/highlights_crates_io.scm");
pub const PASSWD_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/passwd/injections_crates_io.scm");
pub const PASSWD_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/passwd/locals_crates_io.scm");

pub const PBTXT_HIGHLIGHTS: &str = include_str!("../generated_queries/pbtxt/highlights.scm");
pub const PBTXT_INJECTIONS: &str = include_str!("../generated_queries/pbtxt/injections.scm");
pub const PBTXT_LOCALS: &str = include_str!("../generated_queries/pbtxt/locals.scm");
pub const PBTXT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pbtxt/highlights_crates_io.scm");
pub const PBTXT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pbtxt/injections_crates_io.scm");
pub const PBTXT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pbtxt/locals_crates_io.scm");

pub const PEM_HIGHLIGHTS: &str = include_str!("../generated_queries/pem/highlights.scm");
pub const PEM_INJECTIONS: &str = include_str!("../generated_queries/pem/injections.scm");
pub const PEM_LOCALS: &str = include_str!("../generated_queries/pem/locals.scm");
pub const PEM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pem/highlights_crates_io.scm");
pub const PEM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pem/injections_crates_io.scm");
pub const PEM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pem/locals_crates_io.scm");

pub const PERL_HIGHLIGHTS: &str = include_str!("../generated_queries/perl/highlights.scm");
pub const PERL_INJECTIONS: &str = include_str!("../generated_queries/perl/injections.scm");
pub const PERL_LOCALS: &str = include_str!("../generated_queries/perl/locals.scm");
pub const PERL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/perl/highlights_crates_io.scm");
pub const PERL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/perl/injections_crates_io.scm");
pub const PERL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/perl/locals_crates_io.scm");

pub const PGN_HIGHLIGHTS: &str = include_str!("../generated_queries/pgn/highlights.scm");
pub const PGN_INJECTIONS: &str = include_str!("../generated_queries/pgn/injections.scm");
pub const PGN_LOCALS: &str = include_str!("../generated_queries/pgn/locals.scm");
pub const PGN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pgn/highlights_crates_io.scm");
pub const PGN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pgn/injections_crates_io.scm");
pub const PGN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pgn/locals_crates_io.scm");

pub const PHP_HIGHLIGHTS: &str = include_str!("../generated_queries/php/highlights.scm");
pub const PHP_INJECTIONS: &str = include_str!("../generated_queries/php/injections.scm");
pub const PHP_LOCALS: &str = include_str!("../generated_queries/php/locals.scm");
pub const PHP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/php/highlights_crates_io.scm");
pub const PHP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/php/injections_crates_io.scm");
pub const PHP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/php/locals_crates_io.scm");

pub const PHP_ONLY_HIGHLIGHTS: &str = include_str!("../generated_queries/php_only/highlights.scm");
pub const PHP_ONLY_INJECTIONS: &str = include_str!("../generated_queries/php_only/injections.scm");
pub const PHP_ONLY_LOCALS: &str = include_str!("../generated_queries/php_only/locals.scm");
pub const PHP_ONLY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/php_only/highlights_crates_io.scm");
pub const PHP_ONLY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/php_only/injections_crates_io.scm");
pub const PHP_ONLY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/php_only/locals_crates_io.scm");

pub const PHPDOC_HIGHLIGHTS: &str = include_str!("../generated_queries/phpdoc/highlights.scm");
pub const PHPDOC_INJECTIONS: &str = include_str!("../generated_queries/phpdoc/injections.scm");
pub const PHPDOC_LOCALS: &str = include_str!("../generated_queries/phpdoc/locals.scm");
pub const PHPDOC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/phpdoc/highlights_crates_io.scm");
pub const PHPDOC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/phpdoc/injections_crates_io.scm");
pub const PHPDOC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/phpdoc/locals_crates_io.scm");

pub const PIOASM_HIGHLIGHTS: &str = include_str!("../generated_queries/pioasm/highlights.scm");
pub const PIOASM_INJECTIONS: &str = include_str!("../generated_queries/pioasm/injections.scm");
pub const PIOASM_LOCALS: &str = include_str!("../generated_queries/pioasm/locals.scm");
pub const PIOASM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pioasm/highlights_crates_io.scm");
pub const PIOASM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pioasm/injections_crates_io.scm");
pub const PIOASM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pioasm/locals_crates_io.scm");

pub const PKL_HIGHLIGHTS: &str = include_str!("../generated_queries/pkl/highlights.scm");
pub const PKL_INJECTIONS: &str = include_str!("../generated_queries/pkl/injections.scm");
pub const PKL_LOCALS: &str = include_str!("../generated_queries/pkl/locals.scm");
pub const PKL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pkl/highlights_crates_io.scm");
pub const PKL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pkl/injections_crates_io.scm");
pub const PKL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pkl/locals_crates_io.scm");

pub const PO_HIGHLIGHTS: &str = include_str!("../generated_queries/po/highlights.scm");
pub const PO_INJECTIONS: &str = include_str!("../generated_queries/po/injections.scm");
pub const PO_LOCALS: &str = include_str!("../generated_queries/po/locals.scm");
pub const PO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/po/highlights_crates_io.scm");
pub const PO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/po/injections_crates_io.scm");
pub const PO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/po/locals_crates_io.scm");

pub const POD_HIGHLIGHTS: &str = include_str!("../generated_queries/pod/highlights.scm");
pub const POD_INJECTIONS: &str = include_str!("../generated_queries/pod/injections.scm");
pub const POD_LOCALS: &str = include_str!("../generated_queries/pod/locals.scm");
pub const POD_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pod/highlights_crates_io.scm");
pub const POD_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pod/injections_crates_io.scm");
pub const POD_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pod/locals_crates_io.scm");

pub const POEFILTER_HIGHLIGHTS: &str = include_str!("../generated_queries/poefilter/highlights.scm");
pub const POEFILTER_INJECTIONS: &str = include_str!("../generated_queries/poefilter/injections.scm");
pub const POEFILTER_LOCALS: &str = include_str!("../generated_queries/poefilter/locals.scm");
pub const POEFILTER_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/poefilter/highlights_crates_io.scm");
pub const POEFILTER_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/poefilter/injections_crates_io.scm");
pub const POEFILTER_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/poefilter/locals_crates_io.scm");

pub const PONY_HIGHLIGHTS: &str = include_str!("../generated_queries/pony/highlights.scm");
pub const PONY_INJECTIONS: &str = include_str!("../generated_queries/pony/injections.scm");
pub const PONY_LOCALS: &str = include_str!("../generated_queries/pony/locals.scm");
pub const PONY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pony/highlights_crates_io.scm");
pub const PONY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pony/injections_crates_io.scm");
pub const PONY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pony/locals_crates_io.scm");

pub const PRINTF_HIGHLIGHTS: &str = include_str!("../generated_queries/printf/highlights.scm");
pub const PRINTF_INJECTIONS: &str = include_str!("../generated_queries/printf/injections.scm");
pub const PRINTF_LOCALS: &str = include_str!("../generated_queries/printf/locals.scm");
pub const PRINTF_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/printf/highlights_crates_io.scm");
pub const PRINTF_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/printf/injections_crates_io.scm");
pub const PRINTF_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/printf/locals_crates_io.scm");

pub const PRISMA_HIGHLIGHTS: &str = include_str!("../generated_queries/prisma/highlights.scm");
pub const PRISMA_INJECTIONS: &str = include_str!("../generated_queries/prisma/injections.scm");
pub const PRISMA_LOCALS: &str = include_str!("../generated_queries/prisma/locals.scm");
pub const PRISMA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/prisma/highlights_crates_io.scm");
pub const PRISMA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/prisma/injections_crates_io.scm");
pub const PRISMA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/prisma/locals_crates_io.scm");

pub const PROBLOG_HIGHLIGHTS: &str = include_str!("../generated_queries/problog/highlights.scm");
pub const PROBLOG_INJECTIONS: &str = include_str!("../generated_queries/problog/injections.scm");
pub const PROBLOG_LOCALS: &str = include_str!("../generated_queries/problog/locals.scm");
pub const PROBLOG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/problog/highlights_crates_io.scm");
pub const PROBLOG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/problog/injections_crates_io.scm");
pub const PROBLOG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/problog/locals_crates_io.scm");

pub const PROLOG_HIGHLIGHTS: &str = include_str!("../generated_queries/prolog/highlights.scm");
pub const PROLOG_INJECTIONS: &str = include_str!("../generated_queries/prolog/injections.scm");
pub const PROLOG_LOCALS: &str = include_str!("../generated_queries/prolog/locals.scm");
pub const PROLOG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/prolog/highlights_crates_io.scm");
pub const PROLOG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/prolog/injections_crates_io.scm");
pub const PROLOG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/prolog/locals_crates_io.scm");

pub const PROMQL_HIGHLIGHTS: &str = include_str!("../generated_queries/promql/highlights.scm");
pub const PROMQL_INJECTIONS: &str = include_str!("../generated_queries/promql/injections.scm");
pub const PROMQL_LOCALS: &str = include_str!("../generated_queries/promql/locals.scm");
pub const PROMQL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/promql/highlights_crates_io.scm");
pub const PROMQL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/promql/injections_crates_io.scm");
pub const PROMQL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/promql/locals_crates_io.scm");

pub const PROTO_HIGHLIGHTS: &str = include_str!("../generated_queries/proto/highlights.scm");
pub const PROTO_INJECTIONS: &str = include_str!("../generated_queries/proto/injections.scm");
pub const PROTO_LOCALS: &str = include_str!("../generated_queries/proto/locals.scm");
pub const PROTO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/proto/highlights_crates_io.scm");
pub const PROTO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/proto/injections_crates_io.scm");
pub const PROTO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/proto/locals_crates_io.scm");

pub const PRQL_HIGHLIGHTS: &str = include_str!("../generated_queries/prql/highlights.scm");
pub const PRQL_INJECTIONS: &str = include_str!("../generated_queries/prql/injections.scm");
pub const PRQL_LOCALS: &str = include_str!("../generated_queries/prql/locals.scm");
pub const PRQL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/prql/highlights_crates_io.scm");
pub const PRQL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/prql/injections_crates_io.scm");
pub const PRQL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/prql/locals_crates_io.scm");

pub const PS1_HIGHLIGHTS: &str = include_str!("../generated_queries/ps1/highlights.scm");
pub const PS1_INJECTIONS: &str = include_str!("../generated_queries/ps1/injections.scm");
pub const PS1_LOCALS: &str = include_str!("../generated_queries/ps1/locals.scm");
pub const PS1_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ps1/highlights_crates_io.scm");
pub const PS1_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ps1/injections_crates_io.scm");
pub const PS1_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ps1/locals_crates_io.scm");

pub const PSV_HIGHLIGHTS: &str = include_str!("../generated_queries/psv/highlights.scm");
pub const PSV_INJECTIONS: &str = include_str!("../generated_queries/psv/injections.scm");
pub const PSV_LOCALS: &str = include_str!("../generated_queries/psv/locals.scm");
pub const PSV_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/psv/highlights_crates_io.scm");
pub const PSV_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/psv/injections_crates_io.scm");
pub const PSV_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/psv/locals_crates_io.scm");

pub const PUG_HIGHLIGHTS: &str = include_str!("../generated_queries/pug/highlights.scm");
pub const PUG_INJECTIONS: &str = include_str!("../generated_queries/pug/injections.scm");
pub const PUG_LOCALS: &str = include_str!("../generated_queries/pug/locals.scm");
pub const PUG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pug/highlights_crates_io.scm");
pub const PUG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pug/injections_crates_io.scm");
pub const PUG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pug/locals_crates_io.scm");

pub const PUPPET_HIGHLIGHTS: &str = include_str!("../generated_queries/puppet/highlights.scm");
pub const PUPPET_INJECTIONS: &str = include_str!("../generated_queries/puppet/injections.scm");
pub const PUPPET_LOCALS: &str = include_str!("../generated_queries/puppet/locals.scm");
pub const PUPPET_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/puppet/highlights_crates_io.scm");
pub const PUPPET_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/puppet/injections_crates_io.scm");
pub const PUPPET_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/puppet/locals_crates_io.scm");

pub const PURESCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/purescript/highlights.scm");
pub const PURESCRIPT_INJECTIONS: &str = include_str!("../generated_queries/purescript/injections.scm");
pub const PURESCRIPT_LOCALS: &str = include_str!("../generated_queries/purescript/locals.scm");
pub const PURESCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/purescript/highlights_crates_io.scm");
pub const PURESCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/purescript/injections_crates_io.scm");
pub const PURESCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/purescript/locals_crates_io.scm");

pub const PYMANIFEST_HIGHLIGHTS: &str = include_str!("../generated_queries/pymanifest/highlights.scm");
pub const PYMANIFEST_INJECTIONS: &str = include_str!("../generated_queries/pymanifest/injections.scm");
pub const PYMANIFEST_LOCALS: &str = include_str!("../generated_queries/pymanifest/locals.scm");
pub const PYMANIFEST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/pymanifest/highlights_crates_io.scm");
pub const PYMANIFEST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/pymanifest/injections_crates_io.scm");
pub const PYMANIFEST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/pymanifest/locals_crates_io.scm");

pub const PYTHON_HIGHLIGHTS: &str = include_str!("../generated_queries/python/highlights.scm");
pub const PYTHON_INJECTIONS: &str = include_str!("../generated_queries/python/injections.scm");
pub const PYTHON_LOCALS: &str = include_str!("../generated_queries/python/locals.scm");
pub const PYTHON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/python/highlights_crates_io.scm");
pub const PYTHON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/python/injections_crates_io.scm");
pub const PYTHON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/python/locals_crates_io.scm");

pub const QL_HIGHLIGHTS: &str = include_str!("../generated_queries/ql/highlights.scm");
pub const QL_INJECTIONS: &str = include_str!("../generated_queries/ql/injections.scm");
pub const QL_LOCALS: &str = include_str!("../generated_queries/ql/locals.scm");
pub const QL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ql/highlights_crates_io.scm");
pub const QL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ql/injections_crates_io.scm");
pub const QL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ql/locals_crates_io.scm");

pub const QML_HIGHLIGHTS: &str = include_str!("../generated_queries/qml/highlights.scm");
pub const QML_INJECTIONS: &str = include_str!("../generated_queries/qml/injections.scm");
pub const QML_LOCALS: &str = include_str!("../generated_queries/qml/locals.scm");
pub const QML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/qml/highlights_crates_io.scm");
pub const QML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/qml/injections_crates_io.scm");
pub const QML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/qml/locals_crates_io.scm");

pub const QMLDIR_HIGHLIGHTS: &str = include_str!("../generated_queries/qmldir/highlights.scm");
pub const QMLDIR_INJECTIONS: &str = include_str!("../generated_queries/qmldir/injections.scm");
pub const QMLDIR_LOCALS: &str = include_str!("../generated_queries/qmldir/locals.scm");
pub const QMLDIR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/qmldir/highlights_crates_io.scm");
pub const QMLDIR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/qmldir/injections_crates_io.scm");
pub const QMLDIR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/qmldir/locals_crates_io.scm");

pub const QUERY_HIGHLIGHTS: &str = include_str!("../generated_queries/query/highlights.scm");
pub const QUERY_INJECTIONS: &str = include_str!("../generated_queries/query/injections.scm");
pub const QUERY_LOCALS: &str = include_str!("../generated_queries/query/locals.scm");
pub const QUERY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/query/highlights_crates_io.scm");
pub const QUERY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/query/injections_crates_io.scm");
pub const QUERY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/query/locals_crates_io.scm");

pub const R_HIGHLIGHTS: &str = include_str!("../generated_queries/r/highlights.scm");
pub const R_INJECTIONS: &str = include_str!("../generated_queries/r/injections.scm");
pub const R_LOCALS: &str = include_str!("../generated_queries/r/locals.scm");
pub const R_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/r/highlights_crates_io.scm");
pub const R_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/r/injections_crates_io.scm");
pub const R_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/r/locals_crates_io.scm");

pub const RACKET_HIGHLIGHTS: &str = include_str!("../generated_queries/racket/highlights.scm");
pub const RACKET_INJECTIONS: &str = include_str!("../generated_queries/racket/injections.scm");
pub const RACKET_LOCALS: &str = include_str!("../generated_queries/racket/locals.scm");
pub const RACKET_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/racket/highlights_crates_io.scm");
pub const RACKET_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/racket/injections_crates_io.scm");
pub const RACKET_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/racket/locals_crates_io.scm");

pub const RALPH_HIGHLIGHTS: &str = include_str!("../generated_queries/ralph/highlights.scm");
pub const RALPH_INJECTIONS: &str = include_str!("../generated_queries/ralph/injections.scm");
pub const RALPH_LOCALS: &str = include_str!("../generated_queries/ralph/locals.scm");
pub const RALPH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ralph/highlights_crates_io.scm");
pub const RALPH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ralph/injections_crates_io.scm");
pub const RALPH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ralph/locals_crates_io.scm");

pub const RASI_HIGHLIGHTS: &str = include_str!("../generated_queries/rasi/highlights.scm");
pub const RASI_INJECTIONS: &str = include_str!("../generated_queries/rasi/injections.scm");
pub const RASI_LOCALS: &str = include_str!("../generated_queries/rasi/locals.scm");
pub const RASI_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rasi/highlights_crates_io.scm");
pub const RASI_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rasi/injections_crates_io.scm");
pub const RASI_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rasi/locals_crates_io.scm");

pub const RAZOR_HIGHLIGHTS: &str = include_str!("../generated_queries/razor/highlights.scm");
pub const RAZOR_INJECTIONS: &str = include_str!("../generated_queries/razor/injections.scm");
pub const RAZOR_LOCALS: &str = include_str!("../generated_queries/razor/locals.scm");
pub const RAZOR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/razor/highlights_crates_io.scm");
pub const RAZOR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/razor/injections_crates_io.scm");
pub const RAZOR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/razor/locals_crates_io.scm");

pub const RBS_HIGHLIGHTS: &str = include_str!("../generated_queries/rbs/highlights.scm");
pub const RBS_INJECTIONS: &str = include_str!("../generated_queries/rbs/injections.scm");
pub const RBS_LOCALS: &str = include_str!("../generated_queries/rbs/locals.scm");
pub const RBS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rbs/highlights_crates_io.scm");
pub const RBS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rbs/injections_crates_io.scm");
pub const RBS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rbs/locals_crates_io.scm");

pub const RE2C_HIGHLIGHTS: &str = include_str!("../generated_queries/re2c/highlights.scm");
pub const RE2C_INJECTIONS: &str = include_str!("../generated_queries/re2c/injections.scm");
pub const RE2C_LOCALS: &str = include_str!("../generated_queries/re2c/locals.scm");
pub const RE2C_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/re2c/highlights_crates_io.scm");
pub const RE2C_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/re2c/injections_crates_io.scm");
pub const RE2C_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/re2c/locals_crates_io.scm");

pub const READLINE_HIGHLIGHTS: &str = include_str!("../generated_queries/readline/highlights.scm");
pub const READLINE_INJECTIONS: &str = include_str!("../generated_queries/readline/injections.scm");
pub const READLINE_LOCALS: &str = include_str!("../generated_queries/readline/locals.scm");
pub const READLINE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/readline/highlights_crates_io.scm");
pub const READLINE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/readline/injections_crates_io.scm");
pub const READLINE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/readline/locals_crates_io.scm");

pub const REGEX_HIGHLIGHTS: &str = include_str!("../generated_queries/regex/highlights.scm");
pub const REGEX_INJECTIONS: &str = include_str!("../generated_queries/regex/injections.scm");
pub const REGEX_LOCALS: &str = include_str!("../generated_queries/regex/locals.scm");
pub const REGEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/regex/highlights_crates_io.scm");
pub const REGEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/regex/injections_crates_io.scm");
pub const REGEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/regex/locals_crates_io.scm");

pub const REGO_HIGHLIGHTS: &str = include_str!("../generated_queries/rego/highlights.scm");
pub const REGO_INJECTIONS: &str = include_str!("../generated_queries/rego/injections.scm");
pub const REGO_LOCALS: &str = include_str!("../generated_queries/rego/locals.scm");
pub const REGO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rego/highlights_crates_io.scm");
pub const REGO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rego/injections_crates_io.scm");
pub const REGO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rego/locals_crates_io.scm");

pub const REQUIREMENTS_HIGHLIGHTS: &str = include_str!("../generated_queries/requirements/highlights.scm");
pub const REQUIREMENTS_INJECTIONS: &str = include_str!("../generated_queries/requirements/injections.scm");
pub const REQUIREMENTS_LOCALS: &str = include_str!("../generated_queries/requirements/locals.scm");
pub const REQUIREMENTS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/requirements/highlights_crates_io.scm");
pub const REQUIREMENTS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/requirements/injections_crates_io.scm");
pub const REQUIREMENTS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/requirements/locals_crates_io.scm");

pub const RESCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/rescript/highlights.scm");
pub const RESCRIPT_INJECTIONS: &str = include_str!("../generated_queries/rescript/injections.scm");
pub const RESCRIPT_LOCALS: &str = include_str!("../generated_queries/rescript/locals.scm");
pub const RESCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rescript/highlights_crates_io.scm");
pub const RESCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rescript/injections_crates_io.scm");
pub const RESCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rescript/locals_crates_io.scm");

pub const RIFLECONF_HIGHLIGHTS: &str = include_str!("../generated_queries/rifleconf/highlights.scm");
pub const RIFLECONF_INJECTIONS: &str = include_str!("../generated_queries/rifleconf/injections.scm");
pub const RIFLECONF_LOCALS: &str = include_str!("../generated_queries/rifleconf/locals.scm");
pub const RIFLECONF_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rifleconf/highlights_crates_io.scm");
pub const RIFLECONF_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rifleconf/injections_crates_io.scm");
pub const RIFLECONF_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rifleconf/locals_crates_io.scm");

pub const RNOWEB_HIGHLIGHTS: &str = include_str!("../generated_queries/rnoweb/highlights.scm");
pub const RNOWEB_INJECTIONS: &str = include_str!("../generated_queries/rnoweb/injections.scm");
pub const RNOWEB_LOCALS: &str = include_str!("../generated_queries/rnoweb/locals.scm");
pub const RNOWEB_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rnoweb/highlights_crates_io.scm");
pub const RNOWEB_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rnoweb/injections_crates_io.scm");
pub const RNOWEB_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rnoweb/locals_crates_io.scm");

pub const ROBOT_HIGHLIGHTS: &str = include_str!("../generated_queries/robot/highlights.scm");
pub const ROBOT_INJECTIONS: &str = include_str!("../generated_queries/robot/injections.scm");
pub const ROBOT_LOCALS: &str = include_str!("../generated_queries/robot/locals.scm");
pub const ROBOT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/robot/highlights_crates_io.scm");
pub const ROBOT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/robot/injections_crates_io.scm");
pub const ROBOT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/robot/locals_crates_io.scm");

pub const ROBOTS_HIGHLIGHTS: &str = include_str!("../generated_queries/robots/highlights.scm");
pub const ROBOTS_INJECTIONS: &str = include_str!("../generated_queries/robots/injections.scm");
pub const ROBOTS_LOCALS: &str = include_str!("../generated_queries/robots/locals.scm");
pub const ROBOTS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/robots/highlights_crates_io.scm");
pub const ROBOTS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/robots/injections_crates_io.scm");
pub const ROBOTS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/robots/locals_crates_io.scm");

pub const ROC_HIGHLIGHTS: &str = include_str!("../generated_queries/roc/highlights.scm");
pub const ROC_INJECTIONS: &str = include_str!("../generated_queries/roc/injections.scm");
pub const ROC_LOCALS: &str = include_str!("../generated_queries/roc/locals.scm");
pub const ROC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/roc/highlights_crates_io.scm");
pub const ROC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/roc/injections_crates_io.scm");
pub const ROC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/roc/locals_crates_io.scm");

pub const RON_HIGHLIGHTS: &str = include_str!("../generated_queries/ron/highlights.scm");
pub const RON_INJECTIONS: &str = include_str!("../generated_queries/ron/injections.scm");
pub const RON_LOCALS: &str = include_str!("../generated_queries/ron/locals.scm");
pub const RON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ron/highlights_crates_io.scm");
pub const RON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ron/injections_crates_io.scm");
pub const RON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ron/locals_crates_io.scm");

pub const RST_HIGHLIGHTS: &str = include_str!("../generated_queries/rst/highlights.scm");
pub const RST_INJECTIONS: &str = include_str!("../generated_queries/rst/injections.scm");
pub const RST_LOCALS: &str = include_str!("../generated_queries/rst/locals.scm");
pub const RST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rst/highlights_crates_io.scm");
pub const RST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rst/injections_crates_io.scm");
pub const RST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rst/locals_crates_io.scm");

pub const RUBY_HIGHLIGHTS: &str = include_str!("../generated_queries/ruby/highlights.scm");
pub const RUBY_INJECTIONS: &str = include_str!("../generated_queries/ruby/injections.scm");
pub const RUBY_LOCALS: &str = include_str!("../generated_queries/ruby/locals.scm");
pub const RUBY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ruby/highlights_crates_io.scm");
pub const RUBY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ruby/injections_crates_io.scm");
pub const RUBY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ruby/locals_crates_io.scm");

pub const RUNESCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/runescript/highlights.scm");
pub const RUNESCRIPT_INJECTIONS: &str = include_str!("../generated_queries/runescript/injections.scm");
pub const RUNESCRIPT_LOCALS: &str = include_str!("../generated_queries/runescript/locals.scm");
pub const RUNESCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/runescript/highlights_crates_io.scm");
pub const RUNESCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/runescript/injections_crates_io.scm");
pub const RUNESCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/runescript/locals_crates_io.scm");

pub const RUSH_HIGHLIGHTS: &str = include_str!("../generated_queries/rush/highlights.scm");
pub const RUSH_INJECTIONS: &str = include_str!("../generated_queries/rush/injections.scm");
pub const RUSH_LOCALS: &str = include_str!("../generated_queries/rush/locals.scm");
pub const RUSH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rush/highlights_crates_io.scm");
pub const RUSH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rush/injections_crates_io.scm");
pub const RUSH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rush/locals_crates_io.scm");

pub const RUST_HIGHLIGHTS: &str = include_str!("../generated_queries/rust/highlights.scm");
pub const RUST_INJECTIONS: &str = include_str!("../generated_queries/rust/injections.scm");
pub const RUST_LOCALS: &str = include_str!("../generated_queries/rust/locals.scm");
pub const RUST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/rust/highlights_crates_io.scm");
pub const RUST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/rust/injections_crates_io.scm");
pub const RUST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/rust/locals_crates_io.scm");

pub const SCALA_HIGHLIGHTS: &str = include_str!("../generated_queries/scala/highlights.scm");
pub const SCALA_INJECTIONS: &str = include_str!("../generated_queries/scala/injections.scm");
pub const SCALA_LOCALS: &str = include_str!("../generated_queries/scala/locals.scm");
pub const SCALA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/scala/highlights_crates_io.scm");
pub const SCALA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/scala/injections_crates_io.scm");
pub const SCALA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/scala/locals_crates_io.scm");

pub const SCFG_HIGHLIGHTS: &str = include_str!("../generated_queries/scfg/highlights.scm");
pub const SCFG_INJECTIONS: &str = include_str!("../generated_queries/scfg/injections.scm");
pub const SCFG_LOCALS: &str = include_str!("../generated_queries/scfg/locals.scm");
pub const SCFG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/scfg/highlights_crates_io.scm");
pub const SCFG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/scfg/injections_crates_io.scm");
pub const SCFG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/scfg/locals_crates_io.scm");

pub const SCHEME_HIGHLIGHTS: &str = include_str!("../generated_queries/scheme/highlights.scm");
pub const SCHEME_INJECTIONS: &str = include_str!("../generated_queries/scheme/injections.scm");
pub const SCHEME_LOCALS: &str = include_str!("../generated_queries/scheme/locals.scm");
pub const SCHEME_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/scheme/highlights_crates_io.scm");
pub const SCHEME_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/scheme/injections_crates_io.scm");
pub const SCHEME_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/scheme/locals_crates_io.scm");

pub const SCSS_HIGHLIGHTS: &str = include_str!("../generated_queries/scss/highlights.scm");
pub const SCSS_INJECTIONS: &str = include_str!("../generated_queries/scss/injections.scm");
pub const SCSS_LOCALS: &str = include_str!("../generated_queries/scss/locals.scm");
pub const SCSS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/scss/highlights_crates_io.scm");
pub const SCSS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/scss/injections_crates_io.scm");
pub const SCSS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/scss/locals_crates_io.scm");

pub const SFACE_HIGHLIGHTS: &str = include_str!("../generated_queries/sface/highlights.scm");
pub const SFACE_INJECTIONS: &str = include_str!("../generated_queries/sface/injections.scm");
pub const SFACE_LOCALS: &str = include_str!("../generated_queries/sface/locals.scm");
pub const SFACE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sface/highlights_crates_io.scm");
pub const SFACE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sface/injections_crates_io.scm");
pub const SFACE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sface/locals_crates_io.scm");

pub const SFLOG_HIGHLIGHTS: &str = include_str!("../generated_queries/sflog/highlights.scm");
pub const SFLOG_INJECTIONS: &str = include_str!("../generated_queries/sflog/injections.scm");
pub const SFLOG_LOCALS: &str = include_str!("../generated_queries/sflog/locals.scm");
pub const SFLOG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sflog/highlights_crates_io.scm");
pub const SFLOG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sflog/injections_crates_io.scm");
pub const SFLOG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sflog/locals_crates_io.scm");

pub const SHADERSLANG_HIGHLIGHTS: &str = include_str!("../generated_queries/shaderslang/highlights.scm");
pub const SHADERSLANG_INJECTIONS: &str = include_str!("../generated_queries/shaderslang/injections.scm");
pub const SHADERSLANG_LOCALS: &str = include_str!("../generated_queries/shaderslang/locals.scm");
pub const SHADERSLANG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/shaderslang/highlights_crates_io.scm");
pub const SHADERSLANG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/shaderslang/injections_crates_io.scm");
pub const SHADERSLANG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/shaderslang/locals_crates_io.scm");

pub const SLIM_HIGHLIGHTS: &str = include_str!("../generated_queries/slim/highlights.scm");
pub const SLIM_INJECTIONS: &str = include_str!("../generated_queries/slim/injections.scm");
pub const SLIM_LOCALS: &str = include_str!("../generated_queries/slim/locals.scm");
pub const SLIM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/slim/highlights_crates_io.scm");
pub const SLIM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/slim/injections_crates_io.scm");
pub const SLIM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/slim/locals_crates_io.scm");

pub const SLINT_HIGHLIGHTS: &str = include_str!("../generated_queries/slint/highlights.scm");
pub const SLINT_INJECTIONS: &str = include_str!("../generated_queries/slint/injections.scm");
pub const SLINT_LOCALS: &str = include_str!("../generated_queries/slint/locals.scm");
pub const SLINT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/slint/highlights_crates_io.scm");
pub const SLINT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/slint/injections_crates_io.scm");
pub const SLINT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/slint/locals_crates_io.scm");

pub const SMALI_HIGHLIGHTS: &str = include_str!("../generated_queries/smali/highlights.scm");
pub const SMALI_INJECTIONS: &str = include_str!("../generated_queries/smali/injections.scm");
pub const SMALI_LOCALS: &str = include_str!("../generated_queries/smali/locals.scm");
pub const SMALI_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/smali/highlights_crates_io.scm");
pub const SMALI_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/smali/injections_crates_io.scm");
pub const SMALI_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/smali/locals_crates_io.scm");

pub const SMITHY_HIGHLIGHTS: &str = include_str!("../generated_queries/smithy/highlights.scm");
pub const SMITHY_INJECTIONS: &str = include_str!("../generated_queries/smithy/injections.scm");
pub const SMITHY_LOCALS: &str = include_str!("../generated_queries/smithy/locals.scm");
pub const SMITHY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/smithy/highlights_crates_io.scm");
pub const SMITHY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/smithy/injections_crates_io.scm");
pub const SMITHY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/smithy/locals_crates_io.scm");

pub const SNAKEMAKE_HIGHLIGHTS: &str = include_str!("../generated_queries/snakemake/highlights.scm");
pub const SNAKEMAKE_INJECTIONS: &str = include_str!("../generated_queries/snakemake/injections.scm");
pub const SNAKEMAKE_LOCALS: &str = include_str!("../generated_queries/snakemake/locals.scm");
pub const SNAKEMAKE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/snakemake/highlights_crates_io.scm");
pub const SNAKEMAKE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/snakemake/injections_crates_io.scm");
pub const SNAKEMAKE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/snakemake/locals_crates_io.scm");

pub const SOLIDITY_HIGHLIGHTS: &str = include_str!("../generated_queries/solidity/highlights.scm");
pub const SOLIDITY_INJECTIONS: &str = include_str!("../generated_queries/solidity/injections.scm");
pub const SOLIDITY_LOCALS: &str = include_str!("../generated_queries/solidity/locals.scm");
pub const SOLIDITY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/solidity/highlights_crates_io.scm");
pub const SOLIDITY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/solidity/injections_crates_io.scm");
pub const SOLIDITY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/solidity/locals_crates_io.scm");

pub const SOQL_HIGHLIGHTS: &str = include_str!("../generated_queries/soql/highlights.scm");
pub const SOQL_INJECTIONS: &str = include_str!("../generated_queries/soql/injections.scm");
pub const SOQL_LOCALS: &str = include_str!("../generated_queries/soql/locals.scm");
pub const SOQL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/soql/highlights_crates_io.scm");
pub const SOQL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/soql/injections_crates_io.scm");
pub const SOQL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/soql/locals_crates_io.scm");

pub const SOSL_HIGHLIGHTS: &str = include_str!("../generated_queries/sosl/highlights.scm");
pub const SOSL_INJECTIONS: &str = include_str!("../generated_queries/sosl/injections.scm");
pub const SOSL_LOCALS: &str = include_str!("../generated_queries/sosl/locals.scm");
pub const SOSL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sosl/highlights_crates_io.scm");
pub const SOSL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sosl/injections_crates_io.scm");
pub const SOSL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sosl/locals_crates_io.scm");

pub const SOURCEPAWN_HIGHLIGHTS: &str = include_str!("../generated_queries/sourcepawn/highlights.scm");
pub const SOURCEPAWN_INJECTIONS: &str = include_str!("../generated_queries/sourcepawn/injections.scm");
pub const SOURCEPAWN_LOCALS: &str = include_str!("../generated_queries/sourcepawn/locals.scm");
pub const SOURCEPAWN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sourcepawn/highlights_crates_io.scm");
pub const SOURCEPAWN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sourcepawn/injections_crates_io.scm");
pub const SOURCEPAWN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sourcepawn/locals_crates_io.scm");

pub const SPARQL_HIGHLIGHTS: &str = include_str!("../generated_queries/sparql/highlights.scm");
pub const SPARQL_INJECTIONS: &str = include_str!("../generated_queries/sparql/injections.scm");
pub const SPARQL_LOCALS: &str = include_str!("../generated_queries/sparql/locals.scm");
pub const SPARQL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sparql/highlights_crates_io.scm");
pub const SPARQL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sparql/injections_crates_io.scm");
pub const SPARQL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sparql/locals_crates_io.scm");

pub const SPROTO_HIGHLIGHTS: &str = include_str!("../generated_queries/sproto/highlights.scm");
pub const SPROTO_INJECTIONS: &str = include_str!("../generated_queries/sproto/injections.scm");
pub const SPROTO_LOCALS: &str = include_str!("../generated_queries/sproto/locals.scm");
pub const SPROTO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sproto/highlights_crates_io.scm");
pub const SPROTO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sproto/injections_crates_io.scm");
pub const SPROTO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sproto/locals_crates_io.scm");

pub const SQL_HIGHLIGHTS: &str = include_str!("../generated_queries/sql/highlights.scm");
pub const SQL_INJECTIONS: &str = include_str!("../generated_queries/sql/injections.scm");
pub const SQL_LOCALS: &str = include_str!("../generated_queries/sql/locals.scm");
pub const SQL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sql/highlights_crates_io.scm");
pub const SQL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sql/injections_crates_io.scm");
pub const SQL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sql/locals_crates_io.scm");

pub const SQUIRREL_HIGHLIGHTS: &str = include_str!("../generated_queries/squirrel/highlights.scm");
pub const SQUIRREL_INJECTIONS: &str = include_str!("../generated_queries/squirrel/injections.scm");
pub const SQUIRREL_LOCALS: &str = include_str!("../generated_queries/squirrel/locals.scm");
pub const SQUIRREL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/squirrel/highlights_crates_io.scm");
pub const SQUIRREL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/squirrel/injections_crates_io.scm");
pub const SQUIRREL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/squirrel/locals_crates_io.scm");

pub const SSHCONFIG_HIGHLIGHTS: &str = include_str!("../generated_queries/sshconfig/highlights.scm");
pub const SSHCONFIG_INJECTIONS: &str = include_str!("../generated_queries/sshconfig/injections.scm");
pub const SSHCONFIG_LOCALS: &str = include_str!("../generated_queries/sshconfig/locals.scm");
pub const SSHCONFIG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sshconfig/highlights_crates_io.scm");
pub const SSHCONFIG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sshconfig/injections_crates_io.scm");
pub const SSHCONFIG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sshconfig/locals_crates_io.scm");

pub const STRACE_HIGHLIGHTS: &str = include_str!("../generated_queries/strace/highlights.scm");
pub const STRACE_INJECTIONS: &str = include_str!("../generated_queries/strace/injections.scm");
pub const STRACE_LOCALS: &str = include_str!("../generated_queries/strace/locals.scm");
pub const STRACE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/strace/highlights_crates_io.scm");
pub const STRACE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/strace/injections_crates_io.scm");
pub const STRACE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/strace/locals_crates_io.scm");

pub const STYLED_HIGHLIGHTS: &str = include_str!("../generated_queries/styled/highlights.scm");
pub const STYLED_INJECTIONS: &str = include_str!("../generated_queries/styled/injections.scm");
pub const STYLED_LOCALS: &str = include_str!("../generated_queries/styled/locals.scm");
pub const STYLED_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/styled/highlights_crates_io.scm");
pub const STYLED_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/styled/injections_crates_io.scm");
pub const STYLED_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/styled/locals_crates_io.scm");

pub const SUPERCOLLIDER_HIGHLIGHTS: &str = include_str!("../generated_queries/supercollider/highlights.scm");
pub const SUPERCOLLIDER_INJECTIONS: &str = include_str!("../generated_queries/supercollider/injections.scm");
pub const SUPERCOLLIDER_LOCALS: &str = include_str!("../generated_queries/supercollider/locals.scm");
pub const SUPERCOLLIDER_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/supercollider/highlights_crates_io.scm");
pub const SUPERCOLLIDER_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/supercollider/injections_crates_io.scm");
pub const SUPERCOLLIDER_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/supercollider/locals_crates_io.scm");

pub const SUPERHTML_HIGHLIGHTS: &str = include_str!("../generated_queries/superhtml/highlights.scm");
pub const SUPERHTML_INJECTIONS: &str = include_str!("../generated_queries/superhtml/injections.scm");
pub const SUPERHTML_LOCALS: &str = include_str!("../generated_queries/superhtml/locals.scm");
pub const SUPERHTML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/superhtml/highlights_crates_io.scm");
pub const SUPERHTML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/superhtml/injections_crates_io.scm");
pub const SUPERHTML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/superhtml/locals_crates_io.scm");

pub const SVELTE_HIGHLIGHTS: &str = include_str!("../generated_queries/svelte/highlights.scm");
pub const SVELTE_INJECTIONS: &str = include_str!("../generated_queries/svelte/injections.scm");
pub const SVELTE_LOCALS: &str = include_str!("../generated_queries/svelte/locals.scm");
pub const SVELTE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/svelte/highlights_crates_io.scm");
pub const SVELTE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/svelte/injections_crates_io.scm");
pub const SVELTE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/svelte/locals_crates_io.scm");

pub const SWAY_HIGHLIGHTS: &str = include_str!("../generated_queries/sway/highlights.scm");
pub const SWAY_INJECTIONS: &str = include_str!("../generated_queries/sway/injections.scm");
pub const SWAY_LOCALS: &str = include_str!("../generated_queries/sway/locals.scm");
pub const SWAY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sway/highlights_crates_io.scm");
pub const SWAY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sway/injections_crates_io.scm");
pub const SWAY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sway/locals_crates_io.scm");

pub const SWIFT_HIGHLIGHTS: &str = include_str!("../generated_queries/swift/highlights.scm");
pub const SWIFT_INJECTIONS: &str = include_str!("../generated_queries/swift/injections.scm");
pub const SWIFT_LOCALS: &str = include_str!("../generated_queries/swift/locals.scm");
pub const SWIFT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/swift/highlights_crates_io.scm");
pub const SWIFT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/swift/injections_crates_io.scm");
pub const SWIFT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/swift/locals_crates_io.scm");

pub const SXHKDRC_HIGHLIGHTS: &str = include_str!("../generated_queries/sxhkdrc/highlights.scm");
pub const SXHKDRC_INJECTIONS: &str = include_str!("../generated_queries/sxhkdrc/injections.scm");
pub const SXHKDRC_LOCALS: &str = include_str!("../generated_queries/sxhkdrc/locals.scm");
pub const SXHKDRC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/sxhkdrc/highlights_crates_io.scm");
pub const SXHKDRC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/sxhkdrc/injections_crates_io.scm");
pub const SXHKDRC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/sxhkdrc/locals_crates_io.scm");

pub const SYSTEMTAP_HIGHLIGHTS: &str = include_str!("../generated_queries/systemtap/highlights.scm");
pub const SYSTEMTAP_INJECTIONS: &str = include_str!("../generated_queries/systemtap/injections.scm");
pub const SYSTEMTAP_LOCALS: &str = include_str!("../generated_queries/systemtap/locals.scm");
pub const SYSTEMTAP_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/systemtap/highlights_crates_io.scm");
pub const SYSTEMTAP_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/systemtap/injections_crates_io.scm");
pub const SYSTEMTAP_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/systemtap/locals_crates_io.scm");

pub const TABLEGEN_HIGHLIGHTS: &str = include_str!("../generated_queries/tablegen/highlights.scm");
pub const TABLEGEN_INJECTIONS: &str = include_str!("../generated_queries/tablegen/injections.scm");
pub const TABLEGEN_LOCALS: &str = include_str!("../generated_queries/tablegen/locals.scm");
pub const TABLEGEN_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tablegen/highlights_crates_io.scm");
pub const TABLEGEN_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tablegen/injections_crates_io.scm");
pub const TABLEGEN_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tablegen/locals_crates_io.scm");

pub const TACT_HIGHLIGHTS: &str = include_str!("../generated_queries/tact/highlights.scm");
pub const TACT_INJECTIONS: &str = include_str!("../generated_queries/tact/injections.scm");
pub const TACT_LOCALS: &str = include_str!("../generated_queries/tact/locals.scm");
pub const TACT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tact/highlights_crates_io.scm");
pub const TACT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tact/injections_crates_io.scm");
pub const TACT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tact/locals_crates_io.scm");

pub const TAL_HIGHLIGHTS: &str = include_str!("../generated_queries/tal/highlights.scm");
pub const TAL_INJECTIONS: &str = include_str!("../generated_queries/tal/injections.scm");
pub const TAL_LOCALS: &str = include_str!("../generated_queries/tal/locals.scm");
pub const TAL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tal/highlights_crates_io.scm");
pub const TAL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tal/injections_crates_io.scm");
pub const TAL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tal/locals_crates_io.scm");

pub const TAPE_HIGHLIGHTS: &str = include_str!("../generated_queries/tape/highlights.scm");
pub const TAPE_INJECTIONS: &str = include_str!("../generated_queries/tape/injections.scm");
pub const TAPE_LOCALS: &str = include_str!("../generated_queries/tape/locals.scm");
pub const TAPE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tape/highlights_crates_io.scm");
pub const TAPE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tape/injections_crates_io.scm");
pub const TAPE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tape/locals_crates_io.scm");

pub const TCL_HIGHLIGHTS: &str = include_str!("../generated_queries/tcl/highlights.scm");
pub const TCL_INJECTIONS: &str = include_str!("../generated_queries/tcl/injections.scm");
pub const TCL_LOCALS: &str = include_str!("../generated_queries/tcl/locals.scm");
pub const TCL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tcl/highlights_crates_io.scm");
pub const TCL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tcl/injections_crates_io.scm");
pub const TCL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tcl/locals_crates_io.scm");

pub const TEAL_HIGHLIGHTS: &str = include_str!("../generated_queries/teal/highlights.scm");
pub const TEAL_INJECTIONS: &str = include_str!("../generated_queries/teal/injections.scm");
pub const TEAL_LOCALS: &str = include_str!("../generated_queries/teal/locals.scm");
pub const TEAL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/teal/highlights_crates_io.scm");
pub const TEAL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/teal/injections_crates_io.scm");
pub const TEAL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/teal/locals_crates_io.scm");

pub const TEMPL_HIGHLIGHTS: &str = include_str!("../generated_queries/templ/highlights.scm");
pub const TEMPL_INJECTIONS: &str = include_str!("../generated_queries/templ/injections.scm");
pub const TEMPL_LOCALS: &str = include_str!("../generated_queries/templ/locals.scm");
pub const TEMPL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/templ/highlights_crates_io.scm");
pub const TEMPL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/templ/injections_crates_io.scm");
pub const TEMPL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/templ/locals_crates_io.scm");

pub const TERA_HIGHLIGHTS: &str = include_str!("../generated_queries/tera/highlights.scm");
pub const TERA_INJECTIONS: &str = include_str!("../generated_queries/tera/injections.scm");
pub const TERA_LOCALS: &str = include_str!("../generated_queries/tera/locals.scm");
pub const TERA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tera/highlights_crates_io.scm");
pub const TERA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tera/injections_crates_io.scm");
pub const TERA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tera/locals_crates_io.scm");

pub const TERRAFORM_HIGHLIGHTS: &str = include_str!("../generated_queries/terraform/highlights.scm");
pub const TERRAFORM_INJECTIONS: &str = include_str!("../generated_queries/terraform/injections.scm");
pub const TERRAFORM_LOCALS: &str = include_str!("../generated_queries/terraform/locals.scm");
pub const TERRAFORM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/terraform/highlights_crates_io.scm");
pub const TERRAFORM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/terraform/injections_crates_io.scm");
pub const TERRAFORM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/terraform/locals_crates_io.scm");

pub const TEX_HIGHLIGHTS: &str = include_str!("../generated_queries/tex/highlights.scm");
pub const TEX_INJECTIONS: &str = include_str!("../generated_queries/tex/injections.scm");
pub const TEX_LOCALS: &str = include_str!("../generated_queries/tex/locals.scm");
pub const TEX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tex/highlights_crates_io.scm");
pub const TEX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tex/injections_crates_io.scm");
pub const TEX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tex/locals_crates_io.scm");

pub const THRIFT_HIGHLIGHTS: &str = include_str!("../generated_queries/thrift/highlights.scm");
pub const THRIFT_INJECTIONS: &str = include_str!("../generated_queries/thrift/injections.scm");
pub const THRIFT_LOCALS: &str = include_str!("../generated_queries/thrift/locals.scm");
pub const THRIFT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/thrift/highlights_crates_io.scm");
pub const THRIFT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/thrift/injections_crates_io.scm");
pub const THRIFT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/thrift/locals_crates_io.scm");

pub const TIGER_HIGHLIGHTS: &str = include_str!("../generated_queries/tiger/highlights.scm");
pub const TIGER_INJECTIONS: &str = include_str!("../generated_queries/tiger/injections.scm");
pub const TIGER_LOCALS: &str = include_str!("../generated_queries/tiger/locals.scm");
pub const TIGER_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tiger/highlights_crates_io.scm");
pub const TIGER_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tiger/injections_crates_io.scm");
pub const TIGER_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tiger/locals_crates_io.scm");

pub const TLA_HIGHLIGHTS: &str = include_str!("../generated_queries/tla/highlights.scm");
pub const TLA_INJECTIONS: &str = include_str!("../generated_queries/tla/injections.scm");
pub const TLA_LOCALS: &str = include_str!("../generated_queries/tla/locals.scm");
pub const TLA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tla/highlights_crates_io.scm");
pub const TLA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tla/injections_crates_io.scm");
pub const TLA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tla/locals_crates_io.scm");

pub const TMUX_HIGHLIGHTS: &str = include_str!("../generated_queries/tmux/highlights.scm");
pub const TMUX_INJECTIONS: &str = include_str!("../generated_queries/tmux/injections.scm");
pub const TMUX_LOCALS: &str = include_str!("../generated_queries/tmux/locals.scm");
pub const TMUX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tmux/highlights_crates_io.scm");
pub const TMUX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tmux/injections_crates_io.scm");
pub const TMUX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tmux/locals_crates_io.scm");

pub const TODOTXT_HIGHLIGHTS: &str = include_str!("../generated_queries/todotxt/highlights.scm");
pub const TODOTXT_INJECTIONS: &str = include_str!("../generated_queries/todotxt/injections.scm");
pub const TODOTXT_LOCALS: &str = include_str!("../generated_queries/todotxt/locals.scm");
pub const TODOTXT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/todotxt/highlights_crates_io.scm");
pub const TODOTXT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/todotxt/injections_crates_io.scm");
pub const TODOTXT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/todotxt/locals_crates_io.scm");

pub const TOML_HIGHLIGHTS: &str = include_str!("../generated_queries/toml/highlights.scm");
pub const TOML_INJECTIONS: &str = include_str!("../generated_queries/toml/injections.scm");
pub const TOML_LOCALS: &str = include_str!("../generated_queries/toml/locals.scm");
pub const TOML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/toml/highlights_crates_io.scm");
pub const TOML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/toml/injections_crates_io.scm");
pub const TOML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/toml/locals_crates_io.scm");

pub const TRACE32_HIGHLIGHTS: &str = include_str!("../generated_queries/trace32/highlights.scm");
pub const TRACE32_INJECTIONS: &str = include_str!("../generated_queries/trace32/injections.scm");
pub const TRACE32_LOCALS: &str = include_str!("../generated_queries/trace32/locals.scm");
pub const TRACE32_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/trace32/highlights_crates_io.scm");
pub const TRACE32_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/trace32/injections_crates_io.scm");
pub const TRACE32_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/trace32/locals_crates_io.scm");

pub const TSV_HIGHLIGHTS: &str = include_str!("../generated_queries/tsv/highlights.scm");
pub const TSV_INJECTIONS: &str = include_str!("../generated_queries/tsv/injections.scm");
pub const TSV_LOCALS: &str = include_str!("../generated_queries/tsv/locals.scm");
pub const TSV_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tsv/highlights_crates_io.scm");
pub const TSV_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tsv/injections_crates_io.scm");
pub const TSV_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tsv/locals_crates_io.scm");

pub const TSX_HIGHLIGHTS: &str = include_str!("../generated_queries/tsx/highlights.scm");
pub const TSX_INJECTIONS: &str = include_str!("../generated_queries/tsx/injections.scm");
pub const TSX_LOCALS: &str = include_str!("../generated_queries/tsx/locals.scm");
pub const TSX_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/tsx/highlights_crates_io.scm");
pub const TSX_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/tsx/injections_crates_io.scm");
pub const TSX_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/tsx/locals_crates_io.scm");

pub const TURTLE_HIGHLIGHTS: &str = include_str!("../generated_queries/turtle/highlights.scm");
pub const TURTLE_INJECTIONS: &str = include_str!("../generated_queries/turtle/injections.scm");
pub const TURTLE_LOCALS: &str = include_str!("../generated_queries/turtle/locals.scm");
pub const TURTLE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/turtle/highlights_crates_io.scm");
pub const TURTLE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/turtle/injections_crates_io.scm");
pub const TURTLE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/turtle/locals_crates_io.scm");

pub const TWIG_HIGHLIGHTS: &str = include_str!("../generated_queries/twig/highlights.scm");
pub const TWIG_INJECTIONS: &str = include_str!("../generated_queries/twig/injections.scm");
pub const TWIG_LOCALS: &str = include_str!("../generated_queries/twig/locals.scm");
pub const TWIG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/twig/highlights_crates_io.scm");
pub const TWIG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/twig/injections_crates_io.scm");
pub const TWIG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/twig/locals_crates_io.scm");

pub const TYPESCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/typescript/highlights.scm");
pub const TYPESCRIPT_INJECTIONS: &str = include_str!("../generated_queries/typescript/injections.scm");
pub const TYPESCRIPT_LOCALS: &str = include_str!("../generated_queries/typescript/locals.scm");
pub const TYPESCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/typescript/highlights_crates_io.scm");
pub const TYPESCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/typescript/injections_crates_io.scm");
pub const TYPESCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/typescript/locals_crates_io.scm");

pub const TYPESPEC_HIGHLIGHTS: &str = include_str!("../generated_queries/typespec/highlights.scm");
pub const TYPESPEC_INJECTIONS: &str = include_str!("../generated_queries/typespec/injections.scm");
pub const TYPESPEC_LOCALS: &str = include_str!("../generated_queries/typespec/locals.scm");
pub const TYPESPEC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/typespec/highlights_crates_io.scm");
pub const TYPESPEC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/typespec/injections_crates_io.scm");
pub const TYPESPEC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/typespec/locals_crates_io.scm");

pub const TYPOSCRIPT_HIGHLIGHTS: &str = include_str!("../generated_queries/typoscript/highlights.scm");
pub const TYPOSCRIPT_INJECTIONS: &str = include_str!("../generated_queries/typoscript/injections.scm");
pub const TYPOSCRIPT_LOCALS: &str = include_str!("../generated_queries/typoscript/locals.scm");
pub const TYPOSCRIPT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/typoscript/highlights_crates_io.scm");
pub const TYPOSCRIPT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/typoscript/injections_crates_io.scm");
pub const TYPOSCRIPT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/typoscript/locals_crates_io.scm");

pub const TYPST_HIGHLIGHTS: &str = include_str!("../generated_queries/typst/highlights.scm");
pub const TYPST_INJECTIONS: &str = include_str!("../generated_queries/typst/injections.scm");
pub const TYPST_LOCALS: &str = include_str!("../generated_queries/typst/locals.scm");
pub const TYPST_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/typst/highlights_crates_io.scm");
pub const TYPST_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/typst/injections_crates_io.scm");
pub const TYPST_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/typst/locals_crates_io.scm");

pub const UDEVRULES_HIGHLIGHTS: &str = include_str!("../generated_queries/udevrules/highlights.scm");
pub const UDEVRULES_INJECTIONS: &str = include_str!("../generated_queries/udevrules/injections.scm");
pub const UDEVRULES_LOCALS: &str = include_str!("../generated_queries/udevrules/locals.scm");
pub const UDEVRULES_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/udevrules/highlights_crates_io.scm");
pub const UDEVRULES_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/udevrules/injections_crates_io.scm");
pub const UDEVRULES_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/udevrules/locals_crates_io.scm");

pub const UNGRAMMAR_HIGHLIGHTS: &str = include_str!("../generated_queries/ungrammar/highlights.scm");
pub const UNGRAMMAR_INJECTIONS: &str = include_str!("../generated_queries/ungrammar/injections.scm");
pub const UNGRAMMAR_LOCALS: &str = include_str!("../generated_queries/ungrammar/locals.scm");
pub const UNGRAMMAR_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ungrammar/highlights_crates_io.scm");
pub const UNGRAMMAR_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ungrammar/injections_crates_io.scm");
pub const UNGRAMMAR_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ungrammar/locals_crates_io.scm");

pub const UNISON_HIGHLIGHTS: &str = include_str!("../generated_queries/unison/highlights.scm");
pub const UNISON_INJECTIONS: &str = include_str!("../generated_queries/unison/injections.scm");
pub const UNISON_LOCALS: &str = include_str!("../generated_queries/unison/locals.scm");
pub const UNISON_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/unison/highlights_crates_io.scm");
pub const UNISON_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/unison/injections_crates_io.scm");
pub const UNISON_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/unison/locals_crates_io.scm");

pub const URSA_HIGHLIGHTS: &str = include_str!("../generated_queries/ursa/highlights.scm");
pub const URSA_INJECTIONS: &str = include_str!("../generated_queries/ursa/injections.scm");
pub const URSA_LOCALS: &str = include_str!("../generated_queries/ursa/locals.scm");
pub const URSA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ursa/highlights_crates_io.scm");
pub const URSA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ursa/injections_crates_io.scm");
pub const URSA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ursa/locals_crates_io.scm");

pub const USD_HIGHLIGHTS: &str = include_str!("../generated_queries/usd/highlights.scm");
pub const USD_INJECTIONS: &str = include_str!("../generated_queries/usd/injections.scm");
pub const USD_LOCALS: &str = include_str!("../generated_queries/usd/locals.scm");
pub const USD_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/usd/highlights_crates_io.scm");
pub const USD_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/usd/injections_crates_io.scm");
pub const USD_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/usd/locals_crates_io.scm");

pub const V_HIGHLIGHTS: &str = include_str!("../generated_queries/v/highlights.scm");
pub const V_INJECTIONS: &str = include_str!("../generated_queries/v/injections.scm");
pub const V_LOCALS: &str = include_str!("../generated_queries/v/locals.scm");
pub const V_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/v/highlights_crates_io.scm");
pub const V_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/v/injections_crates_io.scm");
pub const V_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/v/locals_crates_io.scm");

pub const VALA_HIGHLIGHTS: &str = include_str!("../generated_queries/vala/highlights.scm");
pub const VALA_INJECTIONS: &str = include_str!("../generated_queries/vala/injections.scm");
pub const VALA_LOCALS: &str = include_str!("../generated_queries/vala/locals.scm");
pub const VALA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/vala/highlights_crates_io.scm");
pub const VALA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/vala/injections_crates_io.scm");
pub const VALA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/vala/locals_crates_io.scm");

pub const VERILOG_HIGHLIGHTS: &str = include_str!("../generated_queries/verilog/highlights.scm");
pub const VERILOG_INJECTIONS: &str = include_str!("../generated_queries/verilog/injections.scm");
pub const VERILOG_LOCALS: &str = include_str!("../generated_queries/verilog/locals.scm");
pub const VERILOG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/verilog/highlights_crates_io.scm");
pub const VERILOG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/verilog/injections_crates_io.scm");
pub const VERILOG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/verilog/locals_crates_io.scm");

pub const VHDL_HIGHLIGHTS: &str = include_str!("../generated_queries/vhdl/highlights.scm");
pub const VHDL_INJECTIONS: &str = include_str!("../generated_queries/vhdl/injections.scm");
pub const VHDL_LOCALS: &str = include_str!("../generated_queries/vhdl/locals.scm");
pub const VHDL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/vhdl/highlights_crates_io.scm");
pub const VHDL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/vhdl/injections_crates_io.scm");
pub const VHDL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/vhdl/locals_crates_io.scm");

pub const VIM_HIGHLIGHTS: &str = include_str!("../generated_queries/vim/highlights.scm");
pub const VIM_INJECTIONS: &str = include_str!("../generated_queries/vim/injections.scm");
pub const VIM_LOCALS: &str = include_str!("../generated_queries/vim/locals.scm");
pub const VIM_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/vim/highlights_crates_io.scm");
pub const VIM_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/vim/injections_crates_io.scm");
pub const VIM_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/vim/locals_crates_io.scm");

pub const VRL_HIGHLIGHTS: &str = include_str!("../generated_queries/vrl/highlights.scm");
pub const VRL_INJECTIONS: &str = include_str!("../generated_queries/vrl/injections.scm");
pub const VRL_LOCALS: &str = include_str!("../generated_queries/vrl/locals.scm");
pub const VRL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/vrl/highlights_crates_io.scm");
pub const VRL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/vrl/injections_crates_io.scm");
pub const VRL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/vrl/locals_crates_io.scm");

pub const VTO_HIGHLIGHTS: &str = include_str!("../generated_queries/vto/highlights.scm");
pub const VTO_INJECTIONS: &str = include_str!("../generated_queries/vto/injections.scm");
pub const VTO_LOCALS: &str = include_str!("../generated_queries/vto/locals.scm");
pub const VTO_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/vto/highlights_crates_io.scm");
pub const VTO_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/vto/injections_crates_io.scm");
pub const VTO_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/vto/locals_crates_io.scm");

pub const VUE_HIGHLIGHTS: &str = include_str!("../generated_queries/vue/highlights.scm");
pub const VUE_INJECTIONS: &str = include_str!("../generated_queries/vue/injections.scm");
pub const VUE_LOCALS: &str = include_str!("../generated_queries/vue/locals.scm");
pub const VUE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/vue/highlights_crates_io.scm");
pub const VUE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/vue/injections_crates_io.scm");
pub const VUE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/vue/locals_crates_io.scm");

pub const WAT_HIGHLIGHTS: &str = include_str!("../generated_queries/wat/highlights.scm");
pub const WAT_INJECTIONS: &str = include_str!("../generated_queries/wat/injections.scm");
pub const WAT_LOCALS: &str = include_str!("../generated_queries/wat/locals.scm");
pub const WAT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/wat/highlights_crates_io.scm");
pub const WAT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/wat/injections_crates_io.scm");
pub const WAT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/wat/locals_crates_io.scm");

pub const WGSL_HIGHLIGHTS: &str = include_str!("../generated_queries/wgsl/highlights.scm");
pub const WGSL_INJECTIONS: &str = include_str!("../generated_queries/wgsl/injections.scm");
pub const WGSL_LOCALS: &str = include_str!("../generated_queries/wgsl/locals.scm");
pub const WGSL_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/wgsl/highlights_crates_io.scm");
pub const WGSL_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/wgsl/injections_crates_io.scm");
pub const WGSL_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/wgsl/locals_crates_io.scm");

pub const WGSL_BEVY_HIGHLIGHTS: &str = include_str!("../generated_queries/wgsl_bevy/highlights.scm");
pub const WGSL_BEVY_INJECTIONS: &str = include_str!("../generated_queries/wgsl_bevy/injections.scm");
pub const WGSL_BEVY_LOCALS: &str = include_str!("../generated_queries/wgsl_bevy/locals.scm");
pub const WGSL_BEVY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/wgsl_bevy/highlights_crates_io.scm");
pub const WGSL_BEVY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/wgsl_bevy/injections_crates_io.scm");
pub const WGSL_BEVY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/wgsl_bevy/locals_crates_io.scm");

pub const WING_HIGHLIGHTS: &str = include_str!("../generated_queries/wing/highlights.scm");
pub const WING_INJECTIONS: &str = include_str!("../generated_queries/wing/injections.scm");
pub const WING_LOCALS: &str = include_str!("../generated_queries/wing/locals.scm");
pub const WING_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/wing/highlights_crates_io.scm");
pub const WING_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/wing/injections_crates_io.scm");
pub const WING_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/wing/locals_crates_io.scm");

pub const WIT_HIGHLIGHTS: &str = include_str!("../generated_queries/wit/highlights.scm");
pub const WIT_INJECTIONS: &str = include_str!("../generated_queries/wit/injections.scm");
pub const WIT_LOCALS: &str = include_str!("../generated_queries/wit/locals.scm");
pub const WIT_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/wit/highlights_crates_io.scm");
pub const WIT_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/wit/injections_crates_io.scm");
pub const WIT_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/wit/locals_crates_io.scm");

pub const WXML_HIGHLIGHTS: &str = include_str!("../generated_queries/wxml/highlights.scm");
pub const WXML_INJECTIONS: &str = include_str!("../generated_queries/wxml/injections.scm");
pub const WXML_LOCALS: &str = include_str!("../generated_queries/wxml/locals.scm");
pub const WXML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/wxml/highlights_crates_io.scm");
pub const WXML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/wxml/injections_crates_io.scm");
pub const WXML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/wxml/locals_crates_io.scm");

pub const XCOMPOSE_HIGHLIGHTS: &str = include_str!("../generated_queries/xcompose/highlights.scm");
pub const XCOMPOSE_INJECTIONS: &str = include_str!("../generated_queries/xcompose/injections.scm");
pub const XCOMPOSE_LOCALS: &str = include_str!("../generated_queries/xcompose/locals.scm");
pub const XCOMPOSE_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/xcompose/highlights_crates_io.scm");
pub const XCOMPOSE_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/xcompose/injections_crates_io.scm");
pub const XCOMPOSE_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/xcompose/locals_crates_io.scm");

pub const XDEFAULTS_HIGHLIGHTS: &str = include_str!("../generated_queries/xdefaults/highlights.scm");
pub const XDEFAULTS_INJECTIONS: &str = include_str!("../generated_queries/xdefaults/injections.scm");
pub const XDEFAULTS_LOCALS: &str = include_str!("../generated_queries/xdefaults/locals.scm");
pub const XDEFAULTS_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/xdefaults/highlights_crates_io.scm");
pub const XDEFAULTS_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/xdefaults/injections_crates_io.scm");
pub const XDEFAULTS_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/xdefaults/locals_crates_io.scm");

pub const XML_HIGHLIGHTS: &str = include_str!("../generated_queries/xml/highlights.scm");
pub const XML_INJECTIONS: &str = include_str!("../generated_queries/xml/injections.scm");
pub const XML_LOCALS: &str = include_str!("../generated_queries/xml/locals.scm");
pub const XML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/xml/highlights_crates_io.scm");
pub const XML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/xml/injections_crates_io.scm");
pub const XML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/xml/locals_crates_io.scm");

pub const YAML_HIGHLIGHTS: &str = include_str!("../generated_queries/yaml/highlights.scm");
pub const YAML_INJECTIONS: &str = include_str!("../generated_queries/yaml/injections.scm");
pub const YAML_LOCALS: &str = include_str!("../generated_queries/yaml/locals.scm");
pub const YAML_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/yaml/highlights_crates_io.scm");
pub const YAML_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/yaml/injections_crates_io.scm");
pub const YAML_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/yaml/locals_crates_io.scm");

pub const YANG_HIGHLIGHTS: &str = include_str!("../generated_queries/yang/highlights.scm");
pub const YANG_INJECTIONS: &str = include_str!("../generated_queries/yang/injections.scm");
pub const YANG_LOCALS: &str = include_str!("../generated_queries/yang/locals.scm");
pub const YANG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/yang/highlights_crates_io.scm");
pub const YANG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/yang/injections_crates_io.scm");
pub const YANG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/yang/locals_crates_io.scm");

pub const YUCK_HIGHLIGHTS: &str = include_str!("../generated_queries/yuck/highlights.scm");
pub const YUCK_INJECTIONS: &str = include_str!("../generated_queries/yuck/injections.scm");
pub const YUCK_LOCALS: &str = include_str!("../generated_queries/yuck/locals.scm");
pub const YUCK_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/yuck/highlights_crates_io.scm");
pub const YUCK_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/yuck/injections_crates_io.scm");
pub const YUCK_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/yuck/locals_crates_io.scm");

pub const ZATHURARC_HIGHLIGHTS: &str = include_str!("../generated_queries/zathurarc/highlights.scm");
pub const ZATHURARC_INJECTIONS: &str = include_str!("../generated_queries/zathurarc/injections.scm");
pub const ZATHURARC_LOCALS: &str = include_str!("../generated_queries/zathurarc/locals.scm");
pub const ZATHURARC_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/zathurarc/highlights_crates_io.scm");
pub const ZATHURARC_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/zathurarc/injections_crates_io.scm");
pub const ZATHURARC_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/zathurarc/locals_crates_io.scm");

pub const ZIG_HIGHLIGHTS: &str = include_str!("../generated_queries/zig/highlights.scm");
pub const ZIG_INJECTIONS: &str = include_str!("../generated_queries/zig/injections.scm");
pub const ZIG_LOCALS: &str = include_str!("../generated_queries/zig/locals.scm");
pub const ZIG_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/zig/highlights_crates_io.scm");
pub const ZIG_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/zig/injections_crates_io.scm");
pub const ZIG_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/zig/locals_crates_io.scm");

pub const ZIGGY_HIGHLIGHTS: &str = include_str!("../generated_queries/ziggy/highlights.scm");
pub const ZIGGY_INJECTIONS: &str = include_str!("../generated_queries/ziggy/injections.scm");
pub const ZIGGY_LOCALS: &str = include_str!("../generated_queries/ziggy/locals.scm");
pub const ZIGGY_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ziggy/highlights_crates_io.scm");
pub const ZIGGY_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ziggy/injections_crates_io.scm");
pub const ZIGGY_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ziggy/locals_crates_io.scm");

pub const ZIGGY_SCHEMA_HIGHLIGHTS: &str = include_str!("../generated_queries/ziggy_schema/highlights.scm");
pub const ZIGGY_SCHEMA_INJECTIONS: &str = include_str!("../generated_queries/ziggy_schema/injections.scm");
pub const ZIGGY_SCHEMA_LOCALS: &str = include_str!("../generated_queries/ziggy_schema/locals.scm");
pub const ZIGGY_SCHEMA_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/ziggy_schema/highlights_crates_io.scm");
pub const ZIGGY_SCHEMA_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/ziggy_schema/injections_crates_io.scm");
pub const ZIGGY_SCHEMA_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/ziggy_schema/locals_crates_io.scm");

pub const ZSH_HIGHLIGHTS: &str = include_str!("../generated_queries/zsh/highlights.scm");
pub const ZSH_INJECTIONS: &str = include_str!("../generated_queries/zsh/injections.scm");
pub const ZSH_LOCALS: &str = include_str!("../generated_queries/zsh/locals.scm");
pub const ZSH_HIGHLIGHTS_CRATES_IO: &str = include_str!("../generated_queries/zsh/highlights_crates_io.scm");
pub const ZSH_INJECTIONS_CRATES_IO: &str = include_str!("../generated_queries/zsh/injections_crates_io.scm");
pub const ZSH_LOCALS_CRATES_IO: &str = include_str!("../generated_queries/zsh/locals_crates_io.scm");
