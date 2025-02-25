#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms, missing_docs)]
#![allow(static_mut_refs)]

use std::{
    ffi::{c_char, CStr, CString},
    slice,
    str::FromStr,
};

use syntastica::{
    language_set::SupportedLanguage,
    renderer::{HtmlRenderer, TerminalRenderer},
    style::Color,
    theme::ResolvedTheme,
    Highlights, Processor,
};
use syntastica_parsers_git::{Lang, LanguageSetImpl};

static mut LANG_NAMES_LIST: Vec<String> = vec![];
static mut LANGS_LIST: Vec<Lang> = vec![];
static mut LANGUAGES: Option<LanguageSetImpl> = None;
static mut PROCESSOR: Option<Processor<'static, LanguageSetImpl>> = None;

static mut CODE: String = String::new();
static mut HIGHLIGHTS: Option<Highlights<'static>> = None;

unsafe fn string_from_ptr(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

fn string_to_ptr(string: String) -> *const c_char {
    CString::new(string).unwrap().into_raw()
}

/// Initialize the global [`LanguageSet`](syntastica::language_set::LanguageSet)
/// and [`Processor`].
///
/// This function _must_ be called before any of the others. It accepts a list of languages to
/// load. The function can be called multiple times to re-initialize with a different set of
/// languages.
///
/// # Safety
///
/// The `langs` pointer _must_ be valid, `langs_len` _must_ be the length of the `langs` list, and
/// every pointer in that list _must_ be a pointer to a valid C string.
#[no_mangle]
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn init(langs: *const *const c_char, langs_len: usize) {
    let langs = if langs.is_null() {
        None
    } else {
        let langs = unsafe { slice::from_raw_parts(langs, langs_len) };
        Some(
            langs
                .iter()
                .map(|ptr| unsafe { string_from_ptr(*ptr) })
                .collect(),
        )
    };

    // SAFETY: This application is single-threaded, so it is safe to use mutable statics
    unsafe {
        LANGUAGES = Some(match langs {
            Some(langs) => {
                LANG_NAMES_LIST = langs;
                LANGS_LIST = Vec::with_capacity(LANG_NAMES_LIST.len());
                for name in &LANG_NAMES_LIST {
                    match Lang::for_name(name, &()) {
                        Ok(lang) => LANGS_LIST.push(lang),
                        Err(err) => {
                            eprintln!("initialization failed: {err}");
                            return;
                        }
                    }
                }
                let set = LanguageSetImpl::new();
                if let Err(err) = set.preload(&LANGS_LIST) {
                    eprintln!("initialization failed: {err}");
                }
                set
            }
            None => LanguageSetImpl::new(),
        });
        PROCESSOR = Some(Processor::new(LANGUAGES.as_ref().unwrap()));
    }
}

/// Process and render a piece of code in the given language with the given theme.
///
/// Before this function is called, [`init`] must have been called at least once.
///
/// # Safety
///
/// All parameters must point to valid C strings.
#[no_mangle]
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn highlight(
    code: *const c_char,
    language: *const c_char,
    theme: *const c_char,
    renderer: *const c_char,
) -> *const c_char {
    // SAFETY: This application is single-threaded, so it is safe to use mutable statics
    if unsafe { LANGUAGES.is_none() } {
        eprintln!("`highlight` was called before `init`");
        return std::ptr::null();
    }

    let code = unsafe { string_from_ptr(code) };
    let language = unsafe { string_from_ptr(language) };
    let theme = unsafe { string_from_ptr(theme) };
    let renderer = unsafe { string_from_ptr(renderer) };

    let Ok(language) = Lang::for_name(&language, &()) else {
        eprintln!("unsupported language '{language}'");
        return std::ptr::null();
    };

    let Some(theme) = syntastica_themes::from_str(&theme) else {
        eprintln!("invalid theme '{theme}'");
        return std::ptr::null();
    };

    // SAFETY: This application is single-threaded, so it is safe to use mutable statics
    let highlights = match unsafe { PROCESSOR.as_mut() }
        .expect("`init` was called before")
        .process(&code, language)
    {
        Ok(highlights) => highlights,
        Err(err) => {
            eprintln!("processing failed: {err}");
            return std::ptr::null();
        }
    };

    _render(&highlights, &renderer, theme)
}

/// Process a piece of code in the given language, and save the [`Highlights`] for a following call
/// to [`render`].
///
/// Before this function is called, [`init`] must have been called at least once.
///
/// # Safety
///
/// All parameters must point to valid C strings.
#[no_mangle]
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn process(code: *const c_char, language: *const c_char) {
    // SAFETY: This application is single-threaded, so it is safe to use mutable statics
    if unsafe { LANGUAGES.is_none() } {
        eprintln!("`process` was called before `init`");
        return;
    }

    let code = unsafe { string_from_ptr(code) };
    let language = unsafe { string_from_ptr(language) };

    let Ok(language) = Lang::for_name(&language, &()) else {
        eprintln!("unsupported language '{language}'");
        return;
    };

    // SAFETY: This application is single-threaded, so it is safe to use mutable statics
    unsafe { CODE = code };
    let highlights = match unsafe { PROCESSOR.as_mut() }
        .expect("`init` was called before")
        .process(unsafe { &CODE }, language)
    {
        Ok(highlights) => highlights,
        Err(err) => {
            eprintln!("processing failed: {err}");
            return;
        }
    };

    unsafe { HIGHLIGHTS = Some(highlights) }
}

/// Render the code that was previously processed by calling [`process`] given the name of a
/// [`Renderer`](syntastica::renderer::Renderer).
///
/// Before this function is called, [`process`] must have been called at least once.
///
/// The renderer name is either `HTML` or `Terminal` in any casing. To specify a background color
/// for the terminal renderer, append a hex color literal like `terminal#282828` or `Terminal#fff`.
///
/// # Safety
///
/// All parameters must point to valid C strings.
#[no_mangle]
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn render(theme: *const c_char, renderer: *const c_char) -> *const c_char {
    let theme = unsafe { string_from_ptr(theme) };
    let renderer = unsafe { string_from_ptr(renderer) };

    // SAFETY: This application is single-threaded, so it is safe to use mutable statics
    let highlights = match unsafe { HIGHLIGHTS.as_ref() } {
        Some(highlights) => highlights,
        None => {
            eprintln!("`render` was called before `process`");
            return std::ptr::null();
        }
    };

    let Some(theme) = syntastica_themes::from_str(&theme) else {
        eprintln!("invalid theme '{theme}'");
        return std::ptr::null();
    };

    _render(highlights, &renderer, theme)
}

fn _render(highlights: &Highlights<'_>, renderer: &str, theme: ResolvedTheme) -> *const c_char {
    let out = match renderer.to_lowercase().as_str() {
        "terminal" => syntastica::render(highlights, &mut TerminalRenderer::new(None), theme),
        "html" => syntastica::render(highlights, &mut HtmlRenderer, theme),
        name => match name.strip_prefix("terminal#") {
            Some(color_hex) => match Color::from_str(color_hex) {
                Ok(color) => {
                    syntastica::render(highlights, &mut TerminalRenderer::new(Some(color)), theme)
                }
                Err(err) => {
                    eprintln!("invalid background color '{color_hex}' for TerminalRenderer: {err}");
                    return std::ptr::null();
                }
            },
            None => {
                eprintln!("invalid renderer name '{name}'");
                return std::ptr::null();
            }
        },
    };

    string_to_ptr(out)
}
