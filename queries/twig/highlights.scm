;; Forked from https://raw.githubusercontent.com/gbprod/tree-sitter-twig/7195ee573ab5c3b3bb0e91b042e6f83ac1b11104/queries/highlights.scm
(comment) @comment

(filter_identifier) @function.call

(function_identifier) @function.call

(test) @function.builtin

(variable) @variable

(string) @string

(interpolated_string) @string

(operator) @operator

(number) @number

(boolean) @constant.builtin

(null) @constant.builtin

(keyword) @keyword

(attribute) @attribute

(tag) @tag

(conditional) @conditional

(repeat) @repeat

(method) @method

(parameter) @parameter

[
  "{{"
  "}}"
  "{{-"
  "-}}"
  "{{~"
  "~}}"
  "{%"
  "%}"
  "{%-"
  "-%}"
  "{%~"
  "~%}"
] @tag.delimiter

[
  ","
  "."
  "?"
  ":"
  "="
] @punctuation.delimiter

(interpolated_string
  [
    "#{"
    "}"
  ] @punctuation.delimiter
)

[
  "("
  ")"
  "["
  "]"
  "{"
] @punctuation.bracket

(hash
  ["}"] @punctuation.bracket
)
