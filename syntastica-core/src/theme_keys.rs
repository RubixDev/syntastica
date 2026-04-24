[
    "none",                        // completely disable the highlight

    // punctuation
    "punctuation.delimiter",       // delimiters (e.g. `;` / `.` / `,`)
    "punctuation.bracket",         // brackets (e.g. `()` / `{}` / `[]`)
    "punctuation.special",         // special symbols (e.g. `{}` in string interpolation)

    // literals
    "string",                      // string literals
    "string.documentation",        // string documenting code (e.g. Python docstrings)
    "string.regexp",               // regular expressions
    "string.escape",               // escape sequences
    "string.special",              // other special strings (e.g. dates)
    "string.special.symbol",       // symbols or atoms
    "string.special.url",          // URIs (e.g. hyperlinks)
    "string.special.path",         // filenames
    "character",                   // character literals
    "character.special",           // special characters (e.g. wildcards)
    "boolean",                     // boolean literals
    "number",                      // numeric literals
    "number.float",                // floating-point number literals

    // functions
    "function",                    // function definitions
    "function.builtin",            // built-in functions
    "function.call",               // function calls
    "function.macro",              // preprocessor macros
    "function.method",             // method definitions
    "function.method.call",        // method calls
    "constructor",                 // constructor calls and definitions
    "operator",                    // symbolic operators (e.g. `+` / `*`)

    // keywords
    "keyword",                     // keywords not fitting into specific categories
    "keyword.coroutine",           // keywords related to coroutines (e.g. `go` in Go, `async/await` in Python)
    "keyword.function",            // keywords that define a function (e.g. `func` in Go, `def` in Python)
    "keyword.operator",            // operators that are English words (e.g. `and` / `or`)
    "keyword.import",              // keywords for including or exporting modules (e.g. `import` / `from` in Python)
    "keyword.type",                // keywords describing namespaces and composite types (e.g. `struct`, `enum`)
    "keyword.modifier",            // keywords modifying other constructs (e.g. `const`, `static`, `public`)
    "keyword.repeat",              // keywords related to loops (e.g. `for` / `while`)
    "keyword.return",              // keywords like `return` and `yield`
    "keyword.debug",               // keywords related to debugging
    "keyword.exception",           // keywords related to exceptions (e.g. `throw` / `catch`)
    "keyword.conditional",         // keywords related to conditionals (e.g. `if` / `else`)
    "keyword.conditional.ternary", // ternary operator (e.g. `?` / `:`)
    "keyword.directive",           // various preprocessor directives & shebangs
    "keyword.directive.define",    // preprocessor definition directives

    // types
    "type",                        // type or class definitions and annotations
    "type.builtin",                // built-in types
    "type.definition",             // identifiers in type definitions (e.g. `typedef <type> <identifier>` in C)
    "attribute",                   // attribute annotations (e.g. Python decorators, Rust lifetimes)
    "attribute.builtin",           // builtin annotations (e.g. `@property` in Python)
    "property",                    // the key in key/value pairs

    // identifiers
    "variable",                    // various variable names
    "variable.builtin",            // built-in variable names (e.g. `this`)
    "variable.parameter",          // parameters of a function
    "variable.parameter.builtin",  // special parameters (e.g. `_`, `it`)
    "variable.member",             // object and struct fields
    "constant",                    // constant identifiers
    "constant.builtin",            // built-in constant values
    "constant.macro",              // constants defined by the preprocessor
    "module",                      // modules or namespaces
    "module.builtin",              // built-in modules or namespaces
    "label",                       // GOTO and other labels (e.g. `label:` in C), including heredoc labels

    // markup
    "markup.strong",               // bold text
    "markup.italic",               // italic text
    "markup.strikethrough",        // struck-through text
    "markup.underline",            // underlined text (only for literal underline markup!)
    "markup.heading",              // headings, titles (including markers)
    "markup.heading.1",            // top-level heading
    "markup.heading.2",            // section heading
    "markup.heading.3",            // subsection heading
    "markup.heading.4",            // and so on
    "markup.heading.5",            // and so forth
    "markup.heading.6",            // six levels ought to be enough for anybody
    "markup.quote",                // block quotes
    "markup.math",                 // math environments (e.g. `$ ... $` in LaTeX)
    "markup.link",                 // text references, footnotes, citations, etc.
    "markup.link.label",           // link, reference descriptions
    "markup.link.url",             // URL-style links
    "markup.raw",                  // literal or verbatim text (e.g. inline code)
    "markup.raw.block",            // literal or verbatim text as a stand-alone block
    "markup.list",                 // list markers
    "markup.list.checked",         // checked todo-style list markers
    "markup.list.unchecked",       // unchecked todo-style list markers
    "diff.plus",                   // added text (for diff files)
    "diff.minus",                  // deleted text (for diff files)
    "diff.delta",                  // changed text (for diff files)

    // comments
    "comment",                     // line and block comments
    "comment.documentation",       // comments documenting code
    "comment.error",               // error-type comments (e.g. `ERROR`, `FIXME`, `DEPRECATED`)
    "comment.warning",             // warning-type comments (e.g. `WARNING`, `FIX`, `HACK`)
    "comment.todo",                // todo-type comments (e.g. `TODO`, `WIP`)
    "comment.note",                // note-type comments (e.g. `NOTE`, `INFO`, `XXX`)

    // tags
    "tag",                         // XML-style tag names (and similar)
    "tag.builtin",                 // builtin tag names (e.g. HTML5 tags)
    "tag.attribute",               // XML-style tag attributes
    "tag.delimiter",               // XML-style tag delimiters
    // additional captures used by queries

    "annotation",
    "array",
    "assignment",
    "assignvalue",
    "cImport",
    "char.elm",
    "char.gren",
    "character.escape.elm",
    "character.escape.gren",
    "clean",
    "cmd",
    "comment.block",
    "comment.block.documentation",
    "comment.discard",
    "comment.doc",
    "comment.doc.__attribute__",
    "comment.elm",
    "comment.gren",
    "comment.hint",
    "comment.line",
    "comment.unused",
    "conditional",
    "conditional.ternary",
    "constant.boolean",
    "constant.builtin.boolean",
    "constant.character",
    "constant.character.escape",
    "constant.language",
    "constant.null",
    "constant.numeric",
    "constant.numeric.elm",
    "constant.numeric.float",
    "constant.numeric.gren",
    "constant.numeric.integer",
    "constructor.builtin",
    "define",
    "delimiter",
    "else",
    "embedded",
    "emphasis",
    "emphasis.strong",
    "error",
    "escape",
    "exception",
    "exec",
    "fi",
    "field",
    "field.builtin",
    "float",
    "function.elm",
    "function.gren",
    "function.macro.builtin",
    "function.method.builtin",
    "function.reference",
    "function.signal",
    "function.special",
    "head-var",
    "identifier",
    "identifier.constant",
    "identifier.parameter",
    "if",
    "ignoreme.module",
    "import",
    "include",
    "keyword.constructor",
    "keyword.control",
    "keyword.control.conditional",
    "keyword.control.elm",
    "keyword.control.exception",
    "keyword.control.gren",
    "keyword.control.import",
    "keyword.control.repeat",
    "keyword.control.return",
    "keyword.operator.arrow.elm",
    "keyword.operator.arrow.gren",
    "keyword.operator.assignment.elm",
    "keyword.operator.assignment.gren",
    "keyword.operator.elm",
    "keyword.operator.gren",
    "keyword.other",
    "keyword.other.elm",
    "keyword.other.gren",
    "keyword.other.port.elm",
    "keyword.other.port.gren",
    "keyword.phaser",
    "keyword.storage",
    "keyword.storage.modifier",
    "keyword.storage.modifier.mut",
    "keyword.storage.modifier.ref",
    "keyword.storage.type",
    "keyword.type.elm",
    "keyword.type.gren",
    "keyword.variable",
    "local",
    "local.function.elm",
    "local.function.gren",
    "markup",
    "markup.bold",
    "markup.heading.4.marker",
    "markup.heading.5.marker",
    "markup.heading.marker",
    "meta.import.elm",
    "meta.import.gren",
    "method",
    "method.call",
    "namespace",
    "namespace.roc-special.builtin",
    "operator.binary",
    "operator.repeat",
    "operator.roc-special.in-typedef",
    "operator.unary",
    "output-var",
    "param",
    "parameter",
    "parameter.builtin",
    "parameter.exception",
    "parameter.reference",
    "preproc",
    "property.definition",
    "punctuation",
    "punctuation.bracket.roc-special.in-typedef",
    "punctuation.delimiter.roc-special.in-typedef",
    "punctuation.section.braces",
    "punctuation.separator.comma.elm",
    "punctuation.separator.comma.gren",
    "regexp",
    "repeat",
    "source.glsl",
    "special",
    "special.roc-special.exposed",
    "special.roc-special.package",
    "special.roc-special.provided",
    "storage.type.elm",
    "storage.type.gren",
    "storageclass",
    "string.code",
    "string.elm",
    "string.gren",
    "string.regex",
    "string.special.key",
    "string.special.regex",
    "string.special.uri",
    "structure",
    "symbol",
    "tag.error",
    "text",
    "text.danger",
    "text.emphasis",
    "text.literal",
    "text.note",
    "text.reference",
    "text.strong",
    "text.title",
    "text.todo",
    "text.underline",
    "text.uri",
    "text.warning",
    "title",
    "type.deprecated",
    "type.enum",
    "type.enum.variant",
    "type.parameter",
    "type.qualifier",
    "type.roc-special.inferred",
    "union.elm",
    "union.gren",
    "var.reference",
    "variable.array",
    "variable.global",
    "variable.hash",
    "variable.local",
    "variable.other.member",
    "variable.other.member.roc-special.in-typedef",
    "variable.reference",
    "variable.scalar",
    "warning",
    "whole_identifier",
]
