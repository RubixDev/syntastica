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
  name: (identifier) @property
)

(token
  (identifier) @variable.parameter
)

[
  (brace_word)
  (quote_word)
] @string

(
  (field
    name: (identifier) @_url
    value: (value
      (token
        (brace_word) @string.special.url
      )
    )
  )
  (#any-of? @_url "url" "doi")
)

[
  (key_brace)
  (key_paren)
] @markup.link.label

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
