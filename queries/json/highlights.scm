;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/json/highlights.scm
;; Licensed under the Apache License 2.0
[
  (true)
  (false)
] @boolean

(null) @constant.builtin

(number) @number

(pair
  key: (string) @property
)

(pair
  value: (string) @string
)

(array
  (string) @string
)

[
  ","
  ":"
] @punctuation.delimiter

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(escape_sequence) @string.escape
