#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms, missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::{
    borrow::Cow,
    collections::HashMap,
    ffi::{c_char, c_void, CStr, CString},
    marker::PhantomData,
    str::FromStr,
    sync::{LazyLock, Mutex},
};

use syntastica::{
    language_set::{FileType, HighlightConfiguration, Language, LanguageSet, SupportedLanguage},
    renderer::{HtmlRenderer, TerminalRenderer},
    style::Color,
    theme::ResolvedTheme,
    Highlights, Processor,
};

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;
}

fn alloc_string(str: &str) -> *const c_char {
    let ptr = unsafe { malloc(str.len() + 1) };
    unsafe { memcpy(ptr, str.as_ptr() as *const _, str.len()) };
    unsafe { (ptr as *mut c_char).add(str.len()).write(0) };
    ptr as *const _
}

macro_rules! bail {
    ($errmsg:ident, $($msg:tt)+) => {{
        unsafe { *$errmsg = alloc_string(&format!($($msg)*)) };
        return std::ptr::null();
    }};
}

static LANGUAGES: LazyLock<LangSet> = LazyLock::new(LangSet::default);
static PROCESSOR: LazyLock<Mutex<Processor<'static, LangSet>>> =
    LazyLock::new(|| Mutex::new(Processor::new(&*LANGUAGES)));

syntastica_macros::js_lang_info!();

struct Lang<'set>(&'static str, PhantomData<&'set ()>);

#[derive(Default)]
struct LangSet {
    languages: Mutex<HashMap<&'static str, (&'static HighlightConfiguration, Vec<FileType>)>>,
}

impl LangSet {
    fn add_lang(&self, info: &'static LangInfo) {
        fn cptr_to_str(str: *const c_char) -> &'static str {
            unsafe { CStr::from_ptr(str).to_str().unwrap() }
        }

        let name = cptr_to_str(info.name);
        let mut config = HighlightConfiguration::new(
            unsafe { (&info.language as *const Language).read() },
            name,
            cptr_to_str(info.highlights_query),
            cptr_to_str(info.injections_query),
            cptr_to_str(info.locals_query),
        )
        .unwrap();
        config.configure(syntastica::theme::THEME_KEYS);
        self.languages.lock().unwrap().insert(
            name,
            (
                Box::leak(Box::new(config)),
                unsafe { std::slice::from_raw_parts(info.file_types, info.file_types_len) }
                    .iter()
                    .map(|&ft| FileType::from_str(cptr_to_str(ft)).unwrap())
                    .collect(),
            ),
        );
    }
}

impl<'s> LanguageSet<'s> for LangSet {
    type Language = Lang<'s>;

    fn get_language(
        &self,
        language: Self::Language,
    ) -> syntastica::Result<&HighlightConfiguration> {
        Ok(self
            .languages
            .lock()
            .unwrap()
            .get(language.0)
            .expect("if a Lang instance exists, it should be a loaded language")
            .0)
    }
}

impl<'set> SupportedLanguage<'set, LangSet> for Lang<'set> {
    fn name(&self) -> Cow<'_, str> {
        self.0.into()
    }

    fn for_name(name: impl AsRef<str>, set: &'set LangSet) -> syntastica::Result<Self> {
        let name = name.as_ref();
        set.languages
            .lock()
            .unwrap()
            .get_key_value(name)
            .map(|(&name, _)| Self(name, PhantomData))
            .ok_or_else(|| syntastica::Error::UnsupportedLanguage(name.to_string()))
    }

    fn for_file_type(_file_type: FileType, _set: &'set LangSet) -> Option<Self> {
        None
    }
}

unsafe fn string_from_ptr(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}

fn string_to_ptr(string: String) -> *const c_char {
    CString::new(string).unwrap().into_raw()
}

/// Add a language to the set.
///
/// # Safety
///
/// The `info` param must point to a valid `LangInfo` struct with the `'static` lifetime.
#[no_mangle]
pub unsafe fn add_language(info: *mut LangInfo) {
    LANGUAGES.add_lang(unsafe { &*info });
}

/// Process and render a piece of code in the given language with the given theme.
///
/// # Safety
///
/// All parameters must be valid pointers.
#[no_mangle]
pub unsafe fn highlight(
    errmsg: *mut *const c_char,
    code: *const c_char,
    language: *const c_char,
    theme: *const c_char,
    renderer: *const c_char,
) -> *const c_char {
    let code = unsafe { string_from_ptr(code) };
    let language = unsafe { string_from_ptr(language) };
    let theme = unsafe { string_from_ptr(theme) };
    let renderer = unsafe { string_from_ptr(renderer) };

    let Ok(language) = Lang::for_name(&language, &*LANGUAGES) else {
        bail!(errmsg, "unsupported language '{language}'");
    };

    let Some(theme) = syntastica_themes::from_str(&theme) else {
        bail!(errmsg, "invalid theme '{theme}'");
    };

    let highlights = match PROCESSOR.lock().unwrap().process(&code, language) {
        Ok(highlights) => highlights,
        Err(err) => bail!(errmsg, "processing failed: {err}"),
    };

    _render(errmsg, &highlights, &renderer, theme)
}

/// Process a piece of code in the given language, and return the [`Highlights`] for a following
/// call to [`render`].
///
/// # Safety
///
/// All parameters must be valid pointers.
#[no_mangle]
pub unsafe fn process(
    errmsg: *mut *const c_char,
    code: *const c_char,
    language: *const c_char,
) -> *const c_char {
    let code = unsafe { string_from_ptr(code) };
    let language = unsafe { string_from_ptr(language) };

    let Ok(language) = Lang::for_name(&language, &*LANGUAGES) else {
        bail!(errmsg, "unsupported language '{language}'");
    };

    let highlights = match PROCESSOR.lock().unwrap().process(&code, language) {
        Ok(highlights) => highlights,
        Err(err) => bail!(errmsg, "processing failed: {err}"),
    };

    match serde_json::to_string(&highlights) {
        Ok(highlights) => alloc_string(&highlights),
        Err(err) => bail!(errmsg, "failed serializing highlights: {err}"),
    }
}

/// Render code that was previously processed by calling [`process`] given the name of a
/// [`Renderer`](syntastica::renderer::Renderer).
///
/// The renderer name is either `HTML` or `Terminal` in any casing. To specify a background color
/// for the terminal renderer, append a hex color literal like `terminal#282828` or `Terminal#fff`.
///
/// # Safety
///
/// All parameters must be valid pointers.
#[no_mangle]
pub unsafe fn render(
    errmsg: *mut *const c_char,
    highlights: *const c_char,
    theme: *const c_char,
    renderer: *const c_char,
) -> *const c_char {
    let highlights = Box::leak(Box::new(unsafe { string_from_ptr(highlights) }));
    let highlights_ptr = highlights as *mut _;
    let theme = unsafe { string_from_ptr(theme) };
    let renderer = unsafe { string_from_ptr(renderer) };

    let highlights = match serde_json::from_str::<Highlights<'_>>(highlights) {
        Ok(highlights) => highlights,
        Err(err) => bail!(errmsg, "highlights are invalid JSON: {err}"),
    };

    let Some(theme) = syntastica_themes::from_str(&theme) else {
        bail!(errmsg, "invalid theme '{theme}'");
    };

    let res = _render(errmsg, &highlights, &renderer, theme);
    unsafe { drop(Box::from_raw(highlights_ptr)) };
    res
}

fn _render(
    errmsg: *mut *const c_char,
    highlights: &Highlights<'_>,
    renderer: &str,
    theme: ResolvedTheme,
) -> *const c_char {
    let out = match renderer.to_lowercase().as_str() {
        "terminal" => syntastica::render(highlights, &mut TerminalRenderer::new(None), theme),
        "html" => syntastica::render(highlights, &mut HtmlRenderer, theme),
        name => match name.strip_prefix("terminal#") {
            Some(color_hex) => match Color::from_str(color_hex) {
                Ok(color) => {
                    syntastica::render(highlights, &mut TerminalRenderer::new(Some(color)), theme)
                }
                Err(err) => bail!(
                    errmsg,
                    "invalid background color '{color_hex}' for TerminalRenderer: {err}"
                ),
            },
            None => bail!(errmsg, "invalid renderer name '{name}'"),
        },
    };

    string_to_ptr(out)
}
