;; Forked from https://github.com/rush-rs/tree-sitter-asm/blob/main/queries/asm/highlights.scm
;; Licensed under the MIT license
; General
(label
  (ident) @label
)

(reg) @variable.builtin

(meta
  kind: (_) @function.builtin
)

(instruction
  kind: (_) @function.call
)

; Comments
(line_comment) @comment

; Literals
(int) @number

(string) @string

; Keywords
[
  "byte"
  "word"
  "dword"
  "qword"
  "ptr"
  "rel"
] @keyword

; Operators & Punctuation
[
  "+"
  "-"
  "*"
] @operator

[
  "("
  ")"
  "["
  "]"
] @punctuation.bracket

[
  ","
  ":"
] @punctuation.delimiter
