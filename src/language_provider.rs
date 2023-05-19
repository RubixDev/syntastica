use std::{borrow::Cow, collections::HashMap};

use tree_sitter_highlight::HighlightConfiguration;

pub trait LanguageProvider {
    fn prepare(&mut self) -> Result<HashMap<String, HighlightConfiguration>, crate::Error>;

    fn by_extension(&self, file_extension: &str) -> Option<Cow<'_, str>>;
}

#[cfg(feature = "parsers-some")]
pub struct DefaultLanguageProvider;

#[cfg(feature = "parsers-some")]
macro_rules! langs {
    ($($feat:literal, $name:ident, $extensions:expr);* $(;)?) => {
        impl LanguageProvider for DefaultLanguageProvider {
            fn prepare(&mut self) -> Result<HashMap<String, HighlightConfiguration>, crate::Error> {
                let mut configs = HashMap::new();
                $(
                    #[cfg(feature = $feat)]
                    {
                        const QUERIES: (&str, &str, &str) = syntastica_queries::$name!();
                        configs.insert(
                            stringify!($name).to_owned(),
                            HighlightConfiguration::new(
                                syntastica_parsers::$name(),
                                QUERIES.0,
                                QUERIES.1,
                                QUERIES.2,
                            )?,
                        );
                    }
                )*
                Ok(configs)
            }

            fn by_extension(&self, file_extension: &str) -> Option<Cow<'_, str>> {
                $(
                    #[cfg(feature = $feat)]
                    if $extensions.contains(&file_extension) {
                        return Some(stringify!($name).into());
                    }
                )*
                None
            }
        }
    };
}

#[cfg(feature = "parsers-some")]
langs! {
    "parsers-some", c, ["c", "h"];
    "parsers-some", cpp, ["cc", "cpp", "hpp", "h"];
    "parsers-some", css, ["css"];
    "parsers-some", go, ["go"];
    "parsers-some", html, ["html"];
    "parsers-some", java, ["java"];
    "parsers-some", javascript, ["js", "jsx", "cjs", "mjs"];
    "parsers-some", json, ["json"];
    "parsers-some", python, ["py"];
    "parsers-some", rust, ["rs"];
    "parsers-some", tsx, ["tsx"];
    "parsers-some", typescript, ["ts"];
    "parsers-most", asm, ["asm", "s"];
    "parsers-all", regex, [];
}
