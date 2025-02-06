//! Forked from <https://github.com/tree-sitter/tree-sitter/blob/89edb2ddcaf2928e3197ad6095e1eb1d59bfcc40/cli/loader/src/lib.rs>
//!
//! The MIT License (MIT)
//!
//! Copyright (c) 2018-2023 Max Brunsfeld
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use anyhow::{anyhow, Context, Error, Result};
use libloading::{Library, Symbol};
use once_cell::unsync::OnceCell;
use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::BufReader;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::time::SystemTime;
use std::{env, fs, mem};
use syntastica_core::language_set::HighlightConfiguration;
use tree_sitter::{Language, QueryError, QueryErrorKind};

#[derive(Default)]
pub struct Config {
    pub parser_directories: Vec<PathBuf>,
}

#[cfg(unix)]
const DYLIB_EXTENSION: &str = "so";

#[cfg(windows)]
const DYLIB_EXTENSION: &str = "dll";

const BUILD_TARGET: &str = env!("BUILD_TARGET");

pub struct LanguageConfiguration<'a> {
    pub scope: Option<String>,
    pub _content_regex: Option<Regex>,
    pub _first_line_regex: Option<Regex>,
    pub injection_regex: Option<Regex>,
    pub file_types: Vec<String>,
    pub root_path: PathBuf,
    pub highlights_filenames: Option<Vec<String>>,
    pub injections_filenames: Option<Vec<String>>,
    pub locals_filenames: Option<Vec<String>>,
    pub _tags_filenames: Option<Vec<String>>,
    language_id: usize,
    highlight_config: OnceCell<Option<&'static HighlightConfiguration>>,
    highlight_names: &'a Mutex<Vec<String>>,
    use_all_highlight_names: bool,
}

pub struct Loader {
    parser_lib_path: PathBuf,
    languages_by_id: Vec<(PathBuf, OnceCell<Language>)>,
    language_configurations: Vec<LanguageConfiguration<'static>>,
    language_configuration_ids_by_file_type: HashMap<String, Vec<usize>>,
    highlight_names: Box<Mutex<Vec<String>>>,
    use_all_highlight_names: bool,
    debug_build: bool,
}

unsafe impl Send for Loader {}
unsafe impl Sync for Loader {}

impl Loader {
    pub fn new() -> Result<Self> {
        let parser_lib_path = match env::var("TREE_SITTER_LIBDIR") {
            Ok(path) => PathBuf::from(path),
            _ => dirs::cache_dir()
                .ok_or(anyhow!("Cannot determine cache directory"))?
                .join("tree-sitter")
                .join("lib"),
        };
        Ok(Self::with_parser_lib_path(parser_lib_path))
    }

    pub fn with_parser_lib_path(parser_lib_path: PathBuf) -> Self {
        Loader {
            parser_lib_path,
            languages_by_id: Vec::new(),
            language_configurations: Vec::new(),
            language_configuration_ids_by_file_type: HashMap::new(),
            highlight_names: Box::new(Mutex::new(Vec::new())),
            use_all_highlight_names: true,
            debug_build: false,
        }
    }

    pub fn configure_highlights(&mut self, names: &[impl Borrow<str>]) {
        self.use_all_highlight_names = false;
        let mut highlights = self.highlight_names.lock().unwrap();
        highlights.clear();
        highlights.extend(names.iter().map(|s| s.borrow().to_owned()));
    }

    pub fn find_all_languages(&mut self, config: &Config) -> Result<()> {
        if config.parser_directories.is_empty() {
            eprintln!("Warning: You have not configured any parser directories!");
            eprintln!("Please run `tree-sitter init-config` and edit the resulting");
            eprintln!("configuration file to indicate where we should look for");
            eprintln!("language grammars.");
            eprintln!();
        }
        for parser_container_dir in &config.parser_directories {
            if let Ok(entries) = fs::read_dir(parser_container_dir) {
                for entry in entries {
                    let entry = entry?;
                    if let Some(parser_dir_name) = entry.file_name().to_str() {
                        if parser_dir_name.starts_with("tree-sitter-") {
                            self.find_language_configurations_at_path(
                                &parser_container_dir.join(parser_dir_name),
                            )
                            .ok();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn language_configuration_for_scope(
        &self,
        scope: &str,
    ) -> Result<Option<(Language, &LanguageConfiguration)>> {
        for configuration in &self.language_configurations {
            if configuration.scope.as_ref().map_or(false, |s| s == scope) {
                let language = self.language_for_id(configuration.language_id)?;
                return Ok(Some((language, configuration)));
            }
        }
        Ok(None)
    }

    // pub fn language_configuration_for_file_name(
    //     &self,
    //     path: &Path,
    // ) -> Result<Option<(Language, &LanguageConfiguration)>> {
    //     // Find all the language configurations that match this file name
    //     // or a suffix of the file name.
    //     let configuration_ids = path
    //         .file_name()
    //         .and_then(|n| n.to_str())
    //         .and_then(|file_name| self.language_configuration_ids_by_file_type.get(file_name))
    //         .or_else(|| {
    //             path.extension()
    //                 .and_then(|extension| extension.to_str())
    //                 .and_then(|extension| {
    //                     self.language_configuration_ids_by_file_type.get(extension)
    //                 })
    //         });
    //
    //     if let Some(configuration_ids) = configuration_ids {
    //         if !configuration_ids.is_empty() {
    //             let configuration;
    //
    //             // If there is only one language configuration, then use it.
    //             if configuration_ids.len() == 1 {
    //                 configuration = &self.language_configurations[configuration_ids[0]];
    //             }
    //             // If multiple language configurations match, then determine which
    //             // one to use by applying the configurations' content regexes.
    //             else {
    //                 let file_contents = fs::read(path)
    //                     .with_context(|| format!("Failed to read path {:?}", path))?;
    //                 let file_contents = String::from_utf8_lossy(&file_contents);
    //                 let mut best_score = -2isize;
    //                 let mut best_configuration_id = None;
    //                 for configuration_id in configuration_ids {
    //                     let config = &self.language_configurations[*configuration_id];
    //
    //                     // If the language configuration has a content regex, assign
    //                     // a score based on the length of the first match.
    //                     let score;
    //                     if let Some(content_regex) = &config.content_regex {
    //                         if let Some(mat) = content_regex.find(&file_contents) {
    //                             score = (mat.end() - mat.start()) as isize;
    //                         }
    //                         // If the content regex does not match, then *penalize* this
    //                         // language configuration, so that language configurations
    //                         // without content regexes are preferred over those with
    //                         // non-matching content regexes.
    //                         else {
    //                             score = -1;
    //                         }
    //                     } else {
    //                         score = 0;
    //                     }
    //                     if score > best_score {
    //                         best_configuration_id = Some(*configuration_id);
    //                         best_score = score;
    //                     }
    //                 }
    //
    //                 configuration = &self.language_configurations[best_configuration_id.unwrap()];
    //             }
    //
    //             let language = self.language_for_id(configuration.language_id)?;
    //             return Ok(Some((language, configuration)));
    //         }
    //     }
    //
    //     Ok(None)
    // }

    pub fn language_configuration_for_injection_string(
        &self,
        string: &str,
    ) -> Result<Option<(Language, &LanguageConfiguration)>> {
        let mut best_match_length = 0;
        let mut best_match_position = None;
        for (i, configuration) in self.language_configurations.iter().enumerate() {
            if let Some(injection_regex) = &configuration.injection_regex {
                if let Some(mat) = injection_regex.find(string) {
                    let length = mat.end() - mat.start();
                    if length > best_match_length {
                        best_match_position = Some(i);
                        best_match_length = length;
                    }
                }
            }
        }

        if let Some(i) = best_match_position {
            let configuration = &self.language_configurations[i];
            let language = self.language_for_id(configuration.language_id)?;
            Ok(Some((language, configuration)))
        } else {
            Ok(None)
        }
    }

    fn language_for_id(&self, id: usize) -> Result<Language> {
        let (path, language) = &self.languages_by_id[id];
        language
            .get_or_try_init(|| {
                let src_path = path.join("src");
                self.load_language_at_path(&src_path, &src_path)
            })
            .copied()
    }

    pub fn load_language_at_path(&self, src_path: &Path, header_path: &Path) -> Result<Language> {
        let grammar_path = src_path.join("grammar.json");
        let parser_path = src_path.join("parser.c");
        let mut scanner_path = src_path.join("scanner.c");

        #[derive(Deserialize)]
        struct GrammarJSON {
            name: String,
        }
        let mut grammar_file =
            fs::File::open(grammar_path).with_context(|| "Failed to read grammar.json")?;
        let grammar_json: GrammarJSON = serde_json::from_reader(BufReader::new(&mut grammar_file))
            .with_context(|| "Failed to parse grammar.json")?;

        let scanner_path = if scanner_path.exists() {
            Some(scanner_path)
        } else {
            scanner_path.set_extension("cc");
            if scanner_path.exists() {
                Some(scanner_path)
            } else {
                None
            }
        };

        self.load_language_from_sources(
            &grammar_json.name,
            header_path,
            &parser_path,
            &scanner_path,
        )
    }

    pub fn load_language_from_sources(
        &self,
        name: &str,
        header_path: &Path,
        parser_path: &Path,
        scanner_path: &Option<PathBuf>,
    ) -> Result<Language> {
        let mut lib_name = name.to_string();
        if self.debug_build {
            lib_name.push_str(".debug._");
        }
        let mut library_path = self.parser_lib_path.join(lib_name);
        library_path.set_extension(DYLIB_EXTENSION);

        let recompile = needs_recompile(&library_path, parser_path, scanner_path)
            .with_context(|| "Failed to compare source and binary timestamps")?;

        if recompile {
            fs::create_dir_all(&self.parser_lib_path)?;
            let mut config = cc::Build::new();
            config
                .cpp(true)
                .opt_level(2)
                .cargo_metadata(false)
                .target(BUILD_TARGET)
                .host(BUILD_TARGET);
            let compiler = config.get_compiler();
            let mut command = Command::new(compiler.path());
            for (key, value) in compiler.env() {
                command.env(key, value);
            }

            if cfg!(windows) {
                command.args(["/nologo", "/LD", "/I"]).arg(header_path);
                if self.debug_build {
                    command.arg("/Od");
                } else {
                    command.arg("/O2");
                }
                command.arg(parser_path);
                if let Some(scanner_path) = scanner_path.as_ref() {
                    command.arg(scanner_path);
                }
                command
                    .arg("/link")
                    .arg(format!("/out:{}", library_path.to_str().unwrap()));
            } else {
                command
                    .arg("-shared")
                    .arg("-fPIC")
                    .arg("-fno-exceptions")
                    .arg("-g")
                    .arg("-I")
                    .arg(header_path)
                    .arg("-o")
                    .arg(&library_path);

                if self.debug_build {
                    command.arg("-O0");
                } else {
                    command.arg("-O2");
                }

                // For conditional compilation of external scanner code when
                // used internally by `tree-siteer parse` and other sub commands.
                command.arg("-DTREE_SITTER_INTERNAL_BUILD");

                if let Some(scanner_path) = scanner_path.as_ref() {
                    if scanner_path.extension() == Some("c".as_ref()) {
                        command.arg("-xc").arg("-std=c99").arg(scanner_path);
                    } else {
                        command.arg(scanner_path);
                    }
                }
                command.arg("-xc").arg(parser_path);
            }

            let output = command
                .output()
                .with_context(|| "Failed to execute C compiler")?;
            if !output.status.success() {
                return Err(anyhow!(
                    "Parser compilation failed.\nStdout: {}\nStderr: {}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        let library = unsafe { Library::new(&library_path) }
            .with_context(|| format!("Error opening dynamic library {:?}", &library_path))?;
        let language_fn_name = format!("tree_sitter_{}", replace_dashes_with_underscores(name));
        let language = unsafe {
            let language_fn: Symbol<unsafe extern "C" fn() -> Language> = library
                .get(language_fn_name.as_bytes())
                .with_context(|| format!("Failed to load symbol {}", language_fn_name))?;
            language_fn()
        };
        mem::forget(library);
        Ok(language)
    }

    pub fn find_language_configurations_at_path(
        &mut self,
        parser_path: &Path,
    ) -> Result<&[LanguageConfiguration]> {
        #[derive(Deserialize, Default)]
        #[serde(untagged)]
        enum PathsJSON {
            #[default]
            Empty,
            Single(String),
            Multiple(Vec<String>),
        }

        impl PathsJSON {
            fn into_vec(self) -> Option<Vec<String>> {
                match self {
                    PathsJSON::Empty => None,
                    PathsJSON::Single(s) => Some(vec![s]),
                    PathsJSON::Multiple(s) => Some(s),
                }
            }
        }

        #[derive(Deserialize)]
        struct LanguageConfigurationJSON {
            #[serde(default)]
            path: PathBuf,
            scope: Option<String>,
            #[serde(rename = "file-types")]
            file_types: Option<Vec<String>>,
            #[serde(rename = "content-regex")]
            content_regex: Option<String>,
            #[serde(rename = "first-line-regex")]
            first_line_regex: Option<String>,
            #[serde(rename = "injection-regex")]
            injection_regex: Option<String>,
            #[serde(default)]
            highlights: PathsJSON,
            #[serde(default)]
            injections: PathsJSON,
            #[serde(default)]
            locals: PathsJSON,
            #[serde(default)]
            tags: PathsJSON,
        }

        #[derive(Deserialize)]
        struct PackageJSON {
            #[serde(default)]
            #[serde(rename = "tree-sitter")]
            tree_sitter: Vec<LanguageConfigurationJSON>,
        }

        let initial_language_configuration_count = self.language_configurations.len();

        if let Ok(package_json_contents) = fs::read_to_string(parser_path.join("package.json")) {
            let package_json = serde_json::from_str::<PackageJSON>(&package_json_contents);
            if let Ok(package_json) = package_json {
                let language_count = self.languages_by_id.len();
                for config_json in package_json.tree_sitter {
                    // Determine the path to the parser directory. This can be specified in
                    // the package.json, but defaults to the directory containing the package.json.
                    let language_path = parser_path.join(config_json.path);

                    // Determine if a previous language configuration in this package.json file
                    // already uses the same language.
                    let mut language_id = None;
                    for (id, (path, _)) in
                        self.languages_by_id.iter().enumerate().skip(language_count)
                    {
                        if language_path == *path {
                            language_id = Some(id);
                        }
                    }

                    // If not, add a new language path to the list.
                    let language_id = language_id.unwrap_or_else(|| {
                        self.languages_by_id.push((language_path, OnceCell::new()));
                        self.languages_by_id.len() - 1
                    });

                    let configuration = LanguageConfiguration {
                        root_path: parser_path.to_path_buf(),
                        scope: config_json.scope,
                        language_id,
                        file_types: config_json.file_types.unwrap_or(Vec::new()),
                        _content_regex: Self::regex(config_json.content_regex),
                        _first_line_regex: Self::regex(config_json.first_line_regex),
                        injection_regex: Self::regex(config_json.injection_regex),
                        injections_filenames: config_json.injections.into_vec(),
                        locals_filenames: config_json.locals.into_vec(),
                        _tags_filenames: config_json.tags.into_vec(),
                        highlights_filenames: config_json.highlights.into_vec(),
                        highlight_config: OnceCell::new(),
                        highlight_names: &self.highlight_names,
                        use_all_highlight_names: self.use_all_highlight_names,
                    };

                    for file_type in &configuration.file_types {
                        self.language_configuration_ids_by_file_type
                            .entry(file_type.to_string())
                            .or_default()
                            .push(self.language_configurations.len());
                    }

                    self.language_configurations.push(unsafe {
                        mem::transmute::<LanguageConfiguration<'_>, LanguageConfiguration<'static>>(
                            configuration,
                        )
                    });
                }
            }
        }

        if self.language_configurations.len() == initial_language_configuration_count
            && parser_path.join("src").join("grammar.json").exists()
        {
            let configuration = LanguageConfiguration {
                root_path: parser_path.to_owned(),
                language_id: self.languages_by_id.len(),
                file_types: Vec::new(),
                scope: None,
                _content_regex: None,
                _first_line_regex: None,
                injection_regex: None,
                injections_filenames: None,
                locals_filenames: None,
                highlights_filenames: None,
                _tags_filenames: None,
                highlight_config: OnceCell::new(),
                highlight_names: &self.highlight_names,
                use_all_highlight_names: self.use_all_highlight_names,
            };
            self.language_configurations.push(unsafe {
                mem::transmute::<LanguageConfiguration<'_>, LanguageConfiguration<'static>>(
                    configuration,
                )
            });
            self.languages_by_id
                .push((parser_path.to_owned(), OnceCell::new()));
        }

        Ok(&self.language_configurations[initial_language_configuration_count..])
    }

    fn regex(pattern: Option<String>) -> Option<Regex> {
        pattern.and_then(|r| RegexBuilder::new(&r).multi_line(true).build().ok())
    }
}

impl LanguageConfiguration<'_> {
    pub fn highlight_config(
        &self,
        language: Language,
    ) -> Result<Option<&'static HighlightConfiguration>> {
        self.highlight_config
            .get_or_try_init(|| {
                let (highlights_query, highlight_ranges) =
                    self.read_queries(&self.highlights_filenames, "highlights.scm")?;
                let (injections_query, injection_ranges) =
                    self.read_queries(&self.injections_filenames, "injections.scm")?;
                let (locals_query, locals_ranges) =
                    self.read_queries(&self.locals_filenames, "locals.scm")?;

                if highlights_query.is_empty() {
                    Ok(None)
                } else {
                    let mut result = HighlightConfiguration::new(
                        language,
                        &highlights_query,
                        &injections_query,
                        &locals_query,
                    )
                    .map_err(|error| match error.kind {
                        QueryErrorKind::Language => Error::from(error),
                        _ => {
                            if error.offset < injections_query.len() {
                                Self::include_path_in_query_error(
                                    error,
                                    &injection_ranges,
                                    &injections_query,
                                    0,
                                )
                            } else if error.offset < injections_query.len() + locals_query.len() {
                                Self::include_path_in_query_error(
                                    error,
                                    &locals_ranges,
                                    &locals_query,
                                    injections_query.len(),
                                )
                            } else {
                                Self::include_path_in_query_error(
                                    error,
                                    &highlight_ranges,
                                    &highlights_query,
                                    injections_query.len() + locals_query.len(),
                                )
                            }
                        }
                    })?;
                    let mut all_highlight_names = self.highlight_names.lock().unwrap();
                    if self.use_all_highlight_names {
                        for capture_name in result.query.capture_names() {
                            if !all_highlight_names.contains(capture_name) {
                                all_highlight_names.push(capture_name.clone());
                            }
                        }
                    }
                    result.configure(all_highlight_names.as_slice());
                    let result_ref = Box::leak(Box::new(result));
                    Ok(Some(result_ref))
                }
            })
            .copied()
    }

    fn include_path_in_query_error(
        mut error: QueryError,
        ranges: &[(String, Range<usize>)],
        source: &str,
        start_offset: usize,
    ) -> Error {
        let offset_within_section = error.offset - start_offset;
        let (path, range) = ranges
            .iter()
            .find(|(_, range)| range.contains(&offset_within_section))
            .unwrap();
        error.offset = offset_within_section - range.start;
        error.row = source[range.start..offset_within_section]
            .chars()
            .filter(|c| *c == '\n')
            .count();
        Error::from(error).context(format!("Error in query file {:?}", path))
    }

    #[allow(clippy::type_complexity)]
    fn read_queries(
        &self,
        paths: &Option<Vec<String>>,
        default_path: &str,
    ) -> Result<(String, Vec<(String, Range<usize>)>)> {
        let mut query = String::new();
        let mut path_ranges = Vec::new();
        if let Some(paths) = paths.as_ref() {
            for path in paths {
                let abs_path = self.root_path.join(path);
                let prev_query_len = query.len();
                query += &fs::read_to_string(&abs_path)
                    .with_context(|| format!("Failed to read query file {:?}", path))?;
                path_ranges.push((path.clone(), prev_query_len..query.len()));
            }
        } else {
            let queries_path = self.root_path.join("queries");
            let path = queries_path.join(default_path);
            if path.exists() {
                query = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read query file {:?}", path))?;
                path_ranges.push((default_path.to_string(), 0..query.len()));
            }
        }

        Ok((query, path_ranges))
    }
}

fn needs_recompile(
    lib_path: &Path,
    parser_c_path: &Path,
    scanner_path: &Option<PathBuf>,
) -> Result<bool> {
    if !lib_path.exists() {
        return Ok(true);
    }
    let lib_mtime = mtime(lib_path)?;
    if mtime(parser_c_path)? > lib_mtime {
        return Ok(true);
    }
    if let Some(scanner_path) = scanner_path {
        if mtime(scanner_path)? > lib_mtime {
            return Ok(true);
        }
    }
    Ok(false)
}

fn mtime(path: &Path) -> Result<SystemTime> {
    Ok(fs::metadata(path)?.modified()?)
}

fn replace_dashes_with_underscores(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    for c in name.chars() {
        if c == '-' {
            result.push('_');
        } else {
            result.push(c);
        }
    }
    result
}
