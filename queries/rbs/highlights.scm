;; Forked from https://raw.githubusercontent.com/joker1007/tree-sitter-rbs/5282e2f36d4109f5315c1d9486b5b0c2044622bb/queries/highlights.scm
; Use directive
(use_clause
  [
    (type_name)
    (simple_type_name)
  ] @type
)

; Buitin constants and Keywords
[
  "true"
  "false"
] @boolean

"nil" @constant.builtin

[
  "use"
  "as"
  "class"
  "module"
  "interface"
  "type"
  "def"
  "attr_reader"
  "attr_writer"
  "attr_accessor"
  "end"
  "alias"
] @keyword

"def" @keyword.function

; Members of declaration
[
  "include"
  "extend"
  "prepend"
] @function.method

(visibility) @type.qualifier

(comment) @comment @spell

(method_member
  (method_name
    [
      (identifier)
      (identifier_suffix)
      (constant)
      (constant_suffix)
      (operator)
      (setter)
      (constant_setter)
    ] @method
  )
)

(attribute_member
  (method_name
    [
      (identifier)
      (identifier_suffix)
      (constant)
      (constant_suffix)
      (operator)
      (setter)
      (constant_setter)
    ] @method
  )
)

[
  (ivar_name)
  (cvar_name)
] @property

(alias_member
  (method_name) @function
)

(class_name
  (constant) @type
)

(module_name
  (constant) @type
)

(interface_name
  (interface) @type
)

(alias_name
  (identifier) @type
)

(type_variable) @constant

(namespace
  (constant) @namespace
)

(builtin_type) @type.builtin

(const_name
  (constant) @constant
)

(global_name) @property

; Generics Keywords
[
  (generics_unchecked)
  (generics_variance)
] @keyword

; Standard Arguments
(parameter
  (var_name) @parameter
)

(unnamed_parameter) @parameter

; Keyword Arguments
(keyword) @parameter

; Self
(self) @variable.builtin

; Literal
(type
  (symbol_literal) @symbol
)

(type
  (string_literal
    (escape_sequence) @string.escape
  )
)

(type
  (string_literal) @string
)

(type
  (integer_literal) @number
)

(type
  (record_type
    key: (record_key) @symbol
  )
)

; Annotation
(annotation_text) @attribute

; Operators
[
  "="
  "->"
  "<"
  "**"
  "*"
  "&"
  "|"
  "^"
  "?"
  (rest_operator)
  (block_operator)
  (optional_block_operator)
] @operator

; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  "."
] @punctuation.delimiter

; RBS Inline prefix (keep comment color)
(inline_prefix) @comment

; RBS Inline syntax
(inline_class_annotation) @keyword

(inline_doc
  (var_name) @parameter
)

(inline_doc_comment) @comment

(inline_generic) @keyword

(inline_override) @keyword

(inline_skip) @keyword
