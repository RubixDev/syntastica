//! This example shows how to provide custom languages that are not in any of the parser
//! collections while still allowing usage of languages from an official parser collection.
//!
//! For this, we first define our own [`LanguageSet`] (here [`CustomLanguageSet`]), which handles
//! loading our custom [`HighlightConfiguration`]s. Then we can use the [`Union`] type to combine
//! this set with an existing one. For lazy-loading functionality we make use of [`OnceCell`]s.

use std::borrow::Cow;

use once_cell::sync::OnceCell;
use syntastica::{
    language_set::{
        EitherLang, FileType, HighlightConfiguration, LanguageSet, SupportedLanguage, Union,
    },
    renderer::TerminalRenderer,
    theme::THEME_KEYS,
    Processor,
};
use syntastica_parsers::{Lang, LanguageSetImpl};
use tree_sitter_language::LanguageFn;

/// An enum with all languages that we want to provide ourselves.
enum CustomLang {
    Xml,
    Dtd,
}

impl AsRef<str> for CustomLang {
    fn as_ref(&self) -> &str {
        match self {
            Self::Xml => "xml",
            Self::Dtd => "dtd",
        }
    }
}

/// Implementation of the [`SupportedLanguage`] trait for our [`CustomLang`] type.
impl<'set, T> SupportedLanguage<'set, T> for CustomLang {
    fn name(&self) -> Cow<'_, str> {
        self.as_ref().into()
    }

    fn for_name(name: impl AsRef<str>, _set: &'set T) -> syntastica::Result<Self> {
        match name.as_ref() {
            "xml" => Ok(CustomLang::Xml),
            "dtd" => Ok(CustomLang::Dtd),
            name => Err(syntastica::Error::UnsupportedLanguage(name.to_string())),
        }
    }

    fn for_file_type(file_type: FileType, _set: &'set T) -> Option<Self> {
        match file_type {
            FileType::Xml => Some(CustomLang::Xml),
            FileType::Dtd => Some(CustomLang::Dtd),
            _ => None,
        }
    }
}

/// Out own language set.
#[derive(Default)]
struct CustomLanguageSet {
    // Fields for each of our languages.
    xml_lang: OnceCell<HighlightConfiguration>,
    dtd_lang: OnceCell<HighlightConfiguration>,
}

impl CustomLanguageSet {
    pub fn new() -> Self {
        Self::default()
    }
}

/// The [`LanguageSet`] impl for our [`CustomLanguageSet`].
impl LanguageSet<'_> for CustomLanguageSet {
    // point to our `CustomLang` enum that implements `SupportedLanguage`
    type Language = CustomLang;

    fn get_language(
        &self,
        language: Self::Language,
    ) -> syntastica::Result<&HighlightConfiguration> {
        match language {
            // provide our languages however you like. in this example we use the languages and
            // queries exposed by the `tree-sitter-xml` crate.
            CustomLang::Xml => init_lang(
                language.as_ref(),
                &self.xml_lang,
                tree_sitter_xml::LANGUAGE_XML,
                tree_sitter_xml::XML_HIGHLIGHT_QUERY,
            ),
            CustomLang::Dtd => init_lang(
                language.as_ref(),
                &self.dtd_lang,
                tree_sitter_xml::LANGUAGE_DTD,
                tree_sitter_xml::DTD_HIGHLIGHT_QUERY,
            ),
        }
    }
}

/// A simple helper function for initializing a language.
fn init_lang<'a>(
    name: &str,
    cell: &'a OnceCell<HighlightConfiguration>,
    get_lang: LanguageFn,
    queries: &str,
) -> syntastica::Result<&'a HighlightConfiguration> {
    cell.get_or_try_init(|| {
        let mut conf = HighlightConfiguration::new(
            get_lang.into(),
            name,
            // For simple queries like these, preprocessing wouldn't be necessary, but
            // this is just to show how you would do it with more complex queries.
            // To see what this preprocessing does, see the crate's documentation.
            &syntastica_query_preprocessor::process_highlights("", true, queries),
            "",
            "",
        )?;
        // !! don't forget to configure the HighlightConfiguration with syntastica's theme keys
        conf.configure(THEME_KEYS);
        Ok(conf)
    })
}

fn main() -> syntastica::Result<()> {
    let code_xml = r#"
<?xml version="1.1" encoding="UTF-8" ?>
<!DOCTYPE greeting [
  <!ELEMENT greeting (#PCDATA)>
]>
<greeting>Hello, world!</greeting>
"#;
    let code_dtd = r#"
<!ATTLIST termdef
          id      ID      #REQUIRED
          name    CDATA   #IMPLIED>
<!ATTLIST list
          type    (bullets|ordered|glossary)  "ordered">
<!ATTLIST form method  CDATA   #FIXED "POST">
"#;
    let code_rust = r#"
fn main() {
    Regex::new(r"[a-fA-F0-9_]\s(.*)$");
}
"#;

    // use our set but fall back to the syntastica-parsers collection
    let set = Union::new(CustomLanguageSet::new(), LanguageSetImpl::new());
    let mut processor = Processor::new(&set);
    let mut renderer = TerminalRenderer::new(None);
    let theme = syntastica_themes::one::dark();

    println!(
        "{}",
        syntastica::render(
            // languages are specified by the `EitherLang` enum
            &processor.process(code_xml, EitherLang::Left(CustomLang::Xml))?,
            &mut renderer,
            &theme,
        )
    );
    println!(
        "{}",
        syntastica::render(
            // "left" languages don't need to be explicitly specified as being left
            &processor.process(code_dtd, CustomLang::Dtd)?,
            &mut renderer,
            &theme,
        )
    );
    // languages from the fallback language set can also be used
    println!(
        "{}",
        syntastica::render(
            &processor.process(code_rust, EitherLang::Right(Lang::Rust))?,
            &mut renderer,
            &theme,
        )
    );

    Ok(())
}
