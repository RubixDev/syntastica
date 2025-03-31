;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/yaml/locals.scm
;; Licensed under the Apache License 2.0
[
  (stream)
  (document)
  (block_node)
] @local.scope

(anchor_name) @local.definition

(alias_name) @local.reference
