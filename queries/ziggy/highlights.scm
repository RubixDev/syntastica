;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ziggy/highlights.scm
;; Licensed under the Apache License 2.0
[
  (true)
  (false)
] @constant.builtin

(null) @constant.builtin

[
  (integer)
  (float)
] @number

(struct_field
  key: (_) @keyword
)

(struct
  name: (_) @type
)

(tag) @function

[
  (string)
  (line_string)*
] @string

(comment) @comment

(escape_sequence) @string.escape

"," @punctuation.delimiter

[
  "["
  "]"
  "{"
  "}"
  "("
  ")"
] @punctuation.bracket

(top_comment) @comment
