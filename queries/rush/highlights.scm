;; Forked from https://github.com/rush-rs/tree-sitter-rush/blob/6081a422d4f5dfb0426c09582e82e7070bb749d1/queries/rush/highlights.scm
;; Licensed under the MIT license
; General Identifiers
(ident) @variable

(type
  (ident) @type.builtin
)

; Function definitions
(function_definition
  name: (ident) @function
)

(parameter
  name: (ident) @parameter
)

; Function calls
(call_expr
  func: (ident) @function.call
)

(call_expr
  func: (ident) @function.builtin
  (#any-of? @function.builtin "exit")
)

; Literals
[
  (line_comment)
  (block_comment)
] @comment @spell

(bool) @boolean

(int) @number

(float) @float

(char) @character

; Keywords
[
  "let"
  "mut"
] @keyword

"fn" @keyword.function

"return" @keyword.return

"as" @keyword.operator

[
  "else"
  "if"
] @conditional

[
  "break"
  "continue"
  "loop"
  "while"
  "for"
] @repeat

; Operators & Punctuation
[
  "!"
  "!="
  "%"
  "%="
  "&"
  "&&"
  "&="
  "*"
  "*="
  "+"
  "+="
  "-"
  "-="
  "->"
  "/"
  "/="
  "<"
  "<<"
  "<<="
  "<="
  "="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "^"
  "^="
  "|"
  "|="
  "||"
] @operator

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  ":"
  ";"
] @punctuation.delimiter
