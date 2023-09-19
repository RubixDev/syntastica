(
  (line_comment) @comment.documentation
  (#match? @comment.documentation "^\\/\\/\\/$")
)

(
  (line_comment) @comment.documentation
  (#match? @comment.documentation "^\\/\\/\\/[^\\/]")
)

(
  (block_comment) @comment.documentation
  (#match? @comment.documentation "^\\/[\\*][\\*][^\\*][\\s\\S]*[\\*]\\/$")
)

[
  (line_comment)
  (block_comment)
] @comment @spell

(labeled_statement
  (identifier) @label
)

[
  "throw"
  "throws"
  "finally"
  "try"
  "catch"
] @exception

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

[
  "("
  ")"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  "{"
  "}"
] @punctuation.bracket

[
  ";"
  "."
  "..."
  ","
] @punctuation.delimiter

[
  "exports"
  "import"
  "module"
  "opens"
  "package"
  "provides"
  "requires"
  "uses"
] @include

[
  "for"
  "while"
  "do"
  "continue"
  "break"
] @repeat

(ternary_expression
  [
    "?"
    ":"
  ] @conditional.ternary
)

[
  "if"
  "else"
  "switch"
  "case"
] @conditional

["new"] @keyword.operator

[
  "return"
  "yield"
] @keyword.return

[
  "transient"
  "volatile"
] @storageclass

(modifiers
  "synchronized" @type.qualifier
)

[
  "abstract"
  "final"
  "native"
  "non-sealed"
  "open"
  "private"
  "protected"
  "public"
  "sealed"
  "static"
  "strictfp"
  "transitive"
] @type.qualifier

(synchronized_statement
  "synchronized" @keyword
)

[
  "assert"
  "class"
  "record"
  "default"
  "enum"
  "extends"
  "implements"
  "instanceof"
  "interface"
  "@interface"
  "permits"
  "to"
  "with"
] @keyword

(null_literal) @constant.builtin

[
  (true)
  (false)
] @boolean

[
  (decimal_floating_point_literal)
  (hex_floating_point_literal)
] @float

[
  (hex_integer_literal)
  (decimal_integer_literal)
  (octal_integer_literal)
  (binary_integer_literal)
] @number

(character_literal) @character

(escape_sequence) @string.escape

(string_literal) @string

(this) @variable.builtin

(
  (identifier) @constant
  (#match? @constant "^[A-Z_][A-Z[0-9]_]+$")
)

[
  (boolean_type)
  (integral_type)
  (floating_point_type)
  (void_type)
] @type.builtin

(field_access
  field: (identifier) @field
)

(field_declaration
  declarator: (variable_declarator
    name: (identifier) @field
  )
)

(
  (scoped_identifier
    scope: (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(
  (field_access
    object: (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(
  (method_reference
    .
    (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(
  (method_invocation
    object: (identifier) @type
  )
  (#match? @type "^[A-Z]")
)

(type_identifier) @type

(constructor_declaration
  name: (identifier) @type
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

(annotation_type_declaration
  name: (identifier) @type
)

(interface_declaration
  name: (identifier) @type
)

[
  "@"
  "+"
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
  "->"
  "^"
  "^="
  "&="
  "|="
  "~"
  ">>"
  ">>>"
  "<<"
  "::"
] @operator

(marker_annotation
  name: (identifier) @attribute
)

(annotation
  name: (identifier) @attribute
)

(lambda_expression
  parameters: (identifier) @parameter
)

(inferred_parameters
  (identifier) @parameter
)

(spread_parameter
  (variable_declarator
    name: (identifier) @parameter
  )
)

(catch_formal_parameter
  name: (identifier) @parameter
)

(formal_parameter
  name: (identifier) @parameter
)

(super) @function.builtin

(method_invocation
  name: (identifier) @method.call
)

(method_declaration
  name: (identifier) @method
)

(identifier) @variable
