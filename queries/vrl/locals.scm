;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/vrl/locals.scm
;; Licensed under the Apache License 2.0
(closure_variables
  (ident) @local.definition.parameter
)

[
  (ident)
  (metadata)
] @local.reference

(query
  (event) @local.reference
)

[
  (block)
  (closure)
  (if_statement)
] @local.scope
