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
        quote! {
            #[cfg(feature = #feat)]
            fn #ffi_func() -> ::syntastica_core::provider::Language;
        }
    });
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        let doc = format!(
            "Get the parser for [{}]({}/tree/{}).",
            lang.name, lang.parser.git.url, lang.parser.git.rev
        );
        quote! {
            #[cfg(feature = #feat)]
            #[doc = #doc]
            pub fn #name() -> ::syntastica_core::provider::Language {
                #[cfg(not(all(feature = "docs", doc)))]
                unsafe { #ffi_func() }
                #[cfg(all(feature = "docs", doc))]
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
            pub fn #name() -> ::syntastica_core::provider::Language {
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
            quote! { #name_str, }
        });
    let get_parsers = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let feat = lang.group.to_string();
            let name = format_ident!("{}", lang.name);
            let name_str = &lang.name;
            quote! {
                #[cfg(all(feature = #feat, not(all(feature = "docs", doc))))]
                if self.0.map_or(true, |langs| langs.contains(&#name_str)) {
                    _map.insert(#name_str.to_owned(), #name());
                }
            }
        });
    let get_queries = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let name_str = &lang.name;
        let highlights = format_ident!("{}_HIGHLIGHTS{query_suffix}", lang.name.to_uppercase());
        let injections = format_ident!("{}_INJECTIONS{query_suffix}", lang.name.to_uppercase());
        let locals = format_ident!("{}_LOCALS{query_suffix}", lang.name.to_uppercase());

        quote! {
            _map.insert(#name_str.to_owned(), [
                ::syntastica_queries::#highlights.into(),
                ::syntastica_queries::#injections.into(),
                ::syntastica_queries::#locals.into(),
            ]);
        }
    });
    let by_extension = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let feat = lang.group.to_string();
            let extensions = &lang.file_extensions;
            let name_str = &lang.name;
            quote! {
                #[cfg(feature = #feat)]
                if [#(#extensions),*].contains(&file_extension) {
                    return ::std::option::Option::Some(#name_str.into());
                }
            }
        });
    let lang_count_some = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.group == Group::Some)
        .count();
    let lang_count_all = LANGUAGE_CONFIG.languages.len();
    let lang_count_most = lang_count_all - lang_count_some;
    let lang_count_parsers = quote! {
        if cfg!(feature = "all") {
            #lang_count_all
        } else if cfg!(feature = "most") {
            #lang_count_most
        } else if cfg!(feature = "some") {
            #lang_count_some
        } else {
            0
        }
    };
    quote! {
        #extra

        /// A list of all language names that are supported by this parser collection.
        pub const LANGUAGES: &[&str] = &[#(#list)*];

        #(#functions)*

        // TODO: maybe create enum with all supported languages
        /// An implementation of [`LanguageProvider`](::syntastica_core::provider::LanguageProvider),
        /// providing all parsers in the enabled feature set (see [`all`](LanguageProviderImpl::all))
        /// or a subset of them (see [`with_languages`](LanguageProviderImpl::with_languages)).
        pub struct LanguageProviderImpl<'a>(::std::option::Option<&'a [&'a str]>);

        impl ::syntastica_core::provider::LanguageProvider for LanguageProviderImpl<'_> {
            fn get_parsers(&self) -> ::std::result::Result<::syntastica_core::provider::Parsers, ::syntastica_core::Error> {
                let mut _map: ::syntastica_core::provider::Parsers = ::std::collections::HashMap::with_capacity(#lang_count_parsers);
                #(#get_parsers)*
                ::std::result::Result::Ok(_map)
            }

            fn get_queries(&self) -> ::std::result::Result<::syntastica_core::provider::Queries, ::syntastica_core::Error> {
                let mut _map: ::syntastica_core::provider::Queries = ::std::collections::HashMap::with_capacity(#lang_count_all);
                #(#get_queries)*
                ::std::result::Result::Ok(_map)
            }

            fn for_extension<'a>(
                &self,
                file_extension: &'a str,
            ) -> ::std::option::Option<::std::borrow::Cow<'a, str>> {
                #(#by_extension)*
                ::std::option::Option::None
            }

            // TODO: injection regex
            // fn for_injection<'a>(&self, name: &'a str) -> ::std::option::Option<::std::borrow::Cow<'a, str>> {
            //     ::std::option::Option::None
            // }
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
            let name_str = &lang.name;
            let highlights = format_ident!("{}_HIGHLIGHTS", lang.name.to_uppercase());
            let injections = format_ident!("{}_INJECTIONS", lang.name.to_uppercase());
            let locals = format_ident!("{}_LOCALS", lang.name.to_uppercase());
            quote! {
                #[test]
                fn #name() {
                    let lang = PARSERS.get(#name_str).unwrap().clone();
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
            let name_str = &lang.name;
            let highlights = format_ident!("{}_HIGHLIGHTS_CRATES_IO", lang.name.to_uppercase());
            let injections = format_ident!("{}_INJECTIONS_CRATES_IO", lang.name.to_uppercase());
            let locals = format_ident!("{}_LOCALS_CRATES_IO", lang.name.to_uppercase());
            quote! {
                #[test]
                fn #name() {
                    let lang = PARSERS.get(#name_str).unwrap().clone();
                    validate_query(lang, ::syntastica_queries::#highlights, "highlights");
                    validate_query(lang, ::syntastica_queries::#injections, "injections");
                    validate_query(lang, ::syntastica_queries::#locals, "locals");
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}
