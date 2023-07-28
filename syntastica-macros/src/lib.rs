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
            quote! {
                #[cfg(feature = #name)]
                compile_parser(#name, #url, #rev, #external_c, #external_cpp, #path, #wasm)?;
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn parsers_ffi(_: TokenStream) -> TokenStream {
    let mut dedup_ffi_funcs = LANGUAGE_CONFIG.languages.clone();
    dedup_ffi_funcs.sort_unstable_by_key(|lang| lang.parser.ffi_func.clone());
    dedup_ffi_funcs.dedup_by_key(|lang| lang.parser.ffi_func.clone());
    let extern_c = dedup_ffi_funcs.iter().map(|lang| {
        let name_str = &lang.name;
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        let wasm_cfg = if !lang.wasm {
            // disable some parsers on wasm targets
            quote! { , not(target_family = "wasm") }
        } else if lang.parser.external_scanner.cpp {
            // disable cpp scanners on wasm32-unknown-unknown
            quote! { , not(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", target_env = "")) }
        } else {
            quote! {}
        };
        quote! {
            #[cfg(all(feature = #name_str #wasm_cfg))]
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
            lang.name, lang.parser.git.url, lang.parser.git.rev,
            match (lang.wasm, lang.parser.external_scanner.cpp) {
                (false, _) => "(not supported on WebAssembly targets)",
                (_, true) => "(not supported on the `wasm32-unknown-unknown` target)",
                _ => "",
            },
        );
        // disable cpp scanners on wasm32-unknown-unknown
        let raw_wasm_cfg = quote! { target_family = "wasm" };
        let raw_wasm_cfg_cpp = quote! { all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", target_env = "") };
        let wasm_cfg = match (lang.wasm, lang.parser.external_scanner.cpp) {
            (false, _) => quote! { , #raw_wasm_cfg },
            (_, true) => quote! { , #raw_wasm_cfg_cpp },
            _ => quote! {},
        };
        let not_wasm_cfg = match (lang.wasm, lang.parser.external_scanner.cpp) {
            (false, _) => quote! { , not(#raw_wasm_cfg) },
            (_, true) => quote! { , not(#raw_wasm_cfg_cpp) },
            _ => quote! {},
        };
        quote! {
            #[cfg(any(feature = #feat, feature = #name_str))]
            #[doc = #doc]
            pub fn #name() -> ::syntastica_core::language_set::Language {
                #[cfg(all(not(all(feature = "docs", doc)) #not_wasm_cfg))]
                unsafe { #ffi_func() }
                #[cfg(any(all(feature = "docs", doc) #wasm_cfg))]
                ::std::unimplemented!()
            }
        }
    });
    parsers(
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
    parsers_rust(false, "")
}

#[proc_macro]
pub fn parsers_dep(_: TokenStream) -> TokenStream {
    parsers_rust(true, "_CRATES_IO")
}

fn parsers_rust(crates_io: bool, query_suffix: &str) -> TokenStream {
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let name_str = &lang.name;
        let (doc, body) = match &lang.parser.rust_func {
            Some(func) if (!crates_io || lang.parser.crates_io.is_some()) => {
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
        functions,
        |lang| lang.parser.rust_func.is_some() && (!crates_io || lang.parser.crates_io.is_some()),
        None,
        query_suffix,
    )
}

fn parsers(
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
            quote! { #[cfg(feature = #name_str)] #name_str }
        });
    let extensions = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .flat_map(|lang| {
            let name_str = &lang.name;
            lang.file_extensions
                .iter()
                .map(move |ext| quote! { #[cfg(feature = #name_str)] (#ext, #name_str) })
        });
    let mut langs_sorted_by_group = LANGUAGE_CONFIG.languages.clone();
    langs_sorted_by_group.sort_by_key(|lang| lang.group);
    let func_map = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let name_str = &lang.name;
        quote! {
            #[cfg(feature = #name_str)]
            {
                _map.insert(#name_str, _idx);
                _idx += 1;
            }
        }
    });
    let funcs = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let name = format_ident!("{}", lang.name);
        let name_str = &lang.name;
        quote! { #[cfg(feature = #name_str)] &#name }
    });
    let queries = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let name_str = &lang.name;
        let highlights = format_ident!("{}_HIGHLIGHTS{query_suffix}", lang.name.to_uppercase());
        let injections = format_ident!("{}_INJECTIONS{query_suffix}", lang.name.to_uppercase());
        let locals = format_ident!("{}_LOCALS{query_suffix}", lang.name.to_uppercase());

        quote! { #[cfg(feature = #name_str)] [
            ::syntastica_queries::#highlights,
            ::syntastica_queries::#injections,
            ::syntastica_queries::#locals,
        ] }
    });
    let precomp = langs_sorted_by_group.iter().map(|lang| {
        let name_str = &lang.name;
        let path = format!("../precomp/{name_str}.bin");
        quote! { #[cfg(feature = #name_str)] include_bytes!(#path) }
    });

    let cfg_any = quote! { #[cfg(any(feature = "language-set", feature = "raw-language-set"))] };
    let cfg_precomp = quote! { #[cfg(feature = "language-set")] };
    let cfg_raw = quote! { #[cfg(feature = "raw-language-set")] };

    quote_use! {
        # use std::{borrow::Cow, cell::UnsafeCell, collections::HashMap};

        # use syntastica_core::{
            language_set::{HighlightConfiguration, LanguageSet, Language},
            Error, Result,
            theme::THEME_KEYS,
        };
        # use once_cell::sync::Lazy;

        #extra

        /// A list of all language names that are supported by this parser collection.
        pub const LANGUAGES: &[&str] = &[#(#list),*];
        #cfg_any
        const LANG_COUNT: usize = LANGUAGES.len();

        #(#functions)*

        // TODO: use "perfect" hashmap with compile-time known keys
        #cfg_any
        static EXTENSION_MAP: Lazy<HashMap<&'static str, &'static str>>
            = Lazy::new(|| HashMap::from([#(#extensions),*]));

        // TODO: use "perfect" hashmap with compile-time known keys
        static IDX_MAP: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
            let mut _map = HashMap::new();
            let mut _idx = 0;
            #(#func_map)*
            _map
        });

        #cfg_raw
        const QUERIES: &[[&str; 3]] = &[#(#queries),*];
        #cfg_precomp
        const PRECOMP: &[&[u8]] = &[#(#precomp),*];
        const FUNCS: &[&dyn Fn() -> Language] = &[#(#funcs),*];

        #cfg_raw
        fn __get_conf(idx: usize) -> Result<HighlightConfiguration> {
            let lang = FUNCS[idx]();
            let mut conf = HighlightConfiguration::new(
                lang,
                QUERIES[idx][0],
                QUERIES[idx][1],
                QUERIES[idx][2],
            )?;
            conf.configure(THEME_KEYS);
            Ok(conf)
        }

        #cfg_precomp
        fn __get_conf_precomp(idx: usize) -> Result<HighlightConfiguration> {
            let lang = FUNCS[idx]();
            let mut conf = HighlightConfiguration::deserialize(PRECOMP[idx], lang)?;
            conf.configure(THEME_KEYS);
            Ok(conf)
        }

        /// Try to get a language based on its name.
        ///
        /// For any string in [`LANGUAGES`] this will return the respective
        /// [`Language`](syntastica_core::language_set::Language), for any other string this will
        /// return `None`.
        pub fn from_name(name: &str) -> Option<Language> {
            IDX_MAP.get(&name).map(|&idx| FUNCS[idx]())
        }

        // TODO: maybe create enum with all supported languages

        #cfg_any
        struct LanguageSetInner(UnsafeCell<[Option<HighlightConfiguration>; LANG_COUNT]>);

        #cfg_any
        const INIT: Option<HighlightConfiguration> = None;
        #cfg_any
        impl LanguageSetInner {
            fn new() -> Self {
                Self(UnsafeCell::new([INIT; LANG_COUNT]))
            }

            fn get_language(
                &self,
                name: &str,
                init: impl Fn(usize) -> Result<HighlightConfiguration>,
            ) -> Result<&HighlightConfiguration> {
                if let Some(idx) = IDX_MAP.get(&name) {
                    // SAFETY: We only ever give out shared references to list entries, and only
                    // after they have been initialized. As such it is safe to mutate an entry
                    // which is still `None` and then give out a shared reference.
                    let list = unsafe { self.0.get().as_ref() }.unwrap();
                    match list[*idx].as_ref() {
                        Some(config) => Ok(config),
                        None => {
                            // SAFETY: see above
                            let list = unsafe { self.0.get().as_mut() }.unwrap();
                            let conf = init(*idx)?;
                            list[*idx] = Some(conf);
                            Ok(list[*idx].as_ref().unwrap())
                        }
                    }
                } else {
                    Err(Error::UnsupportedLanguage(name.to_owned()))
                }
            }

            fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>> {
                EXTENSION_MAP
                    .get(&file_extension)
                    .map(|name| (*name).into())
            }

            fn preload(
                &mut self,
                languages: &[&str],
                init: impl Fn(usize) -> Result<HighlightConfiguration>,
            ) -> Result<()> {
                for lang in languages {
                    match IDX_MAP.get(lang) {
                        Some(idx) => {
                            let entry = &mut self.0.get_mut()[*idx];
                            if entry.is_none() {
                                *entry = Some(init(*idx)?);
                            }
                        }
                        None => return Err(Error::UnsupportedLanguage(lang.to_string())),
                    }
                }
                Ok(())
            }

            fn preload_all(&mut self, init: impl Fn(usize) -> Result<HighlightConfiguration>) {
                self.preload(LANGUAGES, init)
                    .expect("constant `LANGUAGES` list should only contain valid names")
            }
        }

        #cfg_any
        impl Default for LanguageSetInner {
            fn default() -> Self {
                Self::new()
            }
        }

        /// An implementation of [`LanguageSet`](syntastica_core::language_set::LanguageSet)
        /// including all languages in the enabled feature set.
        ///
        /// Languages are loaded the first time they are requested and will then be reused for
        /// later accesses. To pre-load a list of languages, use
        /// [`preload`](LanguageSetImpl::preload) or [`preload_all`](LanguageSetImpl::preload_all).
        #[derive(Default)]
        #cfg_precomp
        pub struct LanguageSetImpl(LanguageSetInner);

        #cfg_precomp
        impl LanguageSet for LanguageSetImpl {
            #[inline]
            fn get_language(&self, name: &str) -> syntastica_core::Result<&HighlightConfiguration> {
                self.0.get_language(name, __get_conf_precomp)
            }

            #[inline]
            fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>> {
                self.0.for_extension(file_extension)
            }
        }

        #cfg_precomp
        impl LanguageSetImpl {
            /// Create a new [`LanguageSetImpl`] with no pre-loaded languages.
            #[inline]
            pub fn new() -> Self {
                Self::default()
            }

            /// Pre-load the given list of languages.
            ///
            /// To pre-load all supported languages, use [`preload_all`](LanguageSetImpl::preload_all).
            ///
            /// # Errors
            /// If the `languages` list contains a name of a language that is not included in this set, an
            /// [`Error::UnsupportedLanguage`] error is returned and no further languages are loaded.
            #[inline]
            pub fn preload(&mut self, languages: &[&str]) -> Result<()> {
                self.0.preload(languages, __get_conf_precomp)
            }

            /// Pre-load all languages in this set.
            ///
            /// To pre-load a specific set of languages, use [`preload`](LanguageSetImpl::preload).
            #[inline]
            pub fn preload_all(&mut self) {
                self.0.preload_all(__get_conf_precomp)
            }
        }

        /// An implementation of [`LanguageSet`](syntastica_core::language_set::LanguageSet)
        /// including all languages in the enabled feature set.
        ///
        /// Languages are loaded the first time they are requested and will then be reused for
        /// later accesses. To pre-load a list of languages, use
        /// [`preload`](RawLanguageSetImpl::preload) or [`preload_all`](RawLanguageSetImpl::preload_all).
        #[derive(Default)]
        #cfg_raw
        pub struct RawLanguageSetImpl(LanguageSetInner);

        #cfg_raw
        impl LanguageSet for RawLanguageSetImpl {
            #[inline]
            fn get_language(&self, name: &str) -> Result<&HighlightConfiguration> {
                self.0.get_language(name, __get_conf)
            }

            #[inline]
            fn for_extension<'a>(&self, file_extension: &'a str) -> Option<Cow<'a, str>> {
                self.0.for_extension(file_extension)
            }
        }

        #cfg_raw
        impl RawLanguageSetImpl {
            /// Create a new [`RawLanguageSetImpl`] with no pre-loaded languages.
            #[inline]
            pub fn new() -> Self {
                Self::default()
            }

            /// Pre-load the given list of languages.
            ///
            /// To pre-load all supported languages, use [`preload_all`](RawLanguageSetImpl::preload_all).
            ///
            /// # Errors
            /// If the `languages` list contains a name of a language that is not included in this set, an
            /// [`Error::UnsupportedLanguage`] error is returned and no further languages are loaded.
            #[inline]
            pub fn preload(&mut self, languages: &[&str]) -> Result<()> {
                self.0.preload(languages, __get_conf)
            }

            /// Pre-load all languages in this set.
            ///
            /// To pre-load a specific set of languages, use [`preload`](RawLanguageSetImpl::preload).
            #[inline]
            pub fn preload_all(&mut self) {
                self.0.preload_all(__get_conf)
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
                    validate_query(lang, ::syntastica_queries::#highlights, "highlights");
                    validate_query(lang, ::syntastica_queries::#injections, "injections");
                    validate_query(lang, ::syntastica_queries::#locals, "locals");
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
        .filter(|lang| lang.parser.rust_func.is_some() && lang.parser.crates_io.is_some())
        .map(|lang| {
            let name = format_ident!("{}", lang.name);
            let highlights = format_ident!("{}_HIGHLIGHTS_CRATES_IO", lang.name.to_uppercase());
            let injections = format_ident!("{}_INJECTIONS_CRATES_IO", lang.name.to_uppercase());
            let locals = format_ident!("{}_LOCALS_CRATES_IO", lang.name.to_uppercase());
            quote! {
                #[test]
                fn #name() {
                    let lang = ::syntastica_parsers::#name();
                    validate_query(lang, ::syntastica_queries::#highlights, "highlights");
                    validate_query(lang, ::syntastica_queries::#injections, "injections");
                    validate_query(lang, ::syntastica_queries::#locals, "locals");
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}
