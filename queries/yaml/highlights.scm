;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-yaml/7708026449bed86239b1cd5bce6e3c34dbca6415/queries/highlights.scm
(boolean_scalar) @boolean

(null_scalar) @constant.builtin

[
  (double_quote_scalar)
  (single_quote_scalar)
  (block_scalar)
  (string_scalar)
] @string

[
  (integer_scalar)
  (float_scalar)
] @number

(comment) @comment

[
  (anchor_name)
  (alias_name)
] @label

(tag) @type

[
  (yaml_directive)
  (tag_directive)
  (reserved_directive)
] @attribute

(block_mapping_pair
  key: (flow_node
    [
      (double_quote_scalar)
      (single_quote_scalar)
    ] @property
  )
)

(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @property
    )
  )
)

(flow_mapping
  (_
    key: (flow_node
      [
        (double_quote_scalar)
        (single_quote_scalar)
      ] @property
    )
  )
)

(flow_mapping
  (_
    key: (flow_node
      (plain_scalar
        (string_scalar) @property
      )
    )
  )
)

[
  ","
  "-"
  ":"
  ">"
  "?"
  "|"
] @punctuation.delimiter

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "*"
  "&"
  "---"
  "..."
] @punctuation.special
