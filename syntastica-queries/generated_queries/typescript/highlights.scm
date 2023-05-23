(property_signature
  name: (property_identifier) @method
  type: (type_annotation
    [
      (union_type
        (parenthesized_type
          (function_type)
        )
      )
      (function_type)
    ]
  )
)

(method_signature
  name: (_) @method
)

(ambient_declaration
  (function_signature
    name: (identifier) @function
  )
)

(arrow_function
  parameter: (identifier) @parameter
)

(required_parameter
  (array_pattern
    (identifier) @parameter
  )
)

(required_parameter
  (object_pattern
    (pair_pattern
      value: (identifier) @parameter
    )
  )
)

(required_parameter
  (object_pattern
    (shorthand_property_identifier_pattern) @parameter
  )
)

(required_parameter
  (rest_pattern
    (identifier) @parameter
  )
)

(optional_parameter
  (identifier) @parameter
)

(required_parameter
  (identifier) @parameter
)

(
  (undefined) @variable.builtin
)

(conditional_type
  [
    "?"
    ":"
  ] @conditional.ternary
)

(template_type
  [
    "${"
    "}"
  ] @punctuation.special
)

(optional_parameter
  "?" @punctuation.special
)

(property_signature
  "?" @punctuation.special
)

(method_signature
  "?" @punctuation.special
)

("?."
  @punctuation.delimiter
)

(opting_type_annotation
  "?:" @punctuation.delimiter
)

(index_signature
  ":" @punctuation.delimiter
)

(type_annotation
  ":" @punctuation.delimiter
)

(intersection_type
  "&" @punctuation.delimiter
)

(union_type
  "|" @punctuation.delimiter
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

(non_null_expression
  "!" @operator
)

(
  (template_literal_type) @string
)

(import_statement
  "type"
  (import_clause
    (named_imports
      (
        (import_specifier
          name: (identifier) @type
        )
      )
    )
  )
)

(
  (predefined_type) @type.builtin
)

(
  (type_identifier) @type
)

(
  [
    "abstract"
    "private"
    "protected"
    "public"
    "readonly"
  ] @type.qualifier
)

(as_expression
  "as" @keyword
)

(
  [
    "declare"
    "enum"
    "export"
    "implements"
    "interface"
    "keyof"
    "type"
    "namespace"
    "override"
    "satisfies"
    "module"
    "infer"
  ] @keyword
)

(switch_default
  "default" @conditional
)

(export_statement
  "default" @keyword
)

(
  [
    "throw"
    "try"
    "catch"
    "finally"
  ] @exception
)

(
  [
    "new"
    "delete"
  ] @keyword.operator
)

(
  ["function"] @keyword.function
)

(
  [
    "return"
    "yield"
  ] @keyword.return
)

(
  [
    "async"
    "await"
  ] @keyword.coroutine
)

(
  [
    "break"
    "class"
    "const"
    "debugger"
    "export"
    "extends"
    "get"
    "in"
    "instanceof"
    "let"
    "set"
    "static"
    "target"
    "typeof"
    "var"
    "with"
  ] @keyword
)

(
  [
    "for"
    "of"
    "do"
    "while"
    "continue"
  ] @repeat
)

(namespace_import
  "as" @include
)

(namespace_export
  "as" @include
)

(import_specifier
  "as" @include
)

(export_specifier
  "as" @include
)

(
  [
    "import"
    "from"
  ] @include
)

(
  [
    "if"
    "else"
    "switch"
    "case"
  ] @conditional
)

(
  (template_substitution
    [
      "${"
      "}"
    ] @punctuation.special
  ) @none
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

(unary_expression
  [
    "delete"
    "void"
    "typeof"
  ] @keyword.operator
)

(unary_expression
  [
    "!"
    "~"
    "-"
    "+"
  ] @operator
)

(ternary_expression
  [
    "?"
    ":"
  ] @conditional.ternary
)

(binary_expression
  "/" @operator
)

(
  [
    "--"
    "-"
    "-="
    "&&"
    "+"
    "++"
    "+="
    "&="
    "/="
    "**="
    "<<="
    "<"
    "<="
    "<<"
    "="
    "=="
    "==="
    "!="
    "!=="
    "=>"
    ">"
    ">="
    ">>"
    "||"
    "%"
    "%="
    "*"
    "**"
    ">>>"
    "&"
    "|"
    "^"
    "??"
    "*="
    ">>="
    ">>>="
    "^="
    "|="
    "&&="
    "||="
    "??="
  ] @operator
)

(switch_case
  ":" @punctuation.delimiter
)

(pair_pattern
  ":" @punctuation.delimiter
)

(pair
  ":" @punctuation.delimiter
)

(","
  @punctuation.delimiter
)

("."
  @punctuation.delimiter
)

(";"
  @punctuation.delimiter
)

("..."
  @punctuation.special
)

(
  (identifier) @number
  (#match? @number "^(NaN|Infinity)$")
)

(
  (number) @number
)

(regex
  "/" @punctuation.bracket
)

(
  (regex_pattern) @string.regex
)

(
  (escape_sequence) @string.escape
)

(
  (template_string) @string
)

(
  (string) @string @spell
)

(
  (string_fragment) @preproc
  (#eq? @preproc "use strict")
)

(
  (hash_bang_line) @preproc
)

(
  (comment) @comment.documentation
  (#match? @comment.documentation "^/[*][*][^*].*[*]/$")
)

(
  (comment) @comment @spell
)

(
  [
    (null)
    (undefined)
  ] @constant.builtin
)

(
  [
    (true)
    (false)
  ] @boolean
)

(
  [
    (this)
    (super)
  ] @variable.builtin
)

(namespace_import
  (identifier) @namespace
)

(new_expression
  constructor: (identifier) @constructor
)

(call_expression
  function: (member_expression
    property: [
      (property_identifier)
      (private_property_identifier)
    ] @method.call
  )
)

(call_expression
  function: (identifier) @function.call
)

(assignment_expression
  left: (identifier) @function
  right: (function)
)

(assignment_expression
  left: (identifier) @function
  right: (arrow_function)
)

(variable_declarator
  name: (identifier) @function
  value: (function)
)

(variable_declarator
  name: (identifier) @function
  value: (arrow_function)
)

(assignment_expression
  left: (member_expression
    property: (property_identifier) @method
  )
  right: (function)
)

(assignment_expression
  left: (member_expression
    property: (property_identifier) @method
  )
  right: (arrow_function)
)

(pair
  key: (property_identifier) @method
  value: (arrow_function)
)

(pair
  key: (property_identifier) @method
  value: (function)
)

(method_definition
  name: (property_identifier) @constructor
  (#eq? @constructor "constructor")
)

(method_definition
  name: [
    (property_identifier)
    (private_property_identifier)
  ] @method
)

(generator_function_declaration
  name: (identifier) @function
)

(generator_function
  name: (identifier) @function
)

(function_declaration
  name: (identifier) @function
)

(function
  name: (identifier) @function
)

(
  (identifier) @function.builtin
  (#match? @function.builtin "^(eval|isFinite|isNaN|parseFloat|parseInt|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|require)$")
)

(
  (identifier) @namespace.builtin
  (#eq? @namespace.builtin "Intl")
)

(
  (identifier) @type.builtin
  (#match? @type.builtin "^(Object|Function|Boolean|Symbol|Number|Math|Date|String|RegExp|Map|Set|WeakMap|WeakSet|Promise|Array|Int8Array|Uint8Array|Uint8ClampedArray|Int16Array|Uint16Array|Int32Array|Uint32Array|Float32Array|Float64Array|ArrayBuffer|DataView|Error|EvalError|InternalError|RangeError|ReferenceError|SyntaxError|TypeError|URIError)$")
)

(
  (identifier) @variable.builtin
  (#match? @variable.builtin "^(arguments|module|console|window|document)$")
)

(
  (shorthand_property_identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z[0-9]_]*$")
)

(
  (identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z[0-9]_]*$")
)

(
  (identifier) @type
  (#match? @type "^[A-Z]")
)

(
  (variable_declarator
    name: (object_pattern
      (shorthand_property_identifier_pattern)
    )
  ) @variable
)

(
  (private_property_identifier) @property
)

(
  (shorthand_property_identifier) @property
)

(
  (property_identifier) @property
)

(
  (identifier) @variable
)
