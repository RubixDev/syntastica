;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-udev/2fcb563a4d56a6b8e8c129252325fc6335e4acbf/queries/highlights.scm
(match
  key:
  _ @keyword
)

(assignment
  key:
  _ @property
)

[
  (system_const)
  (run_type)
  (import_type)
  (attribute)
  (kernel_param)
  (seclabel)
] @attribute

(env_var) @constant

(value) @string

(pattern) @string.regexp

(fmt_sub
  .
  _ @variable.builtin
)

(var_sub
  .
  _ @variable.builtin
)

[
  "\\\""
  (c_escape)
] @string.escape

[
  (octal)
  (number)
] @number

[
  (match_op)
  (assignment_op)
] @operator

"+" @punctuation.special

[
  "{"
  "}"
] @punctuation.bracket

[
  ","
  (linebreak)
] @punctuation.delimiter

(comment) @comment
