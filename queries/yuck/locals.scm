;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/yuck/locals.scm
;; Licensed under the Apache License 2.0
[
  (ast_block)
  (list)
  (array)
  (expr)
  (json_array)
  (json_object)
  (parenthesized_expression)
] @local.scope

(symbol) @local.reference

(keyword) @local.definition.field

(json_object
  (simplexpr
    (ident) @local.definition.field
  )
)

(ast_block
  (symbol)
  (ident) @local.definition.type
)
