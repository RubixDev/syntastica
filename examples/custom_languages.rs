//! This example shows how to provide custom languages that are not in any of the parser
//! collections while still allowing usage of languages from an official parser collection.
//!
//! For this, we must define a struct that implements [`LanguageSet`]
//! (here [`ExtendedLanguageSet`]), which contains the base language that we want to extend, as
//! well as our custom [`HighlightConfiguration`]s. For lazy-loading functionality we make use of
//! [`OnceCell`]s.

// TODO: re-enable example when tree-sitter-csv allows newer version of `cc`
fn main() {}

// use std::borrow::Cow;
//
// use once_cell::sync::OnceCell;
// use syntastica::{
//     language_set::{FileType, HighlightConfiguration, Language, LanguageSet, SupportedLanguage},
//     renderer::TerminalRenderer,
//     theme::THEME_KEYS,
//     Processor,
// };
// use syntastica_parsers::LanguageSetImpl;
//
// /// An enum with all languages that we want to provide ourselves.
// enum CustomLang {
//     Csv,
//     Psv,
//     Tsv,
// }
//
// /// An enum wrapping our own languages and the languages from the underlying language set.
// enum Lang {
//     Core(syntastica_parsers::Lang),
//     Custom(CustomLang),
// }
//
// /// Implementation of the [`SupportedLanguage`] trait for our wrapping [`Lang`] type.
// impl SupportedLanguage for Lang {
//     fn name(&self) -> Cow<'_, str> {
//         match self {
//             // for the core languages just forward to the core implementation
//             Lang::Core(core_lang) => core_lang.name(),
//             Lang::Custom(CustomLang::Csv) => "csv".into(),
//             Lang::Custom(CustomLang::Psv) => "psv".into(),
//             Lang::Custom(CustomLang::Tsv) => "tsv".into(),
//         }
//     }
//
//     fn for_name(name: impl AsRef<str>) -> syntastica::Result<Self> {
//         match name.as_ref() {
//             "csv" => Ok(Lang::Custom(CustomLang::Csv)),
//             "psv" => Ok(Lang::Custom(CustomLang::Psv)),
//             "tsv" => Ok(Lang::Custom(CustomLang::Tsv)),
//             // for other languages just forward to the core implementation
//             _ => syntastica_parsers::Lang::for_name(name).map(Lang::Core),
//         }
//     }
//
//     fn for_file_type(file_type: FileType) -> Option<Self> {
//         match file_type {
//             FileType::Csv => Some(Lang::Custom(CustomLang::Csv)),
//             // `tft` doesn't have a `psv` file type, so we omit that here
//             FileType::Tsv => Some(Lang::Custom(CustomLang::Tsv)),
//             // for other file types just forward to the core implementation
//             _ => syntastica_parsers::Lang::for_file_type(file_type).map(Lang::Core),
//         }
//     }
// }
//
// /// The wrapper language set.
// #[derive(Default)]
// struct ExtendedLanguageSet {
//     /// The core language set that we use as our base.
//     core: LanguageSetImpl,
//     // Fields for each of our own languages.
//     csv_lang: OnceCell<HighlightConfiguration>,
//     psv_lang: OnceCell<HighlightConfiguration>,
//     tsv_lang: OnceCell<HighlightConfiguration>,
// }
//
// impl ExtendedLanguageSet {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }
//
// /// The [`LanguageSet`] impl for our [`ExtendedLanguageSet`].
// impl LanguageSet for ExtendedLanguageSet {
//     // point to the wrapping `Lang` enum that implements `SupportedLanguage`
//     type Language = Lang;
//
//     fn get_language(
//         &self,
//         language: Self::Language,
//     ) -> syntastica::Result<&HighlightConfiguration> {
//         match language {
//             // for core languages just forward to the core implementation
//             Lang::Core(core_lang) => self.core.get_language(core_lang),
//             // provide our own languages however you like. in this example we use the languages and
//             // queries exposed by the `tree-sitter-csv` crate.
//             Lang::Custom(CustomLang::Csv) => init_lang(
//                 &self.csv_lang,
//                 tree_sitter_csv::language_csv,
//                 tree_sitter_csv::HIGHLIGHT_QUERY_CSV,
//             ),
//             Lang::Custom(CustomLang::Psv) => init_lang(
//                 &self.psv_lang,
//                 tree_sitter_csv::language_psv,
//                 tree_sitter_csv::HIGHLIGHT_QUERY_PSV,
//             ),
//             Lang::Custom(CustomLang::Tsv) => init_lang(
//                 &self.tsv_lang,
//                 tree_sitter_csv::language_tsv,
//                 tree_sitter_csv::HIGHLIGHT_QUERY_TSV,
//             ),
//         }
//     }
// }
//
// /// A simple helper function for initializing a language.
// fn init_lang<'a>(
//     cell: &'a OnceCell<HighlightConfiguration>,
//     get_lang: fn() -> Language,
//     queries: &str,
// ) -> syntastica::Result<&'a HighlightConfiguration> {
//     cell.get_or_try_init(|| {
//         let mut conf = HighlightConfiguration::new(
//             get_lang(),
//             // For simple queries like these, preprocessing wouldn't be necessary, but
//             // this is just to show how you would do it with more complex queries.
//             // To see what this preprocessing does, see the crate's documentation.
//             &syntastica_query_preprocessor::process_highlights("", false, queries),
//             "",
//             "",
//         )?;
//         // !! don't forget to configure the HighlightConfiguration with syntastica's theme keys
//         conf.configure(THEME_KEYS);
//         Ok(conf)
//     })
// }
//
// fn main() -> syntastica::Result<()> {
//     let code_csv = r#"
// "Name","Age","Salary"
// "John Doe",30,120000
// "#;
//     let code_psv = "
// Name|Age|Salary
// John Doe|30|120000
// ";
//     let code_tsv = "
// Name\tAge\tSalary
// John Doe\t30\t120000
// ";
//     let code_rust = r#"
// fn main() {
//     Regex::new(r"[a-fA-F0-9_]\s(.*)$");
// }
// "#;
//
//     let set = ExtendedLanguageSet::new();
//     let mut processor = Processor::new(&set);
//     let mut renderer = TerminalRenderer::new(None);
//     let theme = syntastica_themes::one::dark();
//
//     println!(
//         "{}",
//         syntastica::render(
//             &processor.process(code_csv, Lang::Custom(CustomLang::Csv))?,
//             &mut renderer,
//             &theme,
//         )
//     );
//     println!(
//         "{}",
//         syntastica::render(
//             &processor.process(code_psv, Lang::Custom(CustomLang::Psv))?,
//             &mut renderer,
//             &theme,
//         )
//     );
//     println!(
//         "{}",
//         syntastica::render(
//             &processor.process(code_tsv, Lang::Custom(CustomLang::Tsv))?,
//             &mut renderer,
//             &theme,
//         )
//     );
//     // languages from the underlying language set can still be used
//     println!(
//         "{}",
//         syntastica::render(
//             &processor.process(code_rust, Lang::Core(syntastica_parsers::Lang::Rust))?,
//             &mut renderer,
//             &theme,
//         )
//     );
//
//     Ok(())
// }
