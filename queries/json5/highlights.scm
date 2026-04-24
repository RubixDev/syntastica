;; Forked from https://raw.githubusercontent.com/Joakker/tree-sitter-json5/aa630ef48903ab99e406a8acd2e2933077cc34e1/queries/highlights.scm
(string) @string

(identifier) @constant

(number) @constant.numeric

(null) @constant.builtin

[
  (true)
  (false)
] @constant.builtin.boolean

(comment) @comment
