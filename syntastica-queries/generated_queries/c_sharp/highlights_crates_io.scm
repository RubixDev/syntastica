[
  "return"
  "yield"
] @keyword.return

(query_expression
  (_
    [
      "from"
      "orderby"
      "select"
      "group"
      "by"
      "ascending"
      "descending"
      "equals"
      "let"
    ] @keyword
  )
)

(parameter_modifier) @operator

[
  "abstract"
  "private"
  "protected"
  "internal"
  "public"
  "partial"
  "sealed"
  "virtual"
] @type.qualifier

[
  "const"
  "extern"
  "readonly"
  "static"
  "volatile"
  "required"
] @storageclass

[
  "async"
  "await"
] @keyword.coroutine

[
  "lock"
  "params"
  "operator"
  "default"
  "implicit"
  "explicit"
  "override"
  "class"
  "delegate"
  "enum"
  "interface"
  "namespace"
  "struct"
  "get"
  "set"
  "init"
  "where"
  "record"
  "event"
  "add"
  "remove"
  "checked"
  "unchecked"
  "fixed"
] @keyword

[
  "with"
  "new"
  "typeof"
  "sizeof"
  "is"
  "and"
  "or"
  "not"
  "stackalloc"
  "in"
  "out"
  "ref"
] @keyword.operator

(alias_qualified_name
  (identifier
    "global"
  ) @include
)

[
  "using"
  "as"
] @include

[
  (this_expression)
  (base_expression)
] @variable.builtin

(type_argument_list
  [
    "<"
    ">"
  ] @punctuation.bracket
)

[
  "["
  "]"
  "{"
  "}"
  "("
  ")"
] @punctuation.bracket

(conditional_expression
  [
    "?"
    ":"
  ] @conditional.ternary
)

[
  ";"
  "."
  ","
  ":"
] @punctuation.delimiter

[
  "+"
  "?"
  ":"
  "++"
  "-"
  "--"
  "&"
  "&&"
  "|"
  "||"
  "!"
  "!="
  "=="
  "*"
  "/"
  "%"
  "<"
  "<="
  ">"
  ">="
  "="
  "-="
  "+="
  "*="
  "/="
  "%="
  "^"
  "^="
  "&="
  "|="
  "~"
  ">>"
  "<<"
  "<<="
  ">>="
  "=>"
] @operator

[
  "try"
  "catch"
  "throw"
  "finally"
] @exception

[
  "while"
  "for"
  "do"
  "continue"
  "goto"
  "foreach"
] @repeat

(elif_directive
  (identifier) @constant
)

(if_directive
  (identifier) @constant
)

[
  "if"
  "else"
  "switch"
  "break"
  "case"
  (if_directive)
  (elif_directive)
  (else_directive)
  (endif_directive)
] @conditional

[
  (nullable_directive)
  (region_directive)
  (endregion_directive)
] @constant.macro

(pragma_directive
  (preproc_string_literal) @string
) @constant.macro

(pragma_directive
  (identifier) @constant
) @constant.macro

(line_directive
  (preproc_integer_literal) @constant
  (preproc_string_literal)
  ? @string
)

(line_directive) @constant.macro

(undef_directive
  (identifier) @constant
) @constant.macro

(define_directive
  (identifier) @constant
) @constant.macro

(error_directive) @exception

(warning_directive) @text.warning

(name_colon
  (identifier) @parameter
)

(type_of_expression
  (identifier) @type
)

(as_expression
  right: (identifier) @type
)

(tuple_expression
  (argument
    (declaration_expression
      type: (identifier) @type
    )
  )
)

(tuple_element
  type: (identifier) @type
)

(for_each_statement
  type: (identifier) @type
)

(attribute
  name: (identifier) @attribute
)

(type_parameter_constraints_clause
  target: (identifier) @type
)

(type_parameter_list
  (type_parameter) @type
)

(type_argument_list
  (identifier) @type
)

(base_list
  (identifier) @type
)

(invocation_expression
  (member_access_expression
    (generic_name
      (identifier) @method
    )
  )
)

(invocation_expression
  function: (generic_name
    .
    (identifier) @method.call
  )
)

(_
  type: (generic_name
    (identifier) @type
  )
)

(property_declaration
  (generic_name
    (identifier) @type
  )
)

(object_creation_expression
  (generic_name
    (identifier) @type
  )
)

(type_constraint
  (generic_name
    (identifier) @type
  )
)

(base_list
  (generic_name
    (identifier) @type
  )
)

(type_argument_list
  (generic_name
    (identifier) @type
  )
)

(type_of_expression
  (generic_name
    (identifier) @type
  )
)

(object_creation_expression
  (identifier) @type
)

(variable_declaration
  (identifier) @type
)

(constructor_initializer
  [
    "base" @constructor
  ]
)

(constructor_declaration
  name: (identifier) @constructor
)

(enum_declaration
  name: (identifier) @type
)

(record_declaration
  name: (identifier) @type
)

(class_declaration
  name: (identifier) @type
)

(interface_declaration
  name: (identifier) @type
)

(catch_declaration
  type: (identifier) @type
)

(nullable_type
  (identifier) @type
)

(property_declaration
  type: (identifier) @type
)

(property_declaration
  name: (identifier) @property
)

(using_directive
  (name_equals
    (identifier) @type.definition
  )
)

(using_directive
  (identifier) @type
)

(
  (comment) @comment.documentation
  (#match? @comment.documentation "^\\/\\/\\/$")
)

(
  (comment) @comment.documentation
  (#match? @comment.documentation "^\\/\\/\\/[^\\/]")
)

(
  (comment) @comment.documentation
  (#match? @comment.documentation "^\\/[\\*][\\*][^\\*][\\s\\S]*[\\*]\\/$")
)

(comment) @comment @spell

(implicit_type) @keyword

[
  (predefined_type)
] @type.builtin

(boolean_literal) @boolean

[
  (string_literal)
  (verbatim_string_literal)
  (interpolated_string_expression)
] @string

(character_literal) @character

(null_literal) @constant.builtin

(real_literal) @float

(integer_literal) @number

(parameter_list
  (parameter
    type: (identifier) @type
  )
)

(parameter_list
  (parameter
    name: (identifier) @parameter
  )
)

(initializer_expression
  (assignment_expression
    left: (identifier) @field
  )
)

(field_declaration
  (variable_declaration
    (variable_declarator
      (identifier) @field
    )
  )
)

(invocation_expression
  (identifier) @method.call
)

(qualified_name
  (identifier) @type
)

(namespace_declaration
  name: [
    (qualified_name)
    (identifier)
  ] @namespace
)

(invocation_expression
  function: (conditional_access_expression
    (member_binding_expression
      name: (identifier) @method.call
    )
  )
)

(invocation_expression
  (member_access_expression
    name: (identifier) @method.call
  )
)

(interpolation) @none

(local_function_statement
  type: (identifier) @type
)

(method_declaration
  type: (identifier) @type
)

(local_function_statement
  name: (identifier) @method
)

(method_declaration
  name: (identifier) @method
)

(
  (identifier) @keyword
  (#eq? @keyword "value")
  (#has-ancestor? @keyword accessor_declaration)
)

(identifier) @variable
