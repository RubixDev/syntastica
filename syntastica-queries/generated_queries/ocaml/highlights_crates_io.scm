(ERROR) @error

[
  (comment)
  (line_number_directive)
  (directive)
  (shebang)
] @comment

(attribute_id) @property

[
  "*"
  "#"
  "::"
  "<-"
] @operator

(value_definition
  [
    (let_operator)
    (let_and_operator)
  ] @keyword
)

(match_expression
  (match_operator) @keyword
)

[
  (prefix_operator)
  (sign_operator)
  (pow_operator)
  (mult_operator)
  (add_operator)
  (concat_operator)
  (rel_operator)
  (and_operator)
  (or_operator)
  (assign_operator)
  (hash_operator)
  (indexing_operator)
  (let_operator)
  (and_operator)
  (match_operator)
] @operator

[
  ","
  "."
  ";"
  ":"
  "="
  "|"
  "~"
  "?"
  "+"
  "-"
  "!"
  ">"
  "&"
  "->"
  ";;"
  ":>"
  "+="
  ":="
  ".."
] @punctuation.delimiter

(object_type
  [
    "<"
    ">"
  ] @punctuation.bracket
)

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "[|"
  "|]"
  "[<"
  "[>"
] @punctuation.bracket

("%"
  @punctuation.special
)

(quoted_item_extension
  [
    "{%%"
    "}"
  ] @punctuation.special
)

(quoted_extension
  [
    "{%"
    "}"
  ] @punctuation.special
)

(item_extension
  [
    "[%%"
    "]"
  ] @punctuation.special
)

(extension
  [
    "[%"
    "]"
  ] @punctuation.special
)

(floating_attribute
  [
    "[@@@"
    "]"
  ] @punctuation.special
)

(item_attribute
  [
    "[@@"
    "]"
  ] @punctuation.special
)

(attribute
  [
    "[@"
    "]"
  ] @punctuation.special
)

[
  "for"
  "to"
  "downto"
  "while"
  "do"
  "done"
] @repeat

[
  "include"
  "open"
] @include

[
  "exception"
  "try"
] @exception

[
  "if"
  "then"
  "else"
] @conditional

[
  "fun"
  "function"
  "functor"
] @keyword.function

[
  "lazy"
  "mutable"
  "nonrec"
  "rec"
  "private"
  "virtual"
] @type.qualifier

[
  "and"
  "as"
  "assert"
  "begin"
  "class"
  "constraint"
  "end"
  "external"
  "in"
  "inherit"
  "initializer"
  "let"
  "match"
  "method"
  "module"
  "new"
  "object"
  "of"
  "sig"
  "struct"
  "type"
  "val"
  "when"
  "with"
] @keyword

[
  (conversion_specification)
  (pretty_printing_indication)
] @string.special

(escape_sequence) @string.escape

(quoted_string
  "{" @string
  "}" @string
) @string

(string) @string

(character) @character

[
  (number)
  (signed_number)
] @number

(boolean) @boolean

(
  (unit) @constant.builtin
  (#set! "priority" 105)
)

[
  (label_name)
  (field_name)
  (instance_variable_name)
] @property

(
  (value_name) @function.builtin
  (#match? @function.builtin "^(raise|raise_notrace|failwith|invalid_arg)$")
)

(application_expression
  function: (value_path
    (value_name) @function
  )
)

(infix_expression
  operator: (rel_operator) @_operator
  right: (value_path
    (value_name) @function
  )
  (#eq? @_operator "|>")
)

(infix_expression
  left: (value_path
    (value_name) @function
  )
  operator: (concat_operator) @_operator
  (#eq? @_operator "@@")
)

(method_name) @method

(external
  (value_name) @function
)

(value_specification
  (value_name) @function
)

(let_binding
  pattern: (value_name) @function
  body: [
    (fun_expression)
    (function_expression)
  ]
)

(let_binding
  pattern: (value_name) @function
  (parameter)
)

(value_pattern) @parameter

[
  (value_name)
  (type_variable)
] @variable

[
  (constructor_name)
  (tag)
] @constructor

[
  (class_name)
  (class_type_name)
  (type_constructor)
] @type

(
  (type_constructor) @type.builtin
  (#match? @type.builtin "^(int|char|bytes|string|float|bool|unit|exn|array|list|option|int32|int64|nativeint|format6|lazy_t)$")
)

[
  (module_name)
  (module_type_name)
] @namespace
