(ERROR) @error

[
  "{"
  ")"
  "("
  "}"
] @punctuation.bracket

[
  "#"
  ","
  "."
  ":"
  "::"
  ";"
] @punctuation.delimiter

[
  (integer_value)
  (float_value)
] @number

[
  (string_value)
  (color_value)
  (unit)
] @string

(
  (plain_value) @type
  (#match? @type "^--")
)

(plain_value) @constant.builtin

(
  (property_name) @type.definition
  (#match? @type.definition "^--")
)

(namespace_name) @namespace

[
  (class_name)
  (id_name)
  (property_name)
  (feature_name)
  (attribute_name)
] @property

(pseudo_class_selector
  (class_name) @property
)

(pseudo_element_selector
  "::"
  (tag_name) @property
)

(attribute_selector
  (plain_value) @string
)

(important) @type.qualifier

[
  "~"
  ">"
  "+"
  "-"
  "*"
  "/"
  "="
  "^="
  "|="
  "~="
  "$="
  "*="
  "and"
  "or"
  "not"
  "only"
] @operator

(function_name) @function

[
  (tag_name)
  (nesting_selector)
  (universal_selector)
] @type

(comment) @comment @spell

("@import"
  @include
)

[
  "@media"
  "@charset"
  "@namespace"
  "@supports"
  "@keyframes"
  (at_keyword)
  (to)
  (from)
] @keyword
