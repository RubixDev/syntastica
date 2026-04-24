;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gstlaunch/highlights.scm
;; Licensed under the Apache License 2.0
[
  "!"
  "="
] @operator

[
  ","
  "."
  ";"
  "/"
] @punctuation.delimiter

[
  "("
  ")"
] @punctuation.bracket

(property
  key: (identifier) @variable.member
)

(value) @string

(string_literal) @string

(cap
  .
  (identifier) @string
  .
  (identifier) @string
)

(simple_element
  type: (_) @type
)

(bin
  type: (_) @type
)
