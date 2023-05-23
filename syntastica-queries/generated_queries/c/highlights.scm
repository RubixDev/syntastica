(
  (ERROR) @error
)

(
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
)

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
  (#match? @comment.documentation "^/[*][*][^*].*[*]/$")
)

(
  (comment) @comment @spell
)

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

(
  (primitive_type) @type.builtin
)

(type_definition
  declarator: (type_identifier) @type.definition
)

(linkage_specification
  "extern" @storageclass
)

(
  (type_qualifier) @type.qualifier
)

(
  (storage_class_specifier) @storageclass
)

(
  [
    (type_identifier)
    (sized_type_specifier)
    (type_descriptor)
  ] @type
)

(
  (statement_identifier) @label
)

(
  (
    (field_identifier) @property
  )
  (#has-ancestor? @property field_declaration)
  (#not-has-ancestor? @property function_declarator)
)

(
  (field_designator) @property
)

(
  (
    (field_expression
      (field_identifier) @property
    )
  ) @_parent
  (#not-has-parent? @_parent template_method function_declarator call_expression)
)

(
  [
    (preproc_arg)
    (preproc_defined)
  ] @function.macro
)

(
  (char_literal) @character
)

(
  (number_literal) @number
)

(
  (null) @constant.builtin
)

(
  (escape_sequence) @string.escape
)

(
  (system_lib_string) @string
)

(
  (string_literal) @string
)

(conditional_expression
  [
    "?"
    ":"
  ] @conditional.ternary
)

(
  [
    (true)
    (false)
  ] @boolean
)

(comma_expression
  [","] @operator
)

(
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
)

(
  [
    "("
    ")"
    "["
    "]"
    "{"
    "}"
  ] @punctuation.bracket
)

("..."
  @punctuation.special
)

(
  [
    ";"
    ":"
    ","
  ] @punctuation.delimiter
)

("#include"
  @include
)

("#define"
  @define
)

(
  [
    "#if"
    "#ifdef"
    "#ifndef"
    "#else"
    "#elif"
    "#endif"
    (preproc_directive)
  ] @preproc
)

(
  [
    "if"
    "else"
    "case"
    "switch"
  ] @conditional
)

(
  [
    "while"
    "for"
    "do"
    "continue"
    "break"
  ] @repeat
)

("return"
  @keyword.return
)

("sizeof"
  @keyword.operator
)

(
  [
    "default"
    "enum"
    "struct"
    "typedef"
    "union"
    "goto"
  ] @keyword
)

(
  (identifier) @variable
)
