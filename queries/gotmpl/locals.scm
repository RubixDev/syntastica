;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/gotmpl/locals.scm
;; Licensed under the Apache License 2.0
[
  (if_action)
  (range_action)
  (block_action)
  (with_action)
  (define_action)
] @local.scope

(variable_definition
  variable: (variable) @local.definition.var
)

(variable) @local.reference
