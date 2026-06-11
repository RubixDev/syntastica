;; Forked from https://github.com/rush-rs/tree-sitter-asm/blob/main/queries/asm/highlights.scm
;; Licensed under the MIT license
; General
(label
  [
    (ident)
    (word)
  ] @label
)

(reg) @variable.builtin

(meta
  kind: (_) @function.builtin
)

(instruction
  kind: (_) @function.builtin
)

(const
  name: (word) @constant
)

; Comments
[
  (line_comment)
  (block_comment)
] @comment

; Literals
(int) @number

(float) @number.float

(string) @string

; Keywords
[
  "byte"
  "word"
  "dword"
  "qword"
  "ptr"
  "rel"
  "label"
  "const"
] @keyword

; Operators & Punctuation
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "|"
  "^"
  "&"
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
