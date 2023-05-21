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
            fn #ffi_func() -> ::syntastica::providers::Language;
        }
    });
    let get_parsers = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name_str = &lang.name;
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        quote! {
            #[cfg(feature = #feat)]
            _map.insert(#name_str.to_owned(), unsafe { #ffi_func() });
        }
    });
    let by_extension = LANGUAGE_CONFIG.languages.iter().map(|lang| {
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
    quote! {
        extern "C" {
            #(#extern_c)*
        }
        pub struct ParserProviderGit;
        impl ::syntastica::providers::ParserProvider for ParserProviderGit {
            fn get_parsers(&self) -> ::std::result::Result<::syntastica::providers::Parsers, ::syntastica::Error> {
                let mut _map: ::std::collections::HashMap<::std::string::String, ::syntastica::providers::Language>
                    = ::std::collections::HashMap::new();
                #(#get_parsers)*
                ::std::result::Result::Ok(_map)
            }

            fn for_extension(
                &self,
                file_extension: &str,
            ) -> ::std::option::Option<::std::borrow::Cow<'_, str>> {
                #(#by_extension)*
                ::std::option::Option::None
            }

            // TODO: injection regex
            // fn for_injection(&self, name: &str) -> ::std::option::Option<::std::borrow::Cow<'_, str>> {
            //     ::std::option::Option::None
            // }
        }
    }
    .into()
}

#[proc_macro]
pub fn queries(_: TokenStream) -> TokenStream {
    let langs = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let name_str = &lang.name;

        let highlights = match lang.queries.nvim_like {
            true => quote! { process_queries(lang, #name_str, "highlights.scm") },
            false => quote! { read_queries(#name_str, "highlights.scm") },
        };
        let injections = match lang.queries.injections {
            true => quote! { read_queries(#name_str, "injections.scm") },
            false => quote! { String::new() },
        };
        let locals = match lang.queries.locals {
            true => quote! { read_queries(#name_str, "locals.scm") },
            false => quote! { String::new() },
        };

        quote! {
            let lang = parsers.remove(#name_str).unwrap();
            let highlights = #highlights;
            let injections = #injections;
            let locals = #locals;
            _map.insert(#name_str, [highlights, injections, locals]);
        }
    });
    quote! {
        {
            let mut parsers = ::syntastica_parsers_git::ParserProviderGit.get_parsers()?;
            let mut _map: ::std::collections::BTreeMap<&'static str, [::std::string::String; 3]>
                = ::std::collections::BTreeMap::new();
            #(#langs)*
            ::std::result::Result::Ok(_map)
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
pub fn queries_provider(_: TokenStream) -> TokenStream {
    let langs = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let name_str = &lang.name;
        let highlights = format_ident!("{}_HIGHLIGHTS", lang.name.to_uppercase());
        let injections = format_ident!("{}_INJECTIONS", lang.name.to_uppercase());
        let locals = format_ident!("{}_LOCALS", lang.name.to_uppercase());

        quote! {
            _map.insert(#name_str.to_owned(), [
                ::syntastica_queries::#highlights.into(),
                ::syntastica_queries::#injections.into(),
                ::syntastica_queries::#locals.into(),
            ]);
        }
    });
    quote! {
        {
            let mut _map: Queries<'static> = ::std::collections::HashMap::new();
            #(#langs)*
            ::std::result::Result::Ok(_map)
        }
    }
    .into()
}
