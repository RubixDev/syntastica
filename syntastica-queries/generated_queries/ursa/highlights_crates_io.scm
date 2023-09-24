(
  (identifier) @variable.builtin
  (#match? @variable.builtin "^(pi)$")
  (#is-not? local)
)

(
  (identifier) @function.builtin
  (#eq? @function.builtin "print")
  (#is-not? local)
)

(named_fn
  (identifier) @function
)

(let
  (identifier) @function
  (lambda)
)

(call
  function: (identifier) @function
)

(identifier) @variable

(property_identifier) @property

[
  (bool)
  (null)
] @constant.builtin

(comment) @comment

(string) @string

(number) @number

[
  ";"
  "."
  ","
] @punctuation.delimiter

[
  "-"
  "+"
  "*"
  "**"
  "/"
  "%"
  "<"
  "<="
  "="
  "=="
  "!="
  ">"
  ">="
] @operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "break"
  (continue)
  "else"
  "fn"
  "if"
  "let"
  "loop"
  "return"
  "use"
] @keyword
