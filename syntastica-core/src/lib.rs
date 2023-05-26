mod error;
pub mod providers;
pub mod style;
pub mod theme;

pub use error::*;

/// All theme keys that are recognized by syntastica. Based on the list from
/// [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/blob/master/CONTRIBUTING.md).
#[rustfmt::skip] // rustfmt messes up the comment alignment
pub const THEME_KEYS: &[&str] = &[
    "comment",               // line and block comments
    "comment.documentation", // comments documenting code
    "error",                 // syntax/parser errors
    "none",                  // completely disable the highlight
    "preproc",               // various preprocessor directives & shebangs
    "define",                // preprocessor definition directives
    "operator",              // symbolic operators (e.g. `+` / `*`)

    // punctuation
    "punctuation.delimiter", // delimiters (e.g. `;` / `.` / `,`)
    "punctuation.bracket",   // brackets (e.g. `()` / `{}` / `[]`)
    "punctuation.special",   // special symbols (e.g. `{}` in string interpolation)

    // literals
    "string",                // string literals
    "string.documentation",  // string documenting code (e.g. Python docstrings)
    "string.regex",          // regular expressions
    "string.escape",         // escape sequences
    "string.special",        // other special strings (e.g. dates)
    "character",             // character literals
    "character.special",     // special characters (e.g. wildcards)
    "boolean",               // boolean literals
    "number",                // numeric literals
    "float",                 // floating-point number literals

    // functions
    "function",              // function definitions
    "function.builtin",      // built-in functions
    "function.call",         // function calls
    "function.macro",        // preprocessor macros
    "method",                // method definitions
    "method.call",           // method calls
    "constructor",           // constructor calls and definitions
    "parameter",             // parameters of a function

    // keywords
    "keyword",               // various keywords
    "keyword.coroutine",     // keywords related to coroutines (e.g. `go` in Go, `async/await` in Python)
    "keyword.function",      // keywords that define a function (e.g. `func` in Go, `def` in Python)
    "keyword.operator",      // operators that are English words (e.g. `and` / `or`)
    "keyword.return",        // keywords like `return` and `yield`
    "conditional",           // keywords related to conditionals (e.g. `if` / `else`)
    "conditional.ternary",   // ternary operator (e.g. `?` / `:`)
    "repeat",                // keywords related to loops (e.g. `for` / `while`)
    "debug",                 // keywords related to debugging
    "label",                 // GOTO and other labels (e.g. `label:` in C)
    "include",               // keywords for including modules (e.g. `import` / `from` in Python)
    "exception",             // keywords related to exceptions (e.g. `throw` / `catch`)

    // types
    "type",                  // type or class definitions and annotations
    "type.builtin",          // built-in types
    "type.definition",       // type definitions (e.g. `typedef` in C)
    "type.qualifier",        // type qualifiers (e.g. `const`)
    "storageclass",          // modifiers that affect storage in memory or life-time
    "attribute",             // attribute annotations (e.g. Python decorators)
    "field",                 // object and struct fields
    "property",              // similar to `@field`

    // identifiers
    "variable",              // various variable names
    "variable.builtin",      // built-in variable names (e.g. `this`)
    "constant",              // constant identifiers
    "constant.builtin",      // built-in constant values
    "constant.macro",        // constants defined by the preprocessor
    "namespace",             // modules or namespaces
    "symbol",                // symbols or atoms

    // text
    "text",                  // non-structured text
    "text.strong",           // bold text
    "text.emphasis",         // text with emphasis
    "text.underline",        // underlined text
    "text.strike",           // strikethrough text
    "text.title",            // text that is part of a title
    "text.literal",          // literal or verbatim text (e.g., inline code)
    "text.quote",            // text quotations
    "text.uri",              // URIs (e.g. hyperlinks)
    "text.math",             // math environments (e.g. `$ ... $` in LaTeX)
    "text.environment",      // text environments of markup languages
    "text.environment.name", // text indicating the type of an environment
    "text.reference",        // text references, footnotes, citations, etc.
    "text.todo",             // todo notes
    "text.note",             // info notes
    "text.warning",          // warning notes
    "text.danger",           // danger/error notes
    "text.diff.add",         // added text (for diff files)
    "text.diff.delete",      // deleted text (for diff files)

    // tags
    "tag",                   // XML tag names
    "tag.attribute",         // XML tag attributes
    "tag.delimiter",         // XML tag delimiters
];
