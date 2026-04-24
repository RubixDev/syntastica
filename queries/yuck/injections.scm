;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/yuck/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(
  (function_call
    name: (ident) @_name
    .
    (simplexpr)
    .
    (simplexpr
      (string
        (string_fragment) @injection.content
      )+
    )
  )
  (#any-of? @_name "replace" "search" "matches" "captures")
  (#set! injection.language "regex")
  (#set! injection.combined)
)

(
  (function_call
    name: (ident) @_name
    .
    (simplexpr)
    .
    (simplexpr
      (string
        (string_fragment) @injection.content
      )+
    )
  )
  (#eq? @_name "jq")
  (#set! injection.language "jq")
  (#set! injection.combined)
)
