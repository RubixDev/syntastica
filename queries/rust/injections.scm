;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/rust/injections.scm
;; Licensed under the Apache License 2.0
(macro_invocation
  macro: [
    (scoped_identifier
      name: (_) @_macro_name
    )
    (identifier) @_macro_name
  ]
  (token_tree) @injection.content
  (#not-any-of? @_macro_name "slint" "html" "json")
  (#set! injection.language "rust")
  (#set! injection.include-children)
)

(macro_invocation
  macro: [
    (scoped_identifier
      name: (_) @injection.language
    )
    (identifier) @injection.language
  ]
  (token_tree) @injection.content
  (#any-of? @injection.language "slint" "html" "json")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.include-children)
)

(macro_definition
  (macro_rule
    left: (token_tree_pattern) @injection.content
    (#set! injection.language "rust")
  )
)

(macro_definition
  (macro_rule
    right: (token_tree) @injection.content
    (#set! injection.language "rust")
  )
)

(
  [
    (line_comment)
    (block_comment)
  ] @injection.content
  (#set! injection.language "comment")
)

(call_expression
  function: (scoped_identifier
    path: (identifier) @_regex
    (#any-of? @_regex "Regex" "RegexBuilder")
    name: (identifier) @_new
    (#eq? @_new "new")
  )
  arguments: (arguments
    (raw_string_literal
      (string_content) @injection.content
    )
  )
  (#set! injection.language "regex")
)

(call_expression
  function: (scoped_identifier
    path: (scoped_identifier
      (identifier) @_regex
      (#any-of? @_regex "Regex" "RegexBuilder")
      .
    )
    name: (identifier) @_new
    (#eq? @_new "new")
  )
  arguments: (arguments
    (raw_string_literal
      (string_content) @injection.content
    )
  )
  (#set! injection.language "regex")
)

(call_expression
  function: (scoped_identifier
    path: (identifier) @_regex
    (#any-of? @_regex "RegexSet" "RegexSetBuilder")
    name: (identifier) @_new
    (#eq? @_new "new")
  )
  arguments: (arguments
    (array_expression
      (raw_string_literal
        (string_content) @injection.content
      )
    )
  )
  (#set! injection.language "regex")
)

(call_expression
  function: (scoped_identifier
    path: (scoped_identifier
      (identifier) @_regex
      (#any-of? @_regex "RegexSet" "RegexSetBuilder")
      .
    )
    name: (identifier) @_new
    (#eq? @_new "new")
  )
  arguments: (arguments
    (array_expression
      (raw_string_literal
        (string_content) @injection.content
      )
    )
  )
  (#set! injection.language "regex")
)

(
  (block_comment) @injection.content
  (#match? @injection.content "/\\*!([a-zA-Z]+:)?re2c")
  (#set! injection.language "re2c")
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
    (raw_string_literal
      (string_content) @injection.content
    )
  )
  (#set! injection.language "regex")
)
