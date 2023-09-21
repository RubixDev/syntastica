;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/json/highlights.scm
;; Licensed under the Apache License 2.0
[
  (true)
  (false)
] @boolean

(null) @constant.builtin

(number) @number

(pair
  key: (string) @label
)

(pair
  value: (string) @string
)

(array
  (string) @string
)

(ERROR) @error

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

(
  ("\""
    @conceal
  )
  (#set! conceal "")
)

(escape_sequence) @string.escape

(
  (escape_sequence) @conceal
  (#eq? @conceal "\\\"")
  (#set! conceal "\"")
)
