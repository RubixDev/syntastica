;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/glimmer/locals.scm
;; Licensed under the Apache License 2.0
[
  (element_node)
  (block_statement)
] @local.scope

(identifier) @local.reference

(block_params
  (identifier) @local.definition.var
)
