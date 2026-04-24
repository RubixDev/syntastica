;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/query/locals.scm
;; Licensed under the Apache License 2.0
(program) @local.scope

(program
  (named_node) @local.scope
)

(program
  (anonymous_node) @local.scope
)

(program
  (grouping) @local.scope
)

(identifier) @local.reference

(named_node
  (capture) @local.definition.var
)

(anonymous_node
  (capture) @local.definition.var
)

(grouping
  (capture) @local.definition.var
)
