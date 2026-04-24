;; Forked from https://raw.githubusercontent.com/ValdezFOmar/tree-sitter-xresources/a81f1ea4478d03c223ef7eace535a36220bddd8c/queries/xresources/highlights.scm
[
  (comment)
  (preprocessor_comment)
] @comment

"#include" @keyword.import

[
  "#define"
  "#undef"
] @keyword.directive.define

[
  "#if"
  "#ifdef"
  "#ifndef"
  "#elif"
  "#elifdef"
  "#elifndef"
  "#else"
  "#endif"
  (directive)
] @keyword.directive

(define_directive
  name: (identifier) @constant.macro
)

(define_function_directive
  name: (identifier) @function.macro
)

(parameters
  (identifier) @variable.parameter
)

"..." @variable.parameter.builtin

(undef_directive
  name: (identifier) @constant.macro
)

(ifdef_directive
  condition: (identifier) @constant.macro
)

(elifdef_directive
  condition: (identifier) @constant.macro
)

(expansion) @markup.raw

(component) @variable.member

(
  (component) @type
  (#match? @type "^[A-Z]")
)

(components
  (component) @property
  .
)

[
  (string)
  (resource_value)
] @string

(escape_sequence) @string.escape

[
  "*"
  (any_component)
] @character.special

[
  "."
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
] @punctuation.bracket
