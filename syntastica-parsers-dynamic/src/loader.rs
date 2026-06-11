//! Forked from <https://github.com/tree-sitter/tree-sitter/blob/v0.25.2/cli/loader/src/lib.rs>
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

use std::{
    borrow::Borrow,
    collections::HashMap,
    env, fs,
    io::{BufRead, BufReader},
    mem,
    path::{Path, PathBuf},
    process::Command,
    sync::{LazyLock, Mutex},
    time::SystemTime,
};

use anyhow::{anyhow, Context, Result};
use etcetera::BaseStrategy as _;
use fs4::fs_std::FileExt;
use indoc::indoc;
use libloading::{Library, Symbol};
use once_cell::unsync::OnceCell;
use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use syntastica_core::language_set::HighlightConfiguration;
use tree_sitter::Language;

static GRAMMAR_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""name":\s*"(.*?)""#).unwrap());

#[derive(Default)]
pub struct Config {
    pub parser_directories: Vec<PathBuf>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(untagged)]
pub enum PathsJSON {
    #[default]
    Empty,
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

impl PathsJSON {
    fn into_vec(self) -> Option<Vec<PathBuf>> {
        match self {
            Self::Empty => None,
            Self::Single(s) => Some(vec![s]),
            Self::Multiple(s) => Some(s),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TreeSitterJSON {
    pub grammars: Vec<Grammar>,
}

impl TreeSitterJSON {
    pub fn from_file(path: &Path) -> Result<Self> {
        Ok(serde_json::from_str(&fs::read_to_string(
            path.join("tree-sitter.json"),
        )?)?)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Grammar {
    pub name: String,
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub external_files: PathsJSON,
    pub file_types: Option<Vec<String>>,
    #[serde(default)]
    pub highlights: PathsJSON,
    #[serde(default)]
    pub injections: PathsJSON,
    #[serde(default)]
    pub locals: PathsJSON,
    pub injection_regex: Option<String>,
    pub first_line_regex: Option<String>,
    pub content_regex: Option<String>,
}

const BUILD_TARGET: &str = env!("BUILD_TARGET");
const BUILD_HOST: &str = env!("BUILD_HOST");

pub struct LanguageConfiguration<'a> {
    pub _content_regex: Option<Regex>,
    pub first_line_regex: Option<Regex>,
    pub injection_regex: Option<Regex>,
    pub file_types: Vec<String>,
    pub root_path: PathBuf,
    pub highlights_filenames: Option<Vec<PathBuf>>,
    pub injections_filenames: Option<Vec<PathBuf>>,
    pub locals_filenames: Option<Vec<PathBuf>>,
    pub language_name: String,
    language_id: usize,
    highlight_names: &'a Mutex<Vec<String>>,
    use_all_highlight_names: bool,
}

pub struct Loader {
    pub parser_lib_path: PathBuf,
    languages_by_id: Vec<(PathBuf, OnceCell<Language>, Option<Vec<PathBuf>>)>,
    language_configurations: Vec<LanguageConfiguration<'static>>,
    language_configuration_ids_by_file_type: HashMap<String, Vec<usize>>,
    language_configuration_in_current_path: Option<usize>,
    language_configuration_ids_by_first_line_regex: HashMap<String, Vec<usize>>,
    highlight_names: Box<Mutex<Vec<String>>>,
    use_all_highlight_names: bool,
}

pub struct CompileConfig<'a> {
    pub src_path: &'a Path,
    pub header_paths: Vec<&'a Path>,
    pub parser_path: PathBuf,
    pub scanner_path: Option<PathBuf>,
    pub external_files: Option<&'a [PathBuf]>,
    pub output_path: Option<PathBuf>,
    pub flags: &'a [&'a str],
    pub name: String,
}

impl<'a> CompileConfig<'a> {
    #[must_use]
    pub fn new(
        src_path: &'a Path,
        externals: Option<&'a [PathBuf]>,
        output_path: Option<PathBuf>,
    ) -> Self {
        Self {
            src_path,
            header_paths: vec![src_path],
            parser_path: src_path.join("parser.c"),
            scanner_path: None,
            external_files: externals,
            output_path,
            flags: &[],
            name: String::new(),
        }
    }
}

unsafe impl Sync for Loader {}

impl Loader {
    pub fn new() -> Result<Self> {
        let parser_lib_path = if let Ok(path) = env::var("TREE_SITTER_LIBDIR") {
            PathBuf::from(path)
        } else {
            if cfg!(target_os = "macos") {
                let legacy_apple_path = etcetera::base_strategy::Apple::new()?
                    .cache_dir() // `$HOME/Library/Caches/`
                    .join("tree-sitter");
                if legacy_apple_path.exists() && legacy_apple_path.is_dir() {
                    std::fs::remove_dir_all(legacy_apple_path)?;
                }
            }

            etcetera::choose_base_strategy()?
                .cache_dir()
                .join("tree-sitter")
                .join("lib")
        };
        Ok(Self::with_parser_lib_path(parser_lib_path))
    }

    #[must_use]
    pub fn with_parser_lib_path(parser_lib_path: PathBuf) -> Self {
        Self {
            parser_lib_path,
            languages_by_id: Vec::new(),
            language_configurations: Vec::new(),
            language_configuration_ids_by_file_type: HashMap::new(),
            language_configuration_in_current_path: None,
            language_configuration_ids_by_first_line_regex: HashMap::new(),
            highlight_names: Box::new(Mutex::new(Vec::new())),
            use_all_highlight_names: true,
        }
    }

    pub fn configure_highlights(&mut self, names: &[impl Borrow<str>]) {
        self.use_all_highlight_names = false;
        let mut highlights = self.highlight_names.lock().unwrap();
        highlights.clear();
        highlights.extend(names.iter().map(|s| s.borrow().to_owned()));
    }

    pub fn find_all_languages(&mut self, config: &Config) -> Result<()> {
        // if config.parser_directories.is_empty() {
        //     eprintln!("Warning: You have not configured any parser directories!");
        //     eprintln!("Please run `tree-sitter init-config` and edit the resulting");
        //     eprintln!("configuration file to indicate where we should look for");
        //     eprintln!("language grammars.");
        //     eprintln!();
        // }
        for parser_container_dir in &config.parser_directories {
            if let Ok(entries) = fs::read_dir(parser_container_dir) {
                for entry in entries {
                    let entry = entry?;
                    if let Some(parser_dir_name) = entry.file_name().to_str() {
                        if parser_dir_name.starts_with("tree-sitter-") {
                            self.find_language_configurations_at_path(
                                &parser_container_dir.join(parser_dir_name),
                                false,
                            )
                            .ok();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn language_configuration_for_name(
        &self,
        name: &str,
    ) -> Result<Option<(Language, &LanguageConfiguration<'_>)>> {
        for configuration in &self.language_configurations {
            if configuration.language_name == name {
                let language = self.language_for_id(configuration.language_id)?;
                return Ok(Some((language, configuration)));
            }
        }
        Ok(None)
    }

    pub fn _language_configuration_for_file_name(
        &self,
        path: &Path,
    ) -> Result<Option<(Language, &LanguageConfiguration<'_>)>> {
        // Find all the language configurations that match this file name
        // or a suffix of the file name.
        let configuration_ids = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|file_name| self.language_configuration_ids_by_file_type.get(file_name))
            .or_else(|| {
                let mut path = path.to_owned();
                let mut extensions = Vec::with_capacity(2);
                while let Some(extension) = path.extension() {
                    extensions.push(extension.to_str()?.to_string());
                    path = PathBuf::from(path.file_stem()?.to_os_string());
                }
                extensions.reverse();
                self.language_configuration_ids_by_file_type
                    .get(&extensions.join("."))
            });

        if let Some(configuration_ids) = configuration_ids {
            if !configuration_ids.is_empty() {
                let configuration = if configuration_ids.len() == 1 {
                    &self.language_configurations[configuration_ids[0]]
                }
                // If multiple language configurations match, then determine which
                // one to use by applying the configurations' content regexes.
                else {
                    let file_contents =
                        fs::read(path).with_context(|| format!("Failed to read path {path:?}"))?;
                    let file_contents = String::from_utf8_lossy(&file_contents);
                    let mut best_score = -2isize;
                    let mut best_configuration_id = None;
                    for configuration_id in configuration_ids {
                        let config = &self.language_configurations[*configuration_id];

                        // If the language configuration has a content regex, assign
                        // a score based on the length of the first match.
                        let score;
                        if let Some(content_regex) = &config._content_regex {
                            if let Some(mat) = content_regex.find(&file_contents) {
                                score = (mat.end() - mat.start()) as isize;
                            }
                            // If the content regex does not match, then *penalize* this
                            // language configuration, so that language configurations
                            // without content regexes are preferred over those with
                            // non-matching content regexes.
                            else {
                                score = -1;
                            }
                        } else {
                            score = 0;
                        }
                        if score > best_score {
                            best_configuration_id = Some(*configuration_id);
                            best_score = score;
                        }
                    }

                    &self.language_configurations[best_configuration_id.unwrap()]
                };

                let language = self.language_for_id(configuration.language_id)?;
                return Ok(Some((language, configuration)));
            }
        }

        Ok(None)
    }

    pub fn language_configuration_for_injection_string(
        &self,
        string: &str,
    ) -> Result<Option<(Language, &LanguageConfiguration<'_>)>> {
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
        let (path, language, externals) = &self.languages_by_id[id];
        language
            .get_or_try_init(|| {
                let src_path = path.join("src");
                self.load_language_at_path(CompileConfig::new(
                    &src_path,
                    externals.as_deref(),
                    None,
                ))
            })
            .cloned()
    }

    pub fn load_language_at_path(&self, mut config: CompileConfig<'_>) -> Result<Language> {
        let grammar_path = config.src_path.join("grammar.json");
        config.name = Self::grammar_json_name(&grammar_path)?;
        self.load_language_at_path_with_name(config)
    }

    pub fn load_language_at_path_with_name(
        &self,
        mut config: CompileConfig<'_>,
    ) -> Result<Language> {
        let lib_name = config.name.to_string();
        let language_fn_name = format!(
            "tree_sitter_{}",
            replace_dashes_with_underscores(&config.name)
        );

        if config.output_path.is_none() {
            fs::create_dir_all(&self.parser_lib_path)?;
        }

        let mut recompile = config.output_path.is_some(); // if specified, always recompile

        let output_path = config.output_path.unwrap_or_else(|| {
            let mut path = self.parser_lib_path.join(lib_name);
            path.set_extension(env::consts::DLL_EXTENSION);
            path
        });
        config.output_path = Some(output_path.clone());

        let parser_path = config.src_path.join("parser.c");
        config.scanner_path = self.get_scanner_path(config.src_path);

        let mut paths_to_check = vec![parser_path];

        if let Some(scanner_path) = config.scanner_path.as_ref() {
            paths_to_check.push(scanner_path.clone());
        }

        paths_to_check.extend(
            config
                .external_files
                .unwrap_or_default()
                .iter()
                .map(|p| config.src_path.join(p)),
        );

        if !recompile {
            recompile = needs_recompile(&output_path, &paths_to_check)
                .with_context(|| "Failed to compare source and binary timestamps")?;
        }

        let lock_path = if env::var("CROSS_RUNNER").is_ok() {
            tempfile::tempdir()
                .unwrap()
                .path()
                .join("tree-sitter")
                .join("lock")
                .join(format!("{}.lock", config.name))
        } else {
            etcetera::choose_base_strategy()?
                .cache_dir()
                .join("tree-sitter")
                .join("lock")
                .join(format!("{}.lock", config.name))
        };

        if let Ok(lock_file) = fs::OpenOptions::new().write(true).open(&lock_path) {
            recompile = false;
            if lock_file.try_lock_exclusive().is_err() {
                // if we can't acquire the lock, another process is compiling the parser, wait for
                // it and don't recompile
                lock_file.lock_exclusive()?;
                recompile = false;
            } else {
                // if we can acquire the lock, check if the lock file is older than 30 seconds, a
                // run that was interrupted and left the lock file behind should not block
                // subsequent runs
                let time = lock_file.metadata()?.modified()?.elapsed()?.as_secs();
                if time > 30 {
                    fs::remove_file(&lock_path)?;
                    recompile = true;
                }
            }
        }

        if recompile {
            fs::create_dir_all(lock_path.parent().unwrap()).with_context(|| {
                format!(
                    "Failed to create directory {:?}",
                    lock_path.parent().unwrap()
                )
            })?;
            let lock_file = fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&lock_path)?;
            lock_file.lock_exclusive()?;

            self.compile_parser_to_dylib(&config, &lock_file, &lock_path)?;

            if config.scanner_path.is_some() {
                self.check_external_scanner(&config.name, &output_path)?;
            }
        }

        let library = unsafe { Library::new(&output_path) }
            .with_context(|| format!("Error opening dynamic library {output_path:?}"))?;
        let language = unsafe {
            let language_fn = library
                .get::<Symbol<'_, unsafe extern "C" fn() -> Language>>(language_fn_name.as_bytes())
                .with_context(|| format!("Failed to load symbol {language_fn_name}"))?;
            language_fn()
        };
        mem::forget(library);
        Ok(language)
    }

    fn compile_parser_to_dylib(
        &self,
        config: &CompileConfig<'_>,
        lock_file: &fs::File,
        lock_path: &Path,
    ) -> Result<()> {
        let mut cc_config = cc::Build::new();
        cc_config
            .cargo_metadata(false)
            .cargo_warnings(false)
            .target(BUILD_TARGET)
            .host(BUILD_HOST)
            .file(&config.parser_path)
            .includes(&config.header_paths)
            .std("c11");

        if let Some(scanner_path) = config.scanner_path.as_ref() {
            cc_config.file(scanner_path);
        }

        cc_config.opt_level(2).extra_warnings(false);

        for flag in config.flags {
            cc_config.define(flag, None);
        }

        let compiler = cc_config.get_compiler();
        let mut command = Command::new(compiler.path());
        command.args(compiler.args());
        for (key, value) in compiler.env() {
            command.env(key, value);
        }

        let output_path = config.output_path.as_ref().unwrap();

        if compiler.is_like_msvc() {
            let out = format!("-out:{}", output_path.to_str().unwrap());
            command.arg("-LD");
            command.arg("-utf-8");
            command.args(cc_config.get_files());
            command.arg("-link").arg(out);
        } else {
            command.arg("-Werror=implicit-function-declaration");
            if cfg!(any(target_os = "macos", target_os = "ios")) {
                command.arg("-dynamiclib");
                // TODO: remove when supported
                command.arg("-UTREE_SITTER_REUSE_ALLOCATOR");
            } else {
                command.arg("-shared");
            }
            command.args(cc_config.get_files());
            command.arg("-o").arg(output_path);
        }

        let output = command.output().with_context(|| {
            format!("Failed to execute the C compiler with the following command:\n{command:?}")
        })?;

        FileExt::unlock(lock_file)?;
        fs::remove_file(lock_path)?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "Parser compilation failed.\nStdout: {}\nStderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    #[cfg(unix)]
    fn check_external_scanner(&self, name: &str, library_path: &Path) -> Result<()> {
        let prefix = if cfg!(any(target_os = "macos", target_os = "ios")) {
            "_"
        } else {
            ""
        };
        let mut must_have = vec![
            format!("{prefix}tree_sitter_{name}_external_scanner_create"),
            format!("{prefix}tree_sitter_{name}_external_scanner_destroy"),
            format!("{prefix}tree_sitter_{name}_external_scanner_serialize"),
            format!("{prefix}tree_sitter_{name}_external_scanner_deserialize"),
            format!("{prefix}tree_sitter_{name}_external_scanner_scan"),
        ];

        let command = Command::new("nm")
            .arg("-W")
            .arg("-U")
            .arg(library_path)
            .output();
        if let Ok(output) = command {
            if output.status.success() {
                let mut found_non_static = false;
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    if line.contains(" T ") {
                        if let Some(function_name) =
                            line.split_whitespace().collect::<Vec<_>>().get(2)
                        {
                            if !line.contains("tree_sitter_") {
                                if !found_non_static {
                                    found_non_static = true;
                                    // eprintln!("Warning: Found non-static non-tree-sitter functions in the external scannner");
                                }
                                // eprintln!("  `{function_name}`");
                            } else {
                                must_have.retain(|f| f != function_name);
                            }
                        }
                    }
                }
                // if found_non_static {
                //     eprintln!("Consider making these functions static, they can cause conflicts when another tree-sitter project uses the same function name");
                // }

                if !must_have.is_empty() {
                    let missing = must_have
                        .iter()
                        .map(|f| format!("  `{f}`"))
                        .collect::<Vec<_>>()
                        .join("\n");

                    return Err(anyhow!(format!(
                        indoc! {"
                            Missing required functions in the external scanner, parsing won't work without these!

                            {}

                            You can read more about this at https://tree-sitter.github.io/tree-sitter/creating-parsers/4-external-scanners
                        "},
                        missing,
                    )));
                }
            }
        }

        Ok(())
    }

    #[cfg(windows)]
    fn check_external_scanner(&self, _name: &str, _library_path: &Path) -> Result<()> {
        // TODO: there's no nm command on windows, whoever wants to implement this can and should :)

        // let mut must_have = vec![
        //     format!("tree_sitter_{name}_external_scanner_create"),
        //     format!("tree_sitter_{name}_external_scanner_destroy"),
        //     format!("tree_sitter_{name}_external_scanner_serialize"),
        //     format!("tree_sitter_{name}_external_scanner_deserialize"),
        //     format!("tree_sitter_{name}_external_scanner_scan"),
        // ];

        Ok(())
    }

    pub fn find_language_configurations_at_path(
        &mut self,
        parser_path: &Path,
        set_current_path_config: bool,
    ) -> Result<&[LanguageConfiguration<'_>]> {
        let initial_language_configuration_count = self.language_configurations.len();

        let ts_json = TreeSitterJSON::from_file(parser_path);
        if let Ok(config) = ts_json {
            let language_count = self.languages_by_id.len();
            for grammar in config.grammars {
                // Determine the path to the parser directory. This can be specified in
                // the tree-sitter.json, but defaults to the directory containing the
                // tree-sitter.json.
                let language_path = parser_path.join(grammar.path.unwrap_or(PathBuf::from(".")));

                // Determine if a previous language configuration in this package.json file
                // already uses the same language.
                let mut language_id = None;
                for (id, (path, _, _)) in
                    self.languages_by_id.iter().enumerate().skip(language_count)
                {
                    if language_path == *path {
                        language_id = Some(id);
                    }
                }

                // If not, add a new language path to the list.
                let language_id = if let Some(language_id) = language_id {
                    language_id
                } else {
                    self.languages_by_id.push((
                            language_path,
                            OnceCell::new(),
                            grammar.external_files.clone().into_vec().map(|files| {
                                files.into_iter()
                                    .map(|path| {
                                       let path = parser_path.join(path);
                                        // prevent p being above/outside of parser_path
                                        if path.starts_with(parser_path) {
                                            Ok(path)
                                        } else {
                                            Err(anyhow!("External file path {path:?} is outside of parser directory {parser_path:?}"))
                                        }
                                    })
                                    .collect::<Result<Vec<_>>>()
                            }).transpose()?,
                        ));
                    self.languages_by_id.len() - 1
                };

                let configuration = LanguageConfiguration {
                    root_path: parser_path.to_path_buf(),
                    language_name: grammar.name,
                    language_id,
                    file_types: grammar.file_types.unwrap_or_default(),
                    _content_regex: Self::regex(grammar.content_regex.as_deref()),
                    first_line_regex: Self::regex(grammar.first_line_regex.as_deref()),
                    injection_regex: Self::regex(grammar.injection_regex.as_deref()),
                    injections_filenames: grammar.injections.into_vec(),
                    locals_filenames: grammar.locals.into_vec(),
                    highlights_filenames: grammar.highlights.into_vec(),
                    highlight_names: &self.highlight_names,
                    use_all_highlight_names: self.use_all_highlight_names,
                };

                for file_type in &configuration.file_types {
                    self.language_configuration_ids_by_file_type
                        .entry(file_type.to_string())
                        .or_default()
                        .push(self.language_configurations.len());
                }
                if let Some(first_line_regex) = &configuration.first_line_regex {
                    self.language_configuration_ids_by_first_line_regex
                        .entry(first_line_regex.to_string())
                        .or_default()
                        .push(self.language_configurations.len());
                }

                self.language_configurations.push(unsafe {
                    mem::transmute::<LanguageConfiguration<'_>, LanguageConfiguration<'static>>(
                        configuration,
                    )
                });

                if set_current_path_config && self.language_configuration_in_current_path.is_none()
                {
                    self.language_configuration_in_current_path =
                        Some(self.language_configurations.len() - 1);
                }
            }
        } else if let Err(e) = ts_json {
            match e.downcast_ref::<std::io::Error>() {
                // This is noisy, and not really an issue.
                Some(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                _ => {
                    // eprintln!(
                    //     "Warning: Failed to parse {} -- {e}",
                    //     parser_path.join("tree-sitter.json").display()
                    // );
                }
            }
        }

        // If we didn't find any language configurations in the tree-sitter.json file,
        // but there is a grammar.json file, then use the grammar file to form a simple
        // language configuration.
        if self.language_configurations.len() == initial_language_configuration_count
            && parser_path.join("src").join("grammar.json").exists()
        {
            let grammar_path = parser_path.join("src").join("grammar.json");
            let language_name = Self::grammar_json_name(&grammar_path)?;
            let configuration = LanguageConfiguration {
                root_path: parser_path.to_owned(),
                language_name,
                language_id: self.languages_by_id.len(),
                file_types: Vec::new(),
                _content_regex: None,
                first_line_regex: None,
                injection_regex: None,
                injections_filenames: None,
                locals_filenames: None,
                highlights_filenames: None,
                highlight_names: &self.highlight_names,
                use_all_highlight_names: self.use_all_highlight_names,
            };
            self.language_configurations.push(unsafe {
                mem::transmute::<LanguageConfiguration<'_>, LanguageConfiguration<'static>>(
                    configuration,
                )
            });
            self.languages_by_id
                .push((parser_path.to_owned(), OnceCell::new(), None));
        }

        Ok(&self.language_configurations[initial_language_configuration_count..])
    }

    fn regex(pattern: Option<&str>) -> Option<Regex> {
        pattern.and_then(|r| RegexBuilder::new(r).multi_line(true).build().ok())
    }

    fn grammar_json_name(grammar_path: &Path) -> Result<String> {
        let file = fs::File::open(grammar_path).with_context(|| {
            format!("Failed to open grammar.json at {}", grammar_path.display())
        })?;

        let first_three_lines = BufReader::new(file)
            .lines()
            .take(3)
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| {
                format!(
                    "Failed to read the first three lines of grammar.json at {}",
                    grammar_path.display()
                )
            })?
            .join("\n");

        let name = GRAMMAR_NAME_REGEX
            .captures(&first_three_lines)
            .and_then(|c| c.get(1))
            .ok_or_else(|| {
                anyhow!(
                    "Failed to parse the language name from grammar.json at {}",
                    grammar_path.display()
                )
            })?;

        Ok(name.as_str().to_string())
    }

    #[must_use]
    pub fn get_scanner_path(&self, src_path: &Path) -> Option<PathBuf> {
        let path = src_path.join("scanner.c");
        path.exists().then_some(path)
    }
}

impl LanguageConfiguration<'_> {
    pub fn highlight_config(
        &self,
        language: Language,
        paths: Option<&[PathBuf]>,
    ) -> syntastica_core::Result<Option<&'static mut HighlightConfiguration>> {
        let (highlights_filenames, injections_filenames, locals_filenames) = match paths {
            Some(paths) => (
                Some(
                    paths
                        .iter()
                        .filter(|p| p.ends_with("highlights.scm"))
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
                Some(
                    paths
                        .iter()
                        .filter(|p| p.ends_with("tags.scm"))
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
                Some(
                    paths
                        .iter()
                        .filter(|p| p.ends_with("locals.scm"))
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
            ),
            None => (None, None, None),
        };
        let highlights_query = self
            .read_queries(
                if highlights_filenames.is_some() {
                    highlights_filenames.as_deref()
                } else {
                    self.highlights_filenames.as_deref()
                },
                "highlights.scm",
            )
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?;
        let injections_query = self
            .read_queries(
                if injections_filenames.is_some() {
                    injections_filenames.as_deref()
                } else {
                    self.injections_filenames.as_deref()
                },
                "injections.scm",
            )
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?;
        let locals_query = self
            .read_queries(
                if locals_filenames.is_some() {
                    locals_filenames.as_deref()
                } else {
                    self.locals_filenames.as_deref()
                },
                "locals.scm",
            )
            .map_err(|err| syntastica_core::Error::Custom(err.to_string()))?;

        if highlights_query.is_empty() {
            Ok(None)
        } else {
            let mut result = HighlightConfiguration::new(
                language,
                &self.language_name,
                &highlights_query,
                &injections_query,
                &locals_query,
            )?;
            let mut all_highlight_names = self.highlight_names.lock().unwrap();
            if self.use_all_highlight_names {
                for capture_name in result.query.capture_names() {
                    if !all_highlight_names.iter().any(|x| x == capture_name) {
                        all_highlight_names.push((*capture_name).to_string());
                    }
                }
            }
            result.configure(all_highlight_names.as_slice());
            drop(all_highlight_names);
            let result_ref = Box::leak(Box::new(result));
            Ok(Some(result_ref))
        }
    }

    #[allow(clippy::type_complexity)]
    fn read_queries(&self, paths: Option<&[PathBuf]>, default_path: &str) -> Result<String> {
        let mut query = String::new();
        if let Some(paths) = paths {
            for path in paths {
                let abs_path = self.root_path.join(path);
                query += &fs::read_to_string(&abs_path)
                    .with_context(|| format!("Failed to read query file {path:?}"))?;
            }
        } else {
            // highlights.scm is needed to test highlights, and tags.scm to test tags
            // if default_path == "highlights.scm" || default_path == "tags.scm" {
            //     eprintln!(
            //         indoc! {"
            //             Warning: you should add a `{}` entry pointing to the highlights path in the `tree-sitter` object in the grammar's tree-sitter.json file.
            //             See more here: https://tree-sitter.github.io/tree-sitter/3-syntax-highlighting#query-paths
            //         "},
            //         default_path.replace(".scm", "")
            //     );
            // }
            let queries_path = self.root_path.join("queries");
            let path = queries_path.join(default_path);
            if path.exists() {
                query = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read query file {path:?}"))?;
            }
        }

        Ok(query)
    }
}

fn needs_recompile(lib_path: &Path, paths_to_check: &[PathBuf]) -> Result<bool> {
    if !lib_path.exists() {
        return Ok(true);
    }
    let lib_mtime =
        mtime(lib_path).with_context(|| format!("Failed to read mtime of {lib_path:?}"))?;
    for path in paths_to_check {
        if mtime(path)? > lib_mtime {
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
