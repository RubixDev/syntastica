;; Forked from https://github.com/rush-rs/tree-sitter-rush/blob/main/queries/rush/highlights.scm
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
  name: (ident) @variable.parameter
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
] @comment

(bool) @boolean

(int) @number

(float) @number.float

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
] @keyword.conditional

[
  "break"
  "continue"
  "loop"
  "while"
  "for"
] @keyword.repeat

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
