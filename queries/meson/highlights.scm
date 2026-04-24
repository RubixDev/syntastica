;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-meson/c84f3540624b81fc44067030afce2ff78d6ede05/queries/highlights.scm
(comment) @comment

(number) @number

(bool) @boolean

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

[
  "="
  "=="
  "and"
  "+"
  "!="
  "+="
  "not"
] @operator

[
  "if"
  "elif"
  "else"
  "endif"
] @conditional

[
  "foreach"
  "endforeach"
  (keyword_break)
  (keyword_continue)
] @repeat

;;; format
(string) @string

["@"] @keyword

(expression_statement
  object: (identifier) @variable
)

(normal_command
  command: (identifier) @function
)

(pair
  key: (identifier) @property
)
