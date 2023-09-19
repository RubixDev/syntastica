(ERROR) @error

[
  "do"
  "while"
  "continue"
  "for"
] @repeat

[
  "try"
  "throw"
  "catch"
  "finally"
  (break_statement)
] @exception

[
  "if"
  "else"
  "switch"
  "default"
] @conditional

(
  (identifier) @variable.builtin
  (#match? @variable.builtin "^(abstract|as|covariant|deferred|dynamic|export|external|factory|Function|get|implements|import|interface|library|operator|mixin|part|set|static|typedef)$")
)

[
  (const_builtin)
  (final_builtin)
  "abstract"
  "covariant"
  "dynamic"
  "external"
  "static"
] @type.qualifier

[
  "async"
  "async*"
  "sync*"
  "await"
  "yield"
] @keyword.coroutine

[
  "deferred"
  "factory"
  "get"
  "implements"
  "interface"
  "library"
  "operator"
  "mixin"
  "part"
  "set"
  "typedef"
] @keyword

["return"] @keyword.return

[
  (case_builtin)
  "late"
  "required"
  "extension"
  "on"
  "class"
  "enum"
  "extends"
  "in"
  "is"
  "new"
  "super"
  "with"
] @keyword

[
  "import"
  "library"
  "export"
  "as"
  "show"
  "hide"
] @include

(documentation_comment) @comment.documentation @spell

(comment) @comment @spell

(null_literal) @constant.builtin

(false) @boolean

(true) @boolean

(string_literal) @string

(symbol_literal) @symbol

[
  (hex_integer_literal)
  (decimal_integer_literal)
  (decimal_floating_point_literal)
] @number

(named_argument
  (label
    (identifier) @parameter
  )
)

(formal_parameter
  name: (identifier) @parameter
)

(this) @variable.builtin

(assignment_expression
  left: (assignable_expression) @variable
)

(conditional_assignable_selector
  (identifier) @property
)

(unconditional_assignable_selector
  (identifier) @property
)

"Function" @type

(
  (identifier) @type
  (#match? @type "^_?[[A-Z]][\\s\\S]*[[a-z]]")
)

(inferred_type) @keyword

(type_alias
  (type_identifier) @type.definition
)

(type_identifier) @type

(
  (scoped_identifier
    scope: (identifier) @type
    name: (identifier) @type
  )
  (#match? @type "^[[A-Z][a-z]]")
)

(void_type) @type

(enum_constant
  name: (identifier) @type
)

(enum_declaration
  name: (identifier) @type
)

(setter_signature
  name: (identifier) @method
)

(getter_signature
  (identifier) @method
)

(function_signature
  name: (identifier) @method
)

(scoped_identifier
  scope: (identifier) @type
)

(constructor_signature
  name: (identifier) @type
)

(class_definition
  name: (identifier) @type
)

[
  ";"
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
  "@"
  "=>"
  ".."
  "??"
  "=="
  "?"
  ":"
  "&&"
  "%"
  "<"
  ">"
  "="
  ">="
  "<="
  "||"
  (multiplicative_operator)
  (increment_operator)
  (is_operator)
  (prefix_operator)
  (equality_operator)
  (additive_operator)
] @operator

(escape_sequence) @string.escape

(template_substitution
  "$" @punctuation.special
  (identifier_dollar_escaped) @variable
) @none

(template_substitution
  "$" @punctuation.special
  "{" @punctuation.special
  "}" @punctuation.special
) @none

(marker_annotation
  name: (identifier) @attribute
)

(annotation
  name: (identifier) @attribute
)

(
  (
    (identifier) @function
    (#match? @function "^_?[[a-z]]")
  )
  .
  (selector
    .
    (argument_part)
  )
) @function

(function_expression_body
  (identifier) @function
)

(super) @function

(dotted_identifier_list) @string
