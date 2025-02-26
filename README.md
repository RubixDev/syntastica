# `syntastica`

<!-- TODO: msrv -->

Modern and easy syntax highlighting using tree-sitter

> **Note**
>
> If viewing this file on [GitHub](https://github.com/RubixDev/syntastica) or
> [crates.io](https://crates.io/crates/syntastica), some links might not be
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
itself. However, the project does provide four different parser collections with
different advantages and drawbacks each. Three of them depend on
[`syntastica-queries`](#syntastica-queries) for the tree-sitter queries. Choose
one, and add it as a dependency next to `syntastica` itself.

The odd one out here is
[`syntastica-parsers-dynamic`](https://crates.io/crates/syntastica-parsers-dynamic),
which unlike the others doesn't actually include any parsers but instead
provides an interface to load them during runtime.

The other three parser collections all provide the same public API and have
features for all supported languages, as well as the three feature groups
`some`, `most`, and `all`. Take a look at the respective crate documentation for
more information.

If you want to additionally use languages that are not in any of these parser
collections or combine multiple sets, have a look at the
[`Union`](language_set::Union) type or the
[custom languages example](https://github.com/RubixDev/syntastica/blob/main/examples/custom_languages.rs).

- [`syntastica-parsers`](https://crates.io/crates/syntastica-parsers) is
  probably the easiest to start with. It uses parsers from
  [crates.io](https://crates.io). This has the main benefit of being well
  integrated in the cargo ecosystem. However, many tree-sitter parsers do not
  get published to crates.io, and those that are, are usually very outdated.
  Thus, this collection is relatively limited.
- <a name="syntastica-parsers-git" href="https://crates.io/crates/syntastica-parsers-git"><code>syntastica-parsers-git</code></a>
  is probably the best choice overall. It contains all supported languages and
  is the only choice when targeting WebAssembly. It pulls pinned revisions of
  parser git repositories in the build script and links to the C and C++ parser
  sources. As such, it does not depend on the upstream parsers to have
  up-to-date Rust bindings. However, this way of fetching the parsers requires
  the `git` command to be accessible and internet access during compilation,
  which may not be desirable. Additionally, compilation can take very long
  unless you manually specify a cache directory that can be reused between
  builds. See the crate's docs for more information on that.
- [`syntastica-parsers-gitdep`](https://github.com/RubixDev/syntastica/tree/main/syntastica-parsers-gitdep)
  is a mix of both of the above. It uses cargo git dependencies to fetch the
  parser repositories and depends on a remote Rust binding (which is why not
  _all_ parsers are included). The main disadvantages are that this collection
  cannot be published to crates.io, because it depends on crates that are not on
  crates.io (namely the parsers). This means, to use it you must also depend on
  it using a git dependency, which in turn forbids your crate to be published on
  crates.io. Unlike [`syntastica-parsers-git`](#syntastica-parsers-git) however,
  the parsers only need to be fetched once by cargo, and subsequent builds will
  be much faster.
- [`syntastica-parsers-dynamic`](https://crates.io/crates/syntastica-parsers-dynamic)
  doesn't include any parsers by itself but instead provides a
  [`LanguageSet`](language_set::LanguageSet) implementation that can find and
  load parsers at runtime. This allows for behavior similar to what the
  tree-sitter CLI does, and opens up more possibilities for end-users, but also
  places more responsibilities on them, as the appropriate queries also need to
  be provided manually.

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
  [`LanguageSet`](language_set::LanguageSet). Unlike the previous crates in this
  list however, you may actually want to depend on this crate yourself, if you
  _only_ need the queries.

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
- [`syntastica-query-preprocessor`](https://crates.io/crates/syntastica-query-preprocessor)
  is a pre-processor for tree-sitter queries which allows usage of
  `; inherits <lang>` comments, conditional skipping of nodes with comments,
  usage of additional predicates like `lua-match?`, `contains?` and `any-of?`,
  Neovim's old injections syntax, and order reversing for priority flipping. The
  crate can be used to use queries designed for Neovim with the official
  [tree-sitter Rust bindings](https://crates.io/crates/tree-sitter) with minimal
  manual changes. Despite having `syntastica` in the name, the crate can be used
  externally and does not depend on any of the other `syntastica-` crates. In
  `syntastica` it is used in the
  [`codegen queries` xtask](https://github.com/RubixDev/syntastica/blob/main/xtask/src/codegen/queries.rs),
  because many of the queries are forked from
  [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter), and to
  adjust the queries for older parser versions from
  [crates.io](https://crates.io).

## WebAssembly support

`syntastica` can be used with WebAssembly, although the current support is a bit
lacking. There are currently two primary ways to use `syntastica` in a
WebAssembly context.

### 1. Using the `tree-sitter-c2rust` runtime

In order to make `syntastica` compile to `wasm32-unknown-unknown` targets,
feature flags can be used to use the
[c2rust transpilation of tree-sitter](https://crates.io/crates/tree-sitter-c2rust)
instead of the
[official C implementation](https://crates.io/crates/tree-sitter). This is only
supported by the `syntastica-parsers-git` parser collection, and only parsers
that don't use an external C++ scanner are available.

To use this approach, simply set `default-features = false` and enable the
`runtime-c2rust` feature for _all_ `syntastica` dependencies. An example using
this approach for use of `syntastica` in a Dioxus project can be found
[here](https://github.com/RubixDev/syntastica/tree/main/examples/wasm/dioxus).

### 2. Using Emscripten / the `syntastica-js` package

`syntastica` can also be compiled to `wasm32-unknown-emscripten` which has much
better support for C and C++ interop. But annoyingly, basically the entire Rust
Wasm ecosystem is built around the `wasm32-unknown-unknown` target (e.g.,
`wasm-pack` and `wasm-bindgen` can only be used with `wasm32-unknown-unknown`),
which makes it very cumbersome to use Emscripten for Rust. In the attempt to
make using `syntastica` on the web a bit easier, the
[`syntastica-js` crate](https://github.com/RubixDev/syntastica/tree/main/syntastica-js)
and accompanying
[`syntastica` NPM package](https://www.npmjs.com/package/syntastica) provide a
JavaScript/TypeScript wrapper around an Emscripten build of `syntastica`.

There are three examples using `syntastica-js`:

- [Usage from TypeScript in the browser with Vite and Svelte](https://github.com/RubixDev/syntastica/tree/main/examples/wasm/vite)
- [Usage from JavaScript in NodeJS for console applications](https://github.com/RubixDev/syntastica/tree/main/examples/wasm/node)
- [Usage from Rust in the browser using wasm-bindgen](https://github.com/RubixDev/syntastica/tree/main/examples/wasm/wasm-pack-with-npm-pkg)

> **Note**
>
> The `syntastica` NPM package is currently not being updated and uses an old
> version of `syntastica`, because the current implementation always includes
> all parsers in one big binary, which would be over 60 MB big with all
> currently supported parsers. The eventual plan is to find a way to split the
> package into multiple binaries that can be fetched from a server on-demand,
> and to provide multiple NPM packages for manual selection of the parsers.

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
- [Detect the language based on a file type](#example-detect-language-from-file-type)
- [Specify a custom theme](#example-custom-theme)

### Example: highlight once

This example shows the easiest and quickest way to use `syntastica`. See the
section about [use cases](#use-cases) for when it is appropriate to use
`syntastica` this way.

```rust
use syntastica::renderer::TerminalRenderer;
use syntastica_parsers::{Lang, LanguageSetImpl};

let output = syntastica::highlight(
    // the code to highlight
    r#"fn main() { println!("42"); }"#,
    // the input's language
    Lang::Rust,
    // use `syntastica-parsers` language set
    &LanguageSetImpl::new(),
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
use syntastica_parsers::{Lang, LanguageSetImpl};

// process the input once, but store the raw highlight information
let highlights = Processor::process_once(
    // the code to highlight
    r#"fn main() { println!("42"); }"#,
    // the input's language
    Lang::Rust,
    // use `syntastica-parsers` language set
    &LanguageSetImpl::new(),
)
.unwrap_or_else(|err| panic!("highlighting failed: {err}"));

// render the highlights to the terminal using the
// gruvbox dark theme on a dark gray background
println!("{}", syntastica::render(
    &highlights,
    &mut TerminalRenderer::new(Some(Color::new(40, 40, 40))),
    syntastica_themes::gruvbox::dark(),
));

// render the same input to HTML using the onelight theme
let html = syntastica::render(
    &highlights,
    &mut HtmlRenderer::new(),
    syntastica_themes::one::light(),
);
// you could for example write that to a file called `index.html`:
// std::fs::write("index.html", html).unwrap();
```

### Example: highlight multiple different inputs

This example shows how a [`Processor`] can be reused if multiple different
inputs should be highlighted.

```rust
use syntastica::{Processor, style::Color, renderer::*};
use syntastica_parsers::{Lang, LanguageSetImpl};

// create a language set and a `Processor`
let language_set = LanguageSetImpl::new();
let mut processor = Processor::new(&language_set);
// Note: `language_set` has to be stored in a variable, because the processor
// is bound to the lifetime of the reference passed to `new`

// process some input
let highlights_rust = processor.process(
    // the code to highlight
    r#"fn main() { println!("42"); }"#,
    // the input's language
    Lang::Rust,
)
.unwrap_or_else(|err| panic!("highlighting failed: {err}"));

// process some other input in another language
let highlights_js = processor.process(r"console.log('42')", Lang::Javascript)
    .unwrap_or_else(|err| panic!("highlighting failed: {err}"));

// render the rust code to the terminal using the
// gruvbox dark theme on a dark gray background
println!("{}", syntastica::render(
    &highlights_rust,
    &mut TerminalRenderer::new(Some(Color::new(40, 40, 40))),
    syntastica_themes::gruvbox::dark(),
));

// render the same rust code to HTML using the onelight theme
let html = syntastica::render(
    &highlights_rust,
    &mut HtmlRenderer::new(),
    syntastica_themes::one::light(),
);
// you could for example write that to a file called `index.html`:
// std::fs::write("index.html", html).unwrap();

// now render the javascript code to the terminal using the
// onedark theme and no background color
println!("{}", syntastica::render(
    &highlights_js,
    &mut TerminalRenderer::new(None),
    syntastica_themes::one::dark(),
));
```

### Example: detect language from file type

This is an alteration of the [first example](#example-highlight-once) showing
how to detect the language to use based on a file type. See that first example
for explanations of the rest of the code.

`syntastica` uses [`tft`](https://crates.io/crates/tft) for file types which
provides automatic detection.

```rust
use syntastica::{renderer::TerminalRenderer, language_set::{LanguageSet, SupportedLanguage}};
use syntastica_parsers::{Lang, LanguageSetImpl};

// detect the file type given a file's path and content.
// this requires a dependency on `tft`
let ft = tft::detect("main.rs", "");

let language_set = LanguageSetImpl::new();
let output = syntastica::highlight(
    r#"fn main() { println!("42"); }"#,
    // the `SupportedLanguage` trait provides a `for_file_type` function
    // which returns an `Option<Lang>`
    // make sure to have the trait in scope
    Lang::for_file_type(ft, &()).unwrap(),
    &language_set,
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
use syntastica_parsers::{Lang, LanguageSetImpl};

let theme = theme! {
    // specify colors using hex literals
    "purple": "#c678dd",
    "blue": "#61afef",
    "green": "#98c379",

    // link to other keys using a `$` sign
    "keyword": "$purple",
    "function": "$blue",

    // specify more styling options in curly braces
    // (note that currently this order is required by the macro)
    "string": {
        color: None,
        bg: None,
        underline: false,
        strikethrough: false,
        italic: true,
        bold: false,
        link: "green",
    },
};

let output = syntastica::highlight(
    r#"fn main() { println!("42"); }"#,
    Lang::Rust,
    &LanguageSetImpl::new(),
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

The entire idea of this project started out as a way to use tree-sitter code
highlighting in a LaTeX project. While working with
[@MikMuellerDev](https://github.com/MikMuellerDev) on
[our paper](https://github.com/rush-rs/paper) on [rush](https://rush-lang.de/) I
created a CLI app called [`lirstings`](https://github.com/rush-rs/lirstings).
The initial sketch simply called out to the `tree-sitter-cli` and converted the
output HTML to LaTeX code. However, not long after that I already implemented
some of the logic myself and made a
[first public commit](https://github.com/rush-rs/lirstings/commit/d2fc87213e8e2d629033f2eba99b2d019883fd43).
This version of `lirstings` (called `ts2tex` at the time) already laid out some
groundwork like
[query pre-processing](https://rubixdev.github.io/syntastica/syntastica_query_preprocessor/)
and [theming](https://rubixdev.github.io/syntastica/syntastica/theme/) that is
still present in `syntastica` today. Towards the end of our project we wanted to
use the same highlighting on our [rush playground](https://play.rush-lang.de/),
which would require `lirstings` to become more general and support WebAssembly.
Work on that started in the
[generalize branch](https://github.com/rush-rs/lirstings/tree/generalize) just
enough to suffice for our needs at the time.

After the entire rush project was done and after taking a break for a while, I
started `syntastica` with the intent to be a library from the ground up, and a
possible replacement for [`syntect`](https://crates.io/crates/syntect). The main
difference from `lirstings` at the start was the parser collection(s), providing
a rigid set of parsers and queries for users. Over time `syntastica` then grew
to the big project it is today.
