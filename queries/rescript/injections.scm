;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/rescript/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(extension_expression
  (extension_identifier) @_name
  (#eq? @_name "re")
  (expression_statement
    (_) @injection.content
    (#set! injection.language "regex")
  )
)

(extension_expression
  (extension_identifier) @_name
  (#eq? @_name "raw")
  (expression_statement
    (_
      (_) @injection.content
      (#set! injection.language "javascript")
    )
  )
)

(extension_expression
  (extension_identifier) @_name
  (#eq? @_name "graphql")
  (expression_statement
    (_
      (_) @injection.content
      (#set! injection.language "graphql")
    )
  )
)

(extension_expression
  (extension_identifier) @_name
  (#eq? @_name "relay")
  (expression_statement
    (_
      (_) @injection.content
      (#set! injection.language "graphql")
    )
  )
)
