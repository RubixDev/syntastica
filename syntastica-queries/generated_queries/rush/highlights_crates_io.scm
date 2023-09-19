[
  ","
  ":"
  ";"
] @punctuation.delimiter

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

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
  "break"
  "continue"
  "loop"
  "while"
  "for"
] @repeat

[
  "else"
  "if"
] @conditional

"as" @keyword.operator

"return" @keyword.return

"fn" @keyword.function

[
  "let"
  "mut"
] @keyword

(char) @character

(float) @float

(int) @number

(bool) @boolean

[
  (line_comment)
  (block_comment)
] @comment @spell

(call_expr
  func: (ident) @function.builtin
  (#match? @function.builtin "^(exit)$")
)

(call_expr
  func: (ident) @function.call
)

(parameter
  name: (ident) @parameter
)

(function_definition
  name: (ident) @function
)

(type
  (ident) @type.builtin
)

(ident) @variable
