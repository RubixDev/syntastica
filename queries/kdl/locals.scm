;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/kdl/locals.scm
;; Licensed under the Apache License 2.0
(document) @local.scope

(node
  (node_children) @local.scope
)

(node_children
  (node) @local.scope
)

(identifier) @local.reference

(node_field) @local.definition.field

(node
  (identifier) @local.definition.type
)

(type) @local.definition.type
