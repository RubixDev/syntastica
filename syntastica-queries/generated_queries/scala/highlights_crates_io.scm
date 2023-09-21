(using_directive_value) @string

(using_directive_key) @parameter

(
  (identifier) @function.builtin
  (#match? @function.builtin "^super$")
)

(
  (identifier) @variable.builtin
  (#match? @variable.builtin "^this$")
)

(
  (identifier) @type
  (#match? @type "^[A-Z]")
)

(operator_identifier) @operator

(case_block
  (case_clause
    ("case") @conditional
  )
)

(
  (block_comment) @comment.documentation
  (#match? @comment.documentation "^\\/[\\*][\\*][^\\*][\\s\\S]*[\\*]\\/$")
)

[
  (comment)
  (block_comment)
] @comment

"return" @keyword.return

[
  "try"
  "catch"
  "throw"
] @exception

[
  "import"
  "export"
] @include

[
  "=>"
  "<-"
  "@"
] @operator

"def" @keyword.function

[
  "do"
  "for"
  "while"
  "yield"
] @repeat

[
  "."
  ","
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "else"
  "if"
  "match"
  "then"
] @conditional

"new" @keyword.operator

(annotation) @attribute

(wildcard) @parameter

(null_literal) @constant.builtin

(inline_modifier) @storageclass

[
  "abstract"
  "final"
  "lazy"
  "sealed"
  "private"
  "protected"
] @type.qualifier

[
  "case"
  "class"
  "enum"
  "extends"
  "derives"
  "finally"
  "object"
  "override"
  "package"
  "trait"
  "type"
  "val"
  "var"
  "with"
  "given"
  "using"
  "end"
  "implicit"
  "extension"
  "with"
] @keyword

(open_modifier) @type.qualifier

(transparent_modifier) @type.qualifier

(infix_modifier) @keyword

(opaque_modifier) @type.qualifier

(interpolation
  "$" @punctuation.special
)

[
  (symbol_literal)
  (string)
  (character_literal)
  (interpolated_string_expression)
] @string

(floating_point_literal) @float

(integer_literal) @number

(boolean_literal) @boolean

(infix_type
  operator: (operator_identifier) @operator
)

(infix_type
  operator: (operator_identifier) @operator
)

(infix_expression
  operator: (operator_identifier) @operator
)

(infix_expression
  operator: (identifier) @operator
)

(field_expression
  value: (identifier) @type
  (#match? @type "^[A-Z]")
)

(field_expression
  field: (identifier) @property
)

(binding
  name: (identifier) @parameter
)

(parameter
  name: (identifier) @parameter
)

(function_definition
  name: (identifier) @function
)

(interpolated_string_expression
  interpolator: (identifier) @function.call
)

(generic_function
  function: (identifier) @function.call
)

(
  (call_expression
    function: (identifier) @constructor
  )
  (#match? @constructor "^[A-Z]")
)

(call_expression
  function: (field_expression
    field: (identifier) @method.call
  )
)

(call_expression
  function: (operator_identifier) @function.call
)

(call_expression
  function: (identifier) @function.call
)

(
  (namespace_selectors
    (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(
  (stable_identifier
    (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(
  (export_declaration
    path: (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(stable_identifier
  (identifier) @namespace
)

(export_declaration
  path: (identifier) @namespace
)

(
  (stable_identifier
    (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(
  (import_declaration
    path: (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(stable_identifier
  (identifier) @namespace
)

(import_declaration
  path: (identifier) @namespace
)

(function_definition
  name: (identifier) @method
)

(function_declaration
  name: (identifier) @method
)

(var_declaration
  name: (identifier) @variable
)

(val_declaration
  name: (identifier) @variable
)

(var_definition
  pattern: (identifier) @variable
)

(val_definition
  pattern: (identifier) @variable
)

(type_identifier) @type

(type_definition
  name: (type_identifier) @type.definition
)

(interpolation
  (block) @none
)

(interpolation
  (identifier) @none
)

(self_type
  (identifier) @parameter
)

(class_parameter
  name: (identifier) @parameter
)

(simple_enum_case
  name: (identifier) @type
)

(full_enum_case
  name: (identifier) @type
)

(trait_definition
  name: (identifier) @type
)

(object_definition
  name: (identifier) @type
)

(enum_definition
  name: (identifier) @type
)

(class_definition
  name: (identifier) @type
)
