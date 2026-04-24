;; Forked from https://raw.githubusercontent.com/interdependence/tree-sitter-htmldjango/3a643167ad9afac5d61e092f08ff5b054576fadf/queries/highlights.scm
[
  (unpaired_comment)
  (paired_comment)
] @comment

[
  "{{"
  "}}"
  "{%"
  "%}"
  (end_paired_statement)
] @tag

[
  (tag_name)
] @function

(variable_name) @variable

(filter_name) @method

(filter_argument) @parameter

(keyword) @keyword

(operator) @operator

(keyword_operator) @keyword.operator

(number) @number

(boolean) @boolean

(string) @string
