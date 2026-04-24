;; Forked from https://raw.githubusercontent.com/neovim/tree-sitter-vimdoc/f061895a0eff1d5b90e4fb60d21d87be3267031a/queries/vimdoc/highlights.scm
(h1
  (delimiter) @markup.heading.1
  (heading) @markup.heading.1
)

(h2
  (delimiter) @markup.heading.2
  (heading) @markup.heading.2
)

(h3
  (heading) @markup.heading.3
)

(column_heading
  (heading) @markup.heading.4
)

(column_heading
  (delimiter) @markup.heading.4.marker
  (#set! conceal "")
)

(tag
  "*" @markup.heading.5.marker
  (#set! conceal "")
  text: (_) @label
)

(taglink
  "|" @markup.link
  (#set! conceal "")
  text: (_) @markup.link
)

(optionlink
  text: (_) @markup.link
)

(codespan
  "`" @markup.raw
  (#set! conceal "")
  text: (_) @markup.raw
)

(
  (codeblock) @markup.raw.block
  (#set! "priority" 90)
)

(codeblock
  [
    ">"
    (language)
  ] @markup.raw.block
  (#set! conceal "")
)

(block
  "<" @markup.raw.block
  (#set! conceal "")
)

(argument) @variable.parameter

(keycode) @string.special

(url) @string.special.url

(modeline) @keyword.directive

(
  (note) @comment.hint
  (#any-of? @comment.hint "Note:" "NOTE:" "Notes:")
)

(
  (note) @comment.warning
  (#any-of? @comment.warning "Warning:" "WARNING:")
)

(
  (note) @comment.error
  (#any-of? @comment.error "Deprecated:" "DEPRECATED:")
)
