use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use schema::*;

mod schema;

static LANGUAGE_CONFIG: Lazy<LanguageConfig> = Lazy::new(|| {
    toml::from_str(include_str!("../languages.toml")).expect("invalid `languages.toml`")
});

#[proc_macro]
pub fn parsers_git(_: TokenStream) -> TokenStream {
    LANGUAGE_CONFIG
        .languages
        .iter()
        .map(|lang| {
            let feat = lang.group.to_string();
            let name = &lang.name;
            let url = &lang.parser.git.url;
            let rev = &lang.parser.git.rev;
            let external_c = lang.parser.external_scanner.c;
            let external_cpp = lang.parser.external_scanner.cpp;
            let path = match &lang.parser.git.path {
                Some(path) => quote! { Some(#path) },
                None => quote! { None },
            };
            quote! {
                #[cfg(feature = #feat)]
                compile_parser(#name, #url, #rev, #external_c, #external_cpp, #path)?;
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn parsers_ffi(_: TokenStream) -> TokenStream {
    let extern_c = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        // disable cpp scanners on wasm32-unknown-unknown
        let wasm_cfg = if lang.parser.external_scanner.cpp {
            quote! { , not(all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", target_env = "")) }
        } else {
            quote! {}
        };
        quote! {
            #[cfg(all(feature = #feat #wasm_cfg))]
            fn #ffi_func() -> ::syntastica_core::language_set::Language;
        }
    });
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        let doc = format!(
            "Get the parser for [{}]({}/tree/{}). {}",
            lang.name, lang.parser.git.url, lang.parser.git.rev,
            match lang.parser.external_scanner.cpp {
                true => "(not supported on the `wasm32-unknown-unknown` target)",
                false => "",
            },
        );
        // disable cpp scanners on wasm32-unknown-unknown
        let raw_wasm_cfg = quote! { all(target_arch = "wasm32", target_vendor = "unknown", target_os = "unknown", target_env = "") };
        let wasm_cfg = match lang.parser.external_scanner.cpp {
            true => quote! { , #raw_wasm_cfg },
            false => quote! {},
        };
        let not_wasm_cfg = match lang.parser.external_scanner.cpp {
            true => quote! { , not(#raw_wasm_cfg) },
            false => quote! {},
        };
        quote! {
            #[cfg(feature = #feat)]
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
            #[cfg(feature = #feat)]
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
            let feat = lang.group.to_string();
            let name_str = &lang.name;
            quote! { #[cfg(feature = #feat)] #name_str }
        });
    let extensions = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .flat_map(|lang| {
            let feat = lang.group.to_string();
            let name_str = &lang.name;
            lang.file_extensions
                .iter()
                .map(move |ext| quote! { #[cfg(feature = #feat)] (#ext, #name_str) })
        });
    let mut langs_sorted_by_group = LANGUAGE_CONFIG.languages.clone();
    langs_sorted_by_group.sort_by_key(|lang| lang.group);
    let func_map = langs_sorted_by_group
        .iter()
        .filter(&filter)
        .enumerate()
        .map(|(index, lang)| {
            let feat = lang.group.to_string();
            let name_str = &lang.name;
            quote! { #[cfg(feature = #feat)] (#name_str, #index) }
        });
    let funcs = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        quote! { #[cfg(feature = #feat)] &#name }
    });
    let queries = langs_sorted_by_group.iter().filter(&filter).map(|lang| {
        let feat = lang.group.to_string();
        let highlights = format_ident!("{}_HIGHLIGHTS{query_suffix}", lang.name.to_uppercase());
        let injections = format_ident!("{}_INJECTIONS{query_suffix}", lang.name.to_uppercase());
        let locals = format_ident!("{}_LOCALS{query_suffix}", lang.name.to_uppercase());

        quote! { #[cfg(feature = #feat)] [
            ::syntastica_queries::#highlights,
            ::syntastica_queries::#injections,
            ::syntastica_queries::#locals,
        ] }
    });

    quote! {
        #extra

        /// A list of all language names that are supported by this parser collection.
        pub const LANGUAGES: &[&str] = &[#(#list),*];
        const LANG_COUNT: usize = LANGUAGES.len();

        #(#functions)*

        // TODO: use "perfect" hashmap with compile-time known keys
        static EXTENSION_MAP: ::once_cell::sync::Lazy<::std::collections::HashMap<&'static str, &'static str>>
            = ::once_cell::sync::Lazy::new(|| ::std::collections::HashMap::from([#(#extensions),*]));

        // TODO: use "perfect" hashmap with compile-time known keys
        static IDX_MAP: ::once_cell::sync::Lazy<::std::collections::HashMap<&'static str, usize>>
            = ::once_cell::sync::Lazy::new(|| ::std::collections::HashMap::from([#(#func_map),*]));

        const QUERIES: &[[&str; 3]] = &[#(#queries),*];

        fn __get_language(idx: usize) -> ::syntastica_core::Result<::syntastica_core::language_set::HighlightConfiguration> {
            let funcs: [&dyn Fn() -> ::syntastica_core::language_set::Language; LANG_COUNT] = [#(#funcs),*];
            let lang = funcs[idx]();
            let mut conf = ::syntastica_core::language_set::HighlightConfiguration::new(
                lang,
                QUERIES[idx][0],
                QUERIES[idx][1],
                QUERIES[idx][2],
            )?;
            conf.configure(::syntastica_core::theme::THEME_KEYS);
            Ok(conf)
        }

        // TODO: maybe create enum with all supported languages
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
