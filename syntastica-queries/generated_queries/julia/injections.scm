(
  (string_literal) @injection.content
  (#set! injection.language "markdown")
  .
  [
    (module_definition)
    (abstract_definition)
    (struct_definition)
    (function_definition)
    (assignment)
    (const_declaration)
  ]
  (#match? @injection.content "^\"\"\"")
  (#offset! @injection.content 0 3 0 -3)
)

(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

(
  (prefixed_string_literal
    prefix: (identifier) @_prefix
  ) @injection.content
  (#set! injection.language "regex")
  (#eq? @_prefix "r")
  (#offset! @injection.content 0 2 0 -1)
)
