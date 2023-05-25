[
  (line_comment)
  (block_comment)
] @injection.content

(#set! injection.language "comment")

(
  (macro_invocation
    (token_tree) @injection.content
  )
  (#set! injection.language "rust")
  (#set! injection.include-children)
)

(
  (macro_rule
    (token_tree) @injection.content
  )
  (#set! injection.language "rust")
  (#set! injection.include-children)
)

(call_expression
  function: (scoped_identifier
    path: (identifier) @_regex
    (#eq? @_regex "Regex")
    name: (identifier) @_new
    (#eq? @_new "new")
  )
  arguments: (arguments
    (raw_string_literal) @injection.content
  )
  (#set! injection.language "regex")
)

(call_expression
  function: (scoped_identifier
    path: (scoped_identifier
      (identifier) @_regex
      (#eq? @_regex "Regex")
      .
    )
    name: (identifier) @_new
    (#eq? @_new "new")
  )
  arguments: (arguments
    (raw_string_literal) @injection.content
  )
  (#set! injection.language "regex")
)

(macro_invocation
  macro: (scoped_identifier
    path: (identifier) @_sqlx
    (#eq? @_sqlx "sqlx")
    name: (identifier) @_query
    (#eq? @_query "query")
  )
  (token_tree
    .
    [
      (string_literal)
      (raw_string_literal)
    ] @injection.content
  )
  (#set! injection.language "sql")
)

(macro_invocation
  macro: (scoped_identifier
    path: (identifier) @_sqlx
    (#eq? @_sqlx "sqlx")
    name: (identifier) @_query_as
    (#eq? @_query_as "query_as")
  )
  (token_tree
    .
    (_)
    [
      (string_literal)
      (raw_string_literal)
    ] @injection.content
  )
  (#set! injection.language "sql")
)

(macro_invocation
  macro: (scoped_identifier
    path: (identifier) @_sqlx
    (#eq? @_sqlx "sqlx")
    name: (identifier) @_query_as
    (#eq? @_query_as "query_unchecked")
  )
  (token_tree
    .
    [
      (string_literal)
      (raw_string_literal)
    ] @injection.content
  )
  (#set! injection.language "sql")
)

(macro_invocation
  macro: (scoped_identifier
    path: (identifier) @_sqlx
    (#eq? @_sqlx "sqlx")
    name: (identifier) @_query_as
    (#eq? @_query_as "query_as_unchecked")
  )
  (token_tree
    .
    (_)
    [
      (string_literal)
      (raw_string_literal)
    ] @injection.content
  )
  (#set! injection.language "sql")
)
