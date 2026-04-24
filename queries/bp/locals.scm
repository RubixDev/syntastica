;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/bp/locals.scm
;; Licensed under the Apache License 2.0
(module
  (property
    field: (identifier) @local.definition.parameter
  )
)

(map_expression
  (property
    field: (identifier) @local.definition.field
  )
)

(assignment
  left: (identifier) @local.definition.var
)

(pattern_binding
  binding: (identifier) @local.definition.var
)

(identifier) @local.reference
