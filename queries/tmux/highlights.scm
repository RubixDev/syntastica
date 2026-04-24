;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/tmux/highlights.scm
;; Licensed under the Apache License 2.0
(comment) @comment @spell

[
  "'"
  (str_single_quotes)
  (str_double_quotes)
] @string

(backslash_escape) @string.escape

(path) @string.special.path

(int) @number

[
  (option)
  (name)
] @variable

(command_line_option) @variable.builtin

(
  (option) @variable.builtin
  (#not-lua-match? @variable.builtin "^@")
)

[
  (if_keyword)
  (elif_keyword)
  (else_keyword)
  (endif_keyword)
] @keyword.conditional

[
  (hidden_keyword)
  (command)
] @keyword

(source_file_directive
  (command) @keyword.import
)

(attribute) @attribute

(function_name) @function.call

"=" @operator

[
  ";"
  "';'"
  ","
  ":"
] @punctuation.delimiter

[
  "#"
  "?"
] @punctuation.special

[
  "#{"
  "}"
  "#["
  "]"
  "["
  "{"
] @punctuation.bracket
