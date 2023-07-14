(macro_invocation
  macro: (identifier) @_ident @exception
  "!" @exception
  (#match? @_ident "assert")
)

(macro_invocation
  macro: (identifier) @_ident @exception
  "!" @exception
  (#eq? @_ident "panic")
)

(empty_type
  "!" @type.builtin
)

(macro_invocation
  "!" @function.macro
)

(inner_attribute_item
  [
    "!"
    "#"
  ] @punctuation.special
)

(attribute_item
  "#" @punctuation.special
)

[
  ","
  "."
  ":"
  "::"
  ";"
] @punctuation.delimiter

(for_lifetimes
  [
    "<"
    ">"
  ] @punctuation.bracket
)

(bracketed_type
  [
    "<"
    ">"
  ] @punctuation.bracket
)

(type_parameters
  [
    "<"
    ">"
  ] @punctuation.bracket
)

(type_arguments
  [
    "<"
    ">"
  ] @punctuation.bracket
)

(closure_parameters
  "|" @punctuation.bracket
)

[
  "("
  ")"
  "["
  "]"
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
  ".."
  "..="
  "/"
  "/="
  "<"
  "<<"
  "<<="
  "<="
  "="
  "=="
  "=>"
  ">"
  ">="
  ">>"
  ">>="
  "?"
  "@"
  "^"
  "^="
  "|"
  "|="
  "||"
] @operator

(for_expression
  "for" @repeat
)

("for"
  @keyword
)

[
  "break"
  "continue"
  "in"
  "loop"
  "while"
] @repeat

[
  "else"
  "if"
] @conditional

(visibility_modifier
  [
    (crate)
    (super)
    (self)
  ] @namespace
)

(scoped_identifier
  [
    (crate)
    (super)
    (self)
  ] @namespace
)

(scoped_use_list
  (self) @namespace
)

(use_list
  (self) @namespace
)

(qualified_type
  "as" @keyword.operator
)

(type_cast_expression
  "as" @keyword.operator
)

[
  "return"
  "yield"
] @keyword.return

("fn"
  @keyword.function
)

(lifetime
  [
    "'"
    (identifier)
  ] @storageclass.lifetime
)

[
  "const"
  "static"
] @storageclass

[
  "ref"
  (mutable_specifier)
] @type.qualifier

[
  "async"
  "await"
] @keyword.coroutine

[
  "default"
  "dyn"
  "enum"
  "extern"
  "impl"
  "let"
  "match"
  "move"
  "pub"
  "struct"
  "trait"
  "type"
  "union"
  "unsafe"
  "where"
] @keyword

(use_as_clause
  "as" @include
)

[
  "use"
  "mod"
] @include

(char_literal) @character

(escape_sequence) @string.escape

[
  (raw_string_literal)
  (string_literal)
] @string

(float_literal) @float

(integer_literal) @number

(boolean_literal) @boolean

(
  (block_comment) @comment.documentation
  (#match? @comment.documentation "^\\/[\\*][!]")
)

(
  (block_comment) @comment.documentation
  (#match? @comment.documentation "^\\/[\\*][\\*][^\\*][\\s\\S]*[\\*]\\/$")
)

(
  (line_comment) @comment.documentation
  (#match? @comment.documentation "^\\/\\/!")
)

(
  (line_comment) @comment.documentation
  (#match? @comment.documentation "^\\/\\/\\/$")
)

(
  (line_comment) @comment.documentation
  (#match? @comment.documentation "^\\/\\/\\/[^\\/]")
)

[
  (line_comment)
  (block_comment)
] @comment @spell

(macro_invocation
  macro: (scoped_identifier
    (identifier) @function.macro
    .
  )
)

(macro_invocation
  macro: (identifier) @function.macro
)

(attribute
  (scoped_identifier
    (identifier) @function.macro
    .
  )
)

(attribute_item
  (attribute
    (identifier) @function.macro
  )
)

(macro_definition
  "macro_rules!" @function.macro
)

(metavariable) @function.macro

("$"
  @function.macro
)

(
  (identifier) @constant.builtin
  (#match? @constant.builtin "^(Some|None|Ok|Err)$")
)

(
  (match_arm
    pattern: (match_pattern
      (scoped_identifier
        name: (identifier) @constant
      )
    )
  )
  (#match? @constant "^[A-Z]")
)

(
  (match_arm
    pattern: (match_pattern
      (identifier) @constant
    )
  )
  (#match? @constant "^[A-Z]")
)

(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @constant
  )
  (#match? @constant "^[A-Z]")
)

(use_as_clause
  alias: (identifier) @type
  (#match? @type "^[A-Z]")
)

(use_list
  (identifier) @type
  (#match? @type "^[A-Z]")
)

(use_list
  (scoped_identifier
    (identifier) @namespace
    .
    (_)
  )
)

(scoped_use_list
  path: (scoped_identifier
    (identifier) @namespace
  )
)

(scoped_use_list
  path: (identifier) @namespace
)

[
  (crate)
  (super)
] @namespace

(
  (scoped_identifier
    name: (identifier) @constant
  )
  (#match? @constant "^[A-Z][A-Z[0-9]_]*$")
)

(
  (scoped_identifier
    name: (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(
  (scoped_identifier
    path: (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(scoped_type_identifier
  (scoped_identifier
    name: (identifier) @namespace
  )
)

(scoped_type_identifier
  path: (identifier) @type
  (#match? @type "^[A-Z]")
)

(scoped_type_identifier
  path: (identifier) @namespace
)

(scoped_identifier
  (scoped_identifier
    name: (identifier) @namespace
  )
)

(scoped_identifier
  path: (identifier) @namespace
)

(enum_variant
  name: (identifier) @constant
)

(
  (field_identifier) @constant
  (#match? @constant "^[A-Z]")
)

(generic_function
  function: (field_expression
    field: (field_identifier) @function.call
  )
)

(generic_function
  function: (scoped_identifier
    name: (identifier) @function.call
  )
)

(generic_function
  function: (identifier) @function.call
)

(call_expression
  function: (field_expression
    field: (field_identifier) @function.call
  )
)

(call_expression
  function: (scoped_identifier
    (identifier) @function.call
    .
  )
)

(call_expression
  function: (identifier) @function.call
)

(closure_parameters
  (_) @parameter
)

(parameter
  (identifier) @parameter
)

(function_signature_item
  (identifier) @function
)

(function_item
  (identifier) @function
)

(loop_label
  [
    "'"
    (identifier)
  ] @label
)

(self) @variable.builtin

(mod_item
  name: (identifier) @namespace
)

(shorthand_field_initializer
  (identifier) @field
)

(field_identifier) @field

(primitive_type) @type.builtin

(type_identifier) @type

(
  (identifier) @constant
  (#match? @constant "^[A-Z][A-Z[0-9]_]*$")
)

(const_item
  name: (identifier) @constant
)

(
  (identifier) @type
  (#match? @type "^[A-Z]")
)

(identifier) @variable
