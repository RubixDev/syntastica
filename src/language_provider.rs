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
    "parsers-some", rust, ["rs"];
    "parsers-some", python, ["py"];
    "parsers-most", asm, ["asm", "s"];
    "parsers-all", regex, [];
}
