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
    ($($feat:literal, $pattern:pat, $name:ident);* $(;)?) => {
        impl LanguageProvider for DefaultLanguageProvider {
            fn prepare(&mut self) -> Result<HashMap<String, HighlightConfiguration>, crate::Error> {
                let mut configs = HashMap::new();
                $(
                    #[cfg(feature = $feat)]
                    {
                        const QUERIES: (&str, &str, &str) = syntastic_queries::$name!();
                        configs.insert(
                            stringify!($name).to_owned(),
                            HighlightConfiguration::new(
                                syntastic_parsers::$name(),
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
                match file_extension {
                    $(
                        #[cfg(feature = $feat)]
                        $pattern => Some(stringify!($name).into()),
                    )*
                    _ => None,
                }
            }
        }
    };
}

#[cfg(feature = "parsers-some")]
langs! {
    "parsers-some", "rs", rust;
    "parsers-some", "py", python;
    "parsers-most", "asm" | "s", asm;
    "parsers-all", "", regex;
}
