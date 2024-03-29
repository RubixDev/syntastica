;; Forked from https://github.com/helix-editor/helix/blob/master/runtime/queries/rust/injections.scm
;; Licensed under the Mozilla Public License 2.0
(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

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

; Highlight SQL in `sqlx::query!()`, `sqlx::query_scalar!()`, and `sqlx::query_scalar_unchecked!()`
(macro_invocation
  macro: (scoped_identifier
    path: (identifier) @_sqlx
    (#eq? @_sqlx "sqlx")
    name: (identifier) @_query
    (#match? @_query "^query(_scalar|_scalar_unchecked)?$")
  )
  (token_tree
    ; Only the first argument is SQL
    .
    [
      (string_literal)
      (raw_string_literal)
    ] @injection.content
  )
  (#set! injection.language "sql")
)

; Highlight SQL in `sqlx::query_as!()` and `sqlx::query_as_unchecked!()`
(macro_invocation
  macro: (scoped_identifier
    path: (identifier) @_sqlx
    (#eq? @_sqlx "sqlx")
    name: (identifier) @_query_as
    (#match? @_query_as "^query_as(_unchecked)?$")
  )
  (token_tree
    ; Only the second argument is SQL
    .
    ; Allow anything as the first argument in case the user has lower case type
    ; names for some reason
    (_)
    [
      (string_literal)
      (raw_string_literal)
    ] @injection.content
  )
  (#set! injection.language "sql")
)

; Highlight regex in macros from lazy_regex crate
(macro_invocation
  macro: (
    (identifier) @_regex_macro
    (#match? @_regex_macro "^(lazy_)?regex|regex_(captures|find|is_match|replace(_all)?)$")
  )
  (token_tree
    ; only the first argument is a regex
    [
      (line_comment)
      (block_comment)
    ]*
    .
    (raw_string_literal) @injection.content
  )
  (#set! injection.language "regex")
)
