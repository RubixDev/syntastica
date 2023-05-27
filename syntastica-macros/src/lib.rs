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
            fn #ffi_func() -> ::syntastica_core::providers::Language;
        }
    });
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        let doc = format!(
            "Get the parser for [{}]({}/tree/{}).",
            lang.name, lang.parser.git.url, lang.parser.git.rev,
        );
        quote! {
            #[cfg(feature = #feat)]
            #[doc = #doc]
            pub fn #name() -> ::syntastica_core::providers::Language {
                #[cfg(not(feature = "docs"))]
                unsafe { #ffi_func() }
                #[cfg(feature = "docs")]
                ::std::unimplemented!()
            }
        }
    });
    parsers(
        functions,
        |_| true,
        Some(quote! {
            #[cfg(not(feature = "docs"))]
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
            pub fn #name() -> ::syntastica_core::providers::Language {
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
    let get_parsers = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let feat = lang.group.to_string();
            let name = format_ident!("{}", lang.name);
            let name_str = &lang.name;
            quote! {
                #[cfg(all(feature = #feat, not(feature = "docs")))]
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

        #(#functions)*

        // TODO: maybe create enum with all supported languages
        /// An implementation of [`LanguageProvider`](::syntastica_core::providers::LanguageProvider),
        /// providing all parsers in the enabled feature set (see [`all`](LanguageProviderImpl::all))
        /// or a subset of them (see [`with_languages`](LanguageProviderImpl::with_languages)).
        pub struct LanguageProviderImpl<'a>(::std::option::Option<&'a [&'a str]>);

        impl ::syntastica_core::providers::LanguageProvider for LanguageProviderImpl<'_> {
            fn get_parsers(&self) -> ::std::result::Result<::syntastica_core::providers::Parsers, ::syntastica_core::Error> {
                let mut _map: ::syntastica_core::providers::Parsers = ::std::collections::HashMap::with_capacity(#lang_count_parsers);
                #(#get_parsers)*
                ::std::result::Result::Ok(_map)
            }

            fn get_queries(&self) -> ::std::result::Result<::syntastica_core::providers::Queries, ::syntastica_core::Error> {
                let mut _map: ::syntastica_core::providers::Queries = ::std::collections::HashMap::with_capacity(#lang_count_all);
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
pub fn parsers_gitdep_toml_deps(_: TokenStream) -> TokenStream {
    parsers_toml_deps(true)
}

#[proc_macro]
pub fn parsers_dep_toml_deps(_: TokenStream) -> TokenStream {
    parsers_toml_deps(false)
}

fn parsers_toml_deps(git: bool) -> TokenStream {
    let mut added_packages = vec![];
    LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.parser.rust_func.is_some() && (git || lang.parser.crates_io.is_some()))
        .filter_map(|lang| {
            let package = &lang.parser.package;
            let url = &lang.parser.git.url;
            let rev = &lang.parser.git.rev;
            if added_packages.contains(&package) {
                None
            } else {
                added_packages.push(package);
                let dep_str = if git {
                    format!(
                        r##"
[dependencies.{package}]
optional = true
git = "{url}"
rev = "{rev}"
"##
                    )
                } else {
                    format!(
                        r##"
[dependencies.{package}]
optional = true
version = "{version}"
"##,
                        version = lang
                            .parser
                            .crates_io
                            .as_ref()
                            .expect("`None` is filtered above if `git` is `false`")
                    )
                };
                Some(quote! {
                    toml += #dep_str;
                })
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn parsers_gitdep_toml_feature_some(_: TokenStream) -> TokenStream {
    parsers_toml_feature(Group::Some, false)
}

#[proc_macro]
pub fn parsers_gitdep_toml_feature_most(_: TokenStream) -> TokenStream {
    parsers_toml_feature(Group::Most, false)
}

#[proc_macro]
pub fn parsers_gitdep_toml_feature_all(_: TokenStream) -> TokenStream {
    parsers_toml_feature(Group::All, false)
}

#[proc_macro]
pub fn parsers_dep_toml_feature_some(_: TokenStream) -> TokenStream {
    parsers_toml_feature(Group::Some, true)
}

#[proc_macro]
pub fn parsers_dep_toml_feature_most(_: TokenStream) -> TokenStream {
    parsers_toml_feature(Group::Most, true)
}

#[proc_macro]
pub fn parsers_dep_toml_feature_all(_: TokenStream) -> TokenStream {
    parsers_toml_feature(Group::All, true)
}

fn parsers_toml_feature(group: Group, crates_io: bool) -> TokenStream {
    let mut added_packages = vec![];
    let mut feature_str = format!("{group} = [\n");
    if let Some(group) = group.next_smaller() {
        feature_str += &format!("    \"{group}\",\n");
    }
    for lang in LANGUAGE_CONFIG.languages.iter().filter(|lang| {
        lang.parser.rust_func.is_some()
            && lang.group == group
            && (!crates_io || lang.parser.crates_io.is_some())
    }) {
        let package = &lang.parser.package;
        if added_packages.contains(&package) {
            continue;
        }
        added_packages.push(package);
        feature_str += &format!("    \"dep:{package}\",\n");
    }
    feature_str += "]\n";

    quote! { toml += #feature_str; }.into()
}

fn query_file(
    lang: &Language,
    enabled: bool,
    filename: &str,
    func: &str,
    crates_io: bool,
) -> proc_macro2::TokenStream {
    let name_str = &lang.name;
    let func = format_ident!("{func}");
    match (lang.queries.nvim_like, enabled) {
        (true, true) => {
            quote! { validate(lang, #name_str, #filename, Some(#func), #crates_io) }
        }
        (false, true) => quote! { validate(lang, #name_str, #filename, None, #crates_io) },
        (_, false) => quote! { String::new() },
    }
}

#[proc_macro]
pub fn queries(_: TokenStream) -> TokenStream {
    let langs = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let name_str = &lang.name;

        let highlights = query_file(lang, true, "highlights.scm", "process_highlights", false);
        let injections = query_file(
            lang,
            lang.queries.injections,
            "injections.scm",
            "process_injections",
            false,
        );
        let locals = query_file(
            lang,
            lang.queries.locals,
            "locals.scm",
            "process_locals",
            false,
        );

        let highlights_crates_io =
            query_file(lang, true, "highlights.scm", "process_highlights", true);
        let injections_crates_io = query_file(
            lang,
            lang.queries.injections,
            "injections.scm",
            "process_injections",
            true,
        );
        let locals_crates_io = query_file(
            lang,
            lang.queries.locals,
            "locals.scm",
            "process_locals",
            true,
        );

        quote! {
            let lang = parsers.remove(#name_str).unwrap();
            _map.insert(
                #name_str,
                [
                    #highlights,
                    #injections,
                    #locals,
                    #highlights_crates_io,
                    #injections_crates_io,
                    #locals_crates_io,
                ],
            );
        }
    });
    quote! {
        {
            let mut parsers = ::syntastica_parsers_git::LanguageProviderImpl::all().get_parsers()?;
            let mut _map: ::std::collections::BTreeMap<&'static str, [::std::string::String; 6]>
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

#[proc_macro]
pub fn parser_list_git(_: TokenStream) -> TokenStream {
    parser_list(|_| true, git_url)
}

#[proc_macro]
pub fn parser_list_gitdep(_: TokenStream) -> TokenStream {
    parser_list(|lang| lang.parser.rust_func.is_some(), git_url)
}

#[proc_macro]
pub fn parser_list_dep(_: TokenStream) -> TokenStream {
    parser_list(
        |lang| lang.parser.rust_func.is_some() && lang.parser.crates_io.is_some(),
        crates_io_url,
    )
}

fn git_url(lang: &Language) -> String {
    format!("{}/tree/{}", lang.parser.git.url, lang.parser.git.rev)
}

fn crates_io_url(lang: &Language) -> String {
    match &lang.parser.crates_io {
        Some(version) => format!("https://docs.rs/{}/{version}/", lang.parser.package),
        None => lang.parser.git.url.clone(),
    }
}

fn parser_list(
    filter: impl Fn(&Language) -> bool,
    url: impl Fn(&Language) -> String,
) -> TokenStream {
    let mut some_list = String::new();
    let mut most_list = String::new();
    let mut all_list = String::new();
    for lang in &LANGUAGE_CONFIG.languages {
        let str = format!(
            "- [{}]({}){}\n",
            lang.name,
            url(lang),
            if filter(lang) {
                ""
            } else {
                " (not supported by this collection)"
            }
        );
        match lang.group {
            Group::Some => some_list += &str,
            Group::Most => most_list += &str,
            Group::All => all_list += &str,
        }
    }

    let parser_list = format!(
        r##"
<!-- dprint-ignore-start -->

<details>
<summary>List of parsers included in the <span class="stab portability"><code>some</code></span> feature</summary>

{some_list}
</details>

<details>
<summary>List of parsers additionally included in the <span class="stab portability"><code>most</code></span> feature</summary>

{most_list}
</details>

<details>
<summary>List of parsers additionally included in the <span class="stab portability"><code>all</code></span> feature</summary>

{all_list}
</details>

<!-- dprint-ignore-end -->
"##
    );
    quote! { readme += #parser_list; }.into()
}
