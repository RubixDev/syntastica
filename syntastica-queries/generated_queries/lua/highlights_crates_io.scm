(ERROR) @error

(string) @string

(number) @number

(hash_bang_line) @preproc

(
  (comment) @comment.documentation
  (#match? @comment.documentation "^[-][-]([ \\t\\n\\v\\f\\r]?)@")
)

(
  (comment) @comment.documentation
  (#match? @comment.documentation "^[-][-][-]")
)

(comment) @comment

(function_call
  (identifier) @function.builtin
  (#match? @function.builtin "^(assert|collectgarbage|dofile|error|getfenv|getmetatable|ipairs|load|loadfile|loadstring|module|next|pairs|pcall|print|rawequal|rawget|rawlen|rawset|require|select|setfenv|setmetatable|tonumber|tostring|type|unpack|xpcall|__add|__band|__bnot|__bor|__bxor|__call|__concat|__div|__eq|__gc|__idiv|__index|__le|__len|__lt|__metatable|__mod|__mul|__name|__newindex|__pairs|__pow|__shl|__shr|__sub|__tostring|__unm)$")
)

(function_call
  name: [
    (identifier) @function.call
    (dot_index_expression
      field: (identifier) @function.call
    )
    (method_index_expression
      method: (identifier) @method.call
    )
  ]
)

(table_constructor
  (field
    name: (identifier) @function
    value: (function_definition)
  )
)

(assignment_statement
  (variable_list
    .
    name: [
      (identifier) @function
      (dot_index_expression
        field: (identifier) @function
      )
    ]
  )
  (expression_list
    .
    value: (function_definition)
  )
)

(function_declaration
  name: (method_index_expression
    method: (identifier) @method
  )
)

(function_declaration
  name: [
    (identifier) @function
    (dot_index_expression
      field: (identifier) @function
    )
  ]
)

(parameters
  (identifier) @parameter
)

(table_constructor
  [
    "{"
    "}"
  ] @constructor
)

(dot_index_expression
  field: (identifier) @field
)

(field
  name: (identifier) @field
)

[
  (false)
  (true)
] @boolean

(nil) @constant.builtin

(vararg_expression) @constant

(
  (identifier) @constant
  (#match? @constant "^[A-Z][A-Z_0-9]*$")
)

(goto_statement
  (identifier) @label
)

(label_statement
  (identifier) @label
)

(variable_list
  attribute: (attribute
    (
      [
        "<"
        ">"
      ] @punctuation.bracket
      (identifier) @attribute
    )
  )
)

(
  (identifier) @keyword.coroutine
  (#eq? @keyword.coroutine "coroutine")
)

(
  (identifier) @namespace.builtin
  (#match? @namespace.builtin "^(_G|debug|io|jit|math|os|package|string|table|utf8)$")
)

(
  (identifier) @variable.builtin
  (#eq? @variable.builtin "self")
)

(
  (identifier) @constant.builtin
  (#eq? @constant.builtin "_VERSION")
)

(identifier) @variable

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ";"
  ":"
  "::"
  ","
  "."
] @punctuation.delimiter

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  "#"
  "=="
  "~="
  "<="
  ">="
  "<"
  ">"
  "="
  "&"
  "~"
  "|"
  "<<"
  ">>"
  "//"
  ".."
] @operator

[
  "and"
  "not"
  "or"
] @keyword.operator

(function_definition
  [
    "function"
    "end"
  ] @keyword.function
)

(function_declaration
  [
    "function"
    "end"
  ] @keyword.function
)

(for_statement
  [
    "for"
    "do"
    "end"
  ] @repeat
)

(else_statement
  [
    "else"
    "end"
  ] @conditional
)

(elseif_statement
  [
    "elseif"
    "then"
    "end"
  ] @conditional
)

(if_statement
  [
    "if"
    "elseif"
    "else"
    "then"
    "end"
  ] @conditional
)

(repeat_statement
  [
    "repeat"
    "until"
  ] @repeat
)

(while_statement
  [
    "while"
    "do"
    "end"
  ] @repeat
)

(do_statement
  [
    "do"
    "end"
  ] @keyword
)

(break_statement) @keyword

[
  "goto"
  "in"
  "local"
] @keyword

"return" @keyword.return
