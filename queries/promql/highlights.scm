;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/promql/highlights.scm
;; Licensed under the Apache License 2.0
; highlights.scm
[
  "*"
  "/"
  "%"
  "+"
  "-"
  ">"
  ">="
  "<"
  "<="
  "="
  "=~"
  "!="
  "!~"
] @operator

[
  "{"
  "}"
  "["
  "]"
  "("
  ")"
] @punctuation.bracket

(float_literal) @number.float

(string_literal) @string

(metric_name) @type

(range_selection) @type

(subquery_range_selection) @type

(label_name) @variable.member

(
  (label_name)
  [
    "=~"
    "!~"
  ]
  (label_value) @string.regexp
)

(
  (label_name)
  [
    "="
    "!="
  ]
  (label_value) @string
)

(function_name) @function.call

(comment) @comment @spell
