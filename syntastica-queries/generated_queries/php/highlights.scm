(ERROR) @error

[
  "="
  "."
  "-"
  "*"
  "/"
  "+"
  "%"
  "**"
  "~"
  "|"
  "^"
  "&"
  "<<"
  ">>"
  "->"
  "?->"
  "=>"
  "<"
  "<="
  ">="
  ">"
  "<>"
  "=="
  "!="
  "==="
  "!=="
  "!"
  "&&"
  "||"
  ".="
  "-="
  "+="
  "*="
  "/="
  "%="
  "**="
  "&="
  "|="
  "^="
  "<<="
  ">>="
  "??="
  "--"
  "++"
  "@"
  "::"
] @operator

[
  (php_tag)
  "?>"
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "#["
] @punctuation.bracket

[
  ","
  ";"
  ":"
  "\\"
] @punctuation.delimiter

[
  "include_once"
  "include"
  "require_once"
  "require"
  "use"
] @include

[
  "catch"
  "finally"
  "throw"
  "try"
] @exception

[
  "continue"
  "do"
  "endfor"
  "endforeach"
  "endwhile"
  "for"
  "foreach"
  "while"
] @repeat

[
  "case"
  "else"
  "elseif"
  "endif"
  "endswitch"
  "if"
  "switch"
  "match"
  "??"
] @conditional

[
  "return"
  "yield"
] @keyword.return

[
  "abstract"
  "const"
  "final"
  "private"
  "protected"
  "public"
  "readonly"
  "static"
] @type.qualifier

[
  "break"
  "class"
  "clone"
  "declare"
  "default"
  "echo"
  "enddeclare"
  "enum"
  "extends"
  "global"
  "goto"
  "implements"
  "insteadof"
  "interface"
  "namespace"
  "new"
  "trait"
  "unset"
] @keyword

[
  "fn"
  "function"
] @keyword.function

[
  "and"
  "as"
  "instanceof"
  "or"
  "xor"
] @keyword.operator

(named_label_statement) @label

(comment) @comment

(float) @float

(integer) @number

(null) @constant.builtin

(boolean) @boolean

(escape_sequence) @string.escape

[
  (string)
  (encapsed_string)
  (heredoc_body)
  (nowdoc_body)
  (shell_command_expression)
] @string

(declare_directive
  [
    "strict_types"
    "ticks"
    "encoding"
  ] @parameter
)

(conditional_expression) @conditional

(attribute_list) @attribute

(namespace_name_as_prefix
  (namespace_name
    (name) @namespace
  )
)

(namespace_definition
  name: (namespace_name
    (name) @namespace
  )
)

(
  (variable_name) @variable.builtin
  (#eq? @variable.builtin "$this")
)

(relative_scope) @variable.builtin

(member_access_expression
  name: (name) @property
)

(member_access_expression
  name: (variable_name
    (name)
  ) @property
)

(property_element
  (variable_name) @property
)

(argument
  (name) @parameter
)

[
  (simple_parameter)
  (variadic_parameter)
] @parameter

(object_creation_expression
  [
    (name) @constructor
    (qualified_name
      (name) @constructor
    )
  ]
)

(method_declaration
  name: (name) @constructor
  (#eq? @constructor "__construct")
)

(nullsafe_member_call_expression
  name: (name) @method
)

(function_definition
  name: (name) @function
)

(member_call_expression
  name: (name) @method.call
)

(scoped_call_expression
  name: (name) @function.call
)

(function_call_expression
  (name) @function.call
)

(function_call_expression
  function: (qualified_name
    (name) @function.call
  )
)

(method_declaration
  name: (name) @method
)

(list_literal
  "list" @function.builtin
)

(array_creation_expression
  "array" @function.builtin
)

(binary_expression
  operator:
  "instanceof"
  right: [
    (name) @type
    (qualified_name
      (name) @type
    )
  ]
)

(use_declaration
  (name) @type
)

(trait_declaration
  name: (name) @type
)

(class_constant_access_expression
  .
  [
    (name) @type
    (qualified_name
      (name) @type
    )
  ]
  (name) @constant
)

(scoped_call_expression
  scope: [
    (name) @type
    (qualified_name
      (name) @type
    )
  ]
)

(class_interface_clause
  [
    (name) @type
    (qualified_name
      (name) @type
    )
  ]
)

(namespace_aliasing_clause
  (name) @type.definition
)

(namespace_use_clause
  [
    (name) @type
    (qualified_name
      (name) @type
    )
  ]
)

(interface_declaration
  name: (name) @type
)

(enum_declaration
  name: (name) @type
)

(base_clause
  [
    (name) @type
    (qualified_name
      (name) @type
    )
  ]
)

(class_declaration
  name: (name) @type
)

(named_type
  [
    (name) @type
    (qualified_name
      (name) @type
    )
  ]
)

[
  (primitive_type)
  (cast_type)
] @type.builtin

(const_declaration
  (const_element
    (name) @constant
  )
)

(
  (name) @constant.builtin
  (#match? @constant.builtin "^__[A-Z][A-Z[0-9]_]+__$")
)

(
  (name) @constant
  (#match? @constant "^_?[A-Z][A-Z[0-9]_]*$")
)

(variable_name) @variable
