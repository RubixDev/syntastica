[
  "*"
  "&"
  "---"
  "..."
] @punctuation.special

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  "-"
  ":"
  ">"
  "?"
  "|"
] @punctuation.delimiter

(flow_mapping
  (_
    key: (flow_node
      (plain_scalar
        (string_scalar) @field
      )
    )
  )
)

(flow_mapping
  (_
    key: (flow_node
      [
        (double_quote_scalar)
        (single_quote_scalar)
      ] @field
    )
  )
)

(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @field
    )
  )
)

(block_mapping_pair
  key: (flow_node
    [
      (double_quote_scalar)
      (single_quote_scalar)
    ] @field
  )
)

[
  (yaml_directive)
  (tag_directive)
  (reserved_directive)
] @preproc

(ERROR) @error

(tag) @type

(alias_name) @type

(anchor_name) @type

(comment) @comment @spell

(float_scalar) @number

(integer_scalar) @number

(escape_sequence) @string.escape

(string_scalar) @string

(
  (block_scalar) @string
  (#set! "priority" 99)
)

(single_quote_scalar) @string

(double_quote_scalar) @string

(null_scalar) @constant.builtin

(boolean_scalar) @boolean
