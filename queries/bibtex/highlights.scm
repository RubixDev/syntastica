;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/bibtex/highlights.scm
;; Licensed under the Apache License 2.0
[
  (string_type)
  (preamble_type)
  (entry_type)
] @keyword

[
  (junk)
  (comment)
] @comment

[
  "="
  "#"
] @operator

(command) @function.builtin

(number) @number

(field
  name: (identifier) @variable.member
)

(token
  (identifier) @variable.parameter
)

[
  (brace_word)
  (quote_word)
] @string

[
  (key_brace)
  (key_paren)
] @string.special.symbol

(string
  name: (identifier) @constant
)

[
  "{"
  "}"
  "("
  ")"
] @punctuation.bracket

"," @punctuation.delimiter
