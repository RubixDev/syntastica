use std::borrow::Cow;

use heck::ToPascalCase;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use quote_use::quote_use;
use schema::*;

mod schema;

static LANGUAGE_CONFIG: Lazy<LanguageConfig> = Lazy::new(|| {
    toml::from_str(include_str!("../languages.toml")).expect("invalid `languages.toml`")
});

#[derive(Debug)]
struct FfiGroup<'a> {
    representative: &'a Language,
    aliases: Vec<&'a Language>,
}

fn ffi_groups(languages: &[Language]) -> Vec<FfiGroup<'_>> {
    let mut group_indices: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::new();
    let mut groups: Vec<FfiGroup<'_>> = Vec::new();

    for lang in languages {
        if let Some(&idx) = group_indices.get(lang.parser.ffi_func.as_str()) {
            groups[idx].aliases.push(lang);
        } else {
            group_indices.insert(lang.parser.ffi_func.as_str(), groups.len());
            groups.push(FfiGroup {
                representative: lang,
                aliases: vec![lang],
            });
        }
    }

    groups
}

fn not_wasm_cfg(lang: &Language) -> proc_macro2::TokenStream {
    let raw_wasm_cfg = quote! { target_family = "wasm" };
    let raw_wasm_unknown_cfg = quote! { all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", target_env = "") };
    match (
        lang.wasm,
        lang.parser.external_scanner.cpp || !lang.wasm_unknown,
    ) {
        (false, _) => quote! { , not(#raw_wasm_cfg) },
        (_, true) => quote! { , not(#raw_wasm_unknown_cfg) },
        _ => quote! {},
    }
}

fn lang_feature_target_cfg(lang: &Language) -> proc_macro2::TokenStream {
    let name_str = &lang.name;
    let not_wasm_cfg = not_wasm_cfg(lang);
    quote! { all(feature = #name_str #not_wasm_cfg) }
}

fn ffi_group_cfg(group: &FfiGroup<'_>) -> proc_macro2::TokenStream {
    let alias_cfgs = group
        .aliases
        .iter()
        .map(|lang| lang_feature_target_cfg(lang))
        .collect::<Vec<_>>();

    match alias_cfgs.as_slice() {
        [cfg] => cfg.clone(),
        _ => quote! { any(#(#alias_cfgs),*) },
    }
}

#[proc_macro]
pub fn parsers_git(_: TokenStream) -> TokenStream {
    ffi_groups(&LANGUAGE_CONFIG.languages)
        .iter()
        .map(|group| {
            // The first language keeps the canonical build metadata for the shared FFI symbol;
            // additional aliases only extend the feature gate.
            let lang = group.representative;
            let cfg = ffi_group_cfg(group);
            let name = &lang.name;
            let url = &lang.parser.git.url;
            let rev = &lang.parser.git.rev;
            let external_c = lang.parser.external_scanner.c;
            let external_cpp = lang.parser.external_scanner.cpp;
            let path = match &lang.parser.git.path {
                Some(path) => quote_use! { Some(#path) },
                None => quote_use! { None },
            };
            let wasm = lang.wasm;
            let wasm_unknown = lang.wasm_unknown;
            let generate = lang.parser.generate;
            quote! {
                #[cfg(#cfg)]
                compile_parser(#name, #url, #rev, #external_c, #external_cpp, #path, #wasm, #wasm_unknown, #generate)?;
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn parsers_ffi(_: TokenStream) -> TokenStream {
    let ffi_groups = ffi_groups(&LANGUAGE_CONFIG.languages);
    let extern_c = ffi_groups.iter().map(|group| {
        let ffi_func = format_ident!("{}", group.representative.parser.ffi_func);
        let cfg = ffi_group_cfg(group);
        quote! {
            #[cfg(#cfg)]
            fn #ffi_func() -> ::syntastica_core::language_set::Language;
        }
    });
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let name_str = &lang.name;
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        let doc = format!(
            "Get the parser for [{}]({}/tree/{}). {}",
            lang.name,
            lang.parser.git.url,
            lang.parser.git.rev,
            match (
                lang.wasm,
                lang.parser.external_scanner.cpp || !lang.wasm_unknown
            ) {
                (false, _) => "(not supported on WebAssembly targets)",
                (_, true) => "(not supported on the `wasm32-unknown-unknown` target)",
                _ => "",
            },
        );
        // disable unsupported parsers on wasm
        let not_wasm_cfg = not_wasm_cfg(lang);
        quote! {
            #[cfg(all(any(feature = #feat, feature = #name_str) #not_wasm_cfg))]
            #[doc = #doc]
            pub fn #name() -> ::syntastica_core::language_set::Language {
                #[cfg(not(all(feature = "docs", doc)))]
                unsafe { #ffi_func() }
                #[cfg(all(feature = "docs", doc))]
                ::std::unimplemented!()
            }
        }
    });
    parsers(
        "syntastica_parsers_git",
        functions,
        |_| true,
        Some(quote! {
            #[cfg(not(all(feature = "docs", doc)))]
            extern "C" {
                #(#extern_c)*
            }
        }),
        "",
    )
}

#[proc_macro]
pub fn parsers_gitdep(_: TokenStream) -> TokenStream {
    parsers_rust("syntastica_parsers_gitdep", false, "")
}

#[proc_macro]
pub fn parsers_dep(_: TokenStream) -> TokenStream {
    parsers_rust("syntastica_parsers", true, "_CRATES_IO")
}

fn parsers_rust(crate_name: &str, crates_io: bool, query_suffix: &str) -> TokenStream {
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let name_str = &lang.name;
        let (doc, body) = match (&lang.parser.rust_const, &lang.parser.rust_func) {
            (Some(ident), _) if lang.parser.supports(!crates_io) => {
                let ident = format_ident!("{ident}");
                let package = format_ident!("{}", lang.parser.package.replace('-', "_"));
                (
                    format!(
                        "Get the parser for [{}]({}/tree/{}).",
                        lang.name, lang.parser.git.url, lang.parser.git.rev,
                    ),
                    quote! { ::syntastica_core::language_set::Language::new(#package::#ident) }
                )
            },
            (_, Some(func)) if lang.parser.supports(!crates_io) => {
                let func = format_ident!("{func}");
                let package = format_ident!("{}", lang.parser.package.replace('-', "_"));
                (
                    format!(
                        "Get the parser for [{}]({}/tree/{}).",
                        lang.name, lang.parser.git.url, lang.parser.git.rev,
                    ),
                    quote! { #package::#func() }
                )
            },
            _ => (
                "**This parser is not supported by this parser collection and thus this function will panic!**"
                    .to_owned(),
                quote! { ::std::unimplemented!() }
            ),
        };
        quote! {
            #[cfg(any(feature = #feat, feature = #name_str))]
            #[doc = #doc]
            pub fn #name() -> ::syntastica_core::language_set::Language {
                #body
            }
        }
    });
    parsers(
        crate_name,
        functions,
        |lang| lang.parser.supports(!crates_io),
        None,
        query_suffix,
    )
}

fn parsers(
    crate_name: &str,
    functions: impl Iterator<Item = proc_macro2::TokenStream>,
    filter: impl Fn(&&Language) -> bool,
    extra: Option<proc_macro2::TokenStream>,
    query_suffix: &str,
) -> TokenStream {
    let list = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let name_str = &lang.name;
            let variant = format_ident!("{}", lang.name.to_pascal_case());
            let not_wasm_cfg = not_wasm_cfg(lang);
            quote! { #[cfg(all(feature = #name_str #not_wasm_cfg))] Lang::#variant }
        });
    let names_list = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let name_str = &lang.name;
            let not_wasm_cfg = not_wasm_cfg(lang);
            quote! { #[cfg(all(feature = #name_str #not_wasm_cfg))] #name_str }
        });
    let file_types = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .flat_map(|lang| {
            let name_str = &lang.name;
            let variant = format_ident!("{}", lang.name.to_pascal_case());
            let not_wasm_cfg = not_wasm_cfg(lang);
            lang.file_types.iter().map(move |ft| {
                let ft = format_ident!("{ft:?}");
                quote! {
                    #[cfg(all(feature = #name_str #not_wasm_cfg))]
                    (::syntastica_core::language_set::FileType::#ft, Lang::#variant)
                }
            })
        });
    let mut langs_sorted_by_group = LANGUAGE_CONFIG.languages.clone();
    langs_sorted_by_group.sort_by_key(|lang| lang.group);
    let func_map = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let name_str = &lang.name;
        let variant = format_ident!("{}", lang.name.to_pascal_case());
        let not_wasm_cfg = not_wasm_cfg(lang);
        quote! {
            #[cfg(all(feature = #name_str #not_wasm_cfg))]
            {
                _map.insert(Lang::#variant, _idx);
                _idx += 1;
            }
        }
    });
    let funcs = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let name = format_ident!("{}", lang.name);
        let name_str = &lang.name;
        let not_wasm_cfg = not_wasm_cfg(lang);
        quote! { #[cfg(all(feature = #name_str #not_wasm_cfg))] #name }
    });
    let queries = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let name_str = &lang.name;
        let highlights = format_ident!("{}_HIGHLIGHTS{query_suffix}", lang.name.to_uppercase());
        let injections = format_ident!("{}_INJECTIONS{query_suffix}", lang.name.to_uppercase());
        let locals = format_ident!("{}_LOCALS{query_suffix}", lang.name.to_uppercase());
        let not_wasm_cfg = not_wasm_cfg(lang);

        quote! { #[cfg(all(feature = #name_str #not_wasm_cfg))] [
            ::syntastica_queries::#highlights,
            ::syntastica_queries::#injections,
            ::syntastica_queries::#locals,
        ] }
    });
    let lang_enum_example_use = format!("use {crate_name}::{{Lang, LANGUAGES, LANGUAGE_NAMES}};");
    let lang_enum = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let feat = lang.group.to_string();
            let name_str = &lang.name;
            let ft_support = if lang.file_types.is_empty() {
                Cow::Borrowed("supports no file types")
            } else {
                format!(
                    "supports these file types: {}",
                    lang.file_types
                        .iter()
                        .map(|ft| format!(
                            "[`{ft}`](::syntastica_core::language_set::FileType::{ft:?})"
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .into()
            };
            let doc = format!("Provides the [`{name_str}`] language, {ft_support}.");
            let variant = format_ident!("{}", lang.name.to_pascal_case());
            let not_wasm_cfg = not_wasm_cfg(lang);
            quote! {
                #[doc = #doc]
                #[cfg(all(any(feature = #feat, feature = #name_str) #not_wasm_cfg))]
                #variant
            }
        });
    let lang_get_match = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let name = format_ident!("{}", lang.name);
            let name_str = &lang.name;
            let variant = format_ident!("{}", lang.name.to_pascal_case());
            let not_wasm_cfg = not_wasm_cfg(lang);
            quote! {
                #[cfg(all(feature = #name_str #not_wasm_cfg))]
                Self::#variant => #name(),
            }
        });
    let lang_set_type = quote! { type Language = Lang; };
    let cfg_test = quote! { #[cfg(test)] };
    let lang_tests = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let name = format_ident!("{}", lang.name);
            let name_str = &lang.name;
            let variant = format_ident!("{}", lang.name.to_pascal_case());
            let not_wasm_cfg = not_wasm_cfg(lang);
            quote! {
                #[test]
                #[cfg(all(feature = #name_str #not_wasm_cfg))]
                fn #name() {
                    assert_eq!(crate::#name(), crate::Lang::#variant.get());
                    let set = crate::LanguageSetImpl::new();
                    let result = ::syntastica_core::language_set::LanguageSet::get_language(
                        &set,
                        crate::Lang::#variant,
                    );
                    assert!(
                        result.is_ok(),
                        "get_language failed for {}: {:?}",
                        #name_str,
                        result.as_ref().err()
                    );
                }
            }
        });

    quote_use! {
        # use std::{borrow::Cow, collections::HashMap};

        # use syntastica_core::{
            language_set::{HighlightConfiguration, LanguageSet, Language, FileType, SupportedLanguage},
            Error, Result,
            theme::THEME_KEYS,
        };
        # use once_cell::sync::{Lazy, OnceCell};

        #extra

        /// A list of all languages supported by the current feature set.
        pub const LANGUAGES: &[Lang] = &[#(#list),*];
        const LANG_COUNT: usize = LANGUAGES.len();

        /// A list of all language names supported by the current feature set.
        pub const LANGUAGE_NAMES: &[&str] = &[#(#names_list),*];

        #(#functions)*

        // TODO: use "perfect" hashmap with compile-time known keys
        static FILE_TYPE_MAP: Lazy<HashMap<FileType, Lang>>
            = Lazy::new(|| HashMap::from([#(#file_types),*]));

        // TODO: use "perfect" hashmap with compile-time known keys
        static IDX_MAP: Lazy<HashMap<Lang, usize>> = Lazy::new(|| {
            let mut _map = HashMap::new();
            let mut _idx = 0;
            #(#func_map)*
            _map
        });

        const QUERIES: &[[&str; 3]] = &[#(#queries),*];
        const FUNCS: &[fn() -> Language] = &[#(#funcs),*];

        /// An enum of every supported language in the current feature set.
        ///
        /// An instance of the respective tree-stter
        /// [`Language`](::syntastica_core::language_set::Language) can be obtained with the
        /// [`get`](Lang::get) method.
        ///
        /// You can also get a [`Lang`] from its name using
        /// [`for_name`](::syntastica_core::language_set::SupportedLanguage::for_name), or for a
        /// [`FileType`](::syntastica_core::language_set::FileType) using
        /// [`for_file_type`](::syntastica_core::language_set::SupportedLanguage::for_file_type).
        /// See the docs for each variant to see its "name" and the supported file types.
        /// Both of these require the
        /// [`SupportedLanguage`](::syntastica_core::language_set::SupportedLanguage) trait to be
        /// in scope.
        ///
        /// See [`LANGUAGES`] for a list containing all variants and [`LANGUAGE_NAMES`] for a list
        /// of all valid names.
        ///
        /// The enum is marked as non-exhaustive for two reasons:
        ///
        /// 1. New languages may be added in the future
        /// 2. The variants are enabled/disabled by features
        ///
        /// # Example
        ///
        /// ```
        #[doc = #lang_enum_example_use]
        /// use syntastica_core::language_set::{SupportedLanguage, FileType};
        ///
        /// // you can get a `Lang` from its name
        /// assert_eq!(Lang::Rust, Lang::for_name("rust", &()).unwrap());
        /// // and for a file type
        /// assert_eq!(Some(Lang::Rust), Lang::for_file_type(FileType::Rust, &()));
        ///
        /// // `LANGUAGES` is a list of all variants,
        /// // `LANGUAGE_NAMES` is a list of all variant names
        /// for (&lang, &name) in LANGUAGES.iter().zip(LANGUAGE_NAMES) {
        ///     assert_eq!(lang, Lang::for_name(name, &()).unwrap());
        ///
        ///     // `Lang` instances can be turned into strings
        ///     assert_eq!(lang, Lang::for_name(<Lang as SupportedLanguage<'_, ()>>::name(&lang), &()).unwrap());
        ///     assert_eq!(lang, Lang::for_name(lang.to_string(), &()).unwrap());
        ///     assert_eq!(lang, Lang::for_name(lang.as_ref(), &()).unwrap());
        ///     let lang_name: &'static str = lang.into();
        ///     assert_eq!(lang, Lang::for_name(lang_name, &()).unwrap());
        /// }
        /// ```
        #[non_exhaustive]
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            ::strum::Display,
            ::strum::AsRefStr,
            ::strum::IntoStaticStr,
            ::strum::EnumString,
        )]
        #[strum(serialize_all = "snake_case", use_phf)]
        pub enum Lang {
            #(#lang_enum),*
        }

        impl Lang {
            /// Get an instance of the respective
            /// [`Language`](::syntastica_core::language_set::Language).
            pub fn get(&self) -> Language {
                match self {
                    #(#lang_get_match)*
                    _ => unreachable!("all variants are matched")
                }
            }

            /// Create an instance of the corresponding
            /// [`HighlightConfiguration`](::syntastica_core::language_set::HighlightConfiguration).
            pub fn get_config(&self) -> Result<HighlightConfiguration> {
                let idx = IDX_MAP[self];
                let lang = FUNCS[idx]();
                let mut conf = HighlightConfiguration::new(
                    lang,
                    LANGUAGE_NAMES[idx],
                    QUERIES[idx][0],
                    QUERIES[idx][1],
                    QUERIES[idx][2],
                )?;
                conf.configure(THEME_KEYS);
                Ok(conf)
            }

            /// Get the highlights query for this language.
            pub fn highlights_query(&self) -> &'static str {
                let idx = IDX_MAP[self];
                QUERIES[idx][0]
            }

            /// Get the injections query for this language.
            pub fn injections_query(&self) -> &'static str {
                let idx = IDX_MAP[self];
                QUERIES[idx][1]
            }

            /// Get the locals query for this language.
            pub fn locals_query(&self) -> &'static str {
                let idx = IDX_MAP[self];
                QUERIES[idx][2]
            }
        }

        impl<S> SupportedLanguage<'_, S> for Lang {
            fn name(&self) -> Cow<'_, str> {
                Cow::Borrowed(self.into())
            }

            fn for_name(name: impl AsRef<str>, _set: &S) -> Result<Self> {
                <Self as ::std::str::FromStr>::from_str(name.as_ref())
                    .map_err(|_| Error::UnsupportedLanguage(name.as_ref().to_owned()))
            }

            fn for_file_type(file_type: FileType, _set: &S) -> Option<Self> {
                FILE_TYPE_MAP
                    .get(&file_type)
                    .map(|name| (*name).into())
            }
        }

        /// An implementation of [`LanguageSet`](::syntastica_core::language_set::LanguageSet)
        /// including all languages in the enabled feature set.
        ///
        /// Languages are loaded the first time they are requested and will then be reused for
        /// later accesses. To pre-load a list of languages, use
        /// [`preload`](LanguageSetImpl::preload) or [`preload_all`](LanguageSetImpl::preload_all).
        pub struct LanguageSetImpl([OnceCell<HighlightConfiguration>; LANG_COUNT]);

        impl LanguageSet<'_> for LanguageSetImpl {
            #lang_set_type

            fn get_language(&self, language: Self::Language) -> Result<&HighlightConfiguration> {
                let idx = IDX_MAP[&language];
                self.0[idx].get_or_try_init(|| language.get_config())
            }
        }

        impl LanguageSetImpl {
            /// Create a new [`LanguageSetImpl`] with no pre-loaded languages.
            pub fn new() -> Self {
                #[allow(clippy::declare_interior_mutable_const)]
                const INIT: OnceCell<HighlightConfiguration> = OnceCell::new();
                Self([INIT; LANG_COUNT])
            }

            /// Pre-load the given list of languages.
            ///
            /// To pre-load all supported languages, use [`preload_all`](LanguageSetImpl::preload_all).
            pub fn preload(&self, languages: &[Lang]) -> Result<()> {
                for lang in languages {
                    let idx = IDX_MAP[lang];
                    let entry = &self.0[idx];
                    if entry.get().is_none() {
                        drop(entry.set(lang.get_config()?));
                    }
                }
                Ok(())
            }

            /// Pre-load all languages in this set.
            ///
            /// To pre-load a specific set of languages, use [`preload`](LanguageSetImpl::preload).
            pub fn preload_all(&self) -> Result<()> {
                self.preload(LANGUAGES)
            }
        }

        impl Default for LanguageSetImpl {
            fn default() -> Self {
                Self::new()
            }
        }

        #cfg_test
        mod tests {
            #(#lang_tests)*
        }
    }
    .into()
}

#[proc_macro]
pub fn queries_test(_: TokenStream) -> TokenStream {
    LANGUAGE_CONFIG
        .languages
        .iter()
        .map(|lang| {
            let name = format_ident!("{}", lang.name);
            let highlights = format_ident!("{}_HIGHLIGHTS", lang.name.to_uppercase());
            let injections = format_ident!("{}_INJECTIONS", lang.name.to_uppercase());
            let locals = format_ident!("{}_LOCALS", lang.name.to_uppercase());
            quote! {
                #[test]
                fn #name() {
                    let lang = ::syntastica_parsers_git::#name();
                    validate_query(&lang, ::syntastica_queries::#highlights, "highlights");
                    validate_query(&lang, ::syntastica_queries::#injections, "injections");
                    validate_query(&lang, ::syntastica_queries::#locals, "locals");
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn queries_test_crates_io(_: TokenStream) -> TokenStream {
    LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.parser.supports_dep())
        .map(|lang| {
            let name = format_ident!("{}", lang.name);
            let highlights = format_ident!("{}_HIGHLIGHTS_CRATES_IO", lang.name.to_uppercase());
            let injections = format_ident!("{}_INJECTIONS_CRATES_IO", lang.name.to_uppercase());
            let locals = format_ident!("{}_LOCALS_CRATES_IO", lang.name.to_uppercase());
            quote! {
                #[test]
                fn #name() {
                    let lang = ::syntastica_parsers::#name();
                    validate_query(&lang, ::syntastica_queries::#highlights, "highlights");
                    validate_query(&lang, ::syntastica_queries::#injections, "injections");
                    validate_query(&lang, ::syntastica_queries::#locals, "locals");
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize_tokens(tokens: proc_macro2::TokenStream) -> String {
        tokens
            .to_string()
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect()
    }

    #[test]
    fn ffi_groups_preserve_the_first_alias_as_the_build_representative() {
        let group = ffi_groups(&LANGUAGE_CONFIG.languages)
            .into_iter()
            .find(|group| group.representative.parser.ffi_func == "tree_sitter_c_sharp")
            .expect("missing c_sharp ffi group");

        assert_eq!(group.representative.name.as_str(), "c_sharp");
        assert_eq!(
            group
                .aliases
                .iter()
                .map(|lang| lang.name.as_str())
                .collect::<Vec<_>>(),
            vec!["c_sharp", "csharp"],
        );
    }

    #[test]
    fn mixed_group_shared_ffi_cfgs_include_all_alias_features() {
        for group in ffi_groups(&LANGUAGE_CONFIG.languages)
            .into_iter()
            .filter(|group| {
                group
                    .aliases
                    .iter()
                    .map(|lang| lang.group)
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    > 1
            })
        {
            let cfg = normalize_tokens(ffi_group_cfg(&group));
            assert!(cfg.starts_with("any("), "{cfg}");

            for alias in &group.aliases {
                let feature = format!("feature=\"{}\"", alias.name);
                assert!(
                    cfg.contains(&feature),
                    "shared ffi `{}` is missing alias `{}` in cfg `{cfg}`",
                    group.representative.parser.ffi_func,
                    alias.name,
                );
            }
        }
    }

    #[test]
    fn shared_ffi_groups_keep_build_flags_in_sync() {
        for group in ffi_groups(&LANGUAGE_CONFIG.languages) {
            let representative = group.representative;
            for alias in group.aliases.iter().skip(1) {
                assert_eq!(
                    alias.parser.external_scanner, representative.parser.external_scanner,
                    "shared ffi `{}` must use the same external scanner config for `{}` and `{}`",
                    representative.parser.ffi_func, representative.name, alias.name,
                );
                assert_eq!(
                    alias.parser.git.rev, representative.parser.git.rev,
                    "shared ffi `{}` must use the same git rev for `{}` and `{}`",
                    representative.parser.ffi_func, representative.name, alias.name,
                );
                assert_eq!(
                    alias.parser.git.path, representative.parser.git.path,
                    "shared ffi `{}` must use the same parser path for `{}` and `{}`",
                    representative.parser.ffi_func, representative.name, alias.name,
                );
                assert_eq!(
                    alias.parser.generate, representative.parser.generate,
                    "shared ffi `{}` must use the same codegen setting for `{}` and `{}`",
                    representative.parser.ffi_func, representative.name, alias.name,
                );
                assert_eq!(
                    alias.wasm, representative.wasm,
                    "shared ffi `{}` must use the same wasm support for `{}` and `{}`",
                    representative.parser.ffi_func, representative.name, alias.name,
                );
                assert_eq!(
                    alias.wasm_unknown,
                    representative.wasm_unknown,
                    "shared ffi `{}` must use the same wasm32-unknown-unknown support for `{}` and `{}`",
                    representative.parser.ffi_func,
                    representative.name,
                    alias.name,
                );
            }
        }
    }
}

#[cfg(feature = "js")]
#[proc_macro]
pub fn js_lang_info(_: TokenStream) -> TokenStream {
    quote_use! {
        #use core::ffi::c_char;
        /// Information about a loaded language.
        #[repr(C)]
        pub struct LangInfo {
            name: *const c_char,
            file_types: *const *const c_char,
            file_types_len: usize,
            language: Language,
            highlights_query: *const c_char,
            injections_query: *const c_char,
            locals_query: *const c_char,
        }
    }
    .into()
}

#[cfg(feature = "js")]
#[proc_macro]
pub fn js_lang_lib(input: TokenStream) -> TokenStream {
    let lang_name = syn::parse_macro_input!(input as syn::LitStr).value();
    let lang = LANGUAGE_CONFIG
        .languages
        .iter()
        .find(|lang| lang.name == lang_name)
        .unwrap_or_else(|| panic!("language '{lang_name}' is not defined"));

    let func = format_ident!("syntastica_lang_{lang_name}");
    let ffi_func = format_ident!("{}", &lang.parser.ffi_func);
    let name = std::ffi::CString::new(lang_name.as_str()).unwrap();
    let file_types = lang
        .file_types
        .iter()
        .map(|ft| std::ffi::CString::new(ft.as_ref()).unwrap());
    let highlights = format_ident!("{}_HIGHLIGHTS", lang_name.to_uppercase());
    let injections = format_ident!("{}_INJECTIONS", lang_name.to_uppercase());
    let locals = format_ident!("{}_LOCALS", lang_name.to_uppercase());

    quote_use! {
        #use core::ffi::{c_char, c_void};

        extern "C" {
            fn malloc(size: usize) -> *mut c_void;
            fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
            fn #ffi_func() -> Language;
        }

        fn str_to_cstr(str: &'static str) -> *const c_char {
            let ptr = unsafe { malloc(str.len() + 1) };
            unsafe { memcpy(ptr, str.as_ptr() as *const _, str.len()) };
            unsafe { (ptr as *mut c_char).add(str.len()).write(0) }
            ptr as *const _
        }

        #[no_mangle]
        pub fn #func() -> *mut LangInfo {
            let ptr = unsafe { malloc(::core::mem::size_of::<LangInfo>()) } as *mut LangInfo;
            const NAME: *const c_char = #name.as_ptr();
            const FILE_TYPES: &[*const c_char] = &[#(#file_types.as_ptr()),*];
            let info = LangInfo {
                name: NAME,
                file_types: FILE_TYPES.as_ptr(),
                file_types_len: FILE_TYPES.len(),
                language: unsafe { #ffi_func() },
                highlights_query: str_to_cstr(::syntastica_queries::#highlights),
                injections_query: str_to_cstr(::syntastica_queries::#injections),
                locals_query: str_to_cstr(::syntastica_queries::#locals),
            };
            unsafe { ptr.write(info) };
            ptr
        }
    }
    .into()
}

#[cfg(feature = "js")]
#[proc_macro]
pub fn js_lang_build(input: TokenStream) -> TokenStream {
    let lang_name = syn::parse_macro_input!(input as syn::LitStr).value();
    let lang = LANGUAGE_CONFIG
        .languages
        .iter()
        .find(|lang| lang.name == lang_name)
        .unwrap_or_else(|| panic!("language '{lang_name}' is not defined"));

    let url = &lang.parser.git.url;
    let rev = &lang.parser.git.rev;
    let external_c = lang.parser.external_scanner.c;
    let external_cpp = lang.parser.external_scanner.cpp;
    let path = match &lang.parser.git.path {
        Some(path) => quote_use! { Some(#path) },
        None => quote_use! { None },
    };
    let wasm = lang.wasm;
    let wasm_unknown = lang.wasm_unknown;
    let generate = lang.parser.generate;
    quote! {
        compile_parser(#lang_name, #url, #rev, #external_c, #external_cpp, #path, #wasm, #wasm_unknown, #generate)?;
    }
    .into()
}
