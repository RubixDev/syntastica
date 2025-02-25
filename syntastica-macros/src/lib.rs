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

#[proc_macro]
pub fn parsers_git(_: TokenStream) -> TokenStream {
    let mut dedup_ffi_funcs = LANGUAGE_CONFIG.languages.clone();
    dedup_ffi_funcs.sort_unstable_by_key(|lang| lang.parser.ffi_func.clone());
    dedup_ffi_funcs.dedup_by_key(|lang| lang.parser.ffi_func.clone());
    dedup_ffi_funcs
        .iter()
        .map(|lang| {
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
            quote! {
                #[cfg(feature = #name)]
                compile_parser(#name, #url, #rev, #external_c, #external_cpp, #path, #wasm, #wasm_unknown)?;
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
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

#[proc_macro]
pub fn parsers_ffi(_: TokenStream) -> TokenStream {
    let mut dedup_ffi_funcs = LANGUAGE_CONFIG.languages.clone();
    dedup_ffi_funcs.sort_unstable_by_key(|lang| lang.parser.ffi_func.clone());
    dedup_ffi_funcs.dedup_by_key(|lang| lang.parser.ffi_func.clone());
    let extern_c = dedup_ffi_funcs.iter().map(|lang| {
        let name_str = &lang.name;
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        let not_wasm_cfg = not_wasm_cfg(lang);
        quote! {
            #[cfg(all(feature = #name_str #not_wasm_cfg))]
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
        let (doc, body) = match &lang.parser.rust_const {
            Some(ident) if (!crates_io || lang.parser.crates_io.is_some()) => {
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
        |lang| lang.parser.rust_const.is_some() && (!crates_io || lang.parser.crates_io.is_some()),
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

        fn __get_language(idx: usize) -> Result<HighlightConfiguration> {
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
                self.0[idx].get_or_try_init(|| __get_language(idx))
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
                        drop(entry.set(__get_language(idx)?));
                    }
                }
                Ok(())
            }

            /// Pre-load all languages in this set.
            ///
            /// To pre-load a specific set of languages, use [`preload`](LanguageSetImpl::preload).
            pub fn preload_all(&mut self) -> Result<()> {
                self.preload(LANGUAGES)
            }
        }

        impl Default for LanguageSetImpl {
            fn default() -> Self {
                Self::new()
            }
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
        .filter(|lang| lang.parser.rust_const.is_some() && lang.parser.crates_io.is_some())
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
