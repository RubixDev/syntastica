use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Punct, Spacing, Span};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use schema::*;

mod schema;

static LANGUAGE_CONFIG: Lazy<LanguageConfig> = Lazy::new(|| {
    toml::from_str(include_str!("../languages.toml")).expect("invalid `languages.toml`")
});

struct Interp(&'static str);

impl ToTokens for Interp {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Punct::new('#', Spacing::Alone));
        tokens.append(Ident::new(self.0, Span::call_site()));
    }
}

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
            fn #ffi_func() -> ::tree_sitter::Language;
        }
    });
    let func_defs = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        quote! {
            #[cfg(feature = #feat)]
            pub fn #name() -> ::tree_sitter::Language {
                unsafe { #ffi_func() }
            }
        }
    });
    quote! {
        extern "C" {
            #(#extern_c)*
        }
        #(#func_defs)*
    }
    .into()
}

#[proc_macro]
pub fn queries(_: TokenStream) -> TokenStream {
    LANGUAGE_CONFIG
        .languages
        .iter()
        .map(|lang| {
            let feat = lang.group.to_string();
            let name = format_ident!("{}", lang.name);
            let name_str = &lang.name;
            const HIGHLIGHTS: Interp = Interp("highlights");
            const INJECTIONS: Interp = Interp("injections");
            const LOCALS: Interp = Interp("locals");

            let highlights = match lang.queries.nvim_like {
                true => quote! { process_queries(_lang, #name_str, "highlights.scm") },
                false => quote! { read_queries(#name_str, "highlights.scm") },
            };
            let injections = match lang.queries.injections {
                true => quote! { read_queries(#name_str, "injections.scm") },
                false => quote! { "" },
            };
            let locals = match lang.queries.locals {
                true => quote! { read_queries(#name_str, "locals.scm") },
                false => quote! { "" },
            };

            quote! {
                #[cfg(feature = #feat)]
                #[proc_macro]
                pub fn #name(_: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                    let _lang = ::syntastica_parsers::#name();
                    let highlights = #highlights;
                    let injections = #injections;
                    let locals = #locals;
                    ::quote::quote! { (#HIGHLIGHTS, #INJECTIONS, #LOCALS) }.into()
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn queries_test(_: TokenStream) -> TokenStream {
    LANGUAGE_CONFIG
        .languages
        .iter()
        .map(|lang| {
            let name = format_ident!("{}", lang.name);
            quote! {
                #[test]
                fn #name() {
                    let lang = ::syntastica_parsers::#name();
                    const QUERIES: (&str, &str, &str) = ::syntastica_queries::#name!();
                    validate_query(lang, QUERIES.0, "highlights");
                    validate_query(lang, QUERIES.1, "injections");
                    validate_query(lang, QUERIES.2, "locals");
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn lang_provider_prepare(_: TokenStream) -> TokenStream {
    let langs = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = format!("{:#}", lang.group);
        let name = format_ident!("{}", lang.name);
        let name_str = &lang.name;
        quote! {
            #[cfg(feature = #feat)]
            {
                const QUERIES: (&str, &str, &str) = ::syntastica_queries::#name!();
                configs.insert(
                    #name_str.to_owned(),
                    ::tree_sitter_highlight::HighlightConfiguration::new(
                        ::syntastica_parsers::#name(),
                        QUERIES.0,
                        QUERIES.1,
                        QUERIES.2,
                    )?,
                );
            }
        }
    });
    quote! {
        {
            let mut configs = HashMap::new();
            #(#langs)*
            Ok(configs)
        }
    }
    .into()
}

#[proc_macro]
pub fn lang_provider_extensions(_: TokenStream) -> TokenStream {
    let langs = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = format!("{:#}", lang.group);
        let extensions = &lang.file_extensions;
        let name_str = &lang.name;
        quote! {
            #[cfg(feature = #feat)]
            if [#(#extensions),*].contains(&file_extension) {
                return Some(#name_str.into());
            }
        }
    });
    quote! {
        {
            #(#langs)*
            None
        }
    }
    .into()
}
