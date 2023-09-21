[
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  "+"
  "-"
  "*"
] @operator

[
  "byte"
  "word"
  "dword"
  "qword"
  "ptr"
  "rel"
] @keyword

(string) @string

(int) @number

(line_comment) @comment

(instruction
  kind: (_) @function.call
)

(meta
  kind: (_) @function.builtin
)

(reg) @variable.builtin

(label
  (ident) @label
)
