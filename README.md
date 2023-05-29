# `syntastica`

Modern and easy syntax highlighting using tree-sitter

> Note: If viewing this file on [GitHub](https://github.com/RubixDev/syntastica)
> or [crates.io](https://crates.io/crates/syntastica), some links might not be
> working. Go to the
> [custom docs page](https://rubixdev.github.io/syntastica/syntastica/) or the
> [docs.rs page](https://docs.rs/syntastica/) instead, which additionally
> include the [Features](#features) section.

## Overview

To use `syntastica`, you probably want to depend on three crates:

1. The main `syntastica` crate for all the logic.
2. A parser collection to provide language support (see
   [parser collections](#parser-collections))
3. The theme collection for some default themes (see
   [theme collection](#theme-collection))

So for example:

```toml
syntastica = "<version>"
syntastica-parsers = { version = "<version>", features = ["some"] }
syntastica-themes = "<version>"
```

### Use cases

`syntastica` has three main ways of highlighting code, for three different use
cases:

1. Highlight _one_ input _exactly once_: see [`highlight`] and
   [this example](#example-highlight-once)
2. Highlight _one_ input _multiple times_ (e.g. with different themes or
   renderers): see [`Processor::process_once`], [`render`], and
   [this example](#example-highlight-the-same-input-multiple-times)
3. Highlight _multiple_ different inputs _any_ number of times: see
   [`Processor`], [`render`], and
   [this example](#example-highlight-multiple-different-inputs)

## Subprojects

Besides the main `syntastica` crate, many other crates for different purposes
were developed and are included in the repository. This section aims to provide
a good overview.

### Parser collections

The main `syntastica` crate provides no tree-sitter parsers and queries by
itself. However, the project does provide three different parser collections
with different advantages and drawbacks each. All three collections depend on
[`syntastica-queries`](#syntastica-queries) for the tree-sitter queries. Choose
one, and add it as a dependency next to `syntastica` itself.

All three parser collections also provide the same public API and support three
features, one of which has to be enabled: `some`, `most`, and `all`. Take a look
at the respective crate documentation for more information.

- [`syntastica-parsers`](https://crates.io/crates/syntastica-parsers) is
  probably the easiest to start with. It uses parsers from
  [crates.io](https://crates.io). This has the main benefit of being well
  integrated in the cargo ecosystem. However, many tree-sitter parsers do not
  get published to crates.io, and those that are, are usually very outdated.
  Thus, this collection is relatively limited.
- <a name="syntastica-parsers-git" href="https://crates.io/crates/syntastica-parsers-git"><code>syntastica-parsers-git</code></a>
  is probably the best choice overall. It contains all supported languages, and
  [when WebAssembly compilation will be supported](#todo), this will be the
  collection to use. It pulls pinned revisions of parser git repositories in the
  build script and links to the C and C++ parser sources. As such, it does not
  depend on the upstream parsers to have up-to-date Rust bindings. However, this
  way of fetching the parsers requires the `git` command to be accessible and
  internet access during compilation, which may not be desirable. Additionally,
  compilation can take very long, because there is no clean way to cache the
  fetched repositories between builds.
- [`syntastica-parsers-gitdep`](https://github.com/RubixDev/syntastica/tree/main/syntastica-parsers-gitdep)
  is a mix of both of the above. It uses cargo git dependencies to fetch the
  parser repositories and depends on a remote Rust binding (which is why not
  _all_ parsers are included). The main disadvantages are that this collection
  cannot be published to crates.io, because it depends on crates that are not on
  crates.io (namely the parsers). This means, to use it you must also depend on
  it using a git dependency, which in turn forbids your crate to be published on
  crates.io. Unlike [`syntastica-parsers-git`](#syntastica-parsers-git) however,
  the parsers only need to be fetched once by cargo, and following builds will
  be much faster.

### Theme collection

To [render highlighted code](render) to end users, a
[theme](theme::ResolvedTheme) is needed, which specifies the colors to use for
which [theme key](theme::THEME_KEYS). The `syntastica` project comes with a
separate crate containing a few default themes:
[`syntastica-themes`](https://crates.io/crates/syntastica-themes).

If you wish to create your own theme, have a look at the
[custom theme example](#example-custom-theme) and the documentation for the
[`theme!`] macro.

### Crates for internal use

The `syntastica` repository/workspace also includes some crates which are not
meant for outside use, but are instead used internally. These are listed below.

> Note: **There are no guarantees about the public API of these crates!** If,
> for any reason, you have to depend on one of them, then pin the _exact_
> version using `<crate> = "=<version>"`.

- [`syntastica-core`](https://crates.io/crates/syntastica-core) defines types,
  traits, constants, etc. which are used in multiple of the other crates. The
  main `syntastica` crate re-exports all those items transparently, so that
  external projects only need a dependency on that. The items are defined in
  `syntastica-core` however, to avoid cyclic (dev-)dependencies inside this
  workspace.
- [`syntastica-macros`](https://crates.io/crates/syntastica-macros) defines
  procedural macros for use **exclusively** inside this workspace. This crate
  allows the list of languages/parsers to be in _one_ combined `languages.toml`
  file, and the different macros are used in the different places where this
  list needs to be referenced.
- [`syntastica-highlight`](https://crates.io/crates/syntastica-highlight) is a
  fork of
  [`tree-sitter-highlight`](https://crates.io/crates/tree-sitter-highlight),
  which is adjusted and trimmed down for the use in `syntastica`. It contains
  the main highlighting logic.
- <a name="syntastica-queries" href="https://crates.io/crates/syntastica-queries"><code>syntastica-queries</code></a>
  is a collection of tree-sitter queries for all supported languages. It is
  marked as "for internal use", because all three
  [parser collections](#parser-collections) depend on this crate and expose the
  queries through their implementation of
  [`LanguageProvider`](provider::LanguageProvider). Unlike the previous crates
  in this list however, you may actually want to depend on this crate yourself,
  if you _only_ need the queries.

### General side-products

This list includes crates which were developed for `syntastica` but have no
direct association with the main project and can be used completely separately.

- [`rsexpr`](https://crates.io/crates/rsexpr) is a generic S-expression parser
  with added support for square-brackets, strings, and comments. Additionally,
  the parsed S-expressions can be pretty-printed to provide a uniform
  formatting. See
  [`dprint-plugin-sexpr`](https://github.com/RubixDev/dprint-plugin-sexpr) for
  more information on using this as a formatter. In `syntastica` this crate is
  used for parsing (and formatting) the tree-sitter queries in the
  [`queries`](https://github.com/RubixDev/syntastica/tree/main/queries)
  directory. These are processed by `cargo xtask codegen queries` and result in
  the queries inside the
  [`generated_queries`](https://github.com/RubixDev/syntastica/tree/main/syntastica-queries/generated_queries)
  directory, which are the ones that are bundled with
  [`syntastica-queries`](#syntastica-queries).
- [`lua-pattern`](https://crates.io/crates/lua-pattern) is a parser for Lua
  patterns. These are similar to regular expressions, but generally more
  limited. The crate also provides a best-effort conversion to regular
  expression strings. In `syntastica` this is used, as many of the source
  queries are forked from
  [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) which
  makes heavy use of `#lua-match?` predicates for matching with Lua patterns.
  The official tree-sitter Rust bindings do not support Lua pattern matching
  however (obviously), which is why during the processing of the queries (with
  `cargo xtask codegen queries`), all Lua patterns are replaced with regular
  expressions using this crate.

## WebAssembly support

TODO: WebAssembly support

## Examples

This section contains some basic usage examples. More specific examples can be
found in the documentation of some items such as the [`Processor`] type or the
[`render`] function. Additionally, the
[`examples`](https://github.com/RubixDev/syntastica/tree/main/examples)
directory contains a few complete examples.

This is the list of examples found here:

- [Highlight once](#example-highlight-once)
- [Highlight the same input multiple times](#example-highlight-the-same-input-multiple-times)
- [Highlight multiple different inputs](#example-highlight-multiple-different-inputs)
- [Detect the language based on a file extension](#example-detect-language-from-file-extension)
- [Specify a custom theme](#example-custom-theme)

### Example: highlight once

This example shows the easiest and quickest way to use `syntastica`. See the
section about [use cases](#use-cases) for when it is appropriate to use
`syntastica` this way.

```rust
use syntastica::renderer::TerminalRenderer;
use syntastica_parsers::LanguageProviderImpl;

let output = syntastica::highlight(
    // the code to highlight
    r#"fn main() { println!("42"); }"#,
    // the name of the input's language
    "rust",
    // use `syntastica-parsers` language provider with support for just Rust
    &LanguageProviderImpl::with_languages(&["rust"]),
    // use the TerminalRenderer with no background color
    &mut TerminalRenderer::new(None),
    // use the gruvbox dark theme from `syntastica-themes`
    syntastica_themes::gruvbox::dark(),
)
.unwrap_or_else(|err| panic!("highlighting failed: {err}"));

println!("{output}");
```

### Example: highlight the same input multiple times

This example shows how to render the same input with two different themes using
two different renderers.

```rust
use syntastica::{Processor, style::Color, renderer::*};
use syntastica_parsers::LanguageProviderImpl;

// process the input once, but store the raw highlight information
let highlights = Processor::process_once(
    // the code to highlight
    r#"fn main() { println!("42"); }"#,
    // the name of the input's language
    "rust",
    // use `syntastica-parsers` language provider with support for just Rust
    &LanguageProviderImpl::with_languages(&["rust"]),
)
.unwrap_or_else(|err| panic!("highlighting failed: {err}"));

// render the highlights to the terminal using the
// gruvbox dark theme on a dark gray background
println!("{}", syntastica::render(
    &highlights,
    &mut TerminalRenderer::new(Some(Color::new(40, 40, 40))),
    syntastica_themes::gruvbox::dark().resolve_links().unwrap(),
));

// render the same input to HTML using the onelight theme
let html = syntastica::render(
    &highlights,
    &mut HtmlRenderer::new(),
    syntastica_themes::one::light().resolve_links().unwrap(),
);
// you could for example write that to a file called `index.html`:
// std::fs::write("index.html", html).unwrap();
```

### Example: highlight multiple different inputs

This example shows how a [`Processor`] can be reused if multiple different
inputs should be highlighted.

```rust
use syntastica::{Processor, style::Color, renderer::*};
use syntastica_parsers::LanguageProviderImpl;

// create a language provider and a `Processor`
let language_provider = LanguageProviderImpl::with_languages(&["rust", "javascript"]);
let mut processor = Processor::try_from_provider(&language_provider).unwrap();
// Note: `language_provider` has to be in a variable, because the processor
// is bound to the lifetime of the reference passed to `try_from_provider`

// process some input
let highlights_rust = processor.process(
    // the code to highlight
    r#"fn main() { println!("42"); }"#,
    // the name of the input's language
    "rust",
)
.unwrap_or_else(|err| panic!("highlighting failed: {err}"));

// process some other input in another language
let highlights_js = processor.process(r"console.log('42')", "javascript")
    .unwrap_or_else(|err| panic!("highlighting failed: {err}"));

// render the rust code to the terminal using the
// gruvbox dark theme on a dark gray background
println!("{}", syntastica::render(
    &highlights_rust,
    &mut TerminalRenderer::new(Some(Color::new(40, 40, 40))),
    syntastica_themes::gruvbox::dark().resolve_links().unwrap(),
));

// render the same rust code to HTML using the onelight theme
let html = syntastica::render(
    &highlights_rust,
    &mut HtmlRenderer::new(),
    syntastica_themes::one::light().resolve_links().unwrap(),
);
// you could for example write that to a file called `index.html`:
// std::fs::write("index.html", html).unwrap();

// now render the javascript code to the terminal using the
// onedark theme and no background color
println!("{}", syntastica::render(
    &highlights_js,
    &mut TerminalRenderer::new(None),
    syntastica_themes::one::dark().resolve_links().unwrap(),
));
```

### Example: detect language from file extension

This is an alteration of the [first example](#example-highlight-once) showing
how to detect the language to use based on a file extension. See that first
example for explanations of the rest of the code.

```rust
use syntastica::{renderer::TerminalRenderer, provider::LanguageProvider};
use syntastica_parsers::LanguageProviderImpl;

let language_provider = LanguageProviderImpl::with_languages(&["rust"]);
let output = syntastica::highlight(
    r#"fn main() { println!("42"); }"#,
    // the `LanguageProvider` trait also provides a `for_extension` function
    // which returns an `Option<Cow<'static, str>>`
    // make sure to have the trait in scope
    language_provider.for_extension("rs").unwrap().as_ref(),
    &language_provider,
    &mut TerminalRenderer::new(None),
    syntastica_themes::gruvbox::dark(),
)
.unwrap_or_else(|err| panic!("highlighting failed: {err}"));

println!("{output}");
```

### Example: custom theme

This is an alteration of the [first example](#example-highlight-once) showing
how to create a simple custom theme. See that first example for explanations of
the rest of the code, and see the documentation of the [`theme!`] macro for more
information.

```rust
use syntastica::{renderer::TerminalRenderer, theme};
use syntastica_parsers::LanguageProviderImpl;

let theme = theme! {
    // specify colors using hex literals
    "purple": "#c678dd",
    "blue": "#61afef",
    "green": "#98c379",

    // link to other keys using a `$` sign
    "keyword": "$purple",
    "function": "$blue",

    // specify more styling options in curly braces
    // (note that currently this order required by the macro)
    "string": {
        color: None,
        underline: false,
        strikethrough: false,
        italic: true,
        bold: false,
        link: "green",
    },
};

let output = syntastica::highlight(
    r#"fn main() { println!("42"); }"#,
    "rust",
    &LanguageProviderImpl::with_languages(&["rust"]),
    &mut TerminalRenderer::new(None),
    theme,
)
.unwrap_or_else(|err| panic!("highlighting failed: {err}"));

println!("{output}");
```

## Versioning

All crates in this workspace whose names start with `syntastica` share the same
version. The typical semantic versioning rules are used across the public APIs
of _all_ of these, except for
[the ones listed as internal](#crates-for-internal-use). The
[other crates in this workspace](#general-side-products) have their own separate
versions.

Versions are specified as `MAJOR.MINOR.PATCH`. As long as the `MAJOR` version
specifier is still at `0`, changes to the `MINOR` version may also be breaking
changes. The `PATCH` part is only incremented if the public API stays exactly
the same.

## Inspiration

TODO: shortly explain origins (lirstings)

## TODO

- [ ] easy compilation to WebAssembly using `tree-sitter-c2rust`
