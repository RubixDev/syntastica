(literal_suffix) @operator

(template_argument_list
  [
    "<"
    ">"
  ] @punctuation.bracket
)

"::" @punctuation.delimiter

"<=>" @operator

[
  "new"
  "delete"
  "xor"
  "bitand"
  "bitor"
  "compl"
  "not"
  "xor_eq"
  "and_eq"
  "or_eq"
  "not_eq"
  "and"
  "or"
] @keyword.operator

[
  "public"
  "private"
  "protected"
  "virtual"
  "final"
] @type.qualifier

[
  "co_yield"
  "co_return"
] @keyword.coroutine.return

["co_await"] @keyword.coroutine

[
  "class"
  "decltype"
  "explicit"
  "friend"
  "namespace"
  "override"
  "template"
  "typename"
  "using"
  "concept"
  "requires"
] @keyword

[
  "try"
  "catch"
  "noexcept"
  "throw"
] @exception

(raw_string_literal) @string

(false) @boolean

(true) @boolean

(nullptr) @constant.builtin

(this) @variable.builtin

(
  (field_initializer
    (field_identifier) @constructor
    (argument_list)
  )
  (#match? @constructor "^[A-Z]")
)

(
  (call_expression
    function: (field_expression
      field: (field_identifier) @constructor
    )
  )
  (#match? @constructor "^[A-Z]")
)

(
  (call_expression
    function: (qualified_identifier
      name: (identifier) @constructor
    )
  )
  (#match? @constructor "^[A-Z]")
)

(
  (call_expression
    function: (identifier) @constructor
  )
  (#match? @constructor "^[A-Z]")
)

(
  (function_declarator
    (qualified_identifier
      (identifier) @constructor
    )
  )
  (#match? @constructor "^[A-Z]")
)

(call_expression
  (field_expression
    (field_identifier) @method.call
  )
)

(function_declarator
  (template_method
    (field_identifier) @method
  )
)

(
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (qualified_identifier
          (template_function
            (identifier) @function.call
          )
        )
      )
    )
  ) @_parent
  (#has-ancestor? @_parent call_expression)
)

(call_expression
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (template_function
          (identifier) @function.call
        )
      )
    )
  )
)

(call_expression
  (qualified_identifier
    (qualified_identifier
      (template_function
        (identifier) @function.call
      )
    )
  )
)

(call_expression
  (qualified_identifier
    (template_function
      (identifier) @function.call
    )
  )
)

(call_expression
  (template_function
    (identifier) @function.call
  )
)

(
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (qualified_identifier
          (identifier) @function.call
        )
      )
    )
  ) @_parent
  (#has-ancestor? @_parent call_expression)
)

(call_expression
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (identifier) @function.call
      )
    )
  )
)

(call_expression
  (qualified_identifier
    (qualified_identifier
      (identifier) @function.call
    )
  )
)

(call_expression
  (qualified_identifier
    (identifier) @function.call
  )
)

"static_assert" @function.builtin

"operator" @function

(operator_name) @function

(function_declarator
  (template_function
    (identifier) @function
  )
)

(
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (qualified_identifier
          (identifier) @function
        )
      )
    )
  ) @_parent
  (#has-ancestor? @_parent function_declarator)
)

(function_declarator
  (qualified_identifier
    (qualified_identifier
      (qualified_identifier
        (identifier) @function
      )
    )
  )
)

(function_declarator
  (qualified_identifier
    (qualified_identifier
      (identifier) @function
    )
  )
)

(function_declarator
  (qualified_identifier
    (identifier) @function
  )
)

(destructor_name
  (identifier) @method
)

(using_declaration
  .
  "using"
  .
  "namespace"
  .
  [
    (qualified_identifier)
    (identifier)
  ] @namespace
)

(case_statement
  value: (qualified_identifier
    (identifier) @constant
  )
)

(
  (namespace_identifier) @type
  (#match? @type "^[A-Z]")
)

(namespace_identifier) @namespace

(auto) @type.builtin

(alias_declaration
  name: (type_identifier) @type.definition
)

(concept_definition
  name: (identifier) @type.definition
)

(function_declarator
  declarator: (field_identifier) @method
)

(field_initializer
  (field_identifier) @property
)

(field_declaration
  (field_identifier) @field
)

(
  (
    (field_expression
      (field_identifier) @method
    )
  ) @_parent
  (#has-parent? @_parent template_method function_declarator)
)

(optional_parameter_declaration
  declarator: (_) @parameter
)

(variadic_parameter_declaration
  declarator: (variadic_declarator
    (_) @parameter
  )
)

(parameter_declaration
  declarator: (reference_declarator) @parameter
)

(
  (identifier) @field
  (#match? @field "(^_|^m_|_$)")
)

(ERROR) @error

[
  "__attribute__"
  "__cdecl"
  "__clrcall"
  "__stdcall"
  "__fastcall"
  "__thiscall"
  "__vectorcall"
  "_unaligned"
  "__unaligned"
  "__declspec"
  (attribute_declaration)
] @attribute

(preproc_params
  (identifier) @parameter
)

(parameter_declaration
  declarator: (pointer_declarator) @parameter
)

(parameter_declaration
  declarator: (identifier) @parameter
)

(
  (comment) @comment.documentation
  (#match? @comment.documentation "^\\/[\\*][\\*][^\\*][\\s\\S]*[\\*]\\/$")
)

(comment) @comment @spell

(preproc_function_def
  name: (identifier) @function.macro
)

(function_declarator
  declarator: (identifier) @function
)

(call_expression
  function: (field_expression
    field: (field_identifier) @function.call
  )
)

(call_expression
  function: (identifier) @function.call
)

(preproc_call
  directive: (preproc_directive) @_u
  argument: (_) @constant
  (#eq? @_u "#undef")
)

(preproc_def
  name: (_) @constant
)

(
  (identifier) @constant.builtin
  (#match? @constant.builtin "^(stderr|stdin|stdout)$")
)

(case_statement
  value: (identifier) @constant
)

(enumerator
  name: (identifier) @constant
)

(
  (identifier) @constant
  (#match? @constant "^[A-Z][A-Z0-9_]+$")
)

(primitive_type) @type.builtin

(type_definition
  declarator: (type_identifier) @type.definition
)

(linkage_specification
  "extern" @storageclass
)

(type_qualifier) @type.qualifier

(storage_class_specifier) @storageclass

[
  (type_identifier)
  (sized_type_specifier)
  (type_descriptor)
] @type

(statement_identifier) @label

(
  (
    (field_identifier) @property
  )
  (#has-ancestor? @property field_declaration)
  (#not-has-ancestor? @property function_declarator)
)

(field_designator) @property

(
  (
    (field_expression
      (field_identifier) @property
    )
  ) @_parent
  (#not-has-parent? @_parent template_method function_declarator call_expression)
)

[
  (preproc_arg)
  (preproc_defined)
] @function.macro

(char_literal) @character

(number_literal) @number

(null) @constant.builtin

(escape_sequence) @string.escape

(system_lib_string) @string

(string_literal) @string

(conditional_expression
  [
    "?"
    ":"
  ] @conditional.ternary
)

[
  (true)
  (false)
] @boolean

(comma_expression
  [","] @operator
)

[
  "="
  "-"
  "*"
  "/"
  "+"
  "%"
  "~"
  "|"
  "&"
  "^"
  "<<"
  ">>"
  "->"
  "."
  "<"
  "<="
  ">="
  ">"
  "=="
  "!="
  "!"
  "&&"
  "||"
  "-="
  "+="
  "*="
  "/="
  "%="
  "|="
  "&="
  "^="
  ">>="
  "<<="
  "--"
  "++"
] @operator

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

"..." @punctuation.special

[
  ";"
  ":"
  ","
] @punctuation.delimiter

"#include" @include

"#define" @define

[
  "#if"
  "#ifdef"
  "#ifndef"
  "#else"
  "#elif"
  "#endif"
  (preproc_directive)
] @preproc

[
  "if"
  "else"
  "case"
  "switch"
] @conditional

[
  "while"
  "for"
  "do"
  "continue"
  "break"
] @repeat

"return" @keyword.return

"sizeof" @keyword.operator

[
  "default"
  "enum"
  "struct"
  "typedef"
  "union"
  "goto"
] @keyword

(identifier) @variable
